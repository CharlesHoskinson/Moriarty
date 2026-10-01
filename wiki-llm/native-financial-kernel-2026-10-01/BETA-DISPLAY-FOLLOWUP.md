# Signed-intent display followup

Repository observation, 2026-10-01. Two retained low-severity review findings were reproduced and repaired while independent ledger compilation review ran.

- F1: bounded parser used its configured string limit but printed1024 unconditionally. It now prints the selected value. Exact diagnostics were checked for limits3 and131072.
- F2: the CLI escaped its JSON detail before passing it into a renderer that escapes all supplementary text. The redundant CLI escape was removed; JSON representation and the renderer's terminal boundary remain. Exact native CLI backslash rendering changed from8 to4 displayed backslashes (two JSON escaping bytes each escaped once at the terminal boundary). PrintableASCII and the original mismatch rejection exit1 were preserved.

Existing targeted JSON and hostile-terminal-input tests ran with actual required Midnight Rust verifier:5passed,0failed,0skipped. This is the scoped changed display path, not a rerun or fresh approval of the full beta distribution or financial pipeline. No cryptographic financial proof or ledger application was run. Full source/result review and publication of the amended beta remain pending.

Immutable local red/green observations, native review outputs, targeted test output and exact changed source snapshots are retained in /home/charl/research/moriarty-signed-intent-2026-10-01/beta-display-followup/manifest.json. Historical signed-intent delivery snapshots and verdicts remain unchanged. Canonical current navigation had already preserved the historical187-test snapshot and linked the193-test delivery; no stale historical count was rewritten.
