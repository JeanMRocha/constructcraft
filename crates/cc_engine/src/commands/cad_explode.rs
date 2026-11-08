//! Exploding an imported drawing (`cad_explode`): its visible primitives
//! become native, editable elements of its view in one undo step. Lines,
//! polylines (bulged segments as arcs), arcs and circles become detail
//! lines, one per segment, in a project line style per layer; fills become
//! filled regions of a solid type per color; texts become text notes of a
//! text type per size; pictures stay in an imported drawing with the same
//! placement. The element's color mode and halftone are not carried over:
//! the line styles take the layer colors.

use super::cad::cad_of;
use super::{add_element, edit, ids_json};
use crate::error::{EngineError, Result};
use crate::registry::{Command, CommandKind};
use crate::session::Session;
use crate::t;
use cc_core::cad::{CadDrawing, CadHAlign, CadImport, CadPrim, CadVAlign, on_paper};
use cc_core::detail::{DetailLine, DetailStyle, LineStyleDef, LineStyleRef, MAX_LINE_STYLES};
use cc_core::region::Region;
use cc_core::sketch::SketchCurve;
use cc_core::{
    Category, ElementId, ElementKind, ElementType, ElementTypeId, LeaderSymbol, LinePattern, LineStyleId, Pattern,
    PatternFill, TextNote, TypeData, ViewId,
};
use glam::DVec2;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};
use std::sync::Arc;

/// Most elements one explode makes: a larger drawing is refused (explode
/// a part of it after hiding layers, or keep it as an imported drawing).
pub const MAX_ELEMENTS: usize = 100_000;
/// Most types (text sizes and fill colors) one explode adds.
pub const MAX_NEW_TYPES: usize = 1_000;
/// Longest line style name made from a layer name.
const MAX_STYLE_NAME: usize = 100;

/// Explode an imported DWG, DXF or SVG drawing into native elements of
/// its view, in one undo step: its lines, polylines, arcs and circles
/// become detail lines (one element per segment, bulged segments as arcs;
/// ellipses and splines are stored as polylines and come out as their
/// segments), its fills filled regions (no boundary, a solid filled region
/// type of the fill's color, reused when the project has one), and its
/// texts text notes (same place, height, rotation and alignment; a text
/// type of that size on paper at the view's scale, reused when one exists).
/// Each layer becomes a project line style of its name and color (light
/// colors darkened for paper), reused when a style of that name exists.
/// Layers hidden in the element (which starts with the layers off or
/// frozen in the file) are left out; the placement (units, scale,
/// position, rotation, mirror) is applied. Pictures stay in an imported
/// drawing in the same view with the same placement; without them the
/// element is removed. The color mode and halftone are not carried over.
/// Refused above 100,000 elements.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CadExplode {
    /// Imported drawing element id.
    pub id: u64,
}

/// A detail line to make, on the style of a layer (`None`: a layer index
/// the drawing does not list, drawn thin).
struct Line {
    layer: Option<u32>,
    curve: SketchCurve,
}

/// What an imported drawing explodes into, in view coordinates.
#[derive(Default)]
struct Exploded {
    lines: Vec<Line>,
    /// Fill color and outline.
    fills: Vec<([u8; 3], Vec<DVec2>)>,
    /// Text size on paper (millimeters) and the note.
    texts: Vec<(f64, TextNote)>,
    /// Primitives that make nothing valid (too small, out of bounds, crossing outlines).
    skipped: usize,
}

impl Exploded {
    fn count(&self) -> usize {
        self.lines.len() + self.fills.len() + self.texts.len()
    }
}

/// The curves of an arc of `sweep` radians from `start` (drawing units)
/// placed by `to` (similarity, mirrored or not), split in two when it turns
/// further than one curve's bulge can.
fn arc_curves(
    center: DVec2,
    radius: f64,
    start: f64,
    sweep: f64,
    to: impl Fn(DVec2) -> DVec2,
    factor: f64,
    mirrored: bool,
) -> Vec<SketchCurve> {
    if sweep.abs() >= TAU - 1e-9 {
        return vec![SketchCurve::Circle { center: to(center), radius: radius * factor }];
    }
    let parts = if sweep.abs() > 1.5 * PI { 2 } else { 1 };
    let step = sweep / parts as f64;
    // A mirror turns the other way.
    let bulge = (step / 4.0).tan() * if mirrored { -1.0 } else { 1.0 };
    (0..parts)
        .map(|i| {
            let s = start + step * i as f64;
            let a = to(center + DVec2::from_angle(s) * radius);
            let b = to(center + DVec2::from_angle(s + step) * radius);
            SketchCurve::Arc { a, b, bulge }
        })
        .collect()
}

/// Add a detail line when its curve is valid in a view.
fn push_line(out: &mut Exploded, view: ViewId, layer: Option<u32>, curve: SketchCurve) {
    if (DetailLine { view, curve, style: LineStyleRef::default() }).is_valid() {
        out.lines.push(Line { layer, curve });
    }
}

/// The visible primitives of `c` in view coordinates (`scale`: the view's, for text sizes).
fn explode(c: &CadImport, view: ViewId, scale: u32) -> Result<Exploded> {
    let f = c.factor();
    if !(f.is_finite() && f > 0.0) {
        return Err(EngineError::invalid("the drawing's units and scale give it no size"));
    }
    let place = c.placement();
    let to = |p: DVec2| place.apply(p * f);
    let mut out = Exploded::default();
    let known = |layer: u32| ((layer as usize) < c.drawing.layers.len()).then_some(layer);
    let paper = f64::from(scale.max(1));
    for prim in c.drawing.prims.iter().filter(|p| c.layer_visible(p.layer())) {
        let layer = known(prim.layer());
        let before = out.count();
        match prim {
            CadPrim::Polyline { pts, bulges, closed, .. } => {
                let n = pts.len();
                let segs = if *closed { n } else { n.saturating_sub(1) };
                for i in 0..segs {
                    let (Some(a), Some(b)) = (pts.get(i), pts.get((i + 1) % n)) else { continue };
                    let (a, b) = (to(*a), to(*b));
                    let bulge = bulges.get(i).copied().unwrap_or(0.0);
                    let bulge = bulge.clamp(-cc_core::arc::MAX_BULGE, cc_core::arc::MAX_BULGE);
                    let curve = if cc_core::arc::Arc::from_bulge(a, b, bulge).is_some() {
                        SketchCurve::Arc { a, b, bulge: if c.mirrored { -bulge } else { bulge } }
                    } else {
                        SketchCurve::Line { a, b }
                    };
                    push_line(&mut out, view, layer, curve);
                }
            }
            CadPrim::Arc { center, radius, start, sweep, .. } => {
                for curve in arc_curves(*center, *radius, *start, *sweep, to, f, c.mirrored) {
                    push_line(&mut out, view, layer, curve);
                }
            }
            CadPrim::Fill { pts, color, .. } => {
                let rgb = color.or_else(|| c.drawing.layer_of(prim).map(|l| l.color)).unwrap_or([0, 0, 0]);
                let outline: Vec<DVec2> = pts.iter().map(|p| to(*p)).collect();
                let region =
                    Region { view, outline: outline.clone(), openings: Vec::new(), sketch: None, boundary: None };
                if region.is_valid() {
                    out.fills.push((on_paper(rgb), outline));
                }
            }
            CadPrim::Text { pos, height, rotation, text, h, v, .. } => {
                let dir = place.apply_vector(DVec2::from_angle(*rotation));
                let height = height * f;
                let size = (height * 1000.0 / paper * 10.0).round().max(1.0) / 10.0;
                let w = cc_core::cad::text_width(text, height);
                let along = match h {
                    CadHAlign::Left => 0.0,
                    CadHAlign::Center => -0.5 * w,
                    CadHAlign::Right => -w,
                };
                let across = match v {
                    CadVAlign::Baseline | CadVAlign::Bottom => 0.0,
                    CadVAlign::Middle => -0.5,
                    CadVAlign::Top => -1.0,
                };
                let up = dir.perp() * height;
                let baseline = to(*pos) + dir * along + up * across;
                // A note's position is the top left of its box; its first line sits 1.25 sizes below it.
                let position = baseline + dir.perp() * (1.25 * size * paper / 1000.0);
                let note = TextNote {
                    view,
                    position,
                    text: text.replace(['\r', '\n'], " "),
                    size_mm: None,
                    font_family: String::new(),
                    alignment: cc_core::TextAlignment::Left,
                    bold: false,
                    italic: false,
                    underline: false,
                    width: None,
                    rotation_deg: dir.y.atan2(dir.x).to_degrees(),
                    leaders: Vec::new(),
                };
                if position.is_finite() && note.rotation_deg.is_finite() && !note.text.trim().is_empty() {
                    out.texts.push((size, note));
                }
            }
        }
        if out.count() == before {
            out.skipped += 1;
        }
        if out.count() > MAX_ELEMENTS {
            return Err(EngineError::rejected(t!("msg-cad-explode-too-many", "max" => MAX_ELEMENTS)));
        }
    }
    Ok(out)
}

/// A free line style name for a layer: its name (trimmed, shortened, `0`
/// when empty), with a number added when it is a built-in style's code or
/// already taken.
fn style_name(layer: &str, taken: impl Fn(&str) -> bool) -> String {
    let base: String = layer.trim().chars().take(MAX_STYLE_NAME - 8).collect();
    let base = if base.is_empty() { "0".to_string() } else { base };
    let free = |n: &str| DetailStyle::from_code(n).is_none() && !taken(n);
    if free(&base) {
        return base;
    }
    (2..10_000).map(|n| format!("{base} ({n})")).find(|n| free(n)).unwrap_or(base)
}

/// The project's line style named `name` (case-insensitive).
fn style_named(p: &cc_core::Project, name: &str) -> Option<LineStyleId> {
    p.line_styles.values().find(|s| s.name.trim().eq_ignore_ascii_case(name.trim())).map(|s| s.id)
}

/// The name of the line style a layer maps to.
fn layer_name(layers: &[cc_core::cad::CadLayer], layer: u32) -> &str {
    layers.get(layer as usize).map(|l| l.name.trim()).filter(|n| !n.is_empty()).unwrap_or("0")
}

/// A filled region type filling solid in `rgb`.
fn solid_fill_type(p: &cc_core::Project, rgb: [u8; 3]) -> Option<ElementTypeId> {
    p.types.values().find_map(|t| match &t.data {
        TypeData::FilledRegion { fill } if fill.foreground == Pattern::Solid && fill.foreground_color == rgb => {
            Some(t.id)
        }
        _ => None,
    })
}

/// A text type of `size` paper millimeters.
fn text_type(p: &cc_core::Project, size: f64) -> Option<ElementTypeId> {
    p.types.values().find_map(|t| match &t.data {
        TypeData::Text { size: s, .. } if (s - size).abs() < 1e-6 => Some(t.id),
        _ => None,
    })
}

fn new_type(e: &mut cc_core::EditCtx, name: String, data: TypeData) -> ElementTypeId {
    let id = ElementTypeId(e.alloc_id());
    e.add_type(ElementType { id, name, data, info: Default::default(), uid: Default::default() })
}

/// A size as written in a type name: one decimal, a trailing `.0` dropped.
fn size_text(size: f64) -> String {
    let s = format!("{size:.1}");
    s.strip_suffix(".0").map(str::to_string).unwrap_or(s)
}

/// A text size in tenths of a millimeter (the key of its type).
fn size_key(size: f64) -> u64 {
    (size * 10.0).round().max(0.0) as u64
}

impl Command for CadExplode {
    const NAME: &'static str = "cad_explode";
    const KIND: CommandKind = CommandKind::Edit;
    fn run(self, s: &mut Session) -> Result<Value> {
        let (id, c) = cad_of(s, self.id)?;
        let p = s.project();
        let level = p.element(id).map(|e| e.level).ok_or_else(|| EngineError::not_found("element"))?;
        let scale = p.view(c.view).map(|v| v.scale).ok_or_else(|| EngineError::not_found("view"))?;
        let exploded = explode(&c, c.view, scale)?;
        let images: Vec<_> = c.drawing.images.iter().filter(|i| c.layer_visible(i.layer)).cloned().collect();
        if exploded.count() == 0 {
            // Only pictures (or nothing visible): nothing to explode.
            return Err(EngineError::invalid(t!("msg-cad-explode-nothing")));
        }

        // The styles and types it would add, within the project's limits.
        let used: std::collections::BTreeSet<u32> = exploded.lines.iter().filter_map(|l| l.layer).collect();
        let new_styles: std::collections::BTreeSet<String> = used
            .iter()
            .map(|l| layer_name(&c.drawing.layers, *l))
            .filter(|n| style_named(p, n).is_none())
            .map(str::to_lowercase)
            .collect();
        if p.line_styles.len() + new_styles.len() > MAX_LINE_STYLES {
            return Err(EngineError::rejected(format!("a project has at most {MAX_LINE_STYLES} line styles")));
        }
        let colors: std::collections::BTreeSet<[u8; 3]> = exploded.fills.iter().map(|(c, _)| *c).collect();
        let sizes: std::collections::BTreeSet<u64> = exploded.texts.iter().map(|(s, _)| size_key(*s)).collect();
        let new_types = colors.iter().filter(|c| solid_fill_type(p, **c).is_none()).count()
            + sizes.iter().filter(|k| text_type(p, **k as f64 / 10.0).is_none()).count();
        if new_types > MAX_NEW_TYPES {
            return Err(EngineError::rejected(format!(
                "the drawing would add {new_types} text and fill types (at most {MAX_NEW_TYPES})"
            )));
        }

        let skipped = exploded.skipped;
        let layers = c.drawing.layers.clone();
        let kept_drawing = (!images.is_empty()).then(|| {
            let mut d =
                CadDrawing { layers: layers.clone(), prims: Vec::new(), images, extents: None, units: c.drawing.units };
            d.compute_extents();
            Arc::new(d)
        });
        let view = c.view;
        let tx = t!("tx-cad-explode", "name" => c.name.clone());
        let (lines, regions, texts, styles, types, kept) = edit(s, tx, |e| {
            // A style per used layer: the project's of that name, else a new one.
            let mut style_of: BTreeMap<u32, LineStyleRef> = BTreeMap::new();
            let mut styles = Vec::new();
            for layer in &used {
                let lname = layer_name(&layers, *layer);
                let sid = match style_named(e.project(), lname) {
                    Some(sid) => sid,
                    None => {
                        let name = style_name(lname, |n| style_named(e.project(), n).is_some());
                        let color = layers.get(*layer as usize).map_or([0, 0, 0], |l| on_paper(l.color));
                        let sid = LineStyleId(e.alloc_id());
                        let weight_mm = DetailStyle::Thin.weight_mm();
                        let def = LineStyleDef { id: sid, name, weight_mm, color, pattern: LinePattern::Solid };
                        e.line_styles_mut().insert(sid, def);
                        styles.push(sid.0);
                        sid
                    }
                };
                style_of.insert(*layer, LineStyleRef::Project(sid));
            }
            let mut types = Vec::new();
            let mut lines = Vec::with_capacity(exploded.lines.len());
            if !exploded.lines.is_empty() {
                let line_type = match e.project().default_type(Category::DetailLine) {
                    Some(t) => t,
                    None => {
                        let t = new_type(e, t!("tpl-type-detail-line"), TypeData::DetailLine {});
                        types.push(t.0);
                        t
                    }
                };
                for l in exploded.lines {
                    let style = l.layer.and_then(|k| style_of.get(&k).copied()).unwrap_or_default();
                    let kind = ElementKind::DetailLine(DetailLine { view, curve: l.curve, style });
                    lines.push(add_element(e, level, line_type, kind));
                }
            }
            // Filled regions, a solid type per color.
            let mut fill_types: BTreeMap<[u8; 3], ElementTypeId> = BTreeMap::new();
            let mut regions = Vec::with_capacity(exploded.fills.len());
            for (rgb, outline) in exploded.fills {
                let type_id = match fill_types.get(&rgb).copied().or_else(|| solid_fill_type(e.project(), rgb)) {
                    Some(t) => t,
                    None => {
                        let hex = format!("#{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2]);
                        let fill = PatternFill {
                            foreground: Pattern::Solid,
                            foreground_color: rgb,
                            background: Pattern::None,
                            background_color: [255, 255, 255],
                        };
                        let name = t!("tpl-type-solid-fill", "color" => hex);
                        let t = new_type(e, name, TypeData::FilledRegion { fill });
                        types.push(t.0);
                        t
                    }
                };
                fill_types.insert(rgb, type_id);
                let region = Region { view, outline, openings: Vec::new(), sketch: None, boundary: None };
                regions.push(add_element(e, level, type_id, ElementKind::FilledRegion(region)));
            }
            // Text notes, a type per size.
            let mut text_types: BTreeMap<u64, ElementTypeId> = BTreeMap::new();
            let mut texts = Vec::with_capacity(exploded.texts.len());
            for (size, note) in exploded.texts {
                let key = size_key(size);
                let type_id = match text_types.get(&key).copied().or_else(|| text_type(e.project(), size)) {
                    Some(t) => t,
                    None => {
                        let data = TypeData::Text { size, symbol: LeaderSymbol::Arrow };
                        let t = new_type(e, t!("tpl-type-text-size", "size" => size_text(size)), data);
                        types.push(t.0);
                        t
                    }
                };
                text_types.insert(key, type_id);
                texts.push(add_element(e, level, type_id, ElementKind::Text(Box::new(note))));
            }
            // Pictures stay in the element; without them it goes.
            let kept = match kept_drawing {
                Some(d) => {
                    if let Some(ElementKind::CadImport(ci)) = e.element_mut(id).map(|x| &mut x.kind) {
                        ci.drawing = d;
                    }
                    Some(id)
                }
                None => {
                    e.remove_element(id);
                    None
                }
            };
            Ok((lines, regions, texts, styles, types, kept))
        })?;
        let created: Vec<ElementId> = lines.iter().chain(&regions).chain(&texts).copied().collect();
        Ok(json!({
            "created": ids_json(&created),
            "detail_lines": lines.len(),
            "filled_regions": regions.len(),
            "texts": texts.len(),
            "line_styles": styles,
            "types": types,
            "kept": kept.map(|i| i.0),
            "skipped": skipped,
        }))
    }
}
