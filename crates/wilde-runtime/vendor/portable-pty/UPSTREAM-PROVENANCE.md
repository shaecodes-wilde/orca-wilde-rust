# Portable PTY pilot patch provenance

Source: exact cached registry package portable-pty 0.9.0, copied locally without modifying the global Cargo cache.
Upstream repository: https://github.com/wezterm/wezterm
Cached .cargo_vcs_info.json: commit f8921727a11b9f8b073e8c24821d72fd41283500, path pty.
License: MIT; original LICENSE.md retained verbatim, copyright 2018 Wez Furlong.
Registry archive portable-pty-0.9.0.crate SHA-256: b4a596a2b3d2752d94f51fac2d4a96737b8705dddd311a32b9af47211f08671e.
UPSTREAM-FILE-HASHES.json records SHA-256 hashes of all 18 exact cached package files before the experiment. Every copied file except src/win/psuedocon.rs was compared by hash and remains byte-for-byte upstream. Added .gitattributes pins this package to LF checkout bytes so platform Git newline settings cannot silently alter the recorded copy; no global configuration is changed.

The Cargo registry-normalized Cargo.toml is retained unchanged; its source/example paths stay inside this directory. Cargo.toml.orig is upstream archival evidence and is not the active manifest. Neither manifest nor upstream Cargo.lock is edited. Root workspace patch/lock composition belongs to A1. Do not use or inherit upstream development workspace paths from Cargo.toml.orig.

The ONLY source change removes PSUEDOCONSOLE_INHERIT_CURSOR from PsuedoCon::new's CreatePseudoConsole flags. PSEUDOCONSOLE_RESIZE_QUIRK and PSEUDOCONSOLE_WIN32_INPUT_MODE remain. The constant itself, API, backend, process ownership, native drop, dependency versions and all other source are unchanged. No guessed cursor responder, escape parser or new cleanup thread.

Reason: A1's guarded Windows fixture run captured exactly four admitted native bytes, ESC [ 6 n (escaped as \x1b[6n), then timed out without WILDE_FIXTURE_READY in all four real integration tests. This proves a cursor-position query was emitted. Inherited-cursor negotiation preventing startup remains a cause hypothesis until this reversible experiment is independently reviewed and tested. A6 approved the narrow source experiment; A1 authorized this scoped cached copy. This file claims no test pass or root-cause confirmation.

Fixture execution still requires WILDE_RUNTIME_FIXTURE_GUARD=owned-runner-watchdog-v1 and A1's independently owned outer runner. Removing the startup flag does not make synchronous ClosePseudoConsole, native read cancellation, process-tree containment or production R08 cleanup bounded. Production R08 acceptance remains NOT ACCEPTED. A1 alone builds/runs; A02 ran no Cargo build, fixture child, native UI or app.

Revert: A1 removes the workspace [patch.crates-io] entry and regenerates its own lock selection, preserving this experiment and evidence for review. No global-cache edits or old-app changes are needed.
