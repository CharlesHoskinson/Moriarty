# Final candidate and result reviews

Both fresh audits returned **APPROVE_WITH_SCOPE**, with no merge blockers, for base `4172d561128194e4fd4a8d80ec1d9b62c4283b99` and frozen staged tree `f91045548545f10cad5959b6df981681c6b223b4`. The [184-file candidate manifest](reviews/FINAL-CANDIDATE.json) identifies that review input. This file, original final reports/identities, the publication scan and copied dep-info captures are subsequent evidence-only additions; they change no executed source or consumed input.

- [Fresh Muse nonsecurity audit](reviews/final-muse.md), [identity receipt](reviews/final-muse-identity.json).
- [Fresh Opus security audit](reviews/final-opus.md), [canonical identity receipt](reviews/final-opus-identity.json).

Both reviewers received full changed active source, contextual compile/read dependencies, exact candidate membership and final actual-result logs/receipts. Neither ran commands or independently decompressed the large trace. The integrator separately checked every consumed current and archived source hash, every RED/GREEN/FINAL child-output digest and byte count, and the decompressed complete producer-output SHA. Historical captures and compressed artifacts omitted from the review prompt remain explicitly outside byte-for-byte reviewer inspection; they are bound in the manifest. Opus's §1 lists that limitation. Its final publication condition is a separate read-only Opus scan.

Opus authored the wrapper and guard tests in earlier contexts. Its final audit was a fresh non-author context but shares the model family; that possible blind spot and Muse's separate review are retained. Muse exposes configured model/provider identity and completed status, with no separate provider canonical modelUsage. Opus's provider returned canonical `claude-opus-5-5`.

## Retained clarifications

These record the final audit's nonblocking qualifications without rewriting immutable earlier evidence:

- In `PREDECESSOR-SOURCE.json`, “current” means the preflight candidate and “NOT EXECUTED” describes preparation time. The later RED receipt records execution. Its 53 Python tests predate the final 79-test set; the wrapper bytes match.
- Width 1/2/4/8 and cut/resume tests cover **N01/N02**. N03–N08 each run through a single grouped call per prefix. All 216 prefix runs compare complete records.
- Missing H1 wording and corpus-derived budget wording in consumed source are follow-up notes, not changes in this delivery. Retained records allocate complete snapshot clones; only fixed trusted inputs are admitted here.
- The comparison JSON alone labels execution not-run. The actual host wrapper receipt supplies the separate execution evidence.
- Comparison-failure, timeout-kill and pre-execution-artifact-change paths have not all been observed end to end. The actual negative control observed compile failure and dependent fail-stop. The seven Rust tests were not observed failing before implementation; their compile was downstream of that failed producer compile.
- Eight random workflow witnesses remain zero. Deterministic scenarios and the directed walk supply separate reachability observations.
- rustc dep-info files are copied under each `host-*/build/` as subsequent evidence. The final review's wording that they were “supplied” was imprecise: the full FINAL receipt supplied their digests and exact parsed membership; the original review manifest did not include the raw `.d` captures. The source/dep-info membership check was executed by the wrapper, not independently rerun by the reviewers. No raw RED `.d` is authenticated by its failed compile receipt.
- One extra final blank line in mechanically extracted `normal_oracle_inputs.rs` is retained to match the tested bytes. All other changed active files pass the whitespace check; this cosmetic warning is explicitly accepted in both reviews.

All limitations and unrun layers in the [delivery report](README.md) remain. The normal GitHub PR supplies the final commit, integration and merge identities; a merge does not close any parent roadmap exit.

The [Opus read-only publication scan](reviews/publication-scan-opus.md) returned **ALLOW_PUBLICATION**, with [canonical identity](reviews/publication-scan-opus-identity.json) and no permission denials. It scanned all 96 non-gzip files then present, including the six final-review evidence additions and the five copied dep-info files. The final scan report/identity are subsequent receipts of that scan. The previously ignored dep-info captures are explicitly staged for this publication.
