# Host architecture corrections after worker 06

The worker supplied complete draft files and reports but timed out during final review.
Its exact output is preserved outside the worktree before these corrections.

The host corrected nonfinancial policy records, used-effect declaration rules and forward policy-metadata resolution.
The signed statement now binds the execution domain, genesis and input state plus claim requirements.
Proof evidence lives outside the signature and trace. A complete ProofContext binds it after deterministic execution and before commit.
This avoids a signature/proof-digest cycle while retaining mandatory claim verification.
The host defined statement/proof hash domains, deterministic count preimages, ordering and malformed-input diagnostics.
Expanded internal AST/typed/manifest metadata has explicit larger finite limits. Public encodings retain their original limits.
All original counts and examples remain subject to actual aggregate representability checks.
The new frontend must materialize and verify complete example encodings; the present source-only checker is not a compiler.
These are architectural candidate edits, not independent review approval or implementation.
