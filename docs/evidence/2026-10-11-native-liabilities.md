# Local native liability result — 2026-10-11

This observation exercises the admitted source-registry liability relation through the production `mori compile`, `simulate`, `prove` and `verify` commands. It is one public local transition. Authenticated predecessor completeness, debtor consent, signed caps, history, recovery, ledger settlement, fraud costs and Fenrir devnet qualification remain open.

Execution candidate: `d06d128d473a5a1606c929462287b7a11a4f88bd`, tree `8ce16a6f05241af794fafa9cdaf7f7554f971aba`. Later evidence-only commits do not change the executed production source. The selected source is [settlement.mori](../../examples/core/native-liability/settlement.mori), action `repay`, using [partial.scenario.json](../../examples/core/native-liability/partial.scenario.json). The source-derived registry contains both `payment` and `reserve`, including the untouched reserve duty present at zero.

## Observed native result

One genuine proof at k11; model1059 rows,137 instructions,28 public inputs,5 ordered effects,6 source steps. Native proof3440 bytes. Exactly six genuine verifier calls including producer self-verification: self, independent fresh-key positive, and four negative controls. Two additional metadata controls refuse before parameter loading. All twelve driver commands had the expected exit; outer exit0. Driver wall time 3.597360s, outer wall time 3.999352s. These are elapsed observations, not performance guarantees.

Complete independent positive receipt equals the producer receipt and reference execution. Alice100→91, Bob10→17, treasury3→5, reserve balance50 unchanged; settled0→7; payment duty10→3; reserve duty remains present0. Ordered effects: transfer7, fee2, liability10→3, state0→7, emit7. A transfer alone never implies discharge.

| Control | Actual result |
| --- | --- |
| Amount7→8 with original metadata | Host claim refusal, no native verifier |
| Amount7→8 with correctly relabelled scenario/execution hashes and original proof | Native verification refusal |
| Untouched reserve duty0→5 with original metadata | Host claim refusal, no native verifier |
| Untouched reserve duty0→5 with correctly relabelled hashes and original proof | Native verification refusal |
| Appended raw proof byte | Native verification refusal |
| First raw proof byte changed | Native verification refusal |

Relabelled controls keep the old proof and every other bundle field, including the trusted registry, unchanged. Native controls produce `mori::backend::verification`, exit1 and empty stdout. The ordinary preflight had separately checked correct metadata in all three contexts with placeholder bytes and a nonexistent SRS directory, before any native allocation.

## Pinned artifacts and resource scope

Public fixed QA artifacts are retained under [native-liabilities-2026-10-11](../../experiments/moriarty-language/evidence/native-liabilities-2026-10-11/): proof, complete receipt, target, observed summary and negative diagnostic hashes. No credentials or private witness material are included.

- Proof bundle SHA256 `74c7fbce7280e4b62fbd76e9ad3cb8314239356efb00e5e5fb0bab9b3c306b42`.
- Raw proof SHA256 `389d4ae8733abad4f23854e520c9543ec917d2d17866d177320dd602853987a9`.
- Source SHA256 `cf653de549a82ac9535d868605d71c1df29d4bf3c0d6df4cf6a1a894e619ccb2`.
- Scenario SHA256 `f37e0e1045353865cfc3592b479a028f55b5778158987afdebb6d69b62e174f5`.
- Canonical target SHA256 `0a79a6226f66efb8c3c6d0a08291a41d2705073c01805fef2d2b36d3cd642aec`.
- Selected ZKIR SHA256 `290db9c08583db83467d11ed783fe999b47700cabff8d27cc1689fd1d5958a8d`.
- Executable SHA256 `6604b4cc1179166895fc8e9feea3260ffe834eecc7db8148782cb6ccbef3f248`.
- Offline SRS degree11,393604 bytes, SHA256 `9901589d7956ff58be0d85569b2f455b77b58c3758026ffb5bbe4807000b96d1`.
- Ledger revision `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`; ZKIR `3.0.0-rc.2`; target local/native-liability-qa.

Both independent resource votes approved this exact candidate, driver `9427d6e1ac7882ded65787b2cf53e09464fa2030fb2b776031a11cbe3a7c95d5`, and proposed config `2c940d56f37c1c8c9b3911acf2bd66360a359907cb76c847716125b1ec497cd5`. Only the authorization Boolean changed in execution config `60298b281c1a399562ca8120e8f1dc01844dd7a2d49e6ab788a1a638aaf4c382`. One GNU timeout covered the driver process group for60s with2s kill grace. Inherited RLIMIT_AS soft/hard2147483648 bytes applies per process; it is not an aggregate group RSS limit. Python assertions stayed enabled. No downloads, parameter writes, retries, second proof, higher-k search, width16 proof, recursion or ledger transaction occurred. This allocation is consumed.

Raw allocation, votes, command vectors/exits/stdout/stderr/mutations and immutable failures remain in `.moriarty-dev/runtime/native-liabilities-20261011` in the maintainer store. Sol resource vote SHA256 `57545b2c8237c83156ecef20692f58dfb712092b2f7817974c55302922a2f259`; sanitized Grok resource vote SHA256 `8415bbc00dbbd1cfdedb7d2cc8eaed38fe4be865ed3d04d1f5a5a0fce1c8f8ee`, session `01a12acc-6794-77b2-916d-10e0660575b4`, request `c6ab9d8e-c969-417c-9398-70f93016b49b`, terminal end_turn, returned model grok-4.6-build HIGH. The earlier cancelled Grok terminal is retained and supplies no vote.

## Ordinary QA and remaining acceptance

Executed ordinary workspace225 passed,0 failed,3 existing native ignored; beta193 passed,0 failed,0 skipped and typecheck passed. Quint25 new and64 historical deterministic tests passed; exact10000 traces/12 steps/seed1010 reached all six witnesses without a sampled safety violation. The TypeScript backend preserves actual UInt128 bounds; the default Rust evaluator limit failure is retained. Seven complete circuits at actual registry widths1/2/16 were measured without parameters; maximum k12 is an engineering observation, not native certification of all shapes.

These checks and four native negatives do not establish adversarial source-to-constraint soundness for all programs. Public ABI, artifact, relation, proof and binding are revision2; historical revision1 evidence remains separately scoped. Source-derived finite registry closure does not authenticate the real predecessor or prove that all real duties were supplied. The complete roadmap U0–U7/M1–M17 and MC01–MC08 exits remain open. Fresh full current-source and actual-result Sol6.1HIGH/Grok4.6HIGH audits precede publication and merge; this result record grants no approval.
