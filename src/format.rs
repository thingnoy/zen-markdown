use eframe::egui;

use crate::theme::Palette;

#[derive(Clone, Copy)]
pub enum FormatAction {
    Wrap(&'static str, &'static str),
    LinePrefix(&'static str),
    Insert(&'static str),
}

pub fn toolbar(ui: &mut egui::Ui, p: &Palette) -> Option<FormatAction> {
    let cmd = if cfg!(target_os = "macos") {
        egui::Modifiers::MAC_CMD
    } else {
        egui::Modifiers::CTRL
    };

    let mut action: Option<FormatAction> = None;

    if button(ui, "B", "Bold (Cmd-B)", true, p).clicked()
        || ui.input_mut(|i| i.consume_key(cmd, egui::Key::B))
    {
        action = Some(FormatAction::Wrap("**", "**"));
    }
    if button(ui, "I", "Italic (Cmd-I)", false, p).clicked()
        || ui.input_mut(|i| i.consume_key(cmd, egui::Key::I))
    {
        action = Some(FormatAction::Wrap("*", "*"));
    }
    if button(ui, "`", "Inline code (Cmd-E)", false, p).clicked()
        || ui.input_mut(|i| i.consume_key(cmd, egui::Key::E))
    {
        action = Some(FormatAction::Wrap("`", "`"));
    }
    if button(ui, "~", "Strikethrough", false, p).clicked() {
        action = Some(FormatAction::Wrap("~~", "~~"));
    }
    if button(ui, "H1", "Heading 1 (Cmd-1)", false, p).clicked()
        || ui.input_mut(|i| i.consume_key(cmd, egui::Key::Num1))
    {
        action = Some(FormatAction::LinePrefix("# "));
    }
    if button(ui, "H2", "Heading 2 (Cmd-2)", false, p).clicked()
        || ui.input_mut(|i| i.consume_key(cmd, egui::Key::Num2))
    {
        action = Some(FormatAction::LinePrefix("## "));
    }
    if button(ui, "-", "Bullet list", false, p).clicked() {
        action = Some(FormatAction::LinePrefix("- "));
    }
    if button(ui, ">", "Quote", false, p).clicked() {
        action = Some(FormatAction::LinePrefix("> "));
    }
    if button(ui, "[]", "Link", false, p).clicked() {
        action = Some(FormatAction::Insert("[text](url)"));
    }

    action
}

fn button(
    ui: &mut egui::Ui,
    label: &str,
    tooltip: &str,
    bold: bool,
    p: &Palette,
) -> egui::Response {
    let color = if bold { p.accent } else { p.text };
    let rich = egui::RichText::new(label).size(13.0).color(color);
    let btn = egui::Button::new(rich)
        .min_size(egui::vec2(28.0, 24.0))
        .rounding(egui::Rounding::same(4.0))
        .fill(p.bg_alt);
    ui.add(btn).on_hover_text(tooltip)
}
