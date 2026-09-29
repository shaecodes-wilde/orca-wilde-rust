# 02 — Destination architecture

## Native, not language-pure

The final first-party desktop interface must run without Electron, React, a DOM-based application shell, xterm.js or Monaco. A native toolkit may use a declarative UI language with Rust logic. A native editor must not secretly depend on Monaco inside a hidden WebView. Temporary bridges are permitted during migration only when explicitly labeled and tracked to retirement.

Mature browser engines, SQLite, the system Git binary, platform APIs, media engines and external agent CLIs can remain non-Rust dependencies. Their costs and licenses remain visible. Do not build a browser engine or port SQLite/Git for language purity.

## Start small, split when responsibility requires it

Proposed location: `native-rust/` (check for existing work first). A1 initially creates a workspace, contracts crate, runtime/domain pilot and test harness. The following are ownership destinations, not a request for empty crates:

| Destination | Responsibility | Owner |
|---|---|---|
| `crates/wilde-contracts/` | IDs, versioned service messages, capability/error vocabulary | A1 |
| `crates/wilde-runtime/` | Execution-host authority, sessions, PTY transport, filesystem/Git/SSH | A2 |
| `crates/wilde-domain/` | Client/project/workspace relationships and business use cases | A3 |
| `crates/wilde-storage/` | SQLite, transactions, migrations, backups | A3 |
| `crates/wilde-ui/` | Native layout, navigation, business presentation, accessibility | A4 |
| `crates/wilde-ide/` | Native terminal model/renderer, editor, diff and previews | A5 |
| `crates/wilde-browser/` | Browser adapter, profile/permission routing and page lifecycle | A6 |
| `crates/wilde-platform/` | OS integration and native integration adapters | A6 |
| `crates/wilde-governor/` | Resource admission, measured pressure, keep-alive policy | A6 |
| `apps/wilded/`, `apps/wilde-desktop/` | Process entry points and service composition | A1 |

Business/domain crates do not import UI frameworks or Electron/Node types. Platform-dependent calls sit behind narrow adapters. No circular crate dependencies or universal mutable global state. Cross-crate shared types belong in small contracts, not duplicated model definitions.

## Process boundaries

The desktop is a client of a durable execution service, not the sole owner of every running task. `wilded` is a proposal for that service, not an existing command. Local UI calls use narrow commands/events; the local transport must authenticate. Windows named pipes with user ACLs and short Unix-domain socket paths are candidates. Do not assume async Rust traits define a network protocol: serialized envelopes, errors, framing, version negotiation and reconnect behavior must be specified separately.

Keep a modest number of purposeful processes. Do not replace every Node module with a separate Rust daemon. Place untrusted web rendering, crash-prone foreign interfaces and persistent session authority in appropriate isolation boundaries. A1/A2/A6 jointly decide whether the PTY host stays independently restartable; desktop closure, connection loss, process shutdown and job termination are separate events.

Use bounded channels, cancellation, backpressure and explicit ownership. Unmounting a visual terminal must not close its running PTY. Avoid retaining every historical terminal line in every presentation client. Recover from a stream gap using a host-owned snapshot/cursor rather than silently dropping bytes that change terminal state.

## Business hierarchy

Owner (client / Wilde Systems / personal) → project → workspace assignment.

A project is not a repository. A workspace is not a running process. A runtime session belongs to an execution host and references a workspace. A3 must map current IDs/relationships through an adapter rather than replacing persisted IDs or treating the existing `ownerId` as the new business-owner concept without inspection.

Store durable records in SQLite; request paginated/projection-oriented data for the visible UI. Preserve existing business request semantics first, then version any response changes. Use materialized summaries only when measured query needs justify them; event sourcing and distributed microservices are not prerequisites.

## Browser and UI must be selected together

A4 owns layout/window surface integration, A5 native rendering integration, A6 the actual browser host. They jointly demonstrate native controls, a live native terminal and an embedded browser inside the same window. A remote Chromium service controlled via CDP is not automatically an embeddable native view. Streaming its pixels has capture/encode/decode/input costs that must be measured and tested for accessibility and text clarity.

Do not assume moving a logged-in page between system WebView and Chromium preserves its cookies, JavaScript heap or unsaved work. Pick a backend when opening the page; changing backend is an explicit reload/re-authentication operation. Prefer one initial browser backend over simultaneously maintaining two incomplete ones.

## What Rust can and cannot prove

Rust ownership does not require tracing GC, but it does not prevent logical leaks, deadlocks, unbounded buffers, allocation churn, foreign-library bugs or browser costs. Faster application coordination does not speed up the external model, database engine, Git binary or Internet. Architecture changes may improve baseline memory/startup; this is a hypothesis until same-workload measurements exist. [W4–W6]

Broad UI conversion is gated on native embedding/input feasibility. Production replacement is gated on feature coverage, rollback, measured resource behavior and Shae's cutover approval.
