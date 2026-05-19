# zen-markdown

A fast, native, zen-like markdown editor — built with Rust + egui.

## Why

Most cross-platform markdown editors are Electron — heavy, slow, hungry on RAM. zen-markdown renders natively via GPU through egui, starts in <200ms, and stays under 50MB RAM.

## Features (Phase 1)

- Split-pane editor + live preview
- Open / save `.md` files
- Syntax highlighting in editor
- Format toolbar + shortcuts (Cmd+B bold, Cmd+I italic, etc.)
- Native on macOS (Apple Silicon + Intel)

## Roadmap

- Phase 2: Inline WYSIWYG mode (experimental)
- Phase 2: Windows + Linux builds
- Phase 2: Mermaid + LaTeX

## Build

```bash
cargo run --release
```

## License

MIT
