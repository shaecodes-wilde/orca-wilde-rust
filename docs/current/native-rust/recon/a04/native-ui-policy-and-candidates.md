# A4 native UI policy and composition reconnaissance

Date: 2026-09-29. Worktree: F:\wr\a04. Base: ce470be8c01a54f68125e4de111cce312cf6ffd6.
Status: policy implementation and upstream reconnaissance; no selected permanent toolkit; no GUI proof.

## Compact source map

| Source | Behavior to preserve or translate |
| --- | --- |
| source-reference/docs/STYLEGUIDE.md | Quiet chrome; surface/foreground pairs; focus rings; explicit loading state; platform modifiers; remote latency |
| source-reference/src/renderer/src/assets/main.css | Canonical token roles, theme pairs, 11/12/13/14px type tiers, 10px base radius and native font mapping input |
| source-reference/src/renderer/src/app-shell/WildeBusinessNav.tsx | Home / Clients / Workspaces / Automations destinations; active and accessible labels |
| source-reference/src/renderer/src/app-shell/WildeBusinessSurface.tsx | Client/project context, bounded back history, stale-result protection, explicit workspace activation; folder and owning remote host checks |
| kickoff/wilde-native-rust-kickoff/docs/07-native-toolkit-and-browser-spikes.md | Two candidates; same-window native input + terminal + embedded page; Windows, Wayland, macOS gates |
| crates/wilde-ui/src/layout.rs | Physical-to-logical layout; suspended children on tiny/minimized windows; finite signed-coordinate limits |
| crates/wilde-ui/src/focus.rs | Native/sidebar/terminal/browser focus intent, no native focus effects |
| crates/wilde-ui/src/launch.rs | Explicit development identity/root and dual isolation signals; unconditional background-launch denial |

Pinned main.css imports wilde-theme.css, but that import target is absent in the source-reference snapshot. Do not infer override values or copy private fonts. Preserve canonical token roles now; A1 must supply approved assets and resolved theme values before rendered identity approval.

## Two native candidates

Versions below are those returned by primary upstream documentation on this date, not locally compiled versions. A targeted local registry inventory found WRY 0.54.4; no Iced or Slint source package appeared. No dependencies were installed.

| Concern | Iced 0.14.0 | Slint 1.18.1 |
| --- | --- | --- |
| Native UI model | Rust widgets/update/view; custom widget route for terminal | Compiled .slint components and Rust; custom rendering/platform route for terminal |
| License | MIT; transitive dependency notices still need audit | Royalty-free, GPLv3, or commercial framework license; examples/docs MIT. Royalty-free proprietary desktop use requires Slint disclosure; distribution terms need owner approval |
| Renderer/backend metadata | wgpu and tiny-skia; winit; X11/Wayland feature paths; Rust 1.88 minimum | Backend selection documented; custom Platform and WindowAdapter interfaces available |
| Parent-handle route | Window implements raw window/display handle traits; window::run callback borrows handle | raw-window-handle-06 feature; window_handle depends on backend and may require a shown window/event-loop iteration |
| Tao composition implication | Normal Iced runtime owns its window/event loop. Borrowed callback cannot trivially return a non-Send WRY owner; custom integration must prove main-thread lifetime | Default backend owns lifecycle; custom adapter permits composition but brings event/render/input obligations |
| Embedded browser | Windows child-parent route is plausible, unexecuted | Windows child-parent route is plausible, unexecuted |
| Wayland browser | Generic WRY child API is insufficient; compositor/GTK composition remains unresolved | Same unresolved browser boundary despite native Wayland UI capability |
| Accessibility, IME, clipboard, overlays | No observed result; native child webview z-order needs explicit test | No observed result; custom backend must preserve accessibility/input and child z-order |
| Actual OS/backend executed | None | None |
| Startup/RAM/output metrics | NOT RUN | NOT RUN |

Sources: [Iced 0.14 manifest](https://docs.rs/crate/iced/0.14.0/source/Cargo.toml), [Iced window handles](https://docs.rs/iced/latest/iced/window/trait.Window.html), [Iced callback API](https://docs.rs/iced/latest/iced/window/fn.run.html), [Slint 1.18.1 docs](https://docs.rs/slint/latest/slint/), [Slint license at v1.18.1](https://github.com/slint-ui/slint/blob/v1.18.1/LICENSE.md), [Slint platform adapter](https://docs.rs/slint/latest/slint/platform/index.html), [Slint window API](https://docs.rs/slint/latest/slint/struct.Window.html), [Slint backend features](https://docs.rs/slint/latest/slint/docs/cargo_features/index.html).

The event-loop/lifetime and browser conclusions are engineering inferences from these APIs, not demonstrated compatibility. Neither candidate is eliminated. No permanent dependency or license choice is made.

## Native visual mapping

| Native role | Existing design role |
| --- | --- |
| Main shell canvas/text | background / foreground |
| Business sidebar rows/text | sidebar / sidebar-foreground; sidebar-accent hover/current; sidebar-ring keyboard focus |
| Terminal/editor backdrop | editor-surface; mono family from canonical font-mono role |
| Native input | input, foreground, border, ring; existing 14px body tier and input radius |
| Floating commands/menus | popover / popover-foreground and documented floating elevation |
| Secondary/path/meta labels | muted-foreground and existing 12/11px tiers |
| Affirmative, back-out, destructive | primary pair; quiet ghost back-out; destructive pair only for actual data loss |

Use existing light/dark values; no new palette, font tiers, elevation tiers, React, HTML first-party controls or font redistribution. Browser input/textarea fixture remains browser test content owned by A6.

## Admission and layout acceptance

`admit` rejects ORCA_BACKGROUND_LAUNCH=1 before any other option, requires --isolated-display and WILDE_NATIVE_ISOLATED_DISPLAY=1 together, exact application ID systems.wilde.native-rust.dev, and an explicit absolute root named wilde-native-rust-dev-profiles with no parent traversal. It does not touch the filesystem, create profiles, validate markers, establish display isolation or start a process. A6 profile inspect must validate the canonical existing root and exact native marker before composition. Symlinks/junctions and actual display identity remain A1/A6 boundaries.

A physical resize plus finite positive DPI produces logical coordinates bounded by i32::MAX. Below 64 logical pixels on either axis all four child regions are suspended; otherwise each region has positive bounds and no region overlaps another. The input occupies the top of the content area; terminal and browser split the remainder; sidebar spans the left side. The 64px threshold is diagnostic scene policy, not final usable product minimum. Win32 physical conversion still needs shared rounded edges and checked signed conversions at A1's boundary; a logical proof alone does not prove physical pixel containment.

Focus traversal is Sidebar -> NativeInput -> Terminal -> Browser with reverse traversal and wrap. Empty/suspended layout yields no target. It computes intent only, preserving native action ownership; child keyboard focus, IME and traversal interception remain untested.

## A1 Windows diagnostic scene proposal (not implemented here)

1. Parse launch inputs, call A4 admit, and inspect A6 canonical native development marker. Verify a genuinely isolated VM/display/CI session before allowing visible windows. Flags alone are insufficient.
2. A1 owns the main-thread Tao parent and event loop; borrow its Win32 native handle into A6's WRY 0.54.4 child. Keep A6 MainThreadOwner on that thread and drop children before parent. Keep the diagnostic identity and synthetic profile separate from production.
3. A1 creates genuine Win32 native sidebar selection and text input controls and places A5's native terminal surface in Terminal bounds. Win32 controls are a temporary diagnostic route; retirement condition is a candidate's integrated scene passing the same matrix. Plain text output must not be labeled terminal completion.
4. Convert Browser bounds into A6 validated logical SurfaceBounds; convert native bounds to checked physical edges consistently for DPI. On resize/scale recompute all regions; on suspension hide child surfaces without focus/activation or zero-size calls. Hidden browser lifetime and terminal session lifetime must be independent, then explicitly released on scene teardown.
5. A6 supplies its fixed synthetic input/textarea page with no native application RPC. Any menus/overlays require native z-order tests above the browser, not screenshots of a separate window.
6. Wire A4 focus intent to platform adapters only inside the isolated test scene. Keep browser internal Tab behavior and native traversal separate until observed keyboard traces establish correct routing. No OS focus/activation API is present in the policy crate.
7. Reuse A3 client/project/workspace contracts after composition; selection must not eagerly open terminals/browser pages. A1 owns application contracts, remote-host validation and explicit workspace activation. Folder workspaces retain the same navigation; SSH disconnect remains unverifiable with no local execution fallback.

## Repeatable checks and missing evidence

Authored tests: repeated resize/nonoverlap, minimize/restore, scale recompute, NaN/zero/infinite DPI and overflow denial, forward/reverse/wrap focus intent, background denial overriding opt-ins, both isolation signals, production identity/profile rejection and explicit admitted root. These are headless policy tests, not mocked GUI tests.

A4 ran rustfmt --edition 2024 on owned Rust files successfully. A1 must register crates/wilde-ui in the root workspace and run cargo test -p wilde-ui with its exclusive two-job build slot; A4 did not edit root Cargo files or run Cargo/rustc/build/dependency install.

NOT RUN: crate compilation/tests; rendered Windows scene; input/focus; IME; accessibility; clipboard/scroll/selection; menus/overlays; repeated child teardown/profile cleanup; multi-monitor DPI; startup/load metrics; Wayland and macOS composition. No screenshots, live browser or terminal parity claims.
