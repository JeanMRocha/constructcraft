//! Text notes and their leaders. Sizes are paper millimeters converted with
//! the view scale, like the other annotations; text widths are estimated
//! from the character count (the same estimate wraps multiline text, places
//! leaders and makes the text clickable).

use crate::drawing::Anchor;
use crate::{Drawing2D, LineStyle};
use cc_core::{ElementId, ElementKind, Leader, LeaderSymbol, Project, TextNote, TypeData, ViewId};
use glam::DVec2;

/// Average character width and line height, as fractions of the text size.
const CHAR_WIDTH: f64 = 0.6;
const LINE_HEIGHT: f64 = 1.6;
/// Fill of leader symbols.
const SYMBOL_FILL: [u8; 4] = [40, 40, 40, 255];
/// Nearly invisible fill that makes the text clickable.
const PICK_AREA: [u8; 4] = [255, 255, 255, 1];

/// Lines of `text`: explicit line breaks, and words wrapped to `width`
/// paper millimeters when given (a word longer than the width is split).
pub fn wrap(text: &str, size: f64, width: Option<f64>) -> Vec<String> {
    let per_line = width.map(|w| ((w / (size * CHAR_WIDTH)).floor() as usize).max(1));
    let mut out = Vec::new();
    for paragraph in text.split('\n') {
        let Some(max) = per_line else {
            out.push(paragraph.trim_end().to_string());
            continue;
        };
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            let mut word: Vec<char> = word.chars().collect();
            // Split words that do not fit on a line of their own.
            while word.len() > max {
                if !line.is_empty() {
                    out.push(std::mem::take(&mut line));
                }
                out.push(word.drain(..max).collect());
            }
            let word: String = word.into_iter().collect();
            let len = line.chars().count();
            if len > 0 && len + 1 + word.chars().count() > max {
                out.push(std::mem::take(&mut line));
            }
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(&word);
        }
        out.push(line);
    }
    out
}

/// Where a note's text goes, in its view's drawing coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct TextLayout {
    /// Baseline start of each line.
    pub lines: Vec<(DVec2, String)>,
    /// Text angle, radians.
    pub angle: f64,
    /// Text size, paper millimeters.
    pub size: f64,
    /// Corners of the text box (top left, top right, bottom right, bottom left).
    pub outline: [DVec2; 4],
    /// Leader attachment points beside the first line, on the left and right.
    pub sides: [DVec2; 2],
}

impl TextLayout {
    /// The attachment point on the side facing `toward`.
    pub fn attach(&self, toward: DVec2) -> DVec2 {
        let [l, r] = self.sides;
        if toward.distance(l) <= toward.distance(r) { l } else { r }
    }
}

/// Layout of `note` with text of `size` paper millimeters at `scale`.
pub fn text_layout(note: &TextNote, size: f64, scale: u32) -> TextLayout {
    let k = scale as f64 / 1000.0;
    let lines = wrap(&note.text, size, note.width);
    let longest = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0) as f64;
    let width = note.width.unwrap_or(longest * size * CHAR_WIDTH).max(size * CHAR_WIDTH);
    let height = lines.len().max(1) as f64 * size * LINE_HEIGHT;
    let angle = note.rotation_deg.to_radians();
    let rot = DVec2::from_angle(angle);
    // Local paper millimeters (x right, y up from the top left) to the view.
    let at = |x: f64, y: f64| note.position + rot.rotate(DVec2::new(x, y) * k);
    let first_middle = -size * 0.8;
    TextLayout {
        lines: lines
            .into_iter()
            .enumerate()
            .map(|(i, text)| {
                let line_width = text.chars().count() as f64 * size * CHAR_WIDTH;
                let x = match note.alignment {
                    cc_core::TextAlignment::Left => 0.0,
                    cc_core::TextAlignment::Center => (width - line_width) * 0.5,
                    cc_core::TextAlignment::Right => width - line_width,
                };
                (at(x, -(i as f64 * LINE_HEIGHT + 1.25) * size), text)
            })
            .collect(),
        angle,
        size,
        outline: [at(0.0, 0.0), at(width, 0.0), at(width, -height), at(0.0, -height)],
        sides: [at(-0.5 * size, first_middle), at(width + 0.5 * size, first_middle)],
    }
}

/// The polyline of a leader: from the text to its elbow (if any) and end.
pub fn leader_points(layout: &TextLayout, leader: &Leader) -> Vec<DVec2> {
    let toward = leader.elbow.unwrap_or(leader.end);
    let mut pts = vec![layout.attach(toward)];
    pts.extend(leader.elbow);
    pts.push(leader.end);
    pts
}

/// The symbol at `end` of a leader coming from `from`, sized for text of
/// `size` paper millimeters at `scale`.
pub fn leader_symbol(
    d: &mut Drawing2D,
    symbol: LeaderSymbol,
    from: DVec2,
    end: DVec2,
    size: f64,
    scale: u32,
    id: Option<ElementId>,
) {
    let len = size.max(2.0) * 0.9 * scale as f64 / 1000.0;
    let u = (end - from).try_normalize().unwrap_or(DVec2::X);
    let v = u.perp();
    let style = LineStyle::Annotation;
    let circle = |r: f64| -> Vec<DVec2> {
        (0..24).map(|i| end + DVec2::from_angle(i as f64 / 24.0 * std::f64::consts::TAU) * r).collect()
    };
    let head = [end, end - u * len + v * len * 0.35, end - u * len - v * len * 0.35];
    let square = {
        let h = len * 0.3;
        vec![end + (u + v) * h, end + (-u + v) * h, end + (-u - v) * h, end + (u - v) * h]
    };
    match symbol {
        LeaderSymbol::Arrow => d.fill(head.to_vec(), SYMBOL_FILL, id),
        LeaderSymbol::OpenArrow => {
            d.line(end, head[1], style, id);
            d.line(end, head[2], style, id);
        }
        LeaderSymbol::Triangle => d.polyline(&head, true, style, id),
        LeaderSymbol::Dot => d.fill(circle(len * 0.25), SYMBOL_FILL, id),
        LeaderSymbol::Circle => d.polyline(&circle(len * 0.35), true, style, id),
        LeaderSymbol::Square => d.polyline(&square, true, style, id),
        LeaderSymbol::FilledSquare => d.fill(square, SYMBOL_FILL, id),
        LeaderSymbol::Tick => {
            let t = (u + v).normalize() * len * 0.5;
            d.line(end - t, end + t, style, id);
        }
        LeaderSymbol::None => {}
    }
}

/// Draw a note: its text, a clickable area and its leaders.
pub fn draw_note(d: &mut Drawing2D, note: &TextNote, size: f64, scale: u32, id: Option<ElementId>) {
    let layout = text_layout(note, size, scale);
    if let Some(id) = id {
        d.fill(layout.outline.to_vec(), PICK_AREA, Some(id));
    }
    for (pos, line) in &layout.lines {
        if !line.is_empty() {
            let face = crate::TextFace {
                family: note.font_family.clone(),
                bold: note.bold,
                italic: note.italic,
                underline: note.underline,
            };
            d.styled_tag_text(*pos, line.clone(), size, Anchor::LeftBottom, layout.angle, face, id);
            if note.underline {
                let width = line.chars().count() as f64 * size * CHAR_WIDTH * scale as f64 / 1000.0;
                let down = DVec2::from_angle(layout.angle).perp() * (size * 0.12 * scale as f64 / 1000.0);
                d.line(*pos - down, *pos + DVec2::from_angle(layout.angle) * width - down, LineStyle::Annotation, id);
            }
        }
    }
    for leader in &note.leaders {
        let pts = leader_points(&layout, leader);
        d.polyline(&pts, false, LineStyle::Annotation, id);
        if let [.., from, end] = pts.as_slice() {
            leader_symbol(d, leader.symbol, *from, *end, size, scale, id);
        }
    }
}

/// Text size of a note's type (paper millimeters).
pub fn note_size(project: &Project, type_id: cc_core::ElementTypeId) -> f64 {
    match project.element_type(type_id).map(|t| &t.data) {
        Some(TypeData::Text { size, .. }) => *size,
        _ => 2.5,
    }
}

/// The text notes placed in `view`.
pub fn notes(d: &mut Drawing2D, project: &Project, view: ViewId, scale: u32) {
    for e in project.elements.values() {
        let ElementKind::Text(note) = &e.kind else { continue };
        if note.view == view {
            draw_note(d, note, note.size_mm.unwrap_or_else(|| note_size(project, e.type_id)), scale, Some(e.id));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(text: &str, width: Option<f64>) -> TextNote {
        TextNote {
            view: ViewId(1),
            position: DVec2::new(10.0, 5.0),
            text: text.into(),
            size_mm: None,
            font_family: String::new(),
            alignment: cc_core::TextAlignment::Left,
            bold: false,
            italic: false,
            underline: false,
            width,
            rotation_deg: 0.0,
            leaders: Vec::new(),
        }
    }

    #[test]
    fn single_line_text_breaks_only_at_line_breaks() {
        assert_eq!(wrap("Concrete wall, see detail 3", 2.5, None), ["Concrete wall, see detail 3"]);
        assert_eq!(wrap("one\ntwo", 2.5, None), ["one", "two"]);
    }

    #[test]
    fn multiline_text_wraps_words_to_its_width() {
        // 15 mm at 2.5 mm text: 10 characters per line.
        assert_eq!(wrap("finish with mortar and paint", 2.5, Some(15.0)), ["finish", "with", "mortar and", "paint"]);
        assert_eq!(wrap("abcdefghijklmnopqrstu", 2.5, Some(15.0)), ["abcdefghij", "klmnopqrst", "u"]);
        assert_eq!(wrap("a\n\nb", 2.5, Some(15.0)), ["a", "", "b"]);
    }

    #[test]
    fn layout_scales_rotates_and_attaches_leaders() {
        // At 1:100, 1 paper mm is 0.1 m.
        let n = note("abc", None);
        let l = text_layout(&n, 2.5, 100);
        assert_eq!(l.outline[0], n.position);
        assert!((l.outline[1].x - (10.0 + 3.0 * 2.5 * 0.6 * 0.1)).abs() < 1e-12);
        assert!(l.lines[0].0.y < 5.0, "the first baseline is below the insertion point");
        // A leader to the left attaches on the left.
        assert!(l.attach(DVec2::new(0.0, 5.0)).x < 10.0);
        assert!(l.attach(DVec2::new(20.0, 5.0)).x > 10.0);
        let turned = text_layout(&TextNote { rotation_deg: 90.0, ..n }, 2.5, 100);
        assert!((turned.outline[1].x - 10.0).abs() < 1e-12 && turned.outline[1].y > 5.0, "{:?}", turned.outline);
    }

    #[test]
    fn leaders_have_one_or_two_segments_and_a_symbol() {
        let mut n = note("Note", None);
        n.leaders = vec![
            Leader { end: DVec2::new(0.0, 0.0), elbow: None, symbol: LeaderSymbol::Arrow },
            Leader { end: DVec2::new(20.0, 0.0), elbow: Some(DVec2::new(18.0, 5.0)), symbol: LeaderSymbol::Circle },
        ];
        let mut d = Drawing2D::default();
        draw_note(&mut d, &n, 2.5, 50, Some(ElementId(7)));
        let lines = d.prims.iter().filter(|p| matches!(p, crate::Prim::Line { .. })).count();
        // 1 segment + 2 segments + 24 for the circle.
        assert_eq!(lines, 1 + 2 + 24);
        assert_eq!(d.hit_test(DVec2::new(10.1, 4.95), 0.0), Some(ElementId(7)), "the text is clickable");
        for s in LeaderSymbol::ALL {
            let mut d = Drawing2D::default();
            leader_symbol(&mut d, s, DVec2::ZERO, DVec2::X, 2.5, 50, None);
            assert_eq!(d.prims.is_empty(), s == LeaderSymbol::None, "{s:?}");
        }
    }
}
