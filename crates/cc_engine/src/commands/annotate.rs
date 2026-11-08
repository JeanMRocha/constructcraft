//! Dimensions: place (associated with what they measure) and drive by value.

use super::{add_element, created, edit};
use crate::error::{EngineError, Result};
use crate::refs::{self, Point, Ref, finite, point};
use crate::registry::{Command, CommandKind};
use crate::session::Session;
use crate::t;
use cc_core::{Category, Dimension, DimensionKind, ElementKind, Leader, LeaderSymbol, TextNote};
use glam::DVec2;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Place a dimension in a 2D view (points in the view's drawing coordinates:
/// plan X/Y in plans; horizontal position and height in elevations and
/// sections). Its ends follow the elements (or level lines) their points
/// lie on, so it keeps measuring them. Aligned dimensions measure from `a`
/// to `b`, linear ones along a horizontal or vertical `direction`, ordinate
/// ones from the origin `a` to `b` along that direction (a running
/// dimension; see `dimension_ordinate_create`), angular ones the angle at
/// `vertex` between the rays through `a` and `b`, radial and diameter ones
/// the arc centered at `a` through `b`.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DimensionCreate {
    /// View id or name.
    pub view: Ref,
    /// First point (center for radial and diameter).
    pub a: Point,
    /// Second point (a point on the arc for radial and diameter).
    pub b: Point,
    /// `aligned` (default), `linear`, `ordinate`, `angular`, `radial` or `diameter`.
    #[serde(default)]
    pub kind: Option<String>,
    /// Distance of the dimension line from `a` (left of a→b positive), or the arc radius for angular ones (default 1).
    #[serde(default)]
    pub offset: Option<f64>,
    /// Vertex of an angular dimension.
    #[serde(default)]
    pub vertex: Option<Point>,
    /// Linear and ordinate dimensions: `horizontal` or `vertical` (default: the larger extent).
    #[serde(default)]
    pub direction: Option<String>,
    /// Dimension type id or name (default: the default dimension type).
    #[serde(default, rename = "type")]
    pub type_ref: Option<Ref>,
}

impl Command for DimensionCreate {
    const NAME: &'static str = "dimension_create";
    const KIND: CommandKind = CommandKind::Edit;
    fn run(self, s: &mut Session) -> Result<Value> {
        let p = s.project();
        let view_id = refs::view(p, &self.view)?;
        let view = p.view(view_id).ok_or_else(|| EngineError::not_found("view"))?;
        if !view.takes_dimensions() {
            return Err(EngineError::invalid(
                "dimensions go in plans, elevations, sections, legends and drafting views",
            ));
        }
        let level = view
            .level()
            .or_else(|| p.levels.first().map(|l| l.id))
            .ok_or_else(|| EngineError::rejected("no levels"))?;
        let (a, b) = (point(self.a)?, point(self.b)?);
        let offset = finite(self.offset.unwrap_or(1.0), "offset")?;
        let kind = match self.kind.as_deref().unwrap_or("aligned") {
            "aligned" => DimensionKind::Aligned,
            kind @ ("linear" | "ordinate") => {
                let horizontal = match self.direction.as_deref() {
                    Some("horizontal") => true,
                    Some("vertical") => false,
                    None => (b - a).x.abs() >= (b - a).y.abs(),
                    Some(other) => return Err(EngineError::invalid(format!("unknown direction '{other}'"))),
                };
                let dir = if horizontal { DVec2::X } else { DVec2::Y };
                if kind == "linear" { DimensionKind::Linear { dir } } else { DimensionKind::Ordinate { dir } }
            }
            "angular" => {
                let vertex =
                    point(self.vertex.ok_or_else(|| EngineError::invalid("an angular dimension needs a vertex"))?)?;
                DimensionKind::Angular { vertex, rays: Default::default() }
            }
            "radial" => DimensionKind::Radial,
            "diameter" => DimensionKind::Diameter,
            other => return Err(EngineError::invalid(format!("unknown dimension kind '{other}'"))),
        };
        let dim = Dimension { view: view_id, a, b, offset, kind, refs: Default::default() };
        if dim.value().is_nan() || dim.value() <= 1e-6 {
            return Err(EngineError::invalid(t!("msg-dimension-zero")));
        }
        let type_id = refs::type_of(p, Category::Dimension, self.type_ref.as_ref())?;
        let drawing = s.drawing(view_id).unwrap_or_default();
        let dim = crate::dimensions::associate(s.project(), &drawing, &dim);
        let id = edit(s, t!("tx-create-dimension"), |e| {
            Ok(add_element(e, level, type_id, ElementKind::Dimension(Box::new(dim))))
        })?;
        Ok(created(s, id))
    }
}

/// Place a chain of dimensions on one dimension line (continuous
/// dimensioning): one dimension from each point to the next, all measured
/// along the same direction and drawn on the same line, in one undo step.
/// Points are in the view's drawing coordinates, as for `dimension_create`,
/// and each end follows the element (or level line) it lies on. In an
/// aligned chain, segments along the direction are aligned dimensions and
/// the others measure along that direction.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DimensionChainCreate {
    /// View id or name.
    pub view: Ref,
    /// The measured points in order (at least 2, at most 500).
    pub points: Vec<Point>,
    /// `aligned` (default: along the first two points) or `linear` (horizontal or vertical).
    #[serde(default)]
    pub kind: Option<String>,
    /// Distance of the dimension line from the first point, positive to the left of the direction (default 1).
    #[serde(default)]
    pub offset: Option<f64>,
    /// Linear chains: `horizontal` or `vertical` (default: the larger extent of the first two points).
    #[serde(default)]
    pub direction: Option<String>,
    /// Direction to measure along, degrees counter-clockwise from the X axis (overrides `direction` and the
    /// direction of the first two points).
    #[serde(default)]
    pub angle: Option<f64>,
    /// Dimension type id or name (default: the default dimension type).
    #[serde(default, rename = "type")]
    pub type_ref: Option<Ref>,
}

/// Most points of a dimension chain.
const MAX_CHAIN_POINTS: usize = 500;

impl Command for DimensionChainCreate {
    const NAME: &'static str = "dimension_chain_create";
    const KIND: CommandKind = CommandKind::Edit;
    fn run(self, s: &mut Session) -> Result<Value> {
        let p = s.project();
        let view_id = refs::view(p, &self.view)?;
        let view = p.view(view_id).ok_or_else(|| EngineError::not_found("view"))?;
        if !view.takes_dimensions() {
            return Err(EngineError::invalid(
                "dimensions go in plans, elevations, sections, legends and drafting views",
            ));
        }
        if self.points.len() < 2 || self.points.len() > MAX_CHAIN_POINTS {
            return Err(EngineError::invalid(format!("a dimension chain needs 2 to {MAX_CHAIN_POINTS} points")));
        }
        let level = view
            .level()
            .or_else(|| p.levels.first().map(|l| l.id))
            .ok_or_else(|| EngineError::rejected("no levels"))?;
        let points = self.points.iter().map(|q| point(*q)).collect::<Result<Vec<_>>>()?;
        let (first, second) = match points.as_slice() {
            [a, b, ..] => (*a, *b),
            _ => return Err(EngineError::invalid("a dimension chain needs at least 2 points")),
        };
        let offset = finite(self.offset.unwrap_or(1.0), "offset")?;
        let aligned = match self.kind.as_deref().unwrap_or("aligned") {
            "aligned" => true,
            "linear" => false,
            other => return Err(EngineError::invalid(format!("unknown chain kind '{other}' (aligned or linear)"))),
        };
        let dir = match (self.angle, self.direction.as_deref()) {
            (Some(angle), _) => axis_clean(DVec2::from_angle(finite(angle, "angle")?.to_radians())),
            (None, Some("horizontal")) => DVec2::X,
            (None, Some("vertical")) => DVec2::Y,
            (None, Some(other)) => return Err(EngineError::invalid(format!("unknown direction '{other}'"))),
            (None, None) if aligned => (second - first).try_normalize().unwrap_or(DVec2::ZERO),
            (None, None) => {
                let d = second - first;
                if d.x.abs() >= d.y.abs() { DVec2::X } else { DVec2::Y }
            }
        };
        let dims = cc_core::dim_line::chain(view_id, &points, dir, offset, aligned)
            .ok_or_else(|| EngineError::invalid(t!("msg-dimension-zero")))?;
        let type_id = refs::type_of(p, Category::Dimension, self.type_ref.as_ref())?;
        let drawing = s.drawing(view_id).unwrap_or_default();
        let dims: Vec<Dimension> =
            dims.iter().map(|d| crate::dimensions::associate(s.project(), &drawing, d)).collect();
        let ids = edit(s, t!("tx-create-dimension"), |e| {
            Ok(dims
                .into_iter()
                .map(|d| add_element(e, level, type_id, ElementKind::Dimension(Box::new(d))))
                .collect::<Vec<_>>())
        })?;
        Ok(serde_json::json!({ "ids": super::ids_json(&ids), "count": ids.len() }))
    }
}

/// Place running (ordinate) dimensions on one dimension line: one dimension
/// from the origin (the first point) to each other point, measured along
/// the same direction, each showing its running total from the origin, in
/// one undo step. Points are in the view's drawing coordinates, as for
/// `dimension_create`, and each end follows the element (or level line) it
/// lies on, so the totals follow the model; giving one a new value with
/// `dimension_set_value` moves what its point measures, keeping the origin.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DimensionOrdinateCreate {
    /// View id or name.
    pub view: Ref,
    /// The origin, then the measured points (at least 2 in all, at most 500).
    pub points: Vec<Point>,
    /// Distance of the dimension line from the origin, positive to the left of the direction (default 1).
    #[serde(default)]
    pub offset: Option<f64>,
    /// `horizontal` or `vertical` (default: along the origin and the second point).
    #[serde(default)]
    pub direction: Option<String>,
    /// Direction to measure along, degrees counter-clockwise from the X axis (overrides `direction`).
    #[serde(default)]
    pub angle: Option<f64>,
    /// Dimension type id or name (default: the default dimension type).
    #[serde(default, rename = "type")]
    pub type_ref: Option<Ref>,
}

impl Command for DimensionOrdinateCreate {
    const NAME: &'static str = "dimension_ordinate_create";
    const KIND: CommandKind = CommandKind::Edit;
    fn run(self, s: &mut Session) -> Result<Value> {
        let p = s.project();
        let view_id = refs::view(p, &self.view)?;
        let view = p.view(view_id).ok_or_else(|| EngineError::not_found("view"))?;
        if !view.takes_dimensions() {
            return Err(EngineError::invalid(
                "dimensions go in plans, elevations, sections, legends and drafting views",
            ));
        }
        if self.points.len() < 2 || self.points.len() > MAX_CHAIN_POINTS {
            return Err(EngineError::invalid(format!("ordinate dimensions need 2 to {MAX_CHAIN_POINTS} points")));
        }
        let level = view
            .level()
            .or_else(|| p.levels.first().map(|l| l.id))
            .ok_or_else(|| EngineError::rejected("no levels"))?;
        let points = self.points.iter().map(|q| point(*q)).collect::<Result<Vec<_>>>()?;
        let (first, second) = match points.as_slice() {
            [a, b, ..] => (*a, *b),
            _ => return Err(EngineError::invalid("ordinate dimensions need at least 2 points")),
        };
        let offset = finite(self.offset.unwrap_or(1.0), "offset")?;
        let dir = match (self.angle, self.direction.as_deref()) {
            (Some(angle), _) => axis_clean(DVec2::from_angle(finite(angle, "angle")?.to_radians())),
            (None, Some("horizontal")) => DVec2::X,
            (None, Some("vertical")) => DVec2::Y,
            (None, Some(other)) => return Err(EngineError::invalid(format!("unknown direction '{other}'"))),
            (None, None) => {
                let d = second - first;
                if d.x.abs() >= d.y.abs() { DVec2::X } else { DVec2::Y }
            }
        };
        let dims = cc_core::dim_line::ordinate(view_id, &points, dir, offset)
            .ok_or_else(|| EngineError::invalid(t!("msg-dimension-zero")))?;
        let type_id = refs::type_of(p, Category::Dimension, self.type_ref.as_ref())?;
        let drawing = s.drawing(view_id).unwrap_or_default();
        let dims: Vec<Dimension> =
            dims.iter().map(|d| crate::dimensions::associate(s.project(), &drawing, d)).collect();
        let ids = edit(s, t!("tx-create-dimension"), |e| {
            Ok(dims
                .into_iter()
                .map(|d| add_element(e, level, type_id, ElementKind::Dimension(Box::new(d))))
                .collect::<Vec<_>>())
        })?;
        Ok(serde_json::json!({ "ids": super::ids_json(&ids), "count": ids.len() }))
    }
}

/// `d` with components within 1e-12 of zero set to zero, so that 0°, 90°,
/// 180° and 270° give exact axes.
fn axis_clean(d: DVec2) -> DVec2 {
    let clean = |v: f64| if v.abs() < 1e-12 { 0.0 } else { v };
    DVec2::new(clean(d.x), clean(d.y)).normalize_or_zero()
}

/// Give a dimension a new value (meters; degrees for angular dimensions):
/// in plans what its moving end measures moves or stretches; between level
/// lines in elevations and sections the upper level moves. Radial and
/// diameter dimensions are read-only.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DimensionSetValue {
    /// Dimension element id.
    pub id: u64,
    /// New value: meters, or degrees for angular dimensions.
    pub value: f64,
}

impl Command for DimensionSetValue {
    const NAME: &'static str = "dimension_set_value";
    const KIND: CommandKind = CommandKind::Edit;
    fn run(self, s: &mut Session) -> Result<Value> {
        let id = refs::element(s.project(), self.id)?;
        let angular = matches!(
            s.project().element(id).map(|e| &e.kind),
            Some(ElementKind::Dimension(d)) if matches!(d.kind, DimensionKind::Angular { .. })
        );
        let value = if angular { self.value.to_radians() } else { self.value };
        crate::dimensions::set_value(s, id, value)?;
        Ok(created(s, id))
    }
}

/// A leader of a new text note.
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct LeaderArgs {
    /// The point the leader points at, where its symbol is drawn.
    pub end: Point,
    /// A bend point that splits the leader in two segments (text → elbow → end).
    #[serde(default)]
    pub elbow: Option<Point>,
    /// End symbol: `Arrow`, `OpenArrow`, `Triangle`, `Dot`, `Circle`, `Square`, `FilledSquare`, `Tick`
    /// or `None` (default: the text type's symbol).
    #[serde(default)]
    pub symbol: Option<String>,
}

fn symbol(name: Option<&str>, default: LeaderSymbol) -> Result<LeaderSymbol> {
    match name {
        None => Ok(default),
        Some(n) => serde_json::from_value(Value::String(n.to_string()))
            .map_err(|_| EngineError::invalid(format!("unknown leader symbol '{n}'"))),
    }
}

fn leader(args: &LeaderArgs, default: LeaderSymbol) -> Result<Leader> {
    Ok(Leader {
        end: point(args.end)?,
        elbow: args.elbow.map(point).transpose()?,
        symbol: symbol(args.symbol.as_deref(), default)?,
    })
}

/// The leader symbol of a text type.
fn type_symbol(p: &cc_core::Project, type_id: cc_core::ElementTypeId) -> LeaderSymbol {
    match p.element_type(type_id).map(|t| &t.data) {
        Some(cc_core::TypeData::Text { symbol, .. }) => *symbol,
        _ => LeaderSymbol::Arrow,
    }
}

/// Place a text note in a 2D view (points in the view's drawing coordinates,
/// as for dimensions). Without `width` it is single-line text (only line
/// breaks in `text` break it); with `width` (paper millimeters) it is
/// multiline text wrapped to that width. Leaders point from the text to
/// what it describes, each with one segment or two (with an `elbow`).
#[derive(Debug, Deserialize, Serialize, JsonSchema, Default)]
#[serde(deny_unknown_fields)]
pub struct TextCreate {
    /// View id or name.
    pub view: Ref,
    /// Top left corner of the text.
    pub position: Point,
    /// The text; a line break starts a new line.
    pub text: String,
    /// Wrapping width in paper millimeters (multiline text).
    #[serde(default)]
    pub width: Option<f64>,
    /// Rotation in degrees counter-clockwise.
    #[serde(default)]
    pub rotation: Option<f64>,
    /// Text type id or name (default: the default text type).
    #[serde(default, rename = "type")]
    pub type_ref: Option<Ref>,
    /// Leaders from the text.
    #[serde(default)]
    pub leaders: Vec<LeaderArgs>,
}

/// The view and its level (or the lowest) for an annotation placed in `view`.
fn annotation_view(p: &cc_core::Project, r: &Ref) -> Result<(cc_core::ViewId, cc_core::LevelId)> {
    let id = refs::view(p, r)?;
    let view = p.view(id).ok_or_else(|| EngineError::not_found("view"))?;
    if matches!(view.kind, cc_core::ViewKind::Schedule { .. } | cc_core::ViewKind::Sheet { .. }) {
        return Err(EngineError::invalid(
            "text goes in plans, elevations, sections, 3D views, legends and drafting views",
        ));
    }
    let level =
        view.level().or_else(|| p.levels.first().map(|l| l.id)).ok_or_else(|| EngineError::rejected("no levels"))?;
    Ok((id, level))
}

impl Command for TextCreate {
    const NAME: &'static str = "text_create";
    const KIND: CommandKind = CommandKind::Edit;
    fn run(self, s: &mut Session) -> Result<Value> {
        let p = s.project();
        let (view, level) = annotation_view(p, &self.view)?;
        if self.text.trim().is_empty() {
            return Err(EngineError::invalid(t!("msg-text-empty")));
        }
        let width = self.width.map(|w| refs::positive(w, "width")).transpose()?;
        let type_id = refs::type_of(p, Category::Text, self.type_ref.as_ref())?;
        let default = type_symbol(p, type_id);
        let leaders = self.leaders.iter().map(|l| leader(l, default)).collect::<Result<Vec<_>>>()?;
        let note = TextNote {
            view,
            position: point(self.position)?,
            text: self.text,
            size_mm: None,
            font_family: String::new(),
            alignment: cc_core::TextAlignment::Left,
            bold: false,
            italic: false,
            underline: false,
            width,
            rotation_deg: finite(self.rotation.unwrap_or(0.0), "rotation")?,
            leaders,
        };
        let kind = ElementKind::Text(Box::new(note));
        let id = edit(s, t!("tx-create-text"), |e| Ok(add_element(e, level, type_id, kind)))?;
        Ok(created(s, id))
    }
}

/// Add a leader to a text note.
#[derive(Debug, Deserialize, Serialize, JsonSchema, Default)]
pub struct TextAddLeader {
    /// Text element id.
    pub id: u64,
    #[serde(flatten)]
    pub leader: LeaderArgs,
}

impl Command for TextAddLeader {
    const NAME: &'static str = "text_add_leader";
    const KIND: CommandKind = CommandKind::Edit;
    fn run(self, s: &mut Session) -> Result<Value> {
        let p = s.project();
        let id = refs::element(p, self.id)?;
        let type_id = p.element(id).map(|e| e.type_id).ok_or_else(|| EngineError::not_found("element"))?;
        let new = leader(&self.leader, type_symbol(p, type_id))?;
        edit(s, t!("tx-add-leader"), |e| match e.element_mut(id).map(|x| &mut x.kind) {
            Some(ElementKind::Text(note)) => {
                note.leaders.push(new);
                Ok(())
            }
            _ => Err(EngineError::invalid(format!("element {} is not a text", id.0))),
        })?;
        Ok(created(s, id))
    }
}

/// Remove a leader from a text note.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TextRemoveLeader {
    /// Text element id.
    pub id: u64,
    /// Index of the leader (from 0, as in `params.leaders`).
    pub index: usize,
}

impl Command for TextRemoveLeader {
    const NAME: &'static str = "text_remove_leader";
    const KIND: CommandKind = CommandKind::Edit;
    fn run(self, s: &mut Session) -> Result<Value> {
        let id = refs::element(s.project(), self.id)?;
        edit(s, t!("tx-remove-leader"), |e| match e.element_mut(id).map(|x| &mut x.kind) {
            Some(ElementKind::Text(note)) if self.index < note.leaders.len() => {
                note.leaders.remove(self.index);
                Ok(())
            }
            Some(ElementKind::Text(_)) => Err(EngineError::not_found(format!("no leader {}", self.index))),
            _ => Err(EngineError::invalid(format!("element {} is not a text", id.0))),
        })?;
        Ok(created(s, id))
    }
}

/// Move the tag of an element in a plan (the tag of a door, window, beam,
/// column or footing): by `delta` from where it is, to `offset` from its
/// default spot, or back there with `reset`. A tag moved away from its
/// element gets a leader to it. Each plan keeps its own tag positions.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TagMove {
    /// Plan id or name.
    pub view: Ref,
    /// Id of the element the tag names.
    pub id: u64,
    /// Move by `[dx, dy]` meters in plan.
    #[serde(default)]
    pub delta: Option<Point>,
    /// Place the tag `[dx, dy]` meters from its default spot.
    #[serde(default)]
    pub offset: Option<Point>,
    /// Put the tag back in its default spot.
    #[serde(default)]
    pub reset: bool,
}

/// Farthest a tag may be moved from its default spot, meters.
const MAX_TAG_OFFSET: f64 = 1000.0;

impl Command for TagMove {
    const NAME: &'static str = "tag_move";
    const KIND: CommandKind = CommandKind::Edit;
    fn run(self, s: &mut Session) -> Result<Value> {
        let p = s.project();
        let view = refs::view(p, &self.view)?;
        if p.view(view).and_then(|v| v.level()).is_none() {
            return Err(EngineError::invalid("tags are moved in floor and ceiling plans"));
        }
        let id = refs::element(p, self.id)?;
        let current = p.view(view).and_then(|v| v.tag_offsets.get(&id).copied()).unwrap_or(DVec2::ZERO);
        let new = match (self.delta, self.offset, self.reset) {
            (Some(d), None, false) => current + point(d)?,
            (None, Some(o), false) => point(o)?,
            (None, None, true) => DVec2::ZERO,
            _ => return Err(EngineError::invalid("give one of delta, offset or reset")),
        };
        if new.length() > MAX_TAG_OFFSET {
            return Err(EngineError::invalid(format!("a tag stays within {MAX_TAG_OFFSET} m of its element")));
        }
        edit(s, t!("tx-move-tag"), |e| {
            let v = e.view_mut(view).ok_or_else(|| EngineError::not_found("view"))?;
            if new == DVec2::ZERO {
                v.tag_offsets.remove(&id);
            } else {
                v.tag_offsets.insert(id, new);
            }
            Ok(())
        })?;
        Ok(serde_json::json!({ "view": view.0, "id": id.0, "offset": [new.x, new.y] }))
    }
}
