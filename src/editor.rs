use eframe::egui;
use egui::text::{CCursor, CCursorRange};

use crate::format::FormatAction;
use crate::theme::Palette;

const EDITOR_ID: &str = "zen_editor";

pub fn show(ui: &mut egui::Ui, text: &mut String, p: &Palette, action: Option<FormatAction>) {
    let palette = p.clone();
    let mut layouter = move |ui: &egui::Ui, src: &str, wrap_width: f32| {
        let mut job = highlight(src, ui.style(), &palette);
        job.wrap.max_width = wrap_width;
        ui.fonts(|f| f.layout_job(job))
    };

    let id = egui::Id::new(EDITOR_ID);

    egui::Frame::none()
        .fill(p.bg_alt)
        .stroke(egui::Stroke::new(1.0, p.stroke))
        .inner_margin(egui::Margin::symmetric(20.0, 18.0))
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("editor_scroll")
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let output = egui::TextEdit::multiline(text)
                        .id(id)
                        .font(egui::TextStyle::Monospace)
                        .frame(false)
                        .desired_rows(30)
                        .desired_width(f32::INFINITY)
                        .lock_focus(true)
                        .layouter(&mut layouter)
                        .show(ui);

                    if let Some(action) = action {
                        apply_action(ui.ctx(), id, output.state, text, action);
                    }
                });
        });
}

fn apply_action(
    ctx: &egui::Context,
    id: egui::Id,
    mut state: egui::text_edit::TextEditState,
    text: &mut String,
    action: FormatAction,
) {
    let (cmin, cmax) = match state.cursor.char_range() {
        Some(range) => {
            let a = range.primary.index;
            let b = range.secondary.index;
            (a.min(b), a.max(b))
        }
        None => {
            let n = text.chars().count();
            (n, n)
        }
    };

    let new_range = match action {
        FormatAction::Wrap(left, right) => {
            let bstart = char_to_byte(text, cmin);
            let bend = char_to_byte(text, cmax);
            let selected = text[bstart..bend].to_string();
            let replacement = format!("{}{}{}", left, selected, right);
            text.replace_range(bstart..bend, &replacement);
            let left_len = left.chars().count();
            CCursorRange::two(
                CCursor::new(cmin + left_len),
                CCursor::new(cmax + left_len),
            )
        }
        FormatAction::LinePrefix(prefix) => {
            let bstart = char_to_byte(text, cmin);
            let line_start = text[..bstart].rfind('\n').map(|i| i + 1).unwrap_or(0);
            text.insert_str(line_start, prefix);
            let plen = prefix.chars().count();
            CCursorRange::one(CCursor::new(cmin + plen))
        }
        FormatAction::Insert(s) => {
            let bstart = char_to_byte(text, cmin);
            text.insert_str(bstart, s);
            CCursorRange::one(CCursor::new(cmin + s.chars().count()))
        }
    };

    state.cursor.set_char_range(Some(new_range));
    state.store(ctx, id);
    ctx.memory_mut(|m| m.request_focus(id));
}

fn char_to_byte(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}

fn highlight(text: &str, style: &egui::Style, p: &Palette) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    let font_id = egui::TextStyle::Monospace.resolve(style);

    for line in text.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            push(&mut job, line, p.accent_cyan, &font_id, false);
        } else if let Some(level) = heading_level(trimmed) {
            let color = p.heading_color(level as u8);
            let mut size = font_id.clone();
            size.size *= 1.0 + (0.35 - 0.04 * level as f32).max(0.0);
            push(&mut job, line, color, &size, false);
        } else if trimmed.starts_with("> ") {
            push(&mut job, line, p.muted, &font_id, true);
        } else if trimmed.starts_with("- ")
            || trimmed.starts_with("* ")
            || trimmed.starts_with("+ ")
        {
            inline_highlight(&mut job, line, &font_id, p);
        } else {
            inline_highlight(&mut job, line, &font_id, p);
        }
    }
    job
}

fn heading_level(s: &str) -> Option<usize> {
    let hashes = s.chars().take_while(|&c| c == '#').count();
    if hashes >= 1 && hashes <= 6 && s.chars().nth(hashes) == Some(' ') {
        Some(hashes)
    } else {
        None
    }
}

fn push(
    job: &mut egui::text::LayoutJob,
    text: &str,
    color: egui::Color32,
    font_id: &egui::FontId,
    italics: bool,
) {
    job.append(
        text,
        0.0,
        egui::TextFormat {
            font_id: font_id.clone(),
            color,
            italics,
            ..Default::default()
        },
    );
}

fn inline_highlight(
    job: &mut egui::text::LayoutJob,
    line: &str,
    font_id: &egui::FontId,
    p: &Palette,
) {
    let bytes = line.as_bytes();
    let mut i = 0usize;
    let mut start = 0usize;

    while i < bytes.len() {
        let rest = &line[i..];
        if rest.starts_with("**") {
            if let Some(end) = rest[2..].find("**") {
                flush(job, &line[start..i], p.text, font_id);
                push(job, &line[i..i + 2 + end + 2], p.heading, font_id, false);
                i += 2 + end + 2;
                start = i;
                continue;
            }
        }
        if rest.starts_with('`') {
            if let Some(end) = rest[1..].find('`') {
                flush(job, &line[start..i], p.text, font_id);
                let segment = &line[i..i + 1 + end + 1];
                job.append(
                    segment,
                    0.0,
                    egui::TextFormat {
                        font_id: font_id.clone(),
                        color: p.code_inline_fg,
                        background: p.code_inline_bg,
                        ..Default::default()
                    },
                );
                i += 1 + end + 1;
                start = i;
                continue;
            }
        }
        if rest.starts_with('[') {
            if let Some(end_bracket) = rest.find(']') {
                if rest.get(end_bracket + 1..end_bracket + 2) == Some("(") {
                    if let Some(end_paren) = rest[end_bracket + 2..].find(')') {
                        let total = end_bracket + 2 + end_paren + 1;
                        flush(job, &line[start..i], p.text, font_id);
                        push(job, &line[i..i + total], p.accent_cyan, font_id, false);
                        i += total;
                        start = i;
                        continue;
                    }
                }
            }
        }
        let b = bytes[i];
        if b == b'*' || b == b'_' {
            let marker = b as char;
            if let Some(end) = line[i + 1..].find(marker) {
                flush(job, &line[start..i], p.text, font_id);
                push(job, &line[i..i + 1 + end + 1], p.text, font_id, true);
                i += 1 + end + 1;
                start = i;
                continue;
            }
        }
        i += 1;
    }
    flush(job, &line[start..], p.text, font_id);
}

fn flush(job: &mut egui::text::LayoutJob, s: &str, color: egui::Color32, font_id: &egui::FontId) {
    if !s.is_empty() {
        push(job, s, color, font_id, false);
    }
}
