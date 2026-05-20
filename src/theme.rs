use eframe::egui;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Clone)]
pub struct Palette {
    pub mode: ThemeMode,
    pub bg: egui::Color32,
    pub bg_alt: egui::Color32,
    pub panel: egui::Color32,
    pub text: egui::Color32,
    pub heading: egui::Color32,
    pub muted: egui::Color32,
    pub accent: egui::Color32,
    pub accent_soft: egui::Color32,
    pub accent_cyan: egui::Color32,
    pub accent_purple: egui::Color32,
    pub accent_green: egui::Color32,
    pub accent_orange: egui::Color32,
    pub code_inline_bg: egui::Color32,
    pub code_inline_fg: egui::Color32,
    pub code_block_bg: egui::Color32,
    pub code_block_fg: egui::Color32,
    pub code_block_border: egui::Color32,
    pub quote_bg: egui::Color32,
    pub stroke: egui::Color32,
}

impl Palette {
    pub fn tokyo_night() -> Self {
        Self {
            mode: ThemeMode::Dark,
            bg: egui::Color32::from_rgb(0x1A, 0x1B, 0x26),
            bg_alt: egui::Color32::from_rgb(0x16, 0x16, 0x1E),
            panel: egui::Color32::from_rgb(0x1F, 0x23, 0x35),
            text: egui::Color32::from_rgb(0xC0, 0xCA, 0xF5),
            heading: egui::Color32::from_rgb(0x7A, 0xA2, 0xF7),
            muted: egui::Color32::from_rgb(0x56, 0x5F, 0x89),
            accent: egui::Color32::from_rgb(0x7A, 0xA2, 0xF7),
            accent_soft: egui::Color32::from_rgb(0xBB, 0x9A, 0xF7),
            accent_cyan: egui::Color32::from_rgb(0x7D, 0xCF, 0xFF),
            accent_purple: egui::Color32::from_rgb(0x9D, 0x7C, 0xD8),
            accent_green: egui::Color32::from_rgb(0x9E, 0xCE, 0x6A),
            accent_orange: egui::Color32::from_rgb(0xFF, 0x9E, 0x64),
            code_inline_bg: egui::Color32::from_rgb(0x29, 0x2E, 0x42),
            code_inline_fg: egui::Color32::from_rgb(0x7D, 0xCF, 0xFF),
            code_block_bg: egui::Color32::from_rgb(0x16, 0x16, 0x1E),
            code_block_fg: egui::Color32::from_rgb(0xC0, 0xCA, 0xF5),
            code_block_border: egui::Color32::from_rgb(0x29, 0x2E, 0x42),
            quote_bg: egui::Color32::from_rgb(0x1F, 0x23, 0x35),
            stroke: egui::Color32::from_rgb(0x29, 0x2E, 0x42),
        }
    }

    pub fn tokyo_day() -> Self {
        Self {
            mode: ThemeMode::Light,
            bg: egui::Color32::from_rgb(0xE1, 0xE2, 0xE7),
            bg_alt: egui::Color32::from_rgb(0xD5, 0xD6, 0xDB),
            panel: egui::Color32::from_rgb(0xD0, 0xD5, 0xE3),
            text: egui::Color32::from_rgb(0x3D, 0x42, 0x6B),
            heading: egui::Color32::from_rgb(0x2E, 0x7D, 0xE9),
            muted: egui::Color32::from_rgb(0x84, 0x8C, 0xB5),
            accent: egui::Color32::from_rgb(0x2E, 0x7D, 0xE9),
            accent_soft: egui::Color32::from_rgb(0x9E, 0x54, 0xF1),
            accent_cyan: egui::Color32::from_rgb(0x00, 0x7C, 0xB5),
            accent_purple: egui::Color32::from_rgb(0x59, 0x32, 0xA3),
            accent_green: egui::Color32::from_rgb(0x58, 0x7A, 0x05),
            accent_orange: egui::Color32::from_rgb(0xB1, 0x5C, 0x00),
            code_inline_bg: egui::Color32::from_rgb(0xCB, 0xCC, 0xD9),
            code_inline_fg: egui::Color32::from_rgb(0x07, 0x87, 0x9D),
            code_block_bg: egui::Color32::from_rgb(0xD5, 0xD6, 0xDB),
            code_block_fg: egui::Color32::from_rgb(0x3D, 0x42, 0x6B),
            code_block_border: egui::Color32::from_rgb(0xC4, 0xC8, 0xDA),
            quote_bg: egui::Color32::from_rgb(0xD5, 0xDA, 0xEB),
            stroke: egui::Color32::from_rgb(0xC4, 0xC8, 0xDA),
        }
    }

    pub fn heading_color(&self, level: u8) -> egui::Color32 {
        match level {
            1 => self.accent,
            2 => self.accent_soft,
            3 => self.accent_cyan,
            4 => self.accent_purple,
            _ => self.text,
        }
    }
}

pub fn apply_visuals(ctx: &egui::Context, p: &Palette) {
    let mut style = (*ctx.style()).clone();
    let v = &mut style.visuals;

    v.dark_mode = matches!(p.mode, ThemeMode::Dark);
    v.window_fill = p.bg;
    v.panel_fill = p.bg;
    v.extreme_bg_color = p.bg_alt;
    v.code_bg_color = p.code_block_bg;
    v.faint_bg_color = p.panel;

    v.override_text_color = Some(p.text);
    v.hyperlink_color = p.accent_cyan;
    v.warn_fg_color = p.accent_orange;
    v.error_fg_color = egui::Color32::from_rgb(0xF7, 0x76, 0x8E);

    let radius = egui::Rounding::same(5.0);

    v.widgets.noninteractive.bg_fill = p.panel;
    v.widgets.noninteractive.weak_bg_fill = p.panel;
    v.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, p.stroke);
    v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, p.text);
    v.widgets.noninteractive.rounding = radius;

    v.widgets.inactive.bg_fill = p.bg_alt;
    v.widgets.inactive.weak_bg_fill = p.bg_alt;
    v.widgets.inactive.bg_stroke = egui::Stroke::new(0.0, p.stroke);
    v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, p.muted);
    v.widgets.inactive.rounding = radius;

    v.widgets.hovered.bg_fill = p.panel;
    v.widgets.hovered.weak_bg_fill = p.panel;
    v.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, p.accent);
    v.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, p.text);
    v.widgets.hovered.rounding = radius;

    v.widgets.active.bg_fill = p.accent.linear_multiply(0.20);
    v.widgets.active.weak_bg_fill = p.accent.linear_multiply(0.16);
    v.widgets.active.bg_stroke = egui::Stroke::new(1.0, p.accent);
    v.widgets.active.fg_stroke = egui::Stroke::new(1.0, p.accent);
    v.widgets.active.rounding = radius;

    v.widgets.open.bg_fill = p.panel;
    v.widgets.open.weak_bg_fill = p.panel;
    v.widgets.open.bg_stroke = egui::Stroke::new(1.0, p.stroke);
    v.widgets.open.fg_stroke = egui::Stroke::new(1.0, p.text);
    v.widgets.open.rounding = radius;

    v.selection.bg_fill = p.accent.linear_multiply(0.30);
    v.selection.stroke = egui::Stroke::new(1.0, p.accent_cyan);

    v.text_cursor.stroke = egui::Stroke::new(2.0, p.accent_cyan);

    v.menu_rounding = egui::Rounding::same(6.0);
    v.window_rounding = egui::Rounding::same(8.0);
    v.window_shadow = egui::epaint::Shadow {
        offset: egui::vec2(0.0, 4.0),
        blur: 16.0,
        spread: 0.0,
        color: egui::Color32::from_black_alpha(60),
    };
    v.popup_shadow = v.window_shadow;

    style.spacing.item_spacing = egui::vec2(6.0, 6.0);
    style.spacing.button_padding = egui::vec2(10.0, 5.0);
    style.spacing.menu_margin = egui::Margin::symmetric(6.0, 5.0);

    ctx.set_style(style);
}

pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "geist-mono".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/GeistMono.ttf")),
    );
    fonts.font_data.insert(
        "geist".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/Geist.ttf")),
    );
    // Thai (and broad Unicode) fallback — Geist has no Thai glyphs, so without
    // this Thai text renders as "????".
    fonts.font_data.insert(
        "noto-thai".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/NotoSansThai.ttf")),
    );

    fonts.families.insert(
        egui::FontFamily::Name("zen-mono".into()),
        vec!["geist-mono".to_owned(), "noto-thai".to_owned()],
    );
    fonts.families.insert(
        egui::FontFamily::Name("zen-sans".into()),
        vec!["geist".to_owned(), "noto-thai".to_owned()],
    );

    {
        let mono = fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default();
        mono.insert(0, "geist-mono".to_owned());
        mono.push("noto-thai".to_owned());
    }
    {
        let prop = fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default();
        prop.insert(0, "geist".to_owned());
        prop.push("noto-thai".to_owned());
    }

    ctx.set_fonts(fonts);

    let mut style = (*ctx.style()).clone();
    use egui::{FontFamily, FontId, TextStyle};
    style.text_styles.insert(
        TextStyle::Body,
        FontId::new(14.5, FontFamily::Name("zen-mono".into())),
    );
    style.text_styles.insert(
        TextStyle::Button,
        FontId::new(12.5, FontFamily::Name("zen-sans".into())),
    );
    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(20.0, FontFamily::Name("zen-mono".into())),
    );
    style.text_styles.insert(
        TextStyle::Monospace,
        FontId::new(13.5, FontFamily::Name("zen-mono".into())),
    );
    style.text_styles.insert(
        TextStyle::Small,
        FontId::new(11.0, FontFamily::Name("zen-sans".into())),
    );
    ctx.set_style(style);
}

pub fn body_line_height(font_size: f32) -> f32 {
    font_size * 1.65
}

pub fn heading_line_height(font_size: f32) -> f32 {
    font_size * 1.3
}
