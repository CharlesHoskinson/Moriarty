# Aggregation oracle: `--step` interface

Context: Aggregation (slice B, `research/sherlock-e2e-spec-20261115/bus/ddd-B.md`). Specs: `sherlock-e2e-node-aggregation` (NAG-001 to NAG-011), `sherlock-e2e-gp-hooks` (GPH-001 to GPH-008); reused NODE, SHP, CLM, CONT, FREG, FVR (modular-leaf), GP-001 to GP-004, LPC-001 to LPC-003. Lean package: `formal/sherlock-e2e/aggregation/lean/` (namespace `Aggregation`). Coverage rows: `coverage.tsv`. Findings: `FINDINGS.md`.

## Build and run (odin)

```
formal/sherlock-e2e/scripts/odin-build.sh aggregation
ssh odin 'cd ~/sherlock/formal-e2e/aggregation/lean && LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step'
```

Replay command:

```
quint_replay.py --oracle ssh odin 'cd ~/sherlock/formal-e2e/aggregation/lean && LEAN_ABORT_ON_PANIC=1 .lake/build/bin/oracle --step' \
  --traces <dir> --op-var lastOp \
  --compare segOn,segPre,segPost,segLo,segHi,leaves,works,depsD,sealed,claimSteps,outcome,lAccepted,mwKeyOk,foreignUnits,suspended
```

## Protocol

- Request `{"state": St, "op": N}`; answer `{"state": St}`, `{"refused": [CODE, ...]}` (state unchanged) or `{"error": TEXT}`.
- `op = code * 16 + arg`, `arg < 16`; the model keeps it in `lastOp`.
- Integers are JSON integers >= 0 (or canonical decimal strings); booleans are JSON booleans; the state object has exactly the 40 keys below.
- The `c*` fields are the candidate input of the next operation. The model chooses them freely in the pre-state; the oracle reads them and returns them unchanged. Do not compare them.
- When several guards fail, the oracle returns every code, in the order of the guard table.
- Default mode (`kink-implementation/1`): `inputs` = every state field plus `ops`; refused ops leave the state unchanged; `could-not-apply` with `outside-capability` when the initial state is outside the domain.

## State fields

| Field | Type | Meaning (requirement) |
|---|---|---|
| `runProg`, `keyRoot`, `regRoot` | nat | `prog_d`, `key_root`, parent `reg_root` of the run (NAG-003, NODE-008) |
| `sMax` | nat | `S_MAX` (NODE-002) |
| `mwColumns` | nat | columns of the MERGE-WORK key (NAG-005) |
| `nodeArity` | nat | node arity (NODE-007) |
| `segOn` | bool | the accumulator holds a SEG |
| `segPre`, `segPost`, `segLo`, `segHi` | nat | accumulated SEG: `pre_d`, `post_d`, `h_lo`, `h_hi` (NODE-001) |
| `chipSets` | nat | bit c set: a leaf of chip set c merged (NAG-004) |
| `leaves`, `works` | nat | merged children, MERGE-WORK nodes |
| `depsD` | nat | `deps_d`; 0 = `EMPTY_DEPS` (CLM-004) |
| `sealed` | bool | SEAL or SEAL-WITH-DEPS done (NODE-003) |
| `claimSteps` | nat | `n_steps` of the sealed claim |
| `outcome` | nat | outcome word fixed at the seal |
| `lAccepted` | bool | `L` accepted the root (NAG-008) |
| `mwKeyOk` | bool | MERGE-WORK key made (NAG-005) |
| `foreignUnits` | nat | FV units accepted under N3 (GPH-003) |
| `suspended` | bool | run suspended (GPH-006) |
| `ckPost`, `ckHi` | nat | checkpoint `post_d` and height |
| `cKind` | nat | candidate key selector: 0 LEAF, 1 NODE (SEG), 2 FOREIGN |
| `cProg`, `cKeyRoot` | nat | candidate `prog_d`, `key_root` |
| `cPre`, `cPost`, `cHPre`, `cSteps` | nat | candidate `pre_d`, `post_d`, `h_pre`, steps |
| `cAirs` | nat | bits: StmtAir 1, ProofShapeAir 2, PublicValuesAir 4; all = 7 (NAG-002) |
| `cRegRoot` | nat | candidate `reg_root` (NODE-008) |
| `cChipSet` | nat | candidate chip set (NAG-004) |
| `cForged` | nat | 0 honest; 1 MW-F01, 2 AFK-F7, 3 GP2-F4.2, 4 AFK-F9(b), 5 GP2-C2.1 (NAG-006, NAG-007) |
| `cKeyRow` | nat | `keys_d` row the candidate is verified at: 0..6 node rows, 7..8 foreign rows (SHP-009, GPH-003) |
| `cStatus` | nat | 0 Running, 1 Halted, 2 Rejected, 3 Exhausted (CONT-003) |
| `cOutcome` | nat | outcome word of a seal, or the word `L` checks (NAG-008) |
| `cChain` | nat | `DL.chain_d` of SEAL-WITH-DEPS (CLM-004) |
| `cManifestOk` | bool | the checkpoint manifest equals the key manifest (CONT-004) |

State domain (`Valid`, else `outside-capability`): `lAccepted` implies `sealed`; `sealed` implies `segOn`, `segLo = 0` and `claimSteps = segHi`; `mwKeyOk` implies `mwColumns <= 5400` and `nodeArity <= 4`; `segLo <= segHi`; `suspended` implies not `sealed`.

## Operations

| code | op | arg | guards in order (code, requirement) | effect |
|---|---|---|---|---|
| 0 | merge | 0 | `suspended` (`suspended`); `sealed` (`sealed`); `cAirs != 7` (`air-missing`, NAG-002); `cKind` not 0/1 (`key-selector`, NODE-009); `cProg != runProg` (`prog-d-mismatch`, NAG-003); `cKeyRoot != keyRoot` (`key-root-mismatch`); `segOn` and (`cPre != segPost` or `cHPre != segHi`) (`link-broken`, NAG-009); LEAF with `cSteps = 0` or `cSteps > sMax`, or `cHPre + cSteps >= 2^64` (`range`, NODE-002); LEAF with `cRegRoot != 0`, or NODE with `cRegRoot != regRoot` (`reg-root-mismatch`, NODE-008); NODE with `cKeyRow >= 7` (`key-row`, SHP-009); `cForged != 0` (`forged-witness`) | `segOn := true`; if not `segOn` before: `segPre := cPre`, `segLo := cHPre`; `segPost := cPost`; `segHi := cHPre + cSteps`; `chipSets := chipSets OR 2^cChipSet`; `leaves += 1` |
| 1 | mergeWork k | k | `suspended`; `sealed`; not `mwKeyOk` (`no-work-key`); not `segOn` (`no-child`); `segOn` and link broken (`link-broken`); `k = 0` or `segHi + k >= 2^64` (`range`); `cForged != 0` (`forged-witness`) | `segPost := cPost`, `segHi += k`, `works += 1` |
| 2 | keygenWork | 0 | `mwColumns > 5400` (`columns`, NAG-005); `nodeArity > 4` (`arity`, NODE-007) | `mwKeyOk := true` |
| 3 | seal | 0 | `suspended`; `sealed`; not `segOn` (`no-child`); `segLo != 0` (`not-from-start`); `cStatus = 0` (`not-terminal`); `depsD != 0` (`deps-nonempty`) | `sealed := true`, `claimSteps := segHi`, `outcome := cOutcome` |
| 4 | consume c | c | `sealed` (`sealed`); `cStatus = 0` (`not-terminal`); `cKind = 1` (`not-a-claim`) | `depsD := depsD * 16 + c + 1` |
| 5 | sealWithDeps | 0 | as seal, but `cChain != depsD` (`deps-mismatch`) in place of `deps-nonempty` | as seal |
| 6 | lCheck | 0 | not `sealed` (`not-sealed`); `cOutcome != outcome` (`outcome-mismatch`) | `lAccepted := true` |
| 7 | foreignUnit | 0 | `cKind != 2` (`key-selector`); `cKeyRow < 7` or `cKeyRow >= 9` (`key-row`); `cForged != 0` (`forged-witness`) | `foreignUnits += 1` |
| 8 | suspend | 0 | `suspended` (`suspended`); `sealed` (`sealed`); not `segOn` (`no-child`) | `suspended := true`, `ckPost := segPost`, `ckHi := segHi` |
| 9 | resume | 0 | not `suspended` (`not-suspended`); not `cManifestOk` (`manifest-mismatch`, CONT-004) | `suspended := false` |
| other | | | `outside-capability` | none |

## Suggested constants and fill

`--const runProg=5 --const keyRoot=9 --const sMax=100 --const mwColumns=5284 --const nodeArity=2`; `regRoot` 0 or a nonzero constant (NAG-011). Model-only fields such as `ckPost`, `ckHi`, `chipSets` may be filled: `--fill ckPost=0,ckHi=0,chipSets=0` when the model does not track them (then leave them out of `--compare`).

## Open interpretation points (ledger)

| ID | Point | Status |
|---|---|---|
| AG-1 | SHP-003 "two enabled prior SEG proofs" against NAG-011 "one linked child accepted": the oracle accepts one linked child and refuses none (F-AGG-1). | held (finding sent) |
| AG-2 | GP-2 modes MERGE, DESCEND, JOIN and DL-STEP are abstracted into one `consume` that extends `deps_d`; the claim content is not modelled. | held |
| AG-3 | The forged witnesses of NAG-006 and NAG-007 are a tag `cForged`, refused by every node op; their constraint-level content is in the regression tests. | held |
| AG-4 | NODE-004 key commitments are abstracted: a LEAF key by `key_root` equality, a NODE key by a node row 0..6, a FOREIGN key by a foreign row 7..8. | held |
| AG-5 | `L` checks the single-run root; the collection form (GPH-008) is not modelled. | held |
