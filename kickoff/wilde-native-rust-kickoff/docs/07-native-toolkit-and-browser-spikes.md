# 07 — The early feasibility experiment

A4, A5 and A6 jointly own this experiment; A1 records the decision. Evaluate **two** viable native candidates first, not every UI library. Iced and Slint are starting candidates, not preapproved dependency selections. Iced documents native renderers using wgpu/tiny-skia. Slint supplies native UI and has license options that must be reviewed for the intended distribution. [W2, W3]

Use actual available versions and record them. Existing CSS/tokens are design input, not directly reusable native widgets. Keep Wilde's identity with static assets and restrained motion; do not ship font files from this package.

## Required integrated scene

One native window with a sidebar/client list, split pane, native text input, native terminal stream rendering and a REAL embedded web page. Test repeated opening/closing, multi-monitor DPI changes, resizing, keyboard focus between surfaces, scrolling, selection, context menus, clipboard, IME and accessibility. Verify browser cleanup and terminal session independence on close/hide. A browser in a separate external window does not pass the embedded-browser gate.

Windows and Linux/Wayland are essential first targets for this user's workflow. macOS remains a required supported-platform gate; lack of a local Mac is a scheduling constraint, not permission to claim support. Headless CI cannot validate all real input/graphics behaviors.

## Browser candidates

Evaluate a platform WebView path and a Chromium-backed embedding path against the same capability checklist. WRY's generic `build_as_child` currently documents Linux X11-only support; its documentation describes GTK-specific paths for Wayland. This does NOT prove seamless composition with a chosen native Rust toolkit. Demonstrate the integration rather than assuming a Rust WebView crate solves it. [W7]

CEF or another mature Chromium host is an architectural candidate requiring separate upstream/API/license verification during the spike. Do not create a browser engine. CDP controls a browser but does not itself put its pixels into a native layout. An offscreen texture/screencast bridge needs measured latency, scaling, input mapping and accessibility behavior; include CPU/GPU/video costs.

Treat OAuth, passkeys/WebAuthn, popup windows, media permissions, protected media, downloads and profile isolation as explicit compatibility checks. Preserve the user's normal session behavior without copying cookies/secrets into logs. Switching between backends does not transparently transfer a live page.

## Native IDE candidates

A5 evaluates reusable terminal parsers/models and native text-editing components. `portable-pty` is one cross-platform PTY candidate; it is not a terminal renderer or proof of ConPTY parity. [W8] Native terminal rendering needs cursor/alternate-screen behavior, Unicode widths, fonts, scrollback, selection and IME. A generic text box is not a full code editor; list required language features, LSP, undo/redo, large files, diffs and save-conflict behavior.

## Decision record

For each candidate record exact versions, license/dependency considerations, actual OS/backend executed, code commit, screenshots/video where appropriate, input/accessibility findings, browser/terminal compatibility, startup/idle/output load metrics and unresolved blockers. Choose by demonstrated fit, not an empty-window RAM claim or model preference.

The team may choose a reversible technical prototype without waiting for product clarification. Production licensing commitments, dropped platform support, changed security boundaries or a switch away from native UI require Shae's decision.
