# Independent expected outcomes for a minimal R5 verify-only successor

Reviewer: independently dispatched Sol reviewer, requested gpt-6.1-sol/high. Date: 2026-10-01. This is an expected-outcome and root-cause review before source author repair. It is not a new source approval, resource/allocation vote, or native execution result.

## Evidence and boundary

Evidence root: `/home/charl/research/moriarty-signed-intent-2026-10-01`.

I read the complete R4 root-cause JSON, R4 control helper, history-case specification, official decoder file, relevant native decoder/history/verifier source, terminal receipts, native acceptance/application/replay records, alternate funding records, configurations, and complete verification log. I independently streamed and matched all 88 absolute-path digests in `NATIVE-FINANCIAL-V10-R4-ACTUAL-RESULT-FREEZE.json` and all 67 in the successful proof result freeze. The two maps have no conflicting common entries; six historical paths in the proof freeze are absent from the actual-result map, so each freeze was checked independently. I also checked all four fault binaries against their specified sizes/digests and the exact successful R4 funding artifacts. This inspection did not import or execute the candidate/helper, invoke native commands, build, fetch, change units, or mutate existing records. No current or future R5 counterpart report was read. The only new file written is this report.

Principal identities:

| Evidence | SHA256 |
| --- | --- |
| R4 actual-result freeze, 88 paths | `ad34703a8e6a5a11249bcefadaa46824781c1ebb8f6a2cc98583409ce60dc29f` |
| Complete R4 root-cause JSON | `7da98368603ad87515880b377ad5d68a17ead4ad19bc40de2e4309c397f3a244` |
| Successful R4 proof result freeze, 67 paths | `e02f9372f9846a600dbf6927b40fe5d0dbdddc5c0cff8a77f4c24bd8570cefa0` |
| R4 control helper | `4dc92714db7aee473e5de8944f0d48217ab96bf30ced963b054bf7074ea98aca` |
| Original immutable history-case specification | `103fa0e30387d64464e8672bc35eba7ba889a93529566bea2e1bf0069f12fa47` |
| Official `serialize/src/deserializable.rs`, ledger checkout `9a8777c` | `d8c026b0e0a8e70abc8ff646697aa86cb205348f8993e9bd42d7781c1e85b7e6` |
| Successful R4 proof, 6336 bytes | `32a8aed312ddaebf5b41d98c38ec4e0d381aa999b556c9c76cdfb729f24636a3` |
| Unchanged native ELF | `ae7e8dab21c4cf09143fa7d58741c0e18ec5f525018de1b8b4099f6fe5a18239` |

The official file is `/home/charl/.cargo/git/checkouts/midnight-ledger-b2f9c59d942dfdca/9a8777c/serialize/src/deserializable.rs`. The native source is `native-ledger-reward-funded-v10-r2-candidate/src/` under the evidence root; the ELF is `/home/charl/research/moriarty-crypto-2026-09-30/target/debug/beta-native-ledger-consumer`.

## Actual result and root cause

R4 proving succeeded: the supervisor receipt has exit 0, no stop reason or supervisor error, preserved source/ELF identities, and `native_postconditions_passed=true`. Its output includes the 6336-byte finalized proof and full native `Success` application. The receipt records actual fee `1117490000000001`, allowance `100000000000000000000`, available DUST `5000000000000000000000`, and remainder `4999998882509999999999`. These satisfy positive fee, fee <= allowance <= available, and fee + remainder = available. The fixed native acceptance predicates include escrow 8990, the 1000/10 A1 output split, expected contract storage, NIGHT accounting/supply invariants, and replay refusal with unchanged accepted state.

The separate R4 verification helper exited 1 after 33.827 seconds. Its supervisor reports no resource stop or supervisor error, preserved source/ELF identities, and `native_postconditions_passed=false`. The first native verifier invocation nevertheless completed successfully: indices 0 through 1034 account for all 1035 PI mutation refusals; missing/extra PI, address, entrypoint, communication, gas, effects, proof corruption/truncation/suffix, four tagged suffix controls, and absent ownership signature were also refused. The native verifier performed full financial acceptance and wrote an independent receipt byte-equal to the original R4 native receipt (SHA256 `3ffc8ee3105aa265087dcacb6f07e82eb120e07dae6077efd7858573a8569c65`). The alternate public funding producer then passed its existing predicates.

The first additional history case, `history-suffix`, returned native exit 1 and created no acceptance output directory. Its log is exactly:

```text
Error: Custom { kind: InvalidData, error: "Not all bytes read deserializing 'midnight:vec(ledger-state[v18]):'; 1 bytes remaining" }
```

The helper failed at line 51 because the case expected `outer tagged decoder trailing bytes`. Native `artifacts.rs:22` calls `tagged_deserialize(...)?` before its redundant local cursor guard at line 23. Official decoder lines 29-30 select `ensure_consumed=true`; lines 112 and 119-131 deserialize the value, count remaining bytes, and return this type-specific `InvalidData` error. Propagation through `?` prevents reaching the local guard. `funding.rs:129-134` decodes history, then claim, before canonicality and reconstructed-history checks; `verify.rs:22` invokes that route before output creation at line 60. The rejection stage and missing output therefore agree with the pinned source. The defect is an incorrect helper expectation, not native acceptance of trailing data.

Only the first of six additional history refusals ran. The other five and the helper's final pristine verification did not run; `history-control-results.json` and `NATIVE_PUBLIC_HISTORY_CONTROL_SUITE_GOOD_AGAIN_OK` are absent. The observed `INDEPENDENT_CONDITIONAL_NATIVE_LEDGER_GOOD_AGAIN_OK` belongs to the successful first native invocation; it does not establish the later helper good-again at lines 55-57. Both original R4 reservations remain consumed and immutable.

## Exact suffix corrections

The successful base history is 20933 bytes, SHA256 `1672df97fd59f14af4f393051cc8e3f33dec805d487fda8540a6fb9c1e40955b`. Its literal header is `midnight:vec(ledger-state[v18]):`. The base claim is 221 bytes, SHA256 `1185a81d8fc859634c052d9fd4f980dd9bb6d0a2a6097c74f2e2555226308d08`; its literal header is `midnight:transaction[v12](signature[v2],proof,pedersen-schnorr[v1]):`. Each pinned suffix fault is exactly its base plus one zero byte. The official error formats the expected global/type tag and the remaining-byte count, so the two corrected `expected_error` values must be:

```text
history-suffix:
Not all bytes read deserializing 'midnight:vec(ledger-state[v18]):'; 1 bytes remaining

claim-suffix:
Not all bytes read deserializing 'midnight:transaction[v12](signature[v2],proof,pedersen-schnorr[v1]):'; 1 bytes remaining
```

The first is directly observed. The second is a source-derived expectation, grounded in the successfully decoded original claim, its actual pinned header, and the exact one-byte suffix; it is not an observed R4 claim-fault result. Preserve the current exact-substring assertion against the full type-specific text. A generic `Error:`, `InvalidData`, trailing-byte fragment, either-message alternative, different tag, or arbitrary remaining-byte count would weaken this requirement. These two cases must continue to require native return code exactly 1, no output directory, matched fault SHA/size in the generated configuration, and absence of both host identity refusal messages. Exit 0, signal termination, panic/crash, premature identity failure, any output directory, or an unmatched diagnostic fails the case.

## Remaining cases and completion conditions

| Case | Pinned artifact SHA256 / bytes | Required outcome |
| --- | --- | --- |
| history-suffix | `a0f1dd4e30fccead9eb126e38beaf0c5738cfb252ab2c38054d73492c92c1da3` / 20934 | Exact history EOF message above; native exit 1; no output |
| history-truncated | `b06b4a5b17023eea799a9d59fc5c8976ab83f466c4fa24a5aef7b8944a614bab` / 10466 | Native decoder error; native exit 1; no output |
| claim-suffix | `33eb5125acc386360290893362be7196df81e29166b3dbcc162f5009906dd4fb` / 222 | Exact claim EOF message above; native exit 1; no output |
| claim-truncated | `0daaa62db251131756541777c4f51378337adf7cce6f5422c74c2adfba47eda9` / 110 | Native decoder error; native exit 1; no output |
| canonical-alternate-history | Produced by the unchanged public-fixture preflight; hash must differ from original history | Exact deterministic reconstruction refusal; native exit 1; no output |
| canonical-alternate-claim | Produced by the same unchanged public-fixture preflight; hash must differ from original claim | Exact deterministic reconstruction refusal; native exit 1; no output |

Both truncations are exactly the first floor(base_length/2) bytes of their respective original artifacts, retaining complete matching outer headers. They must fail during payload deserialization; official line 112 propagates its payload error before the suffix-count branch. R4 never reached these cases, so this evidence does not identify their exact runtime error strings. For the minimal repair, retain their existing `expected_error=null` and existing helper line 52 requirement for actual native `Error:` output, together with the matched SHA/size, single-field mutation, code-1/no-output, and exclusion of host identity failures. This retains the original truncation contract; it must not be extended to the two suffix or canonical-alternate cases. Do not invent a particular `UnexpectedEof` spelling without independent evidence, or count a reconstruction/identity refusal as the intended decoder-stage result. Preserve the raw terminal diagnostic for result review.

For each canonical alternate, change only its own funding artifact path and matching digest in a copy of the original verification configuration. Preserve the original fixture, original genesis, other funding artifact, proof, transaction, statement, VK, SRS, and expected contract. Require the existing exact message:

```text
supervisor-pinned funding predecessor history differs from independent native reconstruction
```

The producer itself must retain exit 0, its exact five-file output set, and all existing `funding-preflight.json` predicates: native default WF, native claim applied, supply invariants, failed originals unchanged, and pristine funding good-again equal, with `proof_produced=false` and `financial_ledger_accepted=false`. Its public changes remain NIGHT value +1 and creation time +1. The alternative history also differs in operation registration: this is not an isolated timestamp-only test or evidence of authenticated external history. R4's producer passed, but neither alternate verifier refusal was observed. Newly produced alternative bytes must be checked/recorded rather than silently trusting old alternate files.

After all six refusals, invoke the complete pristine native verifier with the original successful R4 artifacts in a fresh good-again directory. Require exit 0, full unchanged financial acceptance, and receipt byte equality with the successor's first good receipt and the frozen R4 native receipt. Rehash the original input artifacts after the suite. Preserve the existing eight distinct records: alternate producer, six refusals, and pristine good-again; exact case set; `all_six_refusals=true`; `pristine_good_again=true`; and the helper completion marker. The outer supervisor must also complete with no stop/error, exit 0, preserved identities, unchanged proof output set/digests, and all native postconditions. A first-good marker, producer success, partial record set, or fabricated suite flag alone cannot establish completion.

## Minimal repair scope

Create a new R5 case specification that copies the immutable original and changes only the two suffix `expected_error` strings above. Retain every binary path/digest/size, classification, base digest, alternate fixture delta, and qualification. Update a new helper to consume that specification with its actual final digest and use virgin R5 verification/log paths. Keep the native commands, per-case assertions, six-case order, complete first-good and final good-again, and all acceptance predicates intact.

The successor wrapper must be verify-only and reject a prove allocation. It must adopt the unchanged successful R4 proof directory, root-created 67-path proof freeze, and successful R4 prove receipt; it must not expect or create an R5 proof. Retain the complete frozen R4 actual result as immutable failed-verification provenance and verify its identities before/after successor execution. Give the successor its own source/input freezes, authorization binding, exclusive attempt reservation, config, receipt, logs, and output namespace. Do not overwrite, delete, reset, append to, or reuse either R4 reservation/output namespace. Necessary namespace and pin changes must be explicit in the later exact-source review.

Do not rebuild or modify Rust, ELF, keys, SRS, IR, transaction, proof, statement, genesis, financial predicates, or production handoff. Retain verification limits: wall 120 seconds, group CPU 240 seconds, group RSS/address space 2 GiB, file ceiling 2 GiB, incremental cache 1 GiB, incremental target 2 GiB, total target 10 GiB, free floor 10 GiB, combined proof/verification output 512 MiB, Rayon 2, namespace/environment guards, and process-group termination. Preserve accounting for retained R4 outputs when adding fresh R5 verification outputs. Existing sampling/overshoot qualifications and the prior `RuntimeMaxSec`/`TimeoutStopSec` distinction remain applicable; this report changes no launch cap. No automatic retry or new proof attempt follows a successor failure.

The minimal correction is justified for a separately reviewed, newly authorized verify-only successor. This report supplies expected outcomes, not permission to launch it. Scoped trusted-genesis financial Success has actual evidence; completion of the additional history suite remains pending. Authenticated mint/deployment/funding, generic history/PCD, compiler correspondence, and Preview settlement remain outside this evidence.
