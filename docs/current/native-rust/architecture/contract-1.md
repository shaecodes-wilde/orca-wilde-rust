# Contract 1: local native pilot

Status: proposal repaired after independent A6 review; consumer acknowledgement required. No legacy RPC/stream endpoint is connected or changed.

Execution host is the authority. Missing contact produces unverifiable; only a positive host observation produces exited. Observer detach is separate from termination. Payload host IDs are routing fences, never credentials. A transport must authenticate and authorize the host before calling the bounded JSON ingress function. No authenticated daemon transport is implemented in this cycle.

Envelope major 1 accepts unknown optional JSON fields, rejects incompatible majors and invalid identifiers. Unknown string process verdicts degrade to unverifiable; absent/non-string verdicts remain invalid. Wire bytes are limited before JSON allocation; decoded frames are limited to 64KiB. Session/generation/sequence discontinuity fails with SnapshotRequired without consuming the frame; opaque escape bytes are retained. Recovery snapshots must come from the canonical host model; no snapshot authority is implemented yet.

Pilot IDs remain bounded opaque strings to preserve legacy IDs through an eventual mapping. Distinct business ID types and complete service/error/capability envelopes remain follow-on work. No claim of existing remote wire compatibility is made for this separate protocol.

Consumers A2/A5 must acknowledge the cursor/generation contract, and A6 must review the repaired ingress before acceptance.
