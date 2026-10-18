# Non-blocking follow-ups from the approved runtime audit

These are retained limitations, not completed changes or blockers to candidate-02:

- `compatibility.usable` verifies core entry-file presence and manifest identity, not a complete import graph. Missing optional/new module dependencies can still fail at import. Use package tests and live host checks.
- `store._is_pid_alive` is unused. Keep PID liveness out of reservation recovery; deleting dead code can be considered in a separate candidate.
- `bootstrap_store` retains broad exception handling around inserts. Missing history remains unknown; future cleanup should surface specific bootstrap failures without inventing clean state.
- Foreman launcher path, Linux containment, native Windows shell syntax and non-Codex transcript adapters are explicit portability limits.

Grok source audit: `grok-final-03.json`; Astra independent test/source audit: `astra-final-02.md`. Grok's cancelled attempts have no approval authority.
