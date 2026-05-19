use eframe::egui;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

use crate::theme::Palette;

pub fn show(ui: &mut egui::Ui, src: &str, p: &Palette) {
    egui::ScrollArea::vertical()
        .id_salt("preview_scroll")
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            egui::Frame::none()
                .inner_margin(egui::Margin::symmetric(36.0, 28.0))
                .show(ui, |ui| {
                    ui.style_mut().spacing.item_spacing.y = 12.0;
                    ui.set_max_width(820.0);
                    render(ui, src, p);
                });
        });
}

#[derive(Default, Clone)]
struct InlineStyle {
    bold: bool,
    italic: bool,
    code: bool,
    strike: bool,
    heading: Option<HeadingLevel>,
}

fn render(ui: &mut egui::Ui, src: &str, p: &Palette) {
    let parser = Parser::new_ext(src, pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    let mut style = InlineStyle::default();
    let mut job = egui::text::LayoutJob::default();
    let mut in_code_block = false;
    let mut code_buf = String::new();
    let mut code_lang = String::new();
    let mut in_blockquote = false;
    let mut list_depth: usize = 0;
    let mut ordered_stack: Vec<Option<u64>> = Vec::new();
    let mut suppress_paragraph_break = false;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {
                    if suppress_paragraph_break {
                        suppress_paragraph_break = false;
                    } else {
                        emit_block(ui, &mut job, in_blockquote, p);
                    }
                }
                Tag::Heading { level, .. } => {
                    emit_block(ui, &mut job, in_blockquote, p);
                    style.heading = Some(level);
                    add_heading_spacing(ui, level);
                }
                Tag::BlockQuote(_) => {
                    emit_block(ui, &mut job, in_blockquote, p);
                    in_blockquote = true;
                }
                Tag::CodeBlock(kind) => {
                    emit_block(ui, &mut job, in_blockquote, p);
                    in_code_block = true;
                    code_lang = match kind {
                        pulldown_cmark::CodeBlockKind::Fenced(s) => s.into_string(),
                        _ => String::new(),
                    };
                }
                Tag::List(start) => {
                    emit_block(ui, &mut job, in_blockquote, p);
                    list_depth += 1;
                    ordered_stack.push(start);
                }
                Tag::Item => {
                    emit_block(ui, &mut job, in_blockquote, p);
                    let indent = "  ".repeat(list_depth.saturating_sub(1));
                    let marker = match ordered_stack.last_mut() {
                        Some(Some(n)) => {
                            let s = format!("{}{}. ", indent, n);
                            *n += 1;
                            s
                        }
                        _ => format!("{}- ", indent),
                    };
                    append_with_color(&mut job, &marker, &InlineStyle::default(), p, ui, Some(p.accent_cyan));
                    suppress_paragraph_break = true;
                }
                Tag::Emphasis => style.italic = true,
                Tag::Strong => style.bold = true,
                Tag::Strikethrough => style.strike = true,
                Tag::Link { dest_url, .. } => {
                    append_with_color(
                        &mut job,
                        &format!("→ {}", dest_url),
                        &InlineStyle::default(),
                        p,
                        ui,
                        Some(p.accent_cyan),
                    );
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Paragraph => emit_block(ui, &mut job, in_blockquote, p),
                TagEnd::Heading(_) => {
                    emit_block(ui, &mut job, in_blockquote, p);
                    style.heading = None;
                }
                TagEnd::BlockQuote(_) => {
                    emit_block(ui, &mut job, in_blockquote, p);
                    in_blockquote = false;
                }
                TagEnd::CodeBlock => {
                    render_code_block(ui, &std::mem::take(&mut code_buf), &code_lang, p);
                    in_code_block = false;
                    code_lang.clear();
                }
                TagEnd::List(_) => {
                    emit_block(ui, &mut job, in_blockquote, p);
                    list_depth = list_depth.saturating_sub(1);
                    ordered_stack.pop();
                }
                TagEnd::Item => emit_block(ui, &mut job, in_blockquote, p),
                TagEnd::Emphasis => style.italic = false,
                TagEnd::Strong => style.bold = false,
                TagEnd::Strikethrough => style.strike = false,
                _ => {}
            },
            Event::Text(t) => {
                if in_code_block {
                    code_buf.push_str(&t);
                } else {
                    append(&mut job, &t, &style, in_blockquote, p, ui);
                }
            }
            Event::Code(t) => {
                let mut s = style.clone();
                s.code = true;
                append(&mut job, &t, &s, in_blockquote, p, ui);
            }
            Event::SoftBreak => append(&mut job, " ", &style, in_blockquote, p, ui),
            Event::HardBreak => append(&mut job, "\n", &style, in_blockquote, p, ui),
            Event::Rule => {
                emit_block(ui, &mut job, in_blockquote, p);
                soft_rule(ui, p);
            }
            Event::TaskListMarker(checked) => {
                let m = if checked { "[x] " } else { "[ ] " };
                append_with_color(&mut job, m, &style, p, ui, Some(p.accent_green));
            }
            _ => {}
        }
    }
    emit_block(ui, &mut job, in_blockquote, p);
}

fn emit_block(ui: &mut egui::Ui, job: &mut egui::text::LayoutJob, in_blockquote: bool, p: &Palette) {
    if job.text.is_empty() {
        return;
    }
    let taken = std::mem::take(job);
    let mut taken = taken;
    taken.wrap.max_width = ui.available_width() - if in_blockquote { 28.0 } else { 0.0 };

    if in_blockquote {
        egui::Frame::none()
            .fill(p.quote_bg)
            .inner_margin(egui::Margin {
                left: 18.0,
                right: 14.0,
                top: 10.0,
                bottom: 10.0,
            })
            .rounding(egui::Rounding {
                nw: 0.0,
                ne: 5.0,
                sw: 0.0,
                se: 5.0,
            })
            .show(ui, |ui| {
                let rect = ui.min_rect();
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(rect.left() - 18.0, rect.top()),
                        egui::vec2(3.0, rect.height()),
                    ),
                    egui::Rounding::same(1.5),
                    p.accent,
                );
                ui.label(taken);
            });
    } else {
        ui.label(taken);
    }
}

fn append(
    job: &mut egui::text::LayoutJob,
    text: &str,
    style: &InlineStyle,
    in_blockquote: bool,
    p: &Palette,
    ui: &egui::Ui,
) {
    append_with_color(job, text, style, p, ui, None);
    let _ = in_blockquote;
}

fn append_with_color(
    job: &mut egui::text::LayoutJob,
    text: &str,
    style: &InlineStyle,
    p: &Palette,
    ui: &egui::Ui,
    override_color: Option<egui::Color32>,
) {
    let (font_id, base_color) = resolve_font_and_color(style, p, ui.style());
    let color = override_color.unwrap_or(base_color);
    let bg = if style.code {
        p.code_inline_bg
    } else {
        egui::Color32::TRANSPARENT
    };
    let line_height = if style.heading.is_some() {
        crate::theme::heading_line_height(font_id.size)
    } else if !style.code {
        crate::theme::body_line_height(font_id.size)
    } else {
        font_id.size * 1.55
    };
    job.append(
        text,
        0.0,
        egui::TextFormat {
            font_id,
            color,
            italics: style.italic,
            background: bg,
            strikethrough: if style.strike {
                egui::Stroke::new(1.0, color)
            } else {
                egui::Stroke::NONE
            },
            line_height: Some(line_height),
            ..Default::default()
        },
    );
}

fn resolve_font_and_color(
    style: &InlineStyle,
    p: &Palette,
    ui_style: &egui::Style,
) -> (egui::FontId, egui::Color32) {
    if let Some(level) = style.heading {
        let scale = match level {
            HeadingLevel::H1 => 1.85,
            HeadingLevel::H2 => 1.55,
            HeadingLevel::H3 => 1.3,
            HeadingLevel::H4 => 1.15,
            HeadingLevel::H5 => 1.05,
            HeadingLevel::H6 => 1.0,
        };
        let base = egui::TextStyle::Heading.resolve(ui_style);
        let id = egui::FontId::new(
            base.size * scale,
            egui::FontFamily::Name("zen-mono".into()),
        );
        let lvl_n = match level {
            HeadingLevel::H1 => 1,
            HeadingLevel::H2 => 2,
            HeadingLevel::H3 => 3,
            HeadingLevel::H4 => 4,
            HeadingLevel::H5 => 5,
            HeadingLevel::H6 => 6,
        };
        return (id, p.heading_color(lvl_n));
    }

    if style.code {
        return (
            egui::TextStyle::Monospace.resolve(ui_style),
            p.code_inline_fg,
        );
    }

    let body = egui::TextStyle::Body.resolve(ui_style);
    let color = if style.bold {
        p.heading
    } else {
        p.text
    };
    (body, color)
}

fn add_heading_spacing(ui: &mut egui::Ui, level: HeadingLevel) {
    let extra = match level {
        HeadingLevel::H1 => 14.0,
        HeadingLevel::H2 => 10.0,
        HeadingLevel::H3 => 6.0,
        _ => 2.0,
    };
    ui.add_space(extra);
}

fn soft_rule(ui: &mut egui::Ui, p: &Palette) {
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 18.0),
        egui::Sense::hover(),
    );
    let center_y = rect.center().y;
    ui.painter().line_segment(
        [
            egui::pos2(rect.left() + 4.0, center_y),
            egui::pos2(rect.right() - 4.0, center_y),
        ],
        egui::Stroke::new(1.0, p.stroke),
    );
}

fn render_code_block(ui: &mut egui::Ui, code: &str, lang: &str, p: &Palette) {
    egui::Frame::none()
        .fill(p.code_block_bg)
        .inner_margin(egui::Margin {
            left: 16.0,
            right: 16.0,
            top: 10.0,
            bottom: 14.0,
        })
        .rounding(egui::Rounding::same(6.0))
        .stroke(egui::Stroke::new(1.0, p.code_block_border))
        .show(ui, |ui| {
            if !lang.trim().is_empty() {
                let avail = ui.available_width();
                ui.allocate_ui_with_layout(
                    egui::vec2(avail, 14.0),
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        ui.label(
                            egui::RichText::new(lang.trim().to_lowercase())
                                .size(10.5)
                                .color(p.accent_cyan)
                                .family(egui::FontFamily::Name("zen-mono".into())),
                        );
                    },
                );
                ui.add_space(2.0);
            }
            let font_id = egui::TextStyle::Monospace.resolve(ui.style());
            let mut job = egui::text::LayoutJob::default();
            let highlighted = highlight_simple_rust(code.trim_end_matches('\n'), &font_id, p);
            for (text, color) in highlighted {
                job.append(
                    &text,
                    0.0,
                    egui::TextFormat {
                        font_id: font_id.clone(),
                        color,
                        line_height: Some(font_id.size * 1.55),
                        ..Default::default()
                    },
                );
            }
            job.wrap.max_width = ui.available_width();
            ui.label(job);
        });
}

fn highlight_simple_rust(
    code: &str,
    _font_id: &egui::FontId,
    p: &Palette,
) -> Vec<(String, egui::Color32)> {
    const KEYWORDS: &[&str] = &[
        "fn", "let", "mut", "const", "static", "struct", "enum", "impl", "trait",
        "pub", "use", "mod", "if", "else", "match", "for", "while", "loop",
        "return", "break", "continue", "self", "Self", "as", "in", "where",
        "async", "await", "move", "ref", "true", "false",
    ];
    let mut result: Vec<(String, egui::Color32)> = Vec::new();
    let mut buf = String::new();
    let mut chars = code.chars().peekable();

    let flush_buf = |buf: &mut String, result: &mut Vec<(String, egui::Color32)>, p: &Palette| {
        if buf.is_empty() {
            return;
        }
        let s = std::mem::take(buf);
        let color = if KEYWORDS.contains(&s.as_str()) {
            p.accent_soft
        } else if s.chars().next().map_or(false, |c| c.is_ascii_uppercase()) {
            p.accent_cyan
        } else if s.chars().next().map_or(false, |c| c.is_ascii_digit()) {
            p.accent_orange
        } else {
            p.code_block_fg
        };
        result.push((s, color));
    };

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                flush_buf(&mut buf, &mut result, p);
                let mut s = String::from('"');
                while let Some(&nc) = chars.peek() {
                    s.push(nc);
                    chars.next();
                    if nc == '"' {
                        break;
                    }
                    if nc == '\\' {
                        if let Some(&esc) = chars.peek() {
                            s.push(esc);
                            chars.next();
                        }
                    }
                }
                result.push((s, p.accent_green));
            }
            '/' if chars.peek() == Some(&'/') => {
                flush_buf(&mut buf, &mut result, p);
                let mut s = String::from('/');
                while let Some(&nc) = chars.peek() {
                    s.push(nc);
                    chars.next();
                    if nc == '\n' {
                        break;
                    }
                }
                result.push((s, p.muted));
            }
            c if c.is_alphanumeric() || c == '_' => {
                buf.push(c);
            }
            c => {
                flush_buf(&mut buf, &mut result, p);
                let color = match c {
                    '(' | ')' | '{' | '}' | '[' | ']' => p.muted,
                    ';' | ',' | '.' | ':' => p.muted,
                    '=' | '+' | '-' | '*' | '%' | '&' | '|' | '!' | '<' | '>' => p.accent,
                    _ => p.code_block_fg,
                };
                result.push((c.to_string(), color));
            }
        }
    }
    flush_buf(&mut buf, &mut result, p);
    result
}
