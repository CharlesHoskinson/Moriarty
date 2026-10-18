# Core reserve, partial spend and refund candidate

UNAPPLIED / UNRUN. Base `64c6dedff1d99b33c23832330d7911a5fad9ed59`.

`candidate.patch` adds one ordinary public-consumer regression and seven fixture/custody files. No production changes. The program uses existing Core/1 transfer, fee, liability, discharge and state statements. Application counters keep gross reserved, gross spent and refunds separate. There is no Core allowance primitive or enforcement claim; that interface remains missing.

Two positive tests compare actual library execution and actual public `mori simulate` against the separate hand oracle's full financial execution projection: all 20 typed values/snapshots/deltas, 17 ordered effects, 20 source kinds and 20 literal cumulative effect prefixes. The frames retain all supplied balances, exact assets, observer state and incoming duty. Projection selection and concatenation of actual returned effects do not compute replacement transitions. Full statement fields/ExprIds, Program, work, public byte goldens, Target/Artifact and native results remain open.

The CLI test saves raw stdout, stderr and process outcome before assertions. Set `MORI_CORE_REFUND_OBSERVATIONS` to a fresh absolute directory to retain captures; without it, the test uses a temporary directory and preserves it on failure. No product runtime was run by this author.

Input freeze SHA256: `ce878c845e3b059beff1f441c1f5c5ff2e8cccf24bd250449a5d90039b684e1e`. Independent whole oracle: sibling `two_program_financial_oracles`, terminal FREEZE SHA256 `1b606141dff7ca162bcd132d55d62014187de4f936cfc6528c23c8d22591aefb`. The three selected expected files are exact copies; original whole oracle remains unchanged. ORACLE-CUSTODY.json records hashes, selection and exclusions. The custodian received only frozen inputs, call schedule and normative/DTO sources, not authored test/output/derived values. Author inspected producer source and is not the independent oracle.

Root's bounded reference check after integration/audit:

```sh
MORI_CORE_REFUND_OBSERVATIONS=/absolute/fresh/capture-directory cargo test -p mori --no-default-features --locked --test core_reserve_spend_refund -- --nocapture
```

The parent directory of the chosen capture path must exist. Root owns execution and records the actual outcome. No default native feature, SDK, proof, wallet, provider or target operation is needed by this consumer. No financial/native allocation is reset.

SOURCE/INPUT pins and full inspected files accompany the patch. Static checks cover hashes, exact oracle copies and patch reconstruction only. All 148requirements / 292 scenarios / 20 families and original U/M/MC/SP/G/full finance/composition/targets retain their dispositions. No hostile/security test, admission judgment or security/target verdict is supplied.
