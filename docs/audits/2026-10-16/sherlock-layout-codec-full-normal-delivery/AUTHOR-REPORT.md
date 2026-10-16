# Full N01–N08 host codec observations, 2026-10-16

One fresh, approved standalone pure-stdlib invocation completed an **internal typed Step-mode encode/decode/materialize round trip for all eight existing normal programs: 216 fuel-prefix traces and 5,355 ordered records**. Separately, the same Machine traces matched the complete independently frozen corpus projection, with first_difference:null. The external Machine-to-corpus agreement supplies no codec evidence of its own.

No capacity changed and no Rust attempt was retried. The complete artifact retains every selected prefix and every diagnostic field. This adds a full-normal-corpus typed round-trip observation to the separate earlier 35-test baseline and N01/N02 result; it does not rerun those 35 tests or transfer their evidence to new bytes.

## Exact attempt and bindings

The single command was python3 -I -B run_prefix.py in the NEW full-prefix-01 runtime. Actual rustc 1.99.0 compiled --cfg test with exact 27dep-info members; all 44snapshot members and reviewed source/candidate/tool hashes matched before and after. Executed steps were all ok, and compile-tests/run-tests were explicitly not-selected. The compiler emitted 89 retained warnings. The producer and comparator returned exit 0 with empty stderr. Compilation, producer and comparison took 0.616, 0.566 and 0.164 seconds respectively; these are this attempt's timings, not resource adequacy or general costs.

Manifest SHA256: 909bcd4b0f79af58727463d49a0adf153b3b48b540c915ff8faf69e1a03d87ad. Receipt SHA256: a66f998bbb3820137a5f607aefde2db7a8857f36e8026f7ecf74bfd5db279cc7. New runtime-only source SHA256: b936873a8ce70ff6f1dd1c7399ae8cc6fd9858f1c021410e1c1d6b6df1ef4841.

Complete artifact: 8,003,727bytes; SHA256: 6a168de94ce191b8dff085881bacd4afd4cd287932ebf11cb1acf217d7adc159. Removing its single final newline produces 8,003,726bytes and SHA256: e37111954f533e828113fe9592da448926ca801956e4799c941ebead162eea84, exactly the independent compact-projection identity frozen before the full8 source extension. That is byte identity, not semantic inspection by a digest.

| Case | Fuel-prefix runs | Ordered records |
| --- | ---: | ---: |
| N01 | 3 | 5 |
| N02 | 9 | 44 |
| N03 | 11 | 65 |
| N04 | 19 | 189 |
| N05 | 29 | 434 |
| N06 | 21 | 230 |
| N07 | 40 | 819 |
| N08 | 84 | 3569 |
| Total | 216 | 5355 |

Root task transport read the complete artifact, counted every run/record and case/fuel order, recomputed its hash/length and matched the producer receipt and compare report. It checked all four expectation-source pins, five frozen-file pins, both literal transcriptions and serializer pin, exact compile membership, required steps, release and before/after source/tool/candidate bindings. The existing comparator parsed and compared the complete expected and observed documents; no field or prefix was dropped, truncated or sampled for that comparison.

Fresh actual-result reviewers receive complete exact source, frozen inputs/literals and mechanical complete-check receipts. Their inline raw-reading sample is explicitly 19 runs/292 records: zero-fuel and completion+1 for every case, plus N05 fuel 13, N07 fuel 19 and N08 fuel 41. The complete 8 MB artifact remains retained at its exact pin. No full independent reviewer reading or rehash of all 5,355 raw records is claimed. Requested/returned reviewer identities, high effort, terminal outcomes and their reading limits belong to the adjacent immutable review receipts; this author record does not predict unwritten verdicts.

## Exercised and unexercised scope

The actual path calls existing Machine, prepare_a, encode_observed_a, Step-mode materialize_trace_a and first_difference. It propagates typed preparation/state failures. Every complete typed trace must pass the in-binary internal-preservation gate before output. Expected observations come only from the existing frozen normal and extra-normal JSON through unchanged expected_observations/compare_observations/first_difference, after the existing frozen corpus/source/transcription check and family validation. No evaluator/encoder/decoder supplies expected rows. The expected freeze was not an execution input.

The example dimensions are unchanged: program tables [64; 15], data 64; state data 64/data_fields 64/env_cells 32/value_slots 32/frame_slots 16/control_rows 16/stack_limit 32. Corpus stack limits remain 8 and the observed maximum frame depth is 7. These are fixed examples, not defaults, production capacities, resource admission or an F3 unfolding envelope.

Compared with N01/N02, exercised typed data include Bool, List/Nil/Cons and Leaf values; nonempty current/saved environments; Let/Scope, If, Unary, Match/Cases/CaseResult including NoMatch; Invoke/Enter/Bound(Some); Args/Return frames; multiple arguments and recursion. This does not exercise Rejected outcomes, Error or PrimitiveResult(Err), Enter(None), Bound(None), frame-cap error 4, Branch values, Cons/Branch/Lt operators, nested lists, F3 unfolding or refusal/hostile paths. It is coverage of these fixed normal runs, not general grammar coverage.

Only the internal typed round trip supplies new codec evidence. Program and expression syntax remain retained from PreparedA/source occurrences, so this is not ProgramBlock row reconstruction. Physical StateBlock images existed inside the producer but were not emitted as an independent raw physical-image artifact. The producer did not reach program_bytes, observed_state_bytes_a/expand_state, project_a, materialize_record_a, compare_a or AuxBlock::ordered_bytes. There is no new canonical-mode, canonical-uniqueness, byte-expansion/parser/wire-format, projection, auxiliary-packing or independent physical-codec result.

The plain-child compare step retains the existing S1 binding gap; Root's required post-run rehash and cross-bindings close the recorded gap for this trusted local sequential attempt, without a guard change. Reviewers independently rehashed nothing unless their own separate receipt states otherwise. Both source preflights requested high effort and returned canonical claude-opus-5-5 and muse-wonder-1.0 identities. Their complete reports control all execution conditions; no future run is authorized by this observation.

Direct argv, minimal environment, process groups and 30 seconds per child came from the unchanged inspected host runner. Memory/disk/process count are not bounded, and sysroot/linker/libc/Python-stdlib/kernel are unhashed. Native authority stayed 0. No Cargo, SDK, K, Quint, Lean, Clean, handwritten-circuit, proof, ledger, target or private-material operation ran. Machine::new supplies post-startup state; K constructor rule 74 was not executed. Source UNRUN labels and stale script prose remain source-time history; the separate current records describe this run.

The full8 source remains a NEW runtime-only copy. The earlier observed Wleaf/current35+12 publication is unchanged. Root alone owns canonical registration and Git; no CLI/root/public input endpoint is added and no production Core compile or registration is certified. H1/retained PreparedA trust, CandidateA/F3/F5, C4 and its registration-revert boundary, all retained source-preflight conditions/dissents, P3/full39/L4, G4/P4/P5/F4/MD2/StepInv/Constant typing/D-NAMES/O-ARITH/L04/L05, old caps/counters/failures and every U/M/MC/SP/G financial/proof/native/target obligation remain unchanged. No B-profile or parent acceptance follows.
