# A1 independent review of browser pressure reporting

A6 authored `ecadb3834e577c95c63e17bef28616aad7921434`; A1 reviewed the exact source and observable policy tests independently. Integration `ee715d0` and formatting-only `5c54c4a` retain those semantics. This is a policy-model review, not live webview or resource acceptance.

The existing protection predicate is unchanged: dirty or unknown unsaved state, visibility, media, transfer, automation lease, remote viewer and keep-alive exclude residents from eviction candidates. Warm ordering, cooldown and clock rollback behavior are preserved. The additive protected count includes only excluded residents; remaining pressure is the resident count after planned evictions above the warm allowance, using saturating arithmetic. It does not authorize evicting protected pages or enforce a total resident cap.

The two new scenarios exercise protected-only pressure and mixed protection/cooldown/clock rollback. Together with the existing policy cases, all nine browser tests passed in alpha-02 through alpha-05. Alpha-04's exact log and binary are hash-bound in `evidence/alpha-runs/alpha-20260929-04/summary.json`; alpha-05's repeat log and binary are bound separately. Passing policy tests do not erase either run's runtime failures.

Accepted scope: honest pressure diagnostics with unchanged page protections. Unaccepted scope: browser ownership/disposal, native page permission hooks, protection rechecks at actual disposal, process/resource measurement and P01-P03 efficiency criteria. A6 cannot independently certify its own implementation; this record supplies A1's limited independent review.
