# 05 — Contract v0 proposals

These are design requirements, not a claim that the repository already implements these APIs. A1 approves actual types and serialization after source inspection. Use a minimal vertical-slice contract first; extend by reviewed version changes.

## Identity

Distinct types: `ClientId`, `ProjectId`, `WorkspaceId`, `SessionId`, `ExecutionHostId`, `BrowserPageId`, `BrowserProfileId`, `RequestId`. Preserve stored legacy IDs through explicit mapping. A local path is not a universal ID. Separate business owner identity from runtime/authentication authority.

## Control messages

A serialized request needs protocol version, unique request ID, service/method, validated body and explicit host/workspace target where applicable. Replies distinguish success, validation failure, permission denial, conflict/stale revision, unsupported capability, timeout/cancelled, not found and unavailable host. Return machine-readable codes and safe user-facing text. Never serialize raw credentials or full environment maps in diagnostics.

Mutations define idempotency and cancellation separately. A retried non-idempotent action must not be executed twice. Losing the response does not prove the mutation failed. Connection/session setup negotiates protocol capabilities; unsupported major versions fail clearly. Unknown stream opcodes cannot silently erase required state. A Rust trait alone is not wire compatibility.

## Runtime service — A2

Commands: create session, list/query session, attach/detach observer, write bytes, resize terminal, request interrupt, terminate owned job.

Use structured program plus argument arrays, explicit working directory and selected environment inputs. Do not build shell strings from user data. Profile/host authority validates paths and operations; runtime should run under the user, not elevated by default. UI disconnect ≠ terminate. Unavailable remote host status is `unverifiable`, not `exited`.

The host owns process identity, start time/nonce, PTY and canonical status. Reconcile PID reuse, reconnect and stale messages. Windows ConPTY/job object behavior, POSIX process groups, WSL quoting, SSH trust/host-key checks and agent reconnect are platform acceptance cases, not implementation details to omit.

## Terminal stream and rendering — A2/A5

A2 owns transport/PTY process authority. A5 owns emulation, scrollback model and native drawing. One canonical terminal model per session must be chosen jointly; multiple observers cannot independently produce contradictory session state.

Stream frames identify session, generation, monotonically increasing sequence and byte payload. Define bounded backlog, slow-consumer handling, snapshot/cursor recovery and terminal resize ordering. Never drop arbitrary escape-sequence fragments as a memory optimization. Credit/backpressure limits transport queues; history retention is separately bounded. Model recovery must handle alternate screens, Unicode widths, emoji, images where supported, search and scrollback persistence.

A4 owns screen layout and widget focus; A5 owns terminal/editor widgets. A2 owns the OS file operations used by A5's explorer/editor; A5 owns presentation, save conflict UX and undo/redo semantics.

## Business/storage — A3

Keep SQLite schema/version handling, revisions, transaction rollback, preview expiry, exact-change confirmation, duplicate request semantics, relationship checks and backup behavior compatible with the current command implementation before changing them. Use synthetic export fixtures and verify failed imports are atomic. For any new pagination protocol, provide a legacy adapter or new capability; do not silently change full-snapshot consumers.

The UI never writes SQLite directly. Blocking database work stays out of the UI event loop and async executor hot path. Select one write authority; no uncontrolled legacy and Rust dual writers. Reading a copied database requires a consistent snapshot that includes WAL state, not a blind copy of only the `.db` file while writes are active.

## Browser and native surface — A4/A6

Separate page identity, persistent browser profile, execution host and backend capability. Operations include create/open/navigate, bounds/DPI, input/focus, snapshot, download policy, permission requests and destroy. The surface contract defines native handle lifetime, thread ownership, child-window ordering/clipping, focus transitions, resizing and disposal. Not every backend implements every capability; report unsupported behavior explicitly.

Do not give untrusted websites native application RPC privileges. Profiles isolate cookies/storage by intended identity, and permissions remain scoped and visible. CDP control stays authenticated/local or over a secured authorized channel, never an exposed unauthenticated endpoint.

## Governor — A6, enforced by resource owners

State: active, warm, frozen (only if the backend supports it), dormant, closed. Warm/suspended resident memory is not equal to destroyed page memory. Dormant reload cannot restore an arbitrary live JS heap.

Protection reasons include visible interaction, dirty or unknown-unsaved state, active call/media, upload/download, automation lease, remote viewer and explicitly keep-alive. A pinned bookmark is not automatic execution permission. Use cooldown/hysteresis to prevent reload thrash. Leases expire only by defined policy and safe reconciliation; don't discard an in-flight task merely because a heartbeat was delayed.

Collect unique process IDs and explicit shared/private metrics. Browser count is not RAM attribution. OS memory pressure and budget overruns cause admission/eviction decisions; don't promise a hard whole-app RAM cap that the browser engine cannot guarantee.

## Integrations and voice

A3 owns n8n data ingestion/retention and business records. A6 owns platform/Spotify/OBS/voice adapters and credential access. A4 owns controls. Commands converge on the same validated service API whether issued by UI, CLI or voice. Destructive/client-affecting operations retain preview/confirmation. Start against fake providers; protect user's real workflows, music and stream state.
