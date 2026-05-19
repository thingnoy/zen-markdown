# zen-markdown

A fast, native markdown editor built with Rust + [egui](https://github.com/emilk/egui) — wearing a [Tokyo Night](https://github.com/tokyo-night/tokyo-night-vscode-theme) coat.

![CI](https://github.com/thingnoy/zen-markdown/actions/workflows/ci.yml/badge.svg)
![Release](https://img.shields.io/github/v/release/thingnoy/zen-markdown?display_name=tag)
![License](https://img.shields.io/badge/license-MIT-blue)

Preview-first. Mono everywhere. Starts in <200ms, stays under ~50MB RAM. No Electron.

## Install

### macOS

**Option A — Homebrew (recommended)**

```bash
brew install --cask thingnoy/tap/zen-markdown
```

**Option B — download the .dmg**

1. Grab `zen-markdown-macos-arm64.dmg` from the [latest release](https://github.com/thingnoy/zen-markdown/releases/latest).
2. Open it, drag **zen-markdown** into **Applications**.
3. First launch: the app is not notarized (no paid Apple Developer cert), so macOS will warn. Either:
   - **Right-click** the app → **Open** → **Open**, or
   - run once:
     ```bash
     xattr -dr com.apple.quarantine /Applications/zen-markdown.app
     ```

> Why the warning? Notarization needs an Apple Developer account ($99/yr). The binaries are built in public CI — you can read exactly how in [`.github/workflows/release.yml`](.github/workflows/release.yml).

### Linux

```bash
# install runtime deps (Debian/Ubuntu)
sudo apt-get install -y libgtk-3-0 libxkbcommon0

curl -L -o zen-markdown.tar.gz \
  https://github.com/thingnoy/zen-markdown/releases/latest/download/zen-markdown-linux-x86_64.tar.gz
tar xzf zen-markdown.tar.gz
./zen-markdown
```

### Windows

Download `zen-markdown-windows-x86_64.zip` from the [latest release](https://github.com/thingnoy/zen-markdown/releases/latest), unzip, run `zen-markdown.exe`. SmartScreen may warn on first run (unsigned) → **More info** → **Run anyway**.

### Build from source

```bash
git clone https://github.com/thingnoy/zen-markdown
cd zen-markdown
cargo run --release
```

Build a macOS `.app` locally:

```bash
bash packaging/macos/build-app.sh --install   # → /Applications/zen-markdown.app
```

## Features

- **Preview-first** split-pane editor with live render
- **Selection-aware formatting** — wrap the actual selection, not append
- **Syntax highlighting** in both editor and preview
- **Light / dark** themes (Tokyo Night / Tokyo Day)
- **Geist Mono** typography, color-coded heading levels
- Open / save `.md` with native dialogs, dirty tracking
- Word count + reading-time status bar

## Shortcuts

| Action | Shortcut |
| --- | --- |
| Open file | `Cmd-O` |
| Save | `Cmd-S` |
| Toggle editor | `Cmd-Shift-E` |
| Toggle reader | `Cmd-Shift-P` |
| Toggle theme | `Cmd-Shift-L` |
| Bold / Italic | `Cmd-B` / `Cmd-I` |
| Inline code | `Cmd-E` |
| Heading 1 / 2 | `Cmd-1` / `Cmd-2` |

(On Linux/Windows, `Cmd` → `Ctrl`.)

## Roadmap

- [ ] Inline WYSIWYG mode (experimental)
- [ ] Find / replace
- [ ] File tree sidebar
- [ ] Focus mode (dim non-active paragraph)
- [ ] Cmd-K command palette
- [ ] Mermaid + LaTeX

## License

MIT — see [LICENSE](LICENSE).
