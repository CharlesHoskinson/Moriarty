I'll study U0 through the target-and-enforcement lens only: pins, the enforcement map, trust premises, and the backend matrix. Reading those artifacts and the checkers that score them.The exit gate records pins and premises as open. Next I’ll read how the checkers score a pin as historical, and which enforcement and backend rows actually block U1 and U2.## L2 proposal: target pins and enforcement

### 1. Goal

U1 and U2 need a Midnight target they can execute, and a binding plan that says which stage-relation leaves that target must constrain. U0 has to produce three things.

First, one reverified released tuple: compiler, emitted ZKIRv3, verifier, keys, SRS, ledger, and the node proof-verification flag, demonstrated by prove, then verify, then a one-byte rejection. Failed interfaces stay blocked, as `ROADMAP.md:22` already requires for U1.

Second, a second, explicitly incompatible native-IVC tuple, so recursion is not silently folded into the released path. The March 2027 date stays a planning assumption (`ROADMAP.md:15`, trust premise TP02).

Third, a field disposition over all 84 leaves: which identity fields the released proof must carry, which leaves U2 must enforce at a native boundary, and which leaves are deferred with an owner. Signed-intent authentication has to be located (circuit, ledger primitive, or another native boundary), as `docs/MORIARTY-CONSOLIDATED-DESIGN.md:44-46` requires. It does not have to be implemented in U0.

### 2. Findings

Re-ran at `8f737840`: `check_u0_target_pins.py` prints `11 historical, 0 absent, compatible tuple NOT established`; `check_u0_enforcement_map.py` prints `0 enforced, 0 host-only, 84 NOT_ENFORCED`; `check_u0_trust_backend_matrix.py` prints `24 backend rows specified-only, 9 trust premises`. Those match `deliverables/u0-semantic-contract-2026-09-23/EXIT-GATE.md:11-14`. They are citation checks. On this machine `compactc` is not on `PATH`, `midnightntwrk/proof-server:8.1.0` is not a local image, and `/home/charl/midnight` is absent. The git objects named in the September inspections cannot be re-read here.

**The success state of the pin ledger is unreachable.** `target-pins.schema.json:23-26` sets `compatibleTupleEstablished` to `const: false`. `scripts/check_u0_target_pins.py:40` allows only `historical` and `absent`. Lines 243-244 fail the run if the flag is true. Lines 147-149 always print `NOT established`. Lines 62-64 ban the words `verified`, `reverified`, `compatible`, `current`, and `validated` in `note` and `selectionReason`. `tests/test_u0_target_pins.py:600-608` locks that ban. `scripts/check_u0_exit_gate.py:193-194` (G6) fails if the flag is true or if ZR01 is anything but `specified-only`. The same file at line 230 closes the pin row only when the flag is true, there are no historical rows, and `unresolved` is empty. Line 324 always prints `capabilities open`. `scripts/build_u0_backend_matrix.py:210` and `:234` hardcode `specified-only`. A real tuple cannot turn this gate green without changing the checkers. That matches `docs/FOOTGUNS.md:17-20`: a passing checker is not a capability.

**The 11 rows are not one tuple.** `target-pins.json:4` is false, and `unresolved` has 10 narratives while zero components are `absent`. Selection is an editorial tie-break, not a joint execution.

| Component | Chosen pin | Why it is not a current tuple member |
| --- | --- | --- |
| compact-compiler | 0.31.1 | Observed 2026-09-17. `evidence-packet.md:703` cites `compactc-v0.34.0` notes as "not reproduced". Zip sha256 is recorded, not rehashed. |
| zkir | `7dff84a` | Same-day conflict with `2ffe2d17` (`target-pins.json` claims on the zkir row). Emitted ZKIR bytes are a different object from a git checkout. |
| native-proof-system and verifier | both `695351f1` | Verifier is not a separate release. `astra-backend-integration.md:11`: Poseidon transcript, crate-private context, "not a reproduced proof". |
| proving-keys / verifier-keys | loan `accrue` digests | `build-receipt.json:196`: "compiler key artifacts only; no transaction proof was generated". Other circuits in the same receipt differ. |
| srs-parameters | k=17 `4a9ef6c7…` | Chosen because `checked-encoding-resources.json:2` is `source-preparation-only-not-launch-approved`. Ceremony trust is unaudited (`:26-28`). The only recorded Moriarty prove used **k=14** and was not verified (`evidence-packet.md:767-772`, CLM-0958). |
| ledger | `3fa0d1d` | Tie-break against `a8ab82ba`. The published artifact is `@midnight-ntwrk/ledger-v8@8.1.0`, wasm sha256 `88ff7c7c…` (`launch-runtime-pins.json:68-76`). |
| proof-server | `8.1.0@sha256:801bbc03…` | Image digest in compose. Not paired with a verified proof. Image absent locally now. |
| moriarty-compiler | `f702692` vs base `006c4d91` | Source checkouts. `target-pins.json` unresolved text: no compiler release artifact. |
| k-reference-toolchain | `4a46d123` / K 7.1.337 | Semantics oracle. Two kompile digests. Not a Midnight component. |

The transcript split is recorded, not reproduced. `fable-kernel.md:71`: ledger-accepted proofs use a Blake2b transcript that the PR 738 verifier cannot read. `docs/MORIARTY-BACKEND-REQUIREMENTS.md:15` and `:71`: no fresh fetch, build, or proof for that list. `astra-backend-integration.md:13`: a historical `proof-verifying` cfg returns success when disabled. It does not claim Preview is built that way. It does require the flag from the verifying binary. `experiments/moriarty-native-ivc-r3/README.md:1-11`: the IVC loan harness produced no recursive proof at k=17. `docs/MORIARTY-CONSOLIDATED-DESIGN.md:105` keeps that failure as evidence and does not authorize a larger k.

**Enforcement map is an honest absence ledger with no priorities.** All 84 mechanisms are `[]`. The limitation at `enforcement-map.json:11` is right: generated loan/swap kernels constrain anonymous `Uint` fields and do not name a stage leaf. Concrete near-misses the checker would accept if someone cited the identifier anyway, because it only checks that the symbol occurs on the line (`check_u0_enforcement_map.py` C1–C7):

- `authority.remaining` versus `generated/loan/kernel.compact:15` (`assert(remaining > 0)`), a program-local counter.
- `signedIntent.recipients[]` versus swap `kernel.compact:32`, and `signedIntent.minNetOutcome` versus `:74`.
- `signedIntent.grossDebitCap`, `minNetOutcome`, and `recipients[]` versus `evaluate.ts:290-295` on a different host object.
- `signedIntent.signer` (`enforcement-map.json` signer note): Midnight unshielded-input signatures do not bind this field.

`tests/test_u0_enforcement_map.py:106-108` snapshot-locks the live file at 84 `NOT_ENFORCED`. `host-only` would be a false upgrade for those host checks. Twelve `judgments.*` leaves are statuses of this map, not Midnight constraints. Closing the gate only when `unenforced == 0` (`check_u0_exit_gate.py:231`) pulls U3–U4 leaves into U0.

**Backend matrix: the row that defines compatibility has no owner.** ZR01 is "Pin and validate compatible tuple, including node proof-verification configuration; U0/U1" (`MORIARTY-BACKEND-REQUIREMENTS.md:37`). MNR01 (transcript) refines ZR01/ZR03/ZR06 and MNR05 (real proving versus dummy proofs) is the proof-mode rule, but the responsibility table at lines 37-45 lists only ZR ids. `build_u0_backend_matrix.py:221-222` therefore emits `owner: null`, `milestone: null` for every MNR row (`backend-requirement-matrix.json` MNR01). All 24 rows are specified-only. A delivered row needs pins, fixtures, command, configuration, and retained results (`MORIARTY-BACKEND-REQUIREMENTS.md:47`). None exist.

**Premises that actually block.**

| Id | Status | What it blocks |
| --- | --- | --- |
| TP01, TP09 | open, unresolved-interface | Certifying any native proof. U1 may still measure a named candidate. |
| TP07 | open | U2 acceptance of a canonical signed intention (`ROADMAP.md:23`, `MORIARTY-PRODUCT-CONTRACT.md:39`). |
| TP02 | accepted-assumption | Nothing in U1/U2. Blocks treating March 2027 as a shipped recursion interface. |
| TP06, TP08 | open | U4 private handoff and witness availability (ZR14). |
| TP03 | open | U3 observations and finality (MPLR-010). Blocks U2 only if that program claims an observation. |
| TP04, TP05 | accepted-assumption | Federation stays optional. Timeout is not nonexecution. Neither is an open interface. |

`ROADMAP.md:34` already allows U2 a labeled ledger-induced history profile. That profile must not be reported as MC03/MC06.

### 3. Proposed work

**T1. Local presence receipt.** Output: `deliverables/u0-semantic-contract-2026-09-23/tuple-presence.json`. For each released artifact (compactc 0.31.1 zip `e291b4ba…`, proof-server image `801bbc03…`, ledger-v8 wasm `88ff7c7c…`, the k the proof server loads), record present-and-rehashed, absent-locally, or hash-mismatch. Checker: extend `check_u0_target_pins.py` so `historical` still means "quote contains the pin", and a new `bytes` field is `present` only if the tool recomputes the digest. Exit: no component is described as current unless the bytes were rehashed on this machine. Depends on nothing. Effort **S**. The proof-server image and `/home/charl/midnight` are already absent.

**T2. Make a true tuple representable.** Output: schema and checker change. Replace the single `const: false` with two objects, `releasedTuple` and `ivcTuple`, each `unverified`, `blocked`, or `reverified`. `reverified` is legal only when T3's receipt exists and its hashes match. Drop the banned-word regex on prose. Stop hardcoding `NOT established` and `capabilities open`. Update `tests/test_u0_target_pins.py`. Exit: a fixture receipt flips the released tuple to `reverified` and a missing receipt flips it back. Depends on T1's vocabulary. Effort **M**.

**T3. Released-path prove and verify.** Output: `released-tuple-receipt.json` plus retained proof, VK, public inputs, and command log. Use the already recorded versions, not current upstream `main`: compactc that hashes to the 0.31.1 zip unless T1 shows Preview running a newer compiler, proof-server `8.1.0@sha256:801bbc03…`, `@midnight-ntwrk/ledger-v8@8.1.0`. Smallest circuit that already proved (loan `initialize` / CLM-0958, or hello-world). Record emitted ZKIR bytes hash, that circuit's PK and VK (not `accrue`), the k actually loaded, the transcript name read from the bytes, and the `proof-verifying` flag of the verifying binary. Verify with the ledger library. Mutate one proof byte and one public input; both must reject. Exit: CLM-0958's gap ("proved, not verified") is closed, or the receipt says which artifact is missing. Checker: T2. Depends on T1 and T2. Effort **L**. This is the thin ZR01 positive test. Broader version-skew stays U1.

**T4. IVC incompatibility, without another k=17 synthesis.** Output: `ivc-tuple-receipt.json`. Status `blocked`. Either feed the T3 proof into the `695351f1` verifier and retain the rejection, or show from that pinned source that a contract-call proof is not an IVC `Certificate` (`astra-backend-integration.md:58`). Keep k=17 and the unaudited SRS digest on this tuple only. Exit: the two tuples are not claimed compatible. Depends on the pinned `midnight-zk` tree (absent locally) and, for the byte test, on T3. Effort **M**. Owner of a bridge: U4 (ZR02, ZR08).

**T5. Dispositions on all 84 leaves.** Output: `enforcement-map.json` gains `disposition` and `ownerMilestone`. Status enum stays `enforced` / `host-only` / `NOT_ENFORCED`. Cohort A, disposition `u1-identity`: `circuitIdentity.circuitId`, `compilerPin`, `verifierKeyId`, `zkirVersion`, `profiles.numericProfile`. The T3 public inputs must carry these five; a substituted VK or compiler pin rejects. That is the only enforcement U0 demonstrates. Cohort B, `u2-native`, still `NOT_ENFORCED`: `programIdentity.programId`, `programIdentity.entryPoint`, `domain.chainId`, `domain.domainId`, `domain.stateFrameRef`, `signedIntent.signer`, `intentId`, `feeCap`, `grossDebitCap`, `minNetOutcome`, `recipients[]`, `replayPolicy`, `validity`, the nine `effects.gross|net|fees` account/amount/asset leaves, `authority.replayState`, and the three `failurePolicy` leaves. `programIdentity.sourceRef` and `coreRef` stay unbound until the embedding work marks those declarations present. Cohort C: everything else, with owners U3 (observations, liabilities, obligations, outcome), U4 (predecessors beyond single-lineage, disclosures, resources-as-recursion), U6 (supply changes). `judgments.*` (12) and `schemaVersion` get disposition `not-a-constraint`. Checker: bijection plus "Cohort A `enforced` requires a mutation test in the T3 receipt, not only an identifier". Exit: 84 dispositions, 5 demonstrated, the rest explicitly unenforced. Depends on T3 for Cohort A only. Effort **M**.

**T6. Give the transcript and proof-mode rows an owner.** Output: responsibility rows in `docs/MORIARTY-BACKEND-REQUIREMENTS.md` for MNR01 and MNR05 at U0/U1, beside ZR01; regenerate the matrix so those two are not `owner: null`. Other MNR rows get milestone U4 (MNR02, MNR03, MNR07, MNR08) or U2/U4 split (MNR04 verified-versus-auxiliary effects). Status stays `specified-only` until a receipt exists. Checker: `build_u0_backend_matrix.py --check` plus a non-null owner test for MNR01 and MNR05. Exit: the compatibility blocker has a milestone. Effort **S**.

**T7. Split the exit receipt.** Output: `check_u0_exit_gate.py` and `EXIT-GATE.md`. Two headlines. "Contract frozen" when dispositions, premise `blocks` fields (T8), and both tuple records exist. "Capability demonstrated" only for rows with receipts. Delete G6's requirement that the tuple stay false. Do not require 84 enforced leaves or 24 demonstrated backend rows. Effort **M**, after T2, T5, T6, T8.

**T8. Premise blocking and the intent boundary.** Output: `trust-premises.json` field `blocks` using the table in section 2. Do not change TP01, TP07, or TP09 to `accepted-assumption`. Add one decision paragraph on TP07: U2 will bind a circuit commitment to the canonical intent bytes, and the ledger signature checks that commitment. Unshielded spend signatures and `evaluate.ts` remain non-binding, as the map already says. No Ed25519-in-circuit work. The R3 readme already scoped the first native result as fixed authority. Checker: existing trust checker, plus `blocks` required. Effort **S**.

### 4. Exit redefinition

Yes. Freeze and demonstration are different gates, and the current checker makes "capabilities closed" dead code.

U0 demonstrates: T1's presence facts; T3's prove, verify, and one-byte rejection on the released 8.1.0 path, or a named missing artifact; T4's blocked IVC tuple; Cohort A carried in that proof's public inputs; dispositions for the other 79 leaves; MNR01/MNR05 ownership; `blocks` on the nine premises.

U1 (owner: arithmetic certificate) demonstrates the first numeric primitive against Cohort A, version-skew rejection, and ZR12/ZR13 measurements. It may also try one recursive step and record `blocked`. It does not certify while TP09 is open.

U2 (owner: single-stage path) demonstrates Cohort B, including the TP07 signature boundary, effect readback, replay, and retained-fee failure policy, on two programs. History, if claimed, is labeled ledger-induced (`ROADMAP.md:34`).

U4 owns T-ivc, multi-parent predecessors, private handoff (TP06/TP08), and MC03/MC06. U3 owns TP03.

### 5. Risks and things not to do

- Do not `git pull` Midnight `main` and call the result the pin. Restore the recorded 0.31.1 / 8.1.0 / `695351f1` artifacts. A newer compiler is a new tuple only if T1 shows it is what the released node runs.
- Do not raise k above 17, rerun R3 synthesis, or start a ceremony audit. SRS ceremony trust stays an open premise on the IVC tuple.
- Do not cite `kernel.compact` `remaining`, swap asserts, or `evaluate.ts:290-295` as enforcement. The identifier check is weaker than the English notes.
- Do not mark `signedIntent.*` `host-only`. The host checks a different object. The uncommitted Aeon `repay` trace (guards hold, principal and borrower balance go to −1) is why those host checks cannot carry U2. Fixing that encoding is not this lens.
- Do not close TP01/TP07/TP09 by relabeling them assumptions. The trust checker allows `accepted-assumption` on an unresolved interface, and the exit gate would then count the premise row closed.
- Do not treat a proof-server prove as ledger acceptance. CLM-0958 stopped before verify. `MORIARTY-PRODUCT-CONTRACT.md:21`: Compact is intermediate until pinned ZKIRv3 and Midnight acceptance exist.
- Do not add another compiler, a parallel recursion prototype, or a wait for March 2027 before the released path is measured.

### 6. Disagreements

- With the implemented gate, not with the roadmap sentence. `ROADMAP.md:21` asks for actual pins. The schema, G6, the banned-word list, and the hardcoded OK line make an actual pin a checker failure.
- With one 11-wide "compatible tuple". Released contract-call proofs and the Poseidon IVC verifier are different interfaces (`fable-kernel.md:71`). K and the Moriarty source checkout are not Midnight components. Forcing one boolean is why the flag is stuck false.
- With `check_u0_exit_gate.py:231` and the 24-row close rule. U4/U7 requirements (ZR02, ZR04, ZR05, ZR08, ZR09, ZR14, ZR16) are specified so they are not dropped (`MORIARTY-BACKEND-REQUIREMENTS.md:5`). They are not U0 demonstrations. Twelve judgment leaves are not circuit fields.
- With the k=17 SRS pin as the released degree. The prove that exists used k=14 and was not verified. The k=17 file is an unapproved IVC preparation that produced no proof.
- With using `accrue.prover` as "the" key. The receipt generated no transaction proof, and `initialize` is the circuit CLM-0958 actually proved.
- With reading "11 historical, 10 unresolved" as ten missing components. Absent count is zero. The ten entries are conflicts on pins that were still filled in.

### 7. Top three

1. **T2 + T3.** Change the ledger so a receipt can make the released tuple true, then prove and verify one circuit on compactc 0.31.1, proof-server 8.1.0, and ledger-v8 8.1.0, with transcript name, k, `proof-verifying`, and a one-byte rejection. If the image is still missing, the receipt says so. That is the U1 starting point.
2. **T4 + T5.** Keep IVC `695351f1` / k=17 as a blocked tuple. Dispose all 84 leaves. Demonstrate only the five identity fields in that proof's public inputs. Leave signed intent unenforced and name the U2 boundary.
3. **T6 + T7 + T8.** Put MNR01 and MNR05 on the U0/U1 milestone, record which premises block certification, and split "contract frozen" from "capability demonstrated" so 84 enforced leaves and 24 demonstrated backend rows are not the U0 gate.
