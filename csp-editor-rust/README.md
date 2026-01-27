# CSP Editor (Rust TUI)

An interactive terminal-based Content Security Policy (CSP) builder and editor written in Rust.

## Features

- **Import existing CSP** - Parse and load an existing CSP policy string
- **Interactive directive builder** - Select from common CSP directives:
  - `default-src`, `script-src`, `style-src`, `img-src`, `connect-src`
  - `font-src`, `frame-src`, `media-src`, `object-src`, `manifest-src`
  - `base-uri`, `form-action`, `frame-ancestors`, `report-uri`, `report-to`
  - `worker-src`, `child-src`, `navigate-to`, `prefetch-src`
- **Value validation** - Validates CSP values including:
  - CSP Keywords (`'self'`, `'unsafe-inline'`, `'unsafe-eval'`, `'none'`, etc.)
  - URLs and domains (`https://example.com`, `example.com`)
  - Wildcards (`*.example.com`)
  - WebSocket URLs (`wss://`, `ws://`)
  - Special schemes (`data:`, `blob:`, `filesystem:`, `mediastream:`)
  - Nonce and hash values (`'nonce-...'`, `'sha256-...'`, etc.)
- **Real-time preview** - See the generated CSP string with character count
- **Copy to clipboard** - Export the generated CSP (when clipboard feature is enabled)
- **Keyboard-driven navigation** - Full TUI with intuitive controls

## Building

```bash
# Standard build
cargo build --release

# With clipboard support (requires system clipboard libraries)
cargo build --release --features clipboard
```

## Usage

```bash
./target/release/csp-editor
```

### Keyboard Controls

| Key | Action |
|-----|--------|
| `Tab` | Move to next field |
| `Shift+Tab` | Move to previous field |
| `←` / `→` | Change directive selection / Navigate values |
| `↑` / `↓` | Navigate directives list |
| `Enter` | Import CSP / Add value |
| `d` / `Delete` | Delete selected value |
| `c` | Copy CSP to clipboard |
| `x` | Clear all directives |
| `Esc` | Cancel / Clear input |
| `q` | Quit |
| `Ctrl+q` | Quit (from input fields) |

## Example CSP Strings

Import these example policies to get started:

```
default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'
```

```
default-src 'none'; script-src 'self'; connect-src 'self'; img-src 'self'; style-src 'self'; base-uri 'self'; form-action 'self'
```

## Dependencies

- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
- [regex](https://github.com/rust-lang/regex) - Value validation
- [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs) - Static regex compilation
- [copypasta](https://github.com/alacritty/copypasta) - Clipboard support (optional)

## License

MIT
