use eframe::egui;

use crate::theme::Palette;

pub fn toolbar(ui: &mut egui::Ui, text: &mut String, p: &Palette) {
    let cmd = if cfg!(target_os = "macos") {
        egui::Modifiers::MAC_CMD
    } else {
        egui::Modifiers::CTRL
    };

    if button(ui, "B", "Bold (Cmd-B)", true, p).clicked()
        || ui.input_mut(|i| i.consume_key(cmd, egui::Key::B))
    {
        wrap_selection(text, "**", "**");
    }
    if button(ui, "I", "Italic (Cmd-I)", false, p).clicked()
        || ui.input_mut(|i| i.consume_key(cmd, egui::Key::I))
    {
        wrap_selection(text, "*", "*");
    }
    if button(ui, "`", "Inline code (Cmd-E)", false, p).clicked()
        || ui.input_mut(|i| i.consume_key(cmd, egui::Key::E))
    {
        wrap_selection(text, "`", "`");
    }
    if button(ui, "H1", "Heading 1 (Cmd-1)", false, p).clicked()
        || ui.input_mut(|i| i.consume_key(cmd, egui::Key::Num1))
    {
        line_prefix(text, "# ");
    }
    if button(ui, "H2", "Heading 2 (Cmd-2)", false, p).clicked()
        || ui.input_mut(|i| i.consume_key(cmd, egui::Key::Num2))
    {
        line_prefix(text, "## ");
    }
    if button(ui, "-", "Bullet list", false, p).clicked() {
        line_prefix(text, "- ");
    }
    if button(ui, ">", "Quote", false, p).clicked() {
        line_prefix(text, "> ");
    }
    if button(ui, "[]", "Link", false, p).clicked() {
        text.push_str("[text](url)");
    }
}

fn button(ui: &mut egui::Ui, label: &str, tooltip: &str, bold: bool, p: &Palette) -> egui::Response {
    let color = if bold { p.accent } else { p.text };
    let rich = egui::RichText::new(label).size(13.0).color(color);
    let btn = egui::Button::new(rich)
        .min_size(egui::vec2(28.0, 24.0))
        .rounding(egui::Rounding::same(4.0))
        .fill(p.bg_alt);
    ui.add(btn).on_hover_text(tooltip)
}

fn wrap_selection(text: &mut String, left: &str, right: &str) {
    text.push_str(left);
    text.push_str(right);
}

fn line_prefix(text: &mut String, prefix: &str) {
    if !text.ends_with('\n') && !text.is_empty() {
        text.push('\n');
    }
    text.push_str(prefix);
}
