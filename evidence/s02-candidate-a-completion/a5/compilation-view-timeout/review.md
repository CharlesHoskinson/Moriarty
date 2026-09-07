# Independent durable-package review

Verdict: **PASS for bounded evidence preservation; no blocking finding.** The original compilation gate remains failed by timeout. This is independent native review, not Council approval, H1 adoption, or A5 completion.

I read the package README, packing and audit sources, the preserved failed audit and both subsequent versions, and their actual tool records. Independent read/hash inspection verified the ten parts, the full 154,388,480-byte tar, its 91 unique regular-file members, and every member's size/hash against both package manifests and the retained original at its exact worktree-relative path. No original path was substituted or original receipt rewritten. Archive SHA-256 is `9cd6ff42940804ab9aa68d0e90bc9f5faec28993ad1e1ed186a48f19264dc133`.

The archive retains the 81 originally indexed files, the unchanged index itself, and the actual handoff outer record created after that index: 83 control originals. The other eight members retain the plan, reviews, adoption, source freeze, and root intakes. The two Python blocks in the archived adopted plan are byte-identical to archived `prepare.py` and `record.py`. Independently reading all five complete nested source archives verified their unique member sets and hashes against their own input pins: 146/154, 156/164, 166/174, 176/184, and 186/194 repository members/total pins. The additional eight external pins per stage are preserved references; this review did not rehash those live tools. All 30 frozen Task2 original files also matched their retained source-freeze hashes at review time.

The initial audit failed with the retained KeyError for `run-task3.py` (actual tool chunk `e15ed0`, exit 1). Its assumption that every historical view pin appeared directly in the latest source archive was wrong. Exactly two repository pins are historical-only: `.superpowers/sdd/a5-factoring-receipts/run-task3.py` (`beacf086d818651fbfeb04051678d8771f1511932832cd2c239dc2318858360d`) and `task3-test-first.json` in the same directory (`3fff922103ff0ebe86c5f589e43caea674c813511d973265098457e29054983d`). I independently found both in the retained `task3-pilot-typecheck/source-and-runner.tar.gz` nested inside the latest archive, verified that earlier archive's digest against its original input, and matched both file digests to the earlier input and current view's original pins. The correction traverses that original archive; it does not excuse missing evidence or regenerate a historical source.

The final audit differs from the preceding successful source only by importing `sys`, explicitly rejecting optimization, and reporting Python execution flags. The guard uses an ordinary conditional and `raise SystemExit`, so enabled optimization cannot remove the guard. The retained final invocation explicitly sets `PYTHONOPTIMIZE=0` and uses `-B`; its actual tool response (`f211dc`) reports exit 0, optimization 0, and disabled bytecode. Both successful validation files are byte-identical to their corresponding actual tool stdout. Root independently reran the final audit successfully (`446e6f`, as communicated by root); I reviewed its source and receipts rather than executing it again.

Scope wording is accurate. The four prior gates are bounded typecheck, quantified-prefix, and sample observations. The compile receipt distinguishes the reaped `/usr/bin/time` wrapper's -15 from helper timeout return 124 and actual outer exit 1. Generated JSON, compiler stderr, and GNU-time resource output are retained zero-byte originals. They establish neither a usable compiled input nor reached compiler phase, native elapsed/RSS, alias repair success, a flattening cause, or a semantic counterexample. There is no factored compilation or solver result and no paired size comparison. H1 remains unresolved. Runtime/Python archives remain explicitly pinned external references, not falsely included or freshly validated by this package.

The original tar excludes later package audit sources and outputs by design. They are retained as separate package files; this review pins the inspected versions below and itself remains outside the original archive. No tests, compiler/solver/native reruns, A4 artifact hashing, model/runtime edits, or commits were performed for this review.

| Inspected package file | SHA-256 |
| --- | --- |
| `README.md` | `eaca4923f4b9728758c3d316128821d7fefd45993e06ad19110a725468059c48` |
| `audit.py` | `93812b75afbeac5c90452c6518bbc8bb264d85def3dfd29333c9335bf42d2159` |
| `audit-original-failure.py` | `173c2c06b04a74ff18deeb33dda0bbacc51949f4c1c7d32ef51f9cf400b7fc92` |
| `audit-before-optimize-guard.py` | `d71a4d3a74951234bb0f539bd3dbeb5ab3ebd9859230109cdc1d9a16ac9b220a` |
| `pack-originals.py` | `298ea3589762258dc910af8da0cc05ef26a22d420c346d9820f919fa7b860e81` |
| `archive-manifest.json` | `5a9e9b8581d002686a0c22bc0329b2a8993ea052f85a187f157820c704ce6e79` |
| `original-inputs.json` | `e7d3d75369a1f97a200599ed27afb0cdaca8d7d78ea52eb79f1258bf5c890329` |
| `audit-final-command.json` | `031fe6b32c4588d1f19236f6d6bfe2cda148cee1effe5d0579170cbb490d824d` |
| `validation-final.json` | `d56192b49caf44669c6c3be2e89caca96737d61bf0fa7f8beaacbfcaee759a9a` |
| `audit-original-failure-command.json` | `76331dce696f31fad5504383c2dcc12f2ef7e7956be76a618290769839a99948` |
