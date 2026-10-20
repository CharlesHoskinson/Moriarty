# Retained Push configuration and positive K context projection

## Select the interface

The public backend module is [`mori_backend::sherlock_machine::c6_push_step`](../../../crates/mori-backend/src/sherlock_machine/c6_push_step.rs). Its CI4 source composes bound META and immediate CTX binding with the existing Push relation. The [source record](../../results/sherlock-c6-bound-meta-retained-family-source-20261020/README.md) retains the two fresh independent Muse source opinions. At this documentation cut, its 11 Push and 13 META tests are **UNRUN**, and the integrated backend has not been compiled or checked through the SDK.

| Interface | Inputs and result |
|---|---|
| `C6PushFamilyParams::new(cfg, admission_bounds, limits)` | Owns the original `C6HostConfig`, separate `BoundsC` and `C6HostLimits`; returns a checked retained family or `C6PushStepError`. |
| `cfg()`, `admission_bounds()`, `limits()` | Read the retained inputs. Private fields prevent direct replacement through struct construction. |
| `meta_binding()`, `expected_ctx()`, `caps()`, `geometry()` | Read derived binding, 22-lane CTX, state caps and padded geometry. |
| `validate_geometry(supplied)` | Recomputes geometry/caps from retained config/limits and refuses a mismatch before emission. This is a host check. |
| `build_c6_push_step_circuit(family, meta_len, prog_len, frames_offset, mode, scope)` | Builds the source-declared circuit for the selected family and exact statement extents. Use the public module path; the builder is not re-exported at the `sherlock_machine` root. |
| `witness_from_host_pair(enable, pre, post, cfg, limits)` | Encodes an independently supplied pair without running a recorder. It checks equal host prefixes and derives the witness's family using `pre.admission.bounds`. |
| `witness_from_owner_step(admitted, image, selection, cfg, limits)` | Runs one real A135/A140 owner recorder and creates a witness from that record. This call mutates `image`. |
| `check_c6_push_step_circuit(circuit, witness)` | Calls non-proving `ProofPreimage::check`/preprocess. It neither authenticates verifier family selection nor performs key generation, proving or native verification. No execution of this CI4 checker is reported here. |

`C6MetaConfigBinding` retains **two separate 22-value bounds vectors**: admission bounds and layout bounds. It also retains the complete remaining layout segment, four layout capacities and limits. The family derives expected CTX from the original `cfg.ext` and `cfg.budgets`. Absent extension and present-all-zero extension remain distinct; lane 18 carries raw `u128` lifetime, and lanes 20–21 carry the two budgets.

## Emission and statement boundaries

Push calls bound META once. It uses one retained `C6ProgramContextCanonicalEmitter` for PROG/CTX parsing followed by `emit_ctx_binding`, so the temporary serial continues across both calls. The parent ranges and equates pre/post META, PROG and CTX unconditionally. Canonicality and family-binding assertions use the enable gate; gate zero does not establish family membership. DATA remains one emitted call; separate HELPER canonicality is not integrated.

`C6PushStepWitness::to_statement_frs` declares this order:

```text
enable, selector_is_accept, host_schema, carrier_version,
pre(META, PROG, CTX, IMAGE, DATA, HELPER),
post(META, PROG, CTX, IMAGE, DATA, HELPER)
```

`LocalPublic` uses public-input instructions and no IR return vector. `LedgerCall` uses call inputs, enables communications commitment and returns the post IMAGE/DATA/HELPER lanes. These are source interfaces, not an observed ledger invocation.

The family is **caller-selected**. Copying it from a witness does not establish an independently selected verifier family. Private fields and host derivation checks do not supply authenticated source provenance, verifier/invocation selection, IR reconstruction, issuer keys or a native proof. Those remain separate P1/native obligations.

## K positive projection: what was checked

The [K source and evidence](../../../models/sherlock/k/c6-positive-context-20261020/README.md) define the configuration-only profile `sherlock-c6-context-positive/1`. Four ordered positive inputs retain extension presence, all 16 extension fields, two budgets, 22 bounds, four layout capacities and five host limits. They are unadmitted configuration carriers, not an owner runtime step or a complete `C6HostConfig`.

The [field port](../../../models/sherlock/k/c6-positive-context-20261020/source/context_ports.py) exports `port_input(document)` and `observations_document(tree, public_source)`. The first transports source fields into a K query; the second reads the whole strictly decoded K result. It does not recompute encoded lanes or derived caps in Python.

The common JSON root has exactly `schema`, `profile`, `case_ids`, `observations`, with schema `independent-c6-context-positive/1`. Each ordered observation contains:

| Field | Complete observed carrier |
|---|---|
| `encoded_lanes` | All 22 CTX lanes, in order. |
| `decoded_context` | Original extension option and both budgets, decoded from the produced lanes. |
| `effective_ext` | All 16 effective extension fields. |
| `state_context` | All 27 physical caps, two additional state caps, extension-presence flag and state limit. |
| `state_descriptor` | All 29 cap positions, in declared order. |

The published source copies retain historical absolute paths; they have not been validated as relocated runnable entry points. No command to replay them is introduced here. The [runtime consumer](../../../models/sherlock/k/c6-positive-context-20261020/source/context_runtime.py) bridges primitive typed nodes from `{sort,value}` to `{sort,literal}` and preserves compound `{sort,constructor,arguments}` nodes. Its `compare_manifest` compares entire common JSON and entire typed AST. Object key order is immaterial; exact key sets, array/argument order and native scalar types are retained, including Bool versus Int.

The [Root actual result](../../results/sherlock-k-c6-positive-context-20261020/actual/ROOT-RESULT.json) records `PASS_WHOLE_TYPED_BOTH` for the complete common JSON and typed AST comparisons. The separate [final readback](../../results/sherlock-k-c6-positive-context-20261020/provenance/ROOT-ACTUAL-EXACT7-FINAL-READBACK.json) records all seven phases passing and 13 unit tests verified. The complete actual artifacts were durably frozen with a 134-member barrier before the comparison child opened the two prefrozen expectations; Root recorded zero private semantic reads. The shared natural-method/input and oracle-author correlation remains disclosed. The schema's word `independent` labels the frozen wire; it does not itself prove independence or Rust conformance.

These observations supply no K cost clock, charged transition, full host round-trip, CTX-to-PROG agreement, admission/authentication, circuit/SDK/native proof, stagenet result or EARS closure. The [usage guide](../how-to/select-retained-push-family.md) keeps the source interface and this finite K evidence separate.
