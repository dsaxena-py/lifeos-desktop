# Aeon Desktop

A native macOS/Windows shell around the deployed web app (`https://aeonos.app`)
— a real dock/taskbar app with its own window, not just a pinned browser tab.
It's a thin wrapper: the whole UI comes from the live site (see
`src-tauri/tauri.conf.json`'s `app.windows[0].url`), so any change shipped to
`lifeos-frontend` shows up here automatically with no rebuild of this app.
No local frontend is bundled — `src/` is an unused Tauri placeholder.

## Building locally

You need [Rust](https://www.rust-lang.org/tools/install) and Node.js installed.

```bash
npm install
npm run tauri build
```

- **macOS**: produces `src-tauri/target/release/bundle/macos/Aeon.app` and a
  `.dmg` installer under `bundle/dmg/`.
- **Windows**: run the same two commands on a Windows machine — Tauri can't
  cross-compile a Windows build from macOS (it needs the MSVC linker and the
  WebView2 runtime, neither available here). This produces an `.msi` and an
  NSIS `.exe` under `bundle/msi/` and `bundle/nsis/`.

For a quick dev run without a full production bundle: `npm run tauri dev`.

## Building both platforms without owning a Windows machine

`.github/workflows/release.yml` builds macOS (Apple Silicon + Intel) and
Windows in parallel on GitHub's own runners. Trigger it from the Actions tab
("Run workflow"), or push a tag like `app-v0.1.0` to also cut a published
GitHub Release with every installer attached, ready to download. The
frontend's `/download` page reads this release via GitHub's public API, so
it must stay published (not draft).

## Signing (not set up yet)

These builds are unsigned. That means:

- **macOS**: Gatekeeper will warn "Apple could not verify this app" —
  bypassable via right-click → Open, but not a great first impression.
  Fixing it needs an Apple Developer Program membership ($99/yr), a
  Developer ID certificate, and notarization credentials.
- **Windows**: SmartScreen will show an "unrecognized app" warning —
  bypassable via "More info" → "Run anyway". Fixing it needs a code signing
  certificate (a few options from ~$100–400/yr, or a cheaper
  "OV"-tier cert that still triggers a milder warning until it builds
  reputation).

Neither is required to build, test, or use the app yourself — only to
distribute it to other people without a scary-looking warning.
