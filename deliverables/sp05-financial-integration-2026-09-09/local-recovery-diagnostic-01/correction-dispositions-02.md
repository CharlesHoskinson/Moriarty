# Diagnostic review corrections

The first correctly targeted Grok vote requested changes; it remains retained and does not admit the original candidate.

1. **Python optimization — corrected.** The original `assert __debug__` vanished under `-O`, as did every subsequent assertion. A non-assert `if not __debug__: raise SystemExit(...)` now runs before admission reads or side effects. Exact AST-extracted checks show normal execution reaches an inert canary, while `-O` and `-OO` exit first. All later assertions are therefore mandatory for any admitted execution.
2. **Docker startup allowance — corrected.** The single Docker start command now uses the remaining arming +90-second activation allowance. It cannot extend the independent stop window.
3. **systemd wait — finding contradicted by actual behavior.** An independently bounded, harmless `Type=exec` sleep service returned from `systemd-run` in 0.0327 seconds while the two-second child was still active. It later exited and had PID zero. We keep synchronous start-job acknowledgment; `--wait` was never supplied. Adding `--no-block` would race the immediate active/PID check. No blockchain services were involved.
4. **Launch validator — missing review context supplied.** `validateLocalLaunchPlan` only checks and clones its argument. It does not open private paths or contact a proof server. Its entire file is already in source-candidate03, and the operational entry rechecks every source digest. The full public-preflight fixture additionally recorded no access to its guarded private paths. The extra validator remains; its source is included with this correction.
5. **Stop budget — unchanged and scoped.** The timer accuracy shifts dispatch by at most one second; the service has 25 seconds for the kill/stop commands and five seconds for stop-post cleanup. Docker kill remains the backstop, and the proof server is absent. Actual terminal containment still requires observation. No resource ceiling or financial authority changed.

All prior attempts, dissent, misdirected-review usage and original metadata are preserved. No diagnostic services have been dispatched.
