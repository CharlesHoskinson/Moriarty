# Independent E04-2 turn09 regression

Composed the two exact author09 replacements over scratch candidate08; canonical source was not edited. Seven callable tests ran: five passed, two failed.

- The real pinned Bash PATH reproducer agrees with candidate09 on inherited bin-unwrapped, repeated wrapper first-occurrence removal, and literal K_LIB_DIR/../../bin. Candidate09 names the pre-checkJava helper phase setenv; the older reproducer names the final native-prefix phase setenv. Keep these phases explicit.
- A stage-specific dirname path matching commandSelections is accepted even though it has no selectedExecutables entry and its bytes are never hashed. The test changes only that resolver return and stage pin, uses an actual harmless temporary executable, and calls actual verify_selected and verify_wrapper_and_parser. Nothing executes the temporary helper.
- setenvNativeDirMissing accepts strings false/true and integer 1. Require an explicit boolean and the fixed absence policy; non-directory or dangling-symlink presence must not count as absence.
- Existing per-entry hash, symlink-target and resolved-target drift controls pass. Twenty-seven actual link/target ownership checks pass for the real selected helpers. Real helper identity and current host-evidence membership are recorded in JSON; these are observations, not adopted manifests or refreshed admission.

Required correction: every selected stage path must identify a mandatory full selectedExecutables record, even when its role differs from the command name. Validate bytes, symlink/non-symlink shape and final target, and include that record in full host link/target ownership validation. Never skip the hash because the by-role path differs.

No K, Java, strace, live diagnostic, source approval or admission claim.
