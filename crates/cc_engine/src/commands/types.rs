//! Types and components: change, duplicate, delete, create components.

use super::{edit, patch_value, patch_variant};
use crate::error::{EngineError, Result};
use crate::refs::{self, Ref};
use crate::registry::{Command, CommandKind};
use crate::session::Session;
use crate::t;
use cc_core::roofing::FramingSystem;
use cc_core::{ComponentDef, ComponentKind, ElementType, ElementTypeId, TypeData, expr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Parameter names valid and unique, sound reference planes, lines and
/// dimensions, the right kind of pieces (a title block: sheet pieces only,
/// within their limits; a detail component at least one), every piece
/// resolves. A component may have no pieces while it is shaped: it starts
/// empty, and `component_place` refuses it until it has one.
pub(crate) fn validate(def: &ComponentDef) -> Result<()> {
    for (i, p) in def.params.iter().enumerate() {
        if !expr::is_valid_name(&p.name) {
            return Err(EngineError::invalid(format!("invalid parameter name '{}'", p.name)));
        }
        if def.params[..i].iter().any(|q| q.name == p.name) {
            return Err(EngineError::invalid(format!("parameter '{}' is repeated", p.name)));
        }
    }
    let title_block = def.kind == cc_core::ComponentKind::TitleBlock;
    let two_d = def.kind.is_2d();
    let detail = def.kind == cc_core::ComponentKind::Detail;
    if detail && def.sheet.is_empty() {
        return Err(EngineError::invalid("the component has no pieces"));
    }
    if two_d && !def.items.is_empty() {
        return Err(EngineError::invalid("a title block or detail component has only 2D pieces"));
    }
    if !two_d && !def.sheet.is_empty() {
        return Err(EngineError::invalid("2D pieces go into title blocks and detail components only"));
    }
    if two_d && cc_core::title_block::repair_def(&mut def.clone()) > 0 {
        let what = if title_block { "a sheet piece" } else { "a 2D piece" };
        return Err(EngineError::invalid(format!("{what} is out of bounds (points, text length or picture)")));
    }
    if let Some(problem) = def.ref_problems().into_iter().next() {
        return Err(EngineError::invalid(problem));
    }
    // Doors and windows are checked in a wall 15 cm thick, and need their opening.
    if matches!(def.kind, cc_core::ComponentKind::Door | cc_core::ComponentKind::Window) && def.opening_size().is_none()
    {
        return Err(EngineError::invalid("a door or window needs an opening with a positive width and height"));
    }
    if def.kind == cc_core::ComponentKind::Niche && (def.opening_size().is_none() || def.opening_depth().is_none()) {
        return Err(EngineError::invalid("a niche needs an opening with a positive width, height and depth"));
    }
    let resolved = def.resolve_connectors(&Default::default());
    if let Some(c) = def.connectors.iter().enumerate().find(|(i, _)| !resolved.iter().any(|k| k.index == *i)) {
        return Err(EngineError::invalid(format!("connector '{}': invalid position or diameter", c.1.name)));
    }
    let (_, errors) = cc_geom::resolve_pieces(def, &Default::default(), &[(cc_core::component::WALL_THICKNESS, 0.15)]);
    // A script that fails at run time keeps its last pieces and is reported
    // where it is used (`component_script_set` refuses one that does not load).
    if let Some((i, e)) = errors.iter().find(|(_, e)| !matches!(e, cc_core::component::PartError::Script(_))) {
        let piece = def.items.get(*i).map_or(String::new(), |it| it.name.clone());
        return Err(EngineError::invalid(format!("piece '{piece}': {e}")));
    }
    Ok(())
}

/// A column or beam type's params patch with its `profile` (a name or id, as
/// `type_get` shows it) turned into the profile's id; null is refused.
fn profile_patch(p: &cc_core::Project, data: &TypeData, patch: &Value) -> Result<Value> {
    let mut patch = patch.clone();
    if !matches!(data, TypeData::Column { .. } | TypeData::Beam { .. }) {
        return Ok(patch);
    }
    let Some(slot) = patch.as_object_mut().and_then(|o| o.get_mut("profile")) else { return Ok(patch) };
    let r = match slot {
        Value::Null => {
            return Err(EngineError::invalid(
                "profile: a column or beam type must point to a profile; give another profile instead of null",
            ));
        }
        Value::String(n) => Ref::Name(n.clone()),
        Value::Number(n) => Ref::Id(n.as_u64().ok_or_else(|| EngineError::invalid("profile: not a profile id"))?),
        _ => return Err(EngineError::invalid("profile: a profile name or id")),
    };
    let id = refs::profile(p, &r).map_err(|e| EngineError::invalid(format!("profile: {}", e.message)))?;
    *slot = json!(id.0);
    Ok(patch)
}

pub(crate) fn type_out(s: &Session, id: ElementTypeId) -> Value {
    super::query::type_json(s, id).unwrap_or_else(|| json!({ "id": id.0 }))
}

/// Change a type: its name, its parameters (`params`, merged into what
/// `type_get` shows: layers, sizes, materials, a component's `def`…) and its
/// identity data (`info`: type mark, description, manufacturer, cost…).
/// A column or beam type takes `profile` (a structural profile's name or id;
/// null is refused: these types always point to a profile),
/// `profile_rotation_deg` and `profile_mirror`. `width`/`height` are the box of
/// the turned profile; changing them alone re-points the type to a profile of
/// that section, unturned and unmirrored.
/// Changing a component's definition updates every type of that component,
/// each keeping its own parameter values.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TypeSet {
    #[serde(rename = "type")]
    pub type_ref: Ref,
    /// New name (unique in its category).
    #[serde(default)]
    pub name: Option<String>,
    /// Fields of the type's parameters to change, shaped like `params` in `type_get`.
    #[serde(default)]
    pub params: Option<Value>,
    /// Fields of the identity data to change, shaped like `info` in `type_get`.
    #[serde(default)]
    pub info: Option<Value>,
    /// Component parameters renamed by a new `def`, as `[old, new]` pairs in
    /// order, so the component's other types keep their values.
    #[serde(default)]
    pub renames: Vec<[String; 2]>,
}

impl Command for TypeSet {
    const NAME: &'static str = "type_set";
    const KIND: CommandKind = CommandKind::Edit;
    fn run(self, s: &mut Session) -> Result<Value> {
        let p = s.project();
        let id = refs::element_type(p, &self.type_ref)?;
        let ty = p.element_type(id).ok_or_else(|| EngineError::not_found("type"))?.clone();
        if let Some(name) = &self.name
            && (name.trim().is_empty() || p.types_of(ty.category()).any(|t| t.id != id && t.name.trim() == name.trim()))
        {
            return Err(EngineError::invalid(format!("the name '{name}' is empty or already used")));
        }
        let params = self.params.as_ref().map(|patch| profile_patch(p, &ty.data, patch)).transpose()?;
        let mut data = params.as_ref().map(|patch| patch_variant(&ty.data, patch)).transpose()?;
        // A new roof covering brings the framing it usually lies on, if none was chosen.
        if let (
            Some(TypeData::Roof { covering: Some(kind), framing, .. }),
            TypeData::Roof { covering: old, framing: was, .. },
        ) = (&mut data, &ty.data)
            && Some(*kind) != *old
            && framing.system == FramingSystem::None
            && was.system == FramingSystem::None
        {
            framing.system = kind.spec().framing;
        }
        if data.as_ref().is_some_and(|d| std::mem::discriminant(d) != std::mem::discriminant(&ty.data)) {
            return Err(EngineError::invalid("params cannot change the type's category"));
        }
        let info = self.info.as_ref().map(|patch| patch_value(&ty.info, patch)).transpose()?;
        // A new component definition goes to every type of the component.
        let mut siblings = Vec::new();
        if let (
            Some(TypeData::Component { def: new_def, component: new_name }),
            TypeData::Component { component, def: old_def },
        ) = (&data, &ty.data)
            && (new_def != old_def || new_name != component)
        {
            if new_name.trim().is_empty() {
                return Err(EngineError::invalid("the component needs a name"));
            }
            if new_def != old_def {
                validate(new_def)?;
            }
            siblings = p
                .types
                .values()
                .filter(|t| t.id != id && matches!(&t.data, TypeData::Component { component: c, .. } if c == component))
                .map(|t| t.id)
                .collect();
        }
        let renames: Vec<(String, String)> = self.renames.iter().map(|[a, b]| (a.clone(), b.clone())).collect();
        let renamed_types: Vec<ElementTypeId> = if matches!(&data, Some(TypeData::Component { .. })) {
            siblings.iter().copied().chain([id]).collect()
        } else {
            Vec::new()
        };
        edit(s, t!("tx-modify-type"), |e| {
            super::sheets::rename_sheet_fields(e, &renamed_types, &renames);
            let new_def = match &data {
                Some(TypeData::Component { def, .. }) => Some(def.clone()),
                _ => None,
            };
            let new_component = match &data {
                Some(TypeData::Component { component, .. }) => Some(component.clone()),
                _ => None,
            };
            for sid in &siblings {
                let Some(TypeData::Component { component, def }) = e.type_mut(*sid).map(|t| &mut t.data) else {
                    continue;
                };
                let Some(mut d) = new_def.clone() else { continue };
                for param in &mut d.params {
                    // The name this parameter had before the renames.
                    let old = self
                        .renames
                        .iter()
                        .rev()
                        .fold(param.name.clone(), |n, [from, to]| if n == *to { from.clone() } else { n });
                    if let Some(v) = def.param(&old).map(|q| q.value) {
                        param.value = v;
                    }
                }
                *def = d;
                if let Some(c) = &new_component {
                    *component = c.clone();
                }
            }
            let t = e.type_mut(id).ok_or_else(|| EngineError::not_found("type"))?;
            if let Some(n) = self.name {
                t.name = n.trim().to_string();
            }
            if let Some(d) = data {
                t.data = d;
            }
            if let Some(i) = info {
                t.info = i;
            }
            Ok(())
        })?;
        Ok(type_out(s, id))
    }
}

/// Copy a type under a new name (default "<name> (copy)"); the given elements of the type switch to the copy.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TypeDuplicate {
    #[serde(rename = "type")]
    pub type_ref: Ref,
    /// Name of the copy (default: "<name> (copy)").
    #[serde(default)]
    pub name: Option<String>,
    /// Elements of the type to switch to the copy.
    #[serde(default)]
    pub elements: Vec<u64>,
}

impl Command for TypeDuplicate {
    const NAME: &'static str = "type_duplicate";
    const KIND: CommandKind = CommandKind::Edit;
    fn run(self, s: &mut Session) -> Result<Value> {
        let p = s.project();
        let tid = refs::element_type(p, &self.type_ref)?;
        let ty = p.element_type(tid).ok_or_else(|| EngineError::not_found("type"))?.clone();
        let elements = self.elements.iter().map(|e| refs::element(p, *e)).collect::<Result<Vec<_>>>()?;
        let name = self.name.unwrap_or_else(|| t!("copy-of", "name" => ty.name));
        let new_id = edit(s, t!("tx-duplicate-type"), |e| {
            let id = ElementTypeId(e.alloc_id());
            e.add_type(ElementType { id, name, data: ty.data.clone(), info: ty.info.clone(), uid: Default::default() });
            for el in elements {
                if let Some(x) = e.element_mut(el).filter(|x| x.type_id == tid) {
                    x.type_id = id;
                }
            }
            Ok(id)
        })?;
        Ok(type_out(s, new_id))
    }
}

/// Delete a type that no element uses (the last type of a category is kept,
/// except a title block: sheets on a deleted title block move to the first
/// other one, or to none).
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TypeDelete {
    #[serde(rename = "type")]
    pub type_ref: Ref,
}

impl Command for TypeDelete {
    const NAME: &'static str = "type_delete";
    const KIND: CommandKind = CommandKind::Edit;
    fn run(self, s: &mut Session) -> Result<Value> {
        let p = s.project();
        let id = refs::element_type(p, &self.type_ref)?;
        let category = p.element_type(id).map(|t| t.category()).ok_or_else(|| EngineError::not_found("type"))?;
        if p.elements.values().any(|e| e.type_id == id) {
            return Err(EngineError::rejected("elements use this type"));
        }
        let panel = |c: &cc_core::curtain::CurtainSystem| c.components().contains(&id);
        let curtains_use = p.types.values().filter_map(|t| t.curtain()).any(panel)
            || p.walls().any(|(_, w)| w.curtain.components().any(|c| c == id));
        if curtains_use {
            return Err(EngineError::rejected("curtain walls use this type as a panel"));
        }
        let host_use = p.elements.values().filter_map(|e| e.member_host()).any(|h| h.member_types().contains(&id));
        if host_use {
            return Err(EngineError::rejected(
                "member hosts (steel buildings, trusses) use this type for their members",
            ));
        }
        // Sheets may go without a title block, so the last one may go too.
        let title_block = category == cc_core::Category::TitleBlock;
        if p.types_of(category).count() <= 1 && !title_block {
            return Err(EngineError::rejected("the last type of a category cannot be deleted"));
        }
        let legends_use =
            p.views.values().any(|v| matches!(&v.kind, cc_core::ViewKind::Legend { types } if types.contains(&id)));
        // Sheets on it move to the first other title block, as repair does.
        let replacement = p.title_blocks().into_iter().find(|t| *t != id);
        let sheets: Vec<_> = p
            .views
            .values()
            .filter(|v| matches!(&v.kind, cc_core::ViewKind::Sheet { title_block: Some(t), .. } if *t == id))
            .map(|v| v.id)
            .collect();
        edit(s, t!("tx-delete-type"), |e| {
            for v in &sheets {
                if let Some(cc_core::ViewKind::Sheet { title_block, .. }) = e.view_mut(*v).map(|v| &mut v.kind) {
                    *title_block = replacement;
                }
            }
            if legends_use {
                let views: Vec<_> = e.project().views.keys().copied().collect();
                for v in views {
                    if let Some(cc_core::ViewKind::Legend { types }) = e.view_mut(v).map(|v| &mut v.kind) {
                        types.retain(|t| *t != id);
                    }
                }
            }
            e.remove_type(id);
            Ok(())
        })?;
        Ok(json!({ "deleted": id.0 }))
    }
}

/// Create a component type: `def` holds its parameters and pieces, shaped
/// like a component type's `params.def` in `type_get`. A piece's `shape` is
/// a `Box` {x, y, z, width, depth, height}, a `Cylinder` {x, y, z, radius,
/// height}, an `Extrusion` {points: [[x, y], …], z, height} (a plan outline
/// extruded up), a `Profile` {plane: `Plan`, `Front` or `Side`, points:
/// [{u, v, radius}, …], offset, depth} (lines and arcs drawn on a plane:
/// u, v are x, y in plan, x, z from the front and y, z from the side; the
/// plane stands at `offset` on the remaining axis and extrudes `depth` along
/// it; a point's optional `radius` makes the segment to the next point an
/// arc, positive bulging outwards) or a `Revolution` {plane, points, offset, axis, angle}
/// (the profile turned about the line u = axis of its plane). Every field
/// is an expression over the parameters (meters, degrees). A piece with
/// `void: true` is cut out of the solid pieces it overlaps; one with
/// `plan_only: true` is a plan symbol (a door's swing), drawn in plans and
/// not a solid. A `kind` of
/// `Door` or `Window` with an `opening` {width, height, sill, own_symbol,
/// operation, door_operation} (expressions, then a window's `operation`:
/// `Fixed`, `SideHungLeft`, `SideHungRight` or `DoubleSideHung`, drawn as
/// swings in plans and written as IFC panel properties, and a door's
/// `door_operation`: `SingleSwingLeft` (default), `SingleSwingRight` or
/// `DoubleDoor`, one swing per leaf in plans and the IFC operation type;
/// `door_type_create` makes the built-in ones) makes a door or window type,
/// placed with `door_create` or `window_create`: it cuts that opening in its
/// wall; its frame has x along the wall from the opening's center, y across
/// it (towards the side a door or window opens to), z up from the sill, and
/// `WallThickness` is the wall's thickness.
/// `connectors` [{name, system: `ColdWater`, `HotWater`, `Sanitary`, `Vent`,
/// `Rainwater`, `Power` or `Lighting`, x, y, z, direction: `Down`, `Up`,
/// `Front` (-y), `Back` (+y), `Left` (-x) or `Right` (+x), diameter}] are
/// where `mep_connect` joins pipes and conduits (a toilet's inlet and outlet).
/// With a `kind` in place of `def` the component starts from that kind's
/// own parameters and pieces, to be shaped in the component environment or
/// with the `component_*` commands.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ComponentCreate {
    /// Name of the component and of its first type.
    pub name: String,
    /// The definition: `params` (name, value, kind, instance) and `items` (pieces), shaped like a component type's `params.def` in `type_get`. Left out, the component starts from its `kind`.
    #[serde(default)]
    pub def: Value,
    /// The kind a component made without a `def` starts from: `Furniture`, `Casework`, `Plumbing` and `Electrical` (a box sized by width, depth and height parameters), `Lighting` (a cylinder sized by diameter and height), `Door` (width and height) and `Window` (width, height and sill), which cut their opening in a wall, `Planting` (an empty tree or plant component to shape), or `Generic` (the default: a fixed box and no parameters, which are yours to add). Not given together with `def`, whose own `kind` says it.
    #[serde(default)]
    pub kind: Option<String>,
}

/// A component kind by its name in the model (`Furniture`), in any case.
fn component_kind(text: &str) -> Result<ComponentKind> {
    ComponentKind::ALL
        .into_iter()
        .find(|k| format!("{k:?}").eq_ignore_ascii_case(text.trim()))
        .ok_or_else(|| EngineError::invalid(format!("unknown component kind '{text}'")))
}

/// The definition a new component of `kind` starts from: the kind's own
/// parameters, named in the current language, and pieces sized by them. A
/// generic component has no parameters (they are the user's to add) and one
/// fixed box, since a component needs a piece. `None` for the 2D kinds,
/// which are made by `title_block_create` and `detail_component_create`.
pub fn starter_def(kind: ComponentKind) -> Option<ComponentDef> {
    use cc_core::{ComponentItem, ComponentParam, ComponentShape, ParamKind};
    let tr = |key: &str| crate::i18n::tr(key);
    let param = |name: &str, value: f64| ComponentParam {
        name: name.to_string(),
        value,
        kind: ParamKind::Length,
        instance: false,
        ..Default::default()
    };
    let item = |shape| ComponentItem {
        name: tr("tpl-part-body"),
        shape,
        material: String::new(),
        in_plan: true,
        void: false,
        plan_only: false,
        detail: Default::default(),
    };
    // A box standing on the origin, centered in plan.
    let boxed = |[w, d, h]: [&str; 3]| ComponentShape::Box {
        x: format!("-{w} / 2"),
        y: format!("-{d} / 2"),
        z: "0".into(),
        width: w.to_string(),
        depth: d.to_string(),
        height: h.to_string(),
    };
    let sized = |[width, depth, height]: [f64; 3]| {
        let (w, d, h) = (tr("tpl-param-width"), tr("tpl-param-depth"), tr("tpl-param-height"));
        ComponentDef {
            kind,
            params: vec![param(&w, width), param(&d, depth), param(&h, height)],
            items: vec![item(boxed([&w, &d, &h]))],
            ..Default::default()
        }
    };
    let names = || cc_core::openings::OpeningNames::new(&tr);
    Some(match kind {
        ComponentKind::Furniture => sized([1.2, 0.6, 0.75]),
        ComponentKind::Casework => sized([0.6, 0.6, 0.9]),
        ComponentKind::Plumbing => sized([0.4, 0.5, 0.4]),
        ComponentKind::Electrical => sized([0.08, 0.04, 0.12]),
        ComponentKind::Lighting => {
            // The property label, where it can name a parameter.
            let d = Some(tr("prop-diameter")).filter(|n| expr::is_valid_name(n)).unwrap_or_else(|| "Diameter".into());
            let h = tr("tpl-param-height");
            let shape = ComponentShape::Cylinder {
                x: "0".into(),
                y: "0".into(),
                z: "0".into(),
                radius: format!("{d} / 2"),
                height: h.clone(),
            };
            ComponentDef {
                kind,
                params: vec![param(&d, 0.3), param(&h, 0.1)],
                items: vec![item(shape)],
                ..Default::default()
            }
        }
        // Empty: its pieces are the user's to draw.
        ComponentKind::Generic | ComponentKind::Planting => ComponentDef { kind, ..Default::default() },
        ComponentKind::Door => cc_core::openings::door_def(&names(), 0.8, 2.1, ""),
        ComponentKind::Window => cc_core::openings::window_def(&names(), 1.2, 1.2, 1.0, ""),
        ComponentKind::Niche => cc_core::openings::niche_def(&names(), (0.6, 0.6, 0.1, 0.9)),
        ComponentKind::TitleBlock | ComponentKind::Detail => return None,
    })
}

impl Command for ComponentCreate {
    const NAME: &'static str = "component_create";
    const KIND: CommandKind = CommandKind::Edit;
    fn run(self, s: &mut Session) -> Result<Value> {
        let name = self.name.trim().to_string();
        if name.is_empty() {
            return Err(EngineError::invalid("the component needs a name"));
        }
        let mut def: ComponentDef = match (self.def, self.kind) {
            (Value::Null, kind) => {
                let kind = kind.as_deref().map_or(Ok(ComponentKind::Generic), component_kind)?;
                starter_def(kind)
                    .ok_or_else(|| EngineError::invalid("title blocks and detail components have their own commands"))?
            }
            (_, Some(_)) => return Err(EngineError::invalid("give either `def` or `kind`, not both")),
            (def, None) => {
                serde_json::from_value(def).map_err(|e| EngineError::invalid(format!("invalid def: {e}")))?
            }
        };
        // A script piece's declarations become the parameters and hotspots.
        let sources: Vec<String> = def
            .items
            .iter()
            .filter_map(|i| match &i.shape {
                cc_core::ComponentShape::Script { source } => Some(source.clone()),
                _ => None,
            })
            .collect();
        if sources.len() > 1 {
            return Err(EngineError::invalid("a component holds one script piece"));
        }
        if let Some(source) = sources.first() {
            let decl = cc_script::declare(source).map_err(|e| EngineError::invalid(format!("script: {e}")))?;
            cc_script::object::sync(&mut def.params, &mut def.hotspots, &decl).map_err(EngineError::invalid)?;
        }
        validate(&def)?;
        let id = edit(s, t!("tx-new-component"), |e| {
            let id = ElementTypeId(e.alloc_id());
            let data = TypeData::Component { component: name.clone(), def };
            e.add_type(ElementType { id, name: name.clone(), data, info: Default::default(), uid: Default::default() });
            Ok(id)
        })?;
        Ok(type_out(s, id))
    }
}
