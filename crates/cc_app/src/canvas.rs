//! 2D view canvas: plans, ceilings, elevations, sections, legends and drafting views.

use crate::icons::{self, Icon};
use crate::paint::{PaintOptions, Xf, paint_drawing};
use crate::state::AppState;
use crate::theme::{self, PALETTE};
use cc_core::ViewId;
use egui::{Rect, Response, Sense, Ui};
use glam::DVec2;

/// Screen points per paper millimeter at "sheet size" (96 points per inch).
const POINTS_PER_MM: f64 = 96.0 / 25.4;
/// Pan/zoom states kept for "previous" and "next".
const HISTORY: usize = 50;

/// Zoom commands of the view toolbar menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ZoomCommand {
    /// Drag a window to zoom into.
    Region,
    /// Zoom out by half around the view center.
    OutTwo,
    #[default]
    Fit,
    /// Zoom every open view to fit.
    FitAll,
    /// One paper millimeter shows as one screen millimeter.
    SheetSize,
    Previous,
    Next,
}

impl ZoomCommand {
    pub const ALL: [ZoomCommand; 7] = [
        ZoomCommand::Region,
        ZoomCommand::OutTwo,
        ZoomCommand::Fit,
        ZoomCommand::FitAll,
        ZoomCommand::SheetSize,
        ZoomCommand::Previous,
        ZoomCommand::Next,
    ];

    pub fn key(self) -> &'static str {
        match self {
            ZoomCommand::Region => "zoom-region",
            ZoomCommand::OutTwo => "zoom-out-2x",
            ZoomCommand::Fit => "action-zoom-fit",
            ZoomCommand::FitAll => "zoom-fit-all",
            ZoomCommand::SheetSize => "zoom-sheet-size",
            ZoomCommand::Previous => "zoom-previous",
            ZoomCommand::Next => "zoom-next",
        }
    }

    /// Whether choosing it from the menu makes it the button's command
    /// (previous/next step through the history instead).
    fn becomes_default(self) -> bool {
        !matches!(self, ZoomCommand::Previous | ZoomCommand::Next)
    }
}

/// What the zoom menu can offer in a view.
#[derive(Debug, Clone, Copy, Default)]
pub struct ZoomNav {
    pub back: bool,
    pub forward: bool,
    /// The view has a paper scale ("zoom sheet size").
    pub sheet_size: bool,
}

/// Pan/zoom state of a 2D view.
#[derive(Debug, Clone)]
pub struct View2DState {
    /// Drawing coordinates shown at the canvas center.
    pub center: DVec2,
    /// Screen points per drawing unit.
    pub ppm: f64,
    /// Whether the view has been zoomed to fit at least once (or after "zoom extents").
    pub fitted: bool,
    /// The pan/zoom of the last fit and the canvas size it fitted: while the
    /// view is left as fitted, a resized canvas fits it again.
    auto_fit: Option<(DVec2, f64, egui::Vec2)>,
    /// Earlier and undone (center, ppm) states for "previous" / "next".
    back: Vec<(DVec2, f64)>,
    forward: Vec<(DVec2, f64)>,
    /// Time of the last wheel zoom: a pause starts a new history step.
    last_wheel: f64,
    /// "Zoom in region" waits for a window; the drag's start once it began.
    pub zoom_region: bool,
    region_start: Option<egui::Pos2>,
}

impl Default for View2DState {
    fn default() -> Self {
        Self {
            center: DVec2::ZERO,
            ppm: 40.0,
            fitted: false,
            auto_fit: None,
            back: Vec::new(),
            forward: Vec::new(),
            last_wheel: f64::NEG_INFINITY,
            zoom_region: false,
            region_start: None,
        }
    }
}

impl View2DState {
    pub fn xf(&self, rect: Rect) -> Xf {
        Xf { screen: rect.center(), origin: self.center, scale: self.ppm }
    }

    /// Remember the current pan/zoom before changing it.
    fn remember(&mut self) {
        if self.back.last() != Some(&(self.center, self.ppm)) {
            self.back.push((self.center, self.ppm));
            if self.back.len() > HISTORY {
                self.back.remove(0);
            }
        }
        self.forward.clear();
    }

    pub fn nav(&self) -> ZoomNav {
        ZoomNav { back: !self.back.is_empty(), forward: !self.forward.is_empty(), sheet_size: true }
    }

    /// Step back (or forward) through the pan/zoom history.
    fn step(&mut self, back: bool) {
        let (from, to) = if back { (&mut self.back, &mut self.forward) } else { (&mut self.forward, &mut self.back) };
        if let Some((center, ppm)) = from.pop() {
            to.push((self.center, self.ppm));
            self.center = center;
            self.ppm = ppm;
        }
    }

    /// Run a zoom command; `mm_scale` is the drawing's paper millimeters
    /// per drawing unit (sheets: 1; plans at 1:100: 10).
    pub fn apply(&mut self, cmd: ZoomCommand, mm_scale: f64) {
        match cmd {
            ZoomCommand::Region => {
                self.zoom_region = true;
                self.region_start = None;
            }
            ZoomCommand::OutTwo => {
                self.remember();
                self.ppm = (self.ppm / 2.0).clamp(1e-3, 1e6);
            }
            ZoomCommand::Fit | ZoomCommand::FitAll => self.request_fit(),
            ZoomCommand::SheetSize => {
                self.remember();
                self.ppm = (POINTS_PER_MM * mm_scale).clamp(1e-3, 1e6);
            }
            ZoomCommand::Previous => self.step(true),
            ZoomCommand::Next => self.step(false),
        }
    }

    /// Zoom to fit on the next frame (fitting needs the drawing's bounds),
    /// remembering the current pan/zoom.
    pub fn request_fit(&mut self) {
        if self.fitted {
            self.remember();
        }
        self.fitted = false;
    }

    pub fn fit(&mut self, (lo, hi): (DVec2, DVec2), rect: Rect) {
        let size = (hi - lo).max(DVec2::splat(1e-3));
        let margin = 0.85;
        self.ppm = (rect.width() as f64 / size.x).min(rect.height() as f64 / size.y) * margin;
        self.ppm = self.ppm.clamp(1e-3, 1e6);
        self.center = (lo + hi) * 0.5;
        self.fitted = true;
        self.auto_fit = Some((self.center, self.ppm, rect.size()));
    }

    /// Whether to zoom to fit now: the view was never fitted, or it was left
    /// as fitted and its canvas changed size.
    pub fn needs_fit(&self, rect: Rect) -> bool {
        !self.fitted
            || self.auto_fit.is_some_and(|(c, ppm, size)| c == self.center && ppm == self.ppm && size != rect.size())
    }

    /// Scroll-wheel zoom around the cursor and middle-button pan.
    pub fn navigate(&mut self, ui: &Ui, resp: &Response, rect: Rect) {
        if resp.hovered() {
            let (scroll, pinch) = ui.input(|i| (i.smooth_scroll_delta.y, i.zoom_delta()));
            let factor = (scroll as f64 * 0.0025).exp() * pinch as f64;
            if (factor - 1.0).abs() > 1e-6
                && let Some(cursor) = resp.hover_pos()
            {
                let now = ui.input(|i| i.time);
                if now - self.last_wheel > 0.6 {
                    self.remember();
                }
                self.last_wheel = now;
                let before = self.xf(rect).to_world(cursor);
                self.ppm = (self.ppm * factor).clamp(1e-3, 1e6);
                let after = self.xf(rect).to_world(cursor);
                self.center += before - after;
            }
        }
        if resp.drag_started_by(egui::PointerButton::Middle) {
            self.remember();
        }
        if resp.dragged_by(egui::PointerButton::Middle) {
            let d = resp.drag_delta();
            self.center += DVec2::new(-d.x as f64, d.y as f64) / self.ppm;
        }
    }

    /// "Zoom in region": drag a window and zoom to it; a click or Esc cancels.
    /// Returns true while it takes the pointer (tools wait meanwhile).
    pub fn region(&mut self, ui: &Ui, resp: &Response, painter: &egui::Painter, rect: Rect) -> bool {
        if !self.zoom_region {
            return false;
        }
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) || resp.clicked() {
            self.zoom_region = false;
            self.region_start = None;
            return true;
        }
        if resp.drag_started_by(egui::PointerButton::Primary) {
            self.region_start = ui.input(|i| i.pointer.press_origin());
        }
        let (Some(a), Some(b)) = (self.region_start, resp.interact_pointer_pos().or(resp.hover_pos())) else {
            return true;
        };
        let window = Rect::from_two_pos(a, b);
        painter.rect_filled(window, 0.0, PALETTE.accent.gamma_multiply(0.08));
        painter.rect_stroke(window, 0.0, egui::Stroke::new(1.0, PALETTE.accent), egui::StrokeKind::Middle);
        if resp.drag_stopped() {
            self.zoom_region = false;
            self.region_start = None;
            if window.width() > 4.0 && window.height() > 4.0 {
                let xf = self.xf(rect);
                let (p, q) = (xf.to_world(window.min), xf.to_world(window.max));
                self.remember();
                self.fit((p.min(q), p.max(q)), rect);
                // The window fills the view (no fit margin).
                self.ppm /= 0.85;
            }
        }
        true
    }
}

/// Run a toolbar zoom command on `view` (or, for "fit all", on every view).
pub fn zoom(state: &mut AppState, view: ViewId, cmd: ZoomCommand, mm_scale: f64) {
    if cmd.becomes_default() {
        state.zoom_default = cmd;
    }
    if cmd == ZoomCommand::FitAll {
        for v in state.view2d.values_mut() {
            v.request_fit();
        }
        for v in state.view3d.values_mut() {
            v.request_fit();
        }
        return;
    }
    if let Some(st) = state.view2d.get_mut(&view) {
        st.apply(cmd, mm_scale);
    } else if let Some(v3) = state.view3d.get_mut(&view) {
        v3.apply(cmd);
    }
    if cmd == ZoomCommand::Region {
        state.message = crate::t!("hint-zoom-region");
    }
}

/// Show a graphical 2D view and run the active tool on it.
pub fn show(ui: &mut Ui, state: &mut AppState, view_id: ViewId) {
    let Some(view) = state.project().view(view_id).cloned() else { return };
    let drawing = state.drawing(view_id).unwrap_or_default();
    let (resp, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());
    let rect = resp.rect;
    painter.rect_filled(rect, 0.0, PALETTE.paper);

    let st = state.view2d.entry(view_id).or_default();
    if st.needs_fit(rect) {
        match drawing.fit_bounds() {
            Some(b) if b.1.distance(b.0) > 0.5 => st.fit(b, rect),
            _ => {
                st.center = DVec2::new(5.0, 4.0);
                st.fitted = true;
            }
        }
    }
    st.navigate(ui, &resp, rect);
    let zooming = st.region(ui, &resp, &painter, rect);
    let xf = st.xf(rect);
    let px_per_mm = st.ppm * view.scale as f64 / 1000.0;

    let opts = PaintOptions { highlight: &state.selection, px_per_mm, mono: None, only: None };
    paint_drawing(&painter, &drawing, &xf, &opts);

    if let Some(p) = resp.hover_pos() {
        state.cursor = Some(xf.to_world(p));
    }
    let editing = !zooming
        && (crate::text_tool::canvas(ui, &resp, state, view_id, &xf, &drawing)
            || crate::dim_edit::canvas(ui, &resp, &painter, state, view_id, &xf, px_per_mm, &drawing));
    if !zooming && !editing {
        crate::tools::handle(ui, &resp, &painter, state, &view, &xf, &drawing);
    }
    crate::dim_edit::show(ui, state, view_id);
    crate::text_tool::show(ui, state, view_id);
    temporary_frame(ui, state, rect, view_id);
    let nav = state.view2d.get(&view_id).map(View2DState::nav).unwrap_or_default();
    if let Some(cmd) = view_toolbar(ui, state.zoom_default, rect, view_id, Some(view.scale), nav) {
        zoom(state, view_id, cmd, 1000.0 / view.scale.max(1) as f64);
    }
}

/// While something is temporarily hidden or isolated in the view: a colored
/// border and a chip naming the state, with Reset and Make Permanent.
pub fn temporary_frame(ui: &mut Ui, state: &mut AppState, rect: Rect, view_id: ViewId) {
    if state.session.temporary(view_id).is_none() {
        return;
    }
    let color = theme::palette(ui.ctx()).temporary;
    let painter = ui.painter_at(rect);
    painter.rect_stroke(rect.shrink(1.5), 0.0, egui::Stroke::new(3.0, color), egui::StrokeKind::Inside);
    let chip = Rect::from_min_size(rect.left_top() + egui::vec2(10.0, 10.0), egui::vec2(380.0, 28.0));
    painter.rect_filled(chip, 6.0, color);
    let font = egui::FontId::new(12.0, theme::semibold());
    painter.text(
        egui::pos2(chip.left() + 10.0, chip.center().y),
        egui::Align2::LEFT_CENTER,
        crate::t!("temporary-active"),
        font,
        egui::Color32::WHITE,
    );
    let button = |x: f32, w: f32| Rect::from_min_size(egui::pos2(x, chip.top() + 3.0), egui::vec2(w, 22.0));
    let permanent = ui.put(button(chip.right() - 112.0, 106.0), egui::Button::new(crate::t!("temporary-permanent")));
    let reset = ui.put(button(chip.right() - 186.0, 70.0), egui::Button::new(crate::t!("temporary-reset")));
    if reset.clicked() {
        state.active_view = Some(view_id);
        crate::actions::temporary(state, "reset");
    }
    if permanent.clicked() {
        state.active_view = Some(view_id);
        crate::actions::temporary_permanent(state);
    }
}

/// Floating chip in the view corner with the scale (when given) and the
/// zoom button: a click runs the default zoom command, the arrow opens the
/// menu of all of them. Returns the command to run.
pub fn view_toolbar(
    ui: &mut Ui,
    default: ZoomCommand,
    rect: Rect,
    view_id: ViewId,
    scale: Option<u32>,
    nav: ZoomNav,
) -> Option<ZoomCommand> {
    let painter = ui.painter_at(rect);
    let size = egui::vec2(if scale.is_some() { 110.0 } else { 52.0 }, 28.0);
    let bar = Rect::from_min_size(rect.right_bottom() - size - egui::vec2(12.0, 12.0), size);
    painter.rect_filled(bar.translate(egui::vec2(0.0, 1.0)), 7.0, egui::Color32::from_black_alpha(18));
    painter.rect_filled(bar, 7.0, PALETTE.panel);
    painter.rect_stroke(bar, 7.0, egui::Stroke::new(1.0, PALETTE.border), egui::StrokeKind::Inside);
    if let Some(scale) = scale {
        let label = egui::pos2(bar.left() + 12.0, bar.center().y);
        let font = egui::FontId::new(12.0, theme::semibold());
        painter.text(label, egui::Align2::LEFT_CENTER, format!("1:{scale}"), font, PALETTE.text_muted);
    }
    let button = Rect::from_center_size(egui::pos2(bar.right() - 30.0, bar.center().y), egui::vec2(24.0, 22.0));
    let arrow = Rect::from_center_size(egui::pos2(bar.right() - 11.0, bar.center().y), egui::vec2(14.0, 22.0));
    let r = ui.interact(button, egui::Id::new(("zoom", view_id.0)), Sense::click());
    let a = ui.interact(arrow, egui::Id::new(("zoom-menu", view_id.0)), Sense::click());
    for (area, resp) in [(button, &r), (arrow, &a)] {
        if resp.hovered() {
            painter.rect_filled(area, theme::RADIUS as f32, PALETTE.hover);
        }
    }
    icons::paint(&painter, button.shrink(4.0), Icon::ZoomFit, PALETTE.text_muted);
    icons::paint(&painter, arrow.shrink2(egui::vec2(1.0, 5.0)), Icon::ChevronDown, PALETTE.text_muted);
    let mut chosen = None;
    let tip = crate::i18n::tr(default.key());
    let tip = if default == ZoomCommand::Fit { format!("{tip} (ZE)") } else { tip };
    r.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &tip));
    a.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, crate::t!("canvas-zoom-menu")));
    if r.on_hover_text(tip).clicked() {
        chosen = Some(default);
    }
    egui::Popup::menu(&a).align(egui::RectAlign::TOP_END).show(|ui| {
        for cmd in ZoomCommand::ALL {
            let enabled = match cmd {
                ZoomCommand::Previous => nav.back,
                ZoomCommand::Next => nav.forward,
                ZoomCommand::SheetSize => nav.sheet_size,
                _ => true,
            };
            let button = egui::Button::new(crate::i18n::tr(cmd.key())).selected(cmd == default);
            if ui.add_enabled(enabled, button).clicked() {
                chosen = Some(cmd);
            }
        }
    });
    chosen
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fitted() -> (View2DState, Rect) {
        let rect = Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(800.0, 600.0));
        let mut st = View2DState::default();
        st.fit((DVec2::ZERO, DVec2::new(20.0, 10.0)), rect);
        st.fitted = true;
        (st, rect)
    }

    #[test]
    fn previous_and_next_walk_the_zoom_history() {
        let (mut st, rect) = fitted();
        let start = (st.center, st.ppm);
        assert!(!st.nav().back);
        st.apply(ZoomCommand::OutTwo, 10.0);
        assert!((st.ppm - start.1 / 2.0).abs() < 1e-9);
        st.apply(ZoomCommand::Fit, 10.0);
        st.fit((DVec2::ZERO, DVec2::new(20.0, 10.0)), rect);
        st.apply(ZoomCommand::Previous, 10.0);
        assert!((st.ppm - start.1 / 2.0).abs() < 1e-9);
        st.apply(ZoomCommand::Previous, 10.0);
        assert_eq!((st.center, st.ppm), start);
        assert!(!st.nav().back && st.nav().forward);
        st.apply(ZoomCommand::Next, 10.0);
        assert!((st.ppm - start.1 / 2.0).abs() < 1e-9);
        // A new zoom drops the steps ahead.
        st.apply(ZoomCommand::SheetSize, 10.0);
        assert!(!st.nav().forward);
    }

    #[test]
    fn a_view_left_as_fitted_fits_again_when_resized() {
        let (mut st, rect) = fitted();
        assert!(!st.needs_fit(rect));
        let wider = Rect::from_min_size(rect.min, egui::vec2(1200.0, 600.0));
        assert!(st.needs_fit(wider));
        st.fit((DVec2::ZERO, DVec2::new(20.0, 10.0)), wider);
        assert!(!st.needs_fit(wider));
        // Once zoomed by hand, resizing keeps the zoom.
        st.apply(ZoomCommand::OutTwo, 10.0);
        assert!(!st.needs_fit(rect));
    }

    #[test]
    fn sheet_size_shows_paper_millimeters_at_screen_size() {
        let (mut st, _) = fitted();
        // A plan at 1:50 has 20 paper mm per meter.
        st.apply(ZoomCommand::SheetSize, 20.0);
        assert!((st.ppm - 20.0 * 96.0 / 25.4).abs() < 1e-9);
    }
}
