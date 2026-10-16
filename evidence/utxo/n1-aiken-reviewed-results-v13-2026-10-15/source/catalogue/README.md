# Reviewed N1 catalogue source

`src/mcpcat/` contains all 14 reviewed patched source files.
The [identity record](../N1-BYTE-IDENTITY.json) compares them with the captured current tree.
Thirteen files are unchanged. Only `admission.py` differs.
Its SHA-256 is `784ff2e13cfb1e2237a6bb3fa3aaeba570af69f1d1c7e2d644a74efaea2f7523`.
The adoption declares tree `3dac6ed0553a576aa39d43d404a712e49a40b4d0341b8dda6c03224c9a4f6b3c`.
Publication checks individual bytes and does not invent a new tree-digest serialization.

This package uses the Python standard library and sibling package modules.
The captured run used CPython 3.14 exception wording.
The CLI source is [src/mcpcat/cli.py](src/mcpcat/cli.py).
Catalogue and baseline data remain under the unchanged captured run tree.

| Input | Leaf-relative path |
|---|---|
| Current catalogue | `captured/cardano-catalogue-v12-n1-run-20261015/auth-task/candidate/data/CARDANO-PROPERTY-CATALOGUE-v1.2.json` |
| Catalogue manifest | `captured/cardano-catalogue-v12-n1-run-20261015/auth-task/candidate/data/CATALOGUE-v1.2-MANIFEST.json` |
| Frozen independent inputs and 282-case pack | `captured/cardano-catalogue-v12-n1-run-20261015/pack282-patched/` |
| Author AUTH fixture dependencies | `captured/cardano-catalogue-v12-n1-run-20261015/auth-task/` |
| Captured operators and calls | `captured/cardano-catalogue-v12-n1-run-20261015/ctl/` and `INVOCATIONS.json` |

All these paths resolve under the leaf root, two directories above this README.
The source and data are portable inspection inputs.
The publication layout has not executed the CLI, harness or operators.
Historical drivers retain their original paths and budgets.
No relocated driver or reset budget is supplied.

The [Opus review](../../captured/cardano-catalogue-v12-n1-c5-independent-actual-result-review-20261015/REPORT.md) limits this result to structural UNIT behavior.
Grants remain unsigned. U9 remains `STRUCTURAL_UNVERIFIED`.
Settlement remains `UNSUPPORTED`. Crypto verification remains `NOT_PERFORMED`.
C5 M1 remains `EXPECTED_SURVIVOR_GAP` for AMB-5, never a pack pass.
M2–M10 retain predicate-kill classifications.
M11/M12 retain mixed exception and structured CLI refusal classifications, without graceful-predicate coverage.
The independent 282-case oracle remains separate from same-author AUTH50 and seven unit checks.
The 108 CLI count qualification and original C6 metadata errors remain unchanged.
The bare 3384 row total is never crypto or security coverage.
