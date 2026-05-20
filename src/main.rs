#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// nested `if let` reads clearer than collapsed forms until let-chains stabilize
#![allow(clippy::collapsible_if)]

use eframe::egui;
use std::path::PathBuf;

mod editor;
mod format;
mod preview;
mod theme;

use theme::{Palette, ThemeMode};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1120.0, 760.0])
            .with_min_inner_size([640.0, 440.0])
            .with_title("zen-markdown"),
        ..Default::default()
    };
    let initial_file = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .filter(|p| p.is_file());

    eframe::run_native(
        "zen-markdown",
        options,
        Box::new(move |cc| Ok(Box::new(ZenApp::new(cc, initial_file)))),
    )
}

pub struct ZenApp {
    text: String,
    current_file: Option<PathBuf>,
    dirty: bool,
    show_editor: bool,
    show_preview: bool,
    palette: Palette,
}

impl ZenApp {
    fn new(cc: &eframe::CreationContext<'_>, initial_file: Option<PathBuf>) -> Self {
        theme::install_fonts(&cc.egui_ctx);
        let palette = Palette::tokyo_night();
        theme::apply_visuals(&cc.egui_ctx, &palette);
        let mut app = Self {
            text: DEFAULT_DOC.to_string(),
            current_file: None,
            dirty: false,
            show_editor: false,
            show_preview: true,
            palette,
        };
        if let Some(path) = initial_file {
            app.load_path(path);
        }
        app
    }

    fn load_path(&mut self, path: PathBuf) {
        if let Ok(content) = std::fs::read_to_string(&path) {
            self.text = content;
            self.current_file = Some(path);
            self.dirty = false;
        }
    }

    fn toggle_theme(&mut self, ctx: &egui::Context) {
        self.palette = match self.palette.mode {
            ThemeMode::Dark => Palette::tokyo_day(),
            ThemeMode::Light => Palette::tokyo_night(),
        };
        theme::apply_visuals(ctx, &self.palette);
    }

    fn title(&self) -> String {
        let name = self
            .current_file
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "untitled".to_string());
        if self.dirty {
            format!("● {} — zen-markdown", name)
        } else {
            format!("{} — zen-markdown", name)
        }
    }

    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Markdown", &["md", "markdown", "txt"])
            .pick_file()
        {
            self.load_path(path);
        }
    }

    fn save_file(&mut self) {
        match &self.current_file {
            Some(path) => {
                if std::fs::write(path, &self.text).is_ok() {
                    self.dirty = false;
                }
            }
            None => self.save_as(),
        }
    }

    fn save_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Markdown", &["md"])
            .set_file_name("untitled.md")
            .save_file()
        {
            if std::fs::write(&path, &self.text).is_ok() {
                self.current_file = Some(path);
                self.dirty = false;
            }
        }
    }

    fn doc_stats(&self) -> (usize, usize, usize) {
        let chars = self.text.chars().count();
        let words = self.text.split_whitespace().count();
        let read_min = ((words as f32) / 220.0).ceil() as usize;
        (chars, words, read_min.max(1))
    }
}

impl eframe::App for ZenApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        let c = self.palette.bg;
        [
            c.r() as f32 / 255.0,
            c.g() as f32 / 255.0,
            c.b() as f32 / 255.0,
            1.0,
        ]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(self.title()));

        let cmd = if cfg!(target_os = "macos") {
            egui::Modifiers::MAC_CMD
        } else {
            egui::Modifiers::CTRL
        };

        let mut theme_toggle = false;
        let mut pending_format: Option<format::FormatAction> = None;
        ctx.input_mut(|i| {
            if i.consume_key(cmd, egui::Key::O) {
                self.open_file();
            }
            if i.consume_key(cmd, egui::Key::S) {
                self.save_file();
            }
            if i.consume_key(cmd | egui::Modifiers::SHIFT, egui::Key::S) {
                self.save_as();
            }
            if i.consume_key(cmd | egui::Modifiers::SHIFT, egui::Key::E) {
                self.show_editor = !self.show_editor;
                if !self.show_editor && !self.show_preview {
                    self.show_preview = true;
                }
            }
            if i.consume_key(cmd | egui::Modifiers::SHIFT, egui::Key::P) {
                self.show_preview = !self.show_preview;
                if !self.show_editor && !self.show_preview {
                    self.show_editor = true;
                }
            }
            if i.consume_key(cmd | egui::Modifiers::SHIFT, egui::Key::L) {
                theme_toggle = true;
            }
        });

        // open a file dragged onto the window
        let dropped = ctx.input(|i| i.raw.dropped_files.iter().find_map(|f| f.path.clone()));
        if let Some(path) = dropped {
            self.load_path(path);
            self.show_editor = true;
        }
        let hovering_file = ctx.input(|i| !i.raw.hovered_files.is_empty());

        egui::TopBottomPanel::top("toolbar")
            .frame(
                egui::Frame::none()
                    .fill(self.palette.bg_alt)
                    .inner_margin(egui::Margin::symmetric(14.0, 8.0))
                    .stroke(egui::Stroke::new(1.0, self.palette.stroke)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("zen")
                            .size(13.0)
                            .color(self.palette.accent)
                            .strong(),
                    );
                    ui.label(
                        egui::RichText::new("·")
                            .size(13.0)
                            .color(self.palette.muted),
                    );
                    ui.label(
                        egui::RichText::new("markdown")
                            .size(13.0)
                            .color(self.palette.text),
                    );
                    ui.add_space(12.0);

                    ui.menu_button(
                        egui::RichText::new("file")
                            .color(self.palette.text)
                            .size(12.0),
                        |ui| {
                            if ui.button("open…").clicked() {
                                self.open_file();
                                ui.close_menu();
                            }
                            if ui.button("save").clicked() {
                                self.save_file();
                                ui.close_menu();
                            }
                            if ui.button("save as…").clicked() {
                                self.save_as();
                                ui.close_menu();
                            }
                        },
                    );

                    if self.show_editor {
                        ui.add_space(8.0);
                        if let Some(a) = format::toolbar(ui, &self.palette) {
                            pending_format = Some(a);
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let glyph = match self.palette.mode {
                            ThemeMode::Dark => "○",
                            ThemeMode::Light => "●",
                        };
                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new(glyph)
                                        .size(13.0)
                                        .color(self.palette.accent),
                                )
                                .min_size(egui::vec2(26.0, 22.0))
                                .rounding(egui::Rounding::same(4.0))
                                .fill(self.palette.bg_alt),
                            )
                            .on_hover_text("toggle theme (Cmd-Shift-L)")
                            .clicked()
                        {
                            theme_toggle = true;
                        }
                        ui.add_space(6.0);

                        ui.toggle_value(&mut self.show_preview, " read ")
                            .on_hover_text("Cmd-Shift-P");
                        ui.toggle_value(&mut self.show_editor, " edit ")
                            .on_hover_text("Cmd-Shift-E");
                        if !self.show_editor && !self.show_preview {
                            self.show_preview = true;
                        }
                    });
                });
            });

        egui::TopBottomPanel::bottom("statusbar")
            .frame(
                egui::Frame::none()
                    .fill(self.palette.bg_alt)
                    .inner_margin(egui::Margin {
                        left: 14.0,
                        right: 14.0,
                        top: 6.0,
                        bottom: 6.0,
                    })
                    .stroke(egui::Stroke::new(1.0, self.palette.stroke)),
            )
            .show(ctx, |ui| {
                let (chars, words, read_min) = self.doc_stats();
                ui.horizontal(|ui| {
                    let muted = self.palette.muted;
                    let acc = self.palette.accent_cyan;
                    ui.label(
                        egui::RichText::new(format!("{}w", words))
                            .size(11.0)
                            .color(acc),
                    );
                    ui.label(
                        egui::RichText::new(format!("· {}c", chars))
                            .size(11.0)
                            .color(muted),
                    );
                    ui.label(
                        egui::RichText::new(format!("· {}m", read_min))
                            .size(11.0)
                            .color(muted),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let name = self
                            .current_file
                            .as_ref()
                            .and_then(|p| p.file_name())
                            .map(|n| n.to_string_lossy().into_owned())
                            .unwrap_or_else(|| "untitled.md".to_string());
                        let dot = if self.dirty { "● " } else { "" };
                        ui.label(
                            egui::RichText::new(format!("{}{}", dot, name))
                                .size(11.0)
                                .color(if self.dirty {
                                    self.palette.accent_orange
                                } else {
                                    self.palette.muted
                                }),
                        );
                    });
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(self.palette.bg))
            .show(ctx, |ui| {
                let available = ui.available_size();
                ui.horizontal_top(|ui| {
                    let pane_width = match (self.show_editor, self.show_preview) {
                        (true, true) => available.x * 0.5,
                        _ => available.x,
                    };

                    if self.show_editor {
                        let before = self.text.clone();
                        ui.allocate_ui_with_layout(
                            egui::vec2(pane_width, available.y),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                editor::show(
                                    ui,
                                    &mut self.text,
                                    &self.palette,
                                    pending_format.take(),
                                );
                            },
                        );
                        if before != self.text {
                            self.dirty = true;
                        }
                    }

                    if self.show_preview {
                        ui.allocate_ui_with_layout(
                            egui::vec2(ui.available_width(), available.y),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                preview::show(ui, &self.text, &self.palette);
                            },
                        );
                    }
                });
            });

        if hovering_file {
            let screen = ctx.screen_rect();
            let painter = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("drop_overlay"),
            ));
            painter.rect_filled(screen, 0.0, self.palette.bg.gamma_multiply(0.85));
            painter.text(
                screen.center(),
                egui::Align2::CENTER_CENTER,
                "drop a .md file to open",
                egui::FontId::new(20.0, egui::FontFamily::Name("zen-mono".into())),
                self.palette.accent,
            );
        }

        if theme_toggle {
            self.toggle_theme(ctx);
        }
    }
}

const DEFAULT_DOC: &str = "# zen-markdown

A fast markdown editor in Rust + egui.

Press `Cmd-Shift-E` to write. `Cmd-Shift-L` toggles theme.

## headings carry color

# h1 — blue
## h2 — magenta
### h3 — cyan
#### h4 — purple

## shortcuts

- `Cmd-O` open
- `Cmd-S` save
- `Cmd-Shift-E` toggle editor
- `Cmd-Shift-P` toggle reader
- `Cmd-Shift-L` toggle theme
- `Cmd-B` / `Cmd-I` bold / italic

## code blocks tint syntax

```rust
fn main() {
    let greeting = \"hello, tokyo\";
    println!(\"{}\", greeting);
}
```

## inline

`code` lives in muted cyan on dark gutter. Links like [vercel.com](https://vercel.com) glow cyan. **bold** stays text color but heavier. *italic* leans softly.

> blockquotes earn a blue spine — quiet, structural.

---

Built with Rust + egui · Geist Mono · Tokyo Night
";
