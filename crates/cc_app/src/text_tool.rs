//! Text notes in 2D views: the Text tool (single-line or multiline text,
//! optionally with a leader of one or two segments), the Add Leader tool and
//! the text editor shown over the view. Notes are created and changed
//! through the engine's commands.

use crate::paint::{PREVIEW, PaintOptions, Xf, paint_drawing};
use crate::state::AppState;
use crate::t;
use crate::tools::{Tool, draw_snap, snap_at};
use cc_core::{ElementId, ElementKind, ElementTypeId, Leader, LeaderSymbol, TextNote, ViewId};
use cc_engine::Ref;
use cc_engine::commands::annotate::{LeaderArgs, TextAddLeader, TextCreate};
use cc_views::Drawing2D;
use egui::{Painter, Pos2, Response, Stroke, Ui};
use glam::DVec2;
use std::collections::BTreeSet;

/// The leader a new text gets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LeaderShape {
    #[default]
    None,
    /// One segment from the text to the point.
    Straight,
    /// Two segments: from the text to an elbow, then to the point.
    Elbow,
}

impl LeaderShape {
    pub const ALL: [LeaderShape; 3] = [LeaderShape::None, LeaderShape::Straight, LeaderShape::Elbow];

    pub fn key(self) -> &'static str {
        match self {
            LeaderShape::None => "leader-shape-none",
            LeaderShape::Straight => "leader-shape-straight",
            LeaderShape::Elbow => "leader-shape-elbow",
        }
    }

    /// Points to click before the text: the end, then the elbow.
    fn points(self) -> usize {
        match self {
            LeaderShape::None => 0,
            LeaderShape::Straight => 1,
            LeaderShape::Elbow => 2,
        }
    }
}

/// Options of the text tools (kept between uses, shown in the options bar).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextOptions {
    pub multiline: bool,
    /// Wrapping width of multiline text, paper millimeters.
    pub width: f64,
    pub leader: LeaderShape,
    pub symbol: LeaderSymbol,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self { multiline: false, width: 60.0, leader: LeaderShape::None, symbol: LeaderSymbol::Arrow }
    }
}

/// What the editor's text goes to.
#[derive(Debug, Clone, PartialEq)]
pub enum TextTarget {
    /// A new note at `position` with `leaders`.
    New { type_id: ElementTypeId, position: DVec2, leaders: Vec<Leader>, width: Option<f64> },
    /// An existing note.
    Existing(ElementId),
}

/// The text editor open over a view.
#[derive(Debug, Clone)]
pub struct TextEditor {
    view: ViewId,
    /// Screen point of the editor's top left corner.
    at: Pos2,
    text: String,
    multiline: bool,
    target: TextTarget,
    focus: bool,
}

/// Clicks that make a leader: the end first, then the elbow (if any).
fn leader_from(points: &[DVec2], symbol: LeaderSymbol) -> Option<Leader> {
    match points {
        [end] => Some(Leader { end: *end, elbow: None, symbol }),
        [end, elbow] => Some(Leader { end: *end, elbow: Some(*elbow), symbol }),
        _ => None,
    }
}

/// Paint a note as a preview.
fn ghost(painter: &Painter, xf: &Xf, note: &TextNote, size: f64, scale: u32, px_per_mm: f64) {
    let mut d = Drawing2D::default();
    cc_views::notes::draw_note(&mut d, note, size, scale, None);
    let none = BTreeSet::new();
    let opts = PaintOptions { highlight: &none, px_per_mm, mono: Some(PREVIEW), only: None };
    paint_drawing(painter, &d, xf, &opts);
}

/// The Text tool: click the leader's end and elbow (when the options ask for
/// a leader), then where the text goes, and type it.
#[allow(clippy::too_many_arguments)]
pub fn text_tool(
    ui: &Ui,
    resp: &Response,
    painter: &Painter,
    state: &mut AppState,
    view: &cc_core::View,
    type_id: ElementTypeId,
    xf: &Xf,
    px_per_mm: f64,
) {
    if state.text_edit.is_some() {
        return;
    }
    let Some(raw) = resp.hover_pos().map(|p| xf.to_world(p)) else { return };
    let shift = ui.input(|i| i.modifiers.shift);
    let level = view.level();
    let s = snap_at(state, level, raw, state.interaction.points.last().copied(), &[], xf.scale, shift);
    draw_snap(painter, xf, s);
    let opts = state.text_options;
    let points = state.interaction.points.clone();
    let size = cc_views::notes::note_size(state.project(), type_id);
    let scale = view.scale.max(1);
    if points.len() < opts.leader.points() {
        // Still placing the leader: its end, then its elbow.
        if let Some(last) = points.last() {
            painter.line_segment([xf.to_screen(*last), xf.to_screen(s.point)], Stroke::new(1.5, PREVIEW));
        }
        if resp.clicked() {
            state.interaction.points.push(s.point);
            state.message = if state.interaction.points.len() < opts.leader.points() {
                t!("hint-text-leader-elbow")
            } else {
                t!("hint-text-position")
            };
        }
        return;
    }
    let width = opts.multiline.then_some(opts.width);
    let leaders: Vec<Leader> = leader_from(&points, opts.symbol).into_iter().collect();
    let preview = TextNote {
        view: view.id,
        position: s.point,
        text: t!("text-placeholder"),
        size_mm: None,
        font_family: String::new(),
        alignment: cc_core::TextAlignment::Left,
        bold: false,
        italic: false,
        underline: false,
        width,
        rotation_deg: 0.0,
        leaders,
    };
    ghost(painter, xf, &preview, size, scale, px_per_mm);
    if resp.clicked() {
        let target = TextTarget::New { type_id, position: s.point, leaders: preview.leaders, width };
        state.text_edit = Some(TextEditor {
            view: view.id,
            at: xf.to_screen(s.point),
            text: String::new(),
            multiline: opts.multiline,
            target,
            focus: true,
        });
        state.interaction.points.clear();
    }
}

/// The Add Leader tool: click the end (and elbow) of a new leader of note `id`.
#[allow(clippy::too_many_arguments)]
pub fn add_leader_tool(
    ui: &Ui,
    resp: &Response,
    painter: &Painter,
    state: &mut AppState,
    view: &cc_core::View,
    id: ElementId,
    xf: &Xf,
    px_per_mm: f64,
) {
    let Some(el) = state.project().element(id).cloned() else {
        state.set_tool(Tool::Select);
        return;
    };
    let ElementKind::Text(note) = &el.kind else { return };
    if note.view != view.id {
        return;
    }
    let Some(raw) = resp.hover_pos().map(|p| xf.to_world(p)) else { return };
    let shift = ui.input(|i| i.modifiers.shift);
    let s = snap_at(state, view.level(), raw, state.interaction.points.last().copied(), &[], xf.scale, shift);
    draw_snap(painter, xf, s);
    let opts = state.text_options;
    let elbow = opts.leader == LeaderShape::Elbow;
    let mut points = state.interaction.points.clone();
    points.push(s.point);
    // Preview: the note with the leader being drawn.
    let size = cc_views::notes::note_size(state.project(), el.type_id);
    let mut preview = (**note).clone();
    preview.leaders = leader_from(&points, opts.symbol).into_iter().collect();
    ghost(painter, xf, &preview, size, view.scale.max(1), px_per_mm);
    if !resp.clicked() {
        return;
    }
    if elbow && points.len() < 2 {
        state.interaction.points = points;
        state.message = t!("hint-text-leader-elbow");
        return;
    }
    state.interaction.points.clear();
    if let Some(leader) = leader_from(&points, opts.symbol) {
        let command = TextAddLeader { id: id.0, leader: leader_args(&leader) };
        crate::actions::run_command(state, command);
    }
    state.message = t!("hint-add-leader");
}

fn leader_args(l: &Leader) -> LeaderArgs {
    let symbol = serde_json::to_value(l.symbol).ok().and_then(|v| v.as_str().map(String::from));
    LeaderArgs { end: l.end.to_array(), elbow: l.elbow.map(|e| e.to_array()), symbol }
}

/// A double click on a note with the Select tool opens it in the editor.
/// Returns true when the pointer was used.
pub fn canvas(ui: &Ui, resp: &Response, state: &mut AppState, view: ViewId, xf: &Xf, drawing: &Drawing2D) -> bool {
    let context_menu = text_context_menu(ui, resp, state, view, xf, drawing);
    if state.tool != Tool::Select || !resp.double_clicked() {
        return context_menu;
    }
    let Some(pos) = resp.hover_pos() else { return false };
    let Some(hit) = drawing.hit_test(xf.to_world(pos), 5.0 / xf.scale) else { return false };
    let Some(ElementKind::Text(note)) = state.project().element(hit).map(|e| &e.kind) else { return false };
    if note.view != view {
        return false;
    }
    let (text, multiline, at) = (note.text.clone(), note.width.is_some() || note.text.contains('\n'), note.position);
    state.selection = BTreeSet::from([hit]);
    state.text_edit = Some(TextEditor {
        view,
        at: xf.to_screen(at),
        text,
        multiline,
        target: TextTarget::Existing(hit),
        focus: true,
    });
    true
}

/// Apply rich-text styling directly from a text note's right-click menu.
fn text_context_menu(
    ui: &Ui,
    resp: &Response,
    state: &mut AppState,
    view: ViewId,
    xf: &Xf,
    drawing: &Drawing2D,
) -> bool {
    let target_key = egui::Id::new(("text-context-target", view.0));
    let clicked_text = resp.secondary_clicked().then(|| {
        resp.interact_pointer_pos()
            .and_then(|pos| drawing.hit_test(xf.to_world(pos), 5.0 / xf.scale))
            .filter(|id| matches!(state.project().element(*id).map(|e| &e.kind), Some(ElementKind::Text(note)) if note.view == view))
    }).flatten();
    if resp.secondary_clicked() {
        ui.ctx().data_mut(|data| {
            if let Some(id) = clicked_text {
                data.insert_temp(target_key, id.0);
            } else {
                data.remove::<u64>(target_key);
            }
        });
        if let Some(id) = clicked_text {
            state.selection = BTreeSet::from([id]);
        }
    }
    if clicked_text.is_none() && !resp.context_menu_opened() {
        return false;
    }

    resp.context_menu(|ui| {
        let Some(id) = ui.ctx().data(|data| data.get_temp::<u64>(target_key)).map(ElementId) else {
            ui.close();
            return;
        };
        let Some(ElementKind::Text(note)) = state.project().element(id).map(|e| &e.kind) else {
            ui.close();
            return;
        };
        let note = note.clone();
        ui.label(t!("section-text"));
        ui.separator();
        text_format_controls(ui, state, id, &note);
    });
    true
}

/// The formatting controls shared by the properties palette and the text context menu.
pub(crate) fn text_format_controls(ui: &mut Ui, state: &mut AppState, id: ElementId, note: &TextNote) {
    let set = |state: &mut AppState, field: &str, f: &dyn Fn(&mut TextNote)| {
        crate::engine_edit::element(state, id, Some(crate::properties::key((id, field))), |e| {
            if let ElementKind::Text(n) = &mut e.kind {
                f(n);
            }
        });
    };
    let inherited_size = state
        .project()
        .element(id)
        .and_then(|e| state.project().element_type(e.type_id))
        .and_then(|ty| match &ty.data {
            cc_core::TypeData::Text { size, .. } => Some(*size),
            _ => None,
        })
        .unwrap_or(2.5);
    let mut custom_size = note.size_mm.is_some();
    if ui.checkbox(&mut custom_size, t!("text-custom-size")).changed() {
        set(state, "text-size-mode", &|n| n.size_mm = custom_size.then_some(inherited_size));
    }
    if custom_size {
        let mut size = note.size_mm.unwrap_or(inherited_size);
        if ui.add(egui::DragValue::new(&mut size).speed(0.1).range(0.5..=1000.0).suffix(" mm")).changed() {
            set(state, "text-size", &|n| n.size_mm = Some(size));
        }
    } else {
        ui.label(format!("{}: {} mm", t!("text-size"), cc_views::format::number(inherited_size, 1)));
    }

    let families = cc_views::fonts::system_families();
    let mut family = note.font_family.clone();
    let selected_font = if family.is_empty() { t!("text-font-default") } else { family.clone() };
    egui::ComboBox::from_id_salt(("text-font", id.0)).selected_text(selected_font).show_ui(ui, |ui| {
        ui.selectable_value(&mut family, String::new(), t!("text-font-default"));
        for name in families {
            ui.selectable_value(&mut family, name.clone(), name);
        }
    });
    if family != note.font_family {
        set(state, "text-font", &|n| n.font_family = family.clone());
    }

    let mut alignment = note.alignment;
    egui::ComboBox::from_id_salt(("text-alignment", id.0))
        .selected_text(match alignment {
            cc_core::TextAlignment::Left => t!("text-align-left"),
            cc_core::TextAlignment::Center => t!("text-align-center"),
            cc_core::TextAlignment::Right => t!("text-align-right"),
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut alignment, cc_core::TextAlignment::Left, t!("text-align-left"));
            ui.selectable_value(&mut alignment, cc_core::TextAlignment::Center, t!("text-align-center"));
            ui.selectable_value(&mut alignment, cc_core::TextAlignment::Right, t!("text-align-right"));
        });
    if alignment != note.alignment {
        set(state, "text-alignment", &|n| n.alignment = alignment);
    }

    for (label, field, value) in [
        (t!("text-bold"), "bold", note.bold),
        (t!("text-italic"), "italic", note.italic),
        (t!("text-underline"), "underline", note.underline),
    ] {
        let mut enabled = value;
        if ui.checkbox(&mut enabled, label).changed() {
            set(state, field, &|n| match field {
                "bold" => n.bold = enabled,
                "italic" => n.italic = enabled,
                _ => n.underline = enabled,
            });
        }
    }

    let mut multiline = note.width.is_some();
    if ui.checkbox(&mut multiline, t!("text-multiline")).changed() {
        let width = multiline.then_some(state.text_options.width);
        set(state, "multiline", &|n| n.width = width);
    }
    if let Some(mut width) = note.width {
        ui.horizontal(|ui| {
            ui.label(t!("prop-text-width"));
            if ui.add(egui::DragValue::new(&mut width).speed(1.0).range(5.0..=1000.0).suffix(" mm")).changed() {
                set(state, "width", &|n| n.width = Some(width));
            }
        });
    }
}

/// The text editor over `view`, when open there: Enter (single-line) or
/// Ctrl+Enter (multiline) or OK applies, Esc or Cancel closes it.
pub fn show(ui: &Ui, state: &mut AppState, view: ViewId) {
    let Some(edit) = state.text_edit.as_mut().filter(|e| e.view == view) else { return };
    let (mut apply, mut cancel) = (false, false);
    egui::Area::new(egui::Id::new("text-edit")).fixed_pos(edit.at).order(egui::Order::Foreground).show(
        ui.ctx(),
        |ui| {
            egui::Frame::popup(ui.style()).inner_margin(egui::Margin::same(4)).show(ui, |ui| {
                let r = if edit.multiline {
                    ui.add(egui::TextEdit::multiline(&mut edit.text).desired_width(260.0).desired_rows(3))
                } else {
                    ui.add(egui::TextEdit::singleline(&mut edit.text).desired_width(220.0))
                };
                if edit.focus {
                    r.request_focus();
                    edit.focus = false;
                }
                let (enter, ctrl, escape) = ui.input(|i| {
                    (i.key_pressed(egui::Key::Enter), i.modifiers.command, i.key_pressed(egui::Key::Escape))
                });
                if escape {
                    cancel = true;
                } else if edit.multiline && enter && ctrl {
                    apply = true;
                } else if !edit.multiline && r.lost_focus() {
                    // Enter, or a click elsewhere, applies single-line text.
                    apply = true;
                }
                ui.horizontal(|ui| {
                    if ui.button(t!("action-ok")).clicked() {
                        apply = true;
                    }
                    if ui.button(t!("action-cancel")).clicked() {
                        cancel = true;
                    }
                    if edit.multiline {
                        ui.label(crate::theme::muted(t!("hint-text-multiline")));
                    }
                });
            });
        },
    );
    if !(apply || cancel) {
        return;
    }
    let Some(edit) = state.text_edit.take() else { return };
    if apply && !cancel {
        commit(state, edit);
    }
}

/// Create or change the note with the editor's text (empty text changes nothing).
fn commit(state: &mut AppState, edit: TextEditor) {
    let text = edit.text.trim_end().to_string();
    if text.trim().is_empty() {
        return;
    }
    match edit.target {
        TextTarget::New { type_id, position, leaders, width } => {
            let command = TextCreate {
                view: Ref::Id(edit.view.0),
                position: position.to_array(),
                text,
                width,
                rotation: None,
                type_ref: Some(Ref::Id(type_id.0)),
                leaders: leaders.iter().map(leader_args).collect(),
            };
            if let Some(id) = crate::actions::run_command(state, command).and_then(|o| o["id"].as_u64()) {
                state.selection = BTreeSet::from([ElementId(id)]);
            }
        }
        TextTarget::Existing(id) => {
            crate::engine_edit::element(state, id, None, |e| {
                if let ElementKind::Text(note) = &mut e.kind {
                    note.text = text;
                }
            });
        }
    }
}

/// Options bar of the text tools.
pub fn options(ui: &mut Ui, state: &mut AppState, adding_leader: bool) {
    let o = &mut state.text_options;
    if !adding_leader {
        ui.selectable_value(&mut o.multiline, false, t!("text-single"));
        ui.selectable_value(&mut o.multiline, true, t!("text-multiline"));
        if o.multiline {
            ui.label(t!("prop-text-width"));
            ui.add(egui::DragValue::new(&mut o.width).speed(1.0).range(5.0..=1000.0).suffix(" mm"));
        }
        ui.separator();
    }
    ui.label(t!("label-leader"));
    let shapes: Vec<LeaderShape> =
        LeaderShape::ALL.into_iter().filter(|s| !(adding_leader && *s == LeaderShape::None)).collect();
    if adding_leader && o.leader == LeaderShape::None {
        o.leader = LeaderShape::Straight;
    }
    egui::ComboBox::from_id_salt("text-leader").selected_text(crate::i18n::tr(o.leader.key())).show_ui(ui, |ui| {
        for s in shapes {
            ui.selectable_value(&mut o.leader, s, crate::i18n::tr(s.key()));
        }
    });
    if o.leader != LeaderShape::None {
        ui.label(t!("label-symbol"));
        symbol_combo(ui, "text-symbol", &mut o.symbol);
    }
    let hint = if adding_leader {
        "hint-add-leader"
    } else if o.leader == LeaderShape::None {
        "hint-text-position"
    } else {
        "hint-text"
    };
    ui.label(crate::theme::muted(crate::i18n::tr(hint)));
}

/// A combo box of leader symbols; returns whether it changed.
pub fn symbol_combo(ui: &mut Ui, id: impl std::hash::Hash + std::fmt::Debug, symbol: &mut LeaderSymbol) -> bool {
    let before = *symbol;
    egui::ComboBox::from_id_salt(id).selected_text(crate::i18n::tr(symbol.key())).show_ui(ui, |ui| {
        for s in LeaderSymbol::ALL {
            ui.selectable_value(symbol, s, crate::i18n::tr(s.key()));
        }
    });
    *symbol != before
}

#[cfg(test)]
mod tests {
    use super::*;
    use cc_core::{Category, Document};

    #[test]
    fn the_editor_creates_and_changes_notes() {
        let mut state = AppState::new(Document::new(crate::actions::new_project()));
        let view = state.project().views.values().find(|v| v.is_plan()).map(|v| v.id).unwrap();
        let type_id = state.project().default_type(Category::Text).unwrap();
        // Multiline text with a leader in two segments.
        let leader = leader_from(&[DVec2::new(1.0, 1.0), DVec2::new(2.0, 3.0)], LeaderSymbol::Dot).unwrap();
        let target =
            TextTarget::New { type_id, position: DVec2::new(4.0, 4.0), leaders: vec![leader], width: Some(40.0) };
        let editor = |text: &str, target| TextEditor {
            view,
            at: Pos2::ZERO,
            text: text.into(),
            multiline: true,
            target,
            focus: false,
        };
        commit(
            &mut state,
            editor(
                "Line one
line two
",
                target,
            ),
        );
        let id = *state.selection.iter().next().unwrap();
        let note = match &state.project().element(id).unwrap().kind {
            ElementKind::Text(n) => (**n).clone(),
            other => panic!("{other:?}"),
        };
        assert_eq!(
            note.text,
            "Line one
line two",
            "trailing line breaks are dropped"
        );
        assert_eq!(note.width, Some(40.0));
        assert_eq!(note.leaders[0].elbow, Some(DVec2::new(2.0, 3.0)));
        assert_eq!(note.leaders[0].symbol, LeaderSymbol::Dot);
        // Editing an existing note; empty text changes nothing.
        commit(&mut state, editor("Revised", TextTarget::Existing(id)));
        commit(&mut state, editor("   ", TextTarget::Existing(id)));
        assert!(matches!(&state.project().element(id).unwrap().kind, ElementKind::Text(n) if n.text == "Revised"));
        // One undo step each.
        state.session.doc.undo();
        assert!(
            matches!(&state.project().element(id).unwrap().kind, ElementKind::Text(n) if n.text.starts_with("Line one"))
        );
    }
}
