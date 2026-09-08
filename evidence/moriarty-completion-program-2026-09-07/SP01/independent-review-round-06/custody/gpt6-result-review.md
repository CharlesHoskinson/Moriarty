# SP05 fixed kernel custody result review

**BLOCKED within compiled-runtime custody utility scope.** Candidate `086261fef7332206ae1bbc7128422a58a78c4e0a488abb3be0879c3349ee7a79`,116078bytes. All10owned files,19source inputs,4runtime pins and21fresh artifacts match before/after. No Docker, Preview, SP05, proof, native, full BNF, K or sprint acceptance.

17author tests pass. Independent replay confirms the full loan arithmetic/due history and15failed-call rollback controls. Three concrete defects remain.

## KC01-R1 — Constructor program digest is a caller-controlled label rather than the pinned kernel digest

Paths: `experiments/moriarty-midnight-financial/custody/loan.compact:41`; `experiments/moriarty-midnight-financial/custody/swap.compact:43`; `experiments/moriarty-midnight-financial/custody/generate.mjs:158`; `experiments/moriarty-midnight-financial/custody/generate.mjs:277`.

Packet requires the pinned program digest and exact program envelope, alongside trusted role/payout enrollment.

- Both constructors assign programDigest=disclose(expectedProgram). No equality to loan95b46e39… or swapb00a55b8… is enforced. Later PROGRAM_MISMATCH checks compare the call only with this arbitrary stored value. The actual pinned hashes occur in comments, not executable constraints.
- Independent compiled-runtime reproductions construct each contract with the public nonsecret program value0x7f repeated32times, correct synthetic role secrets and addresses, then call initialize with that same value. Both initialize successfully, although programMatchesPinned=false.
- Trusted enrollment of identities does not make an arbitrary digest identify the code actually executing. This is a fixed-kernel binding defect; no change to the accepted capability route is proposed.

**Correction:** Generate an immutable program constant from the pinned metadata, assign it directly or assert constructor expectedProgram equals it. Keep call-envelope equality checks. Add constructor wrong-digest controls for both contracts and consistency checks for duplicated bindings program values. Network-label authenticity remains a deferred ledger check.

## KC01-R2 — Stored swap reserves are not reconciled with entry balances and actual token effects

Paths: `experiments/moriarty-midnight-financial/custody/swap.compact:95`; `experiments/moriarty-midnight-financial/custody/swap.compact:124`; `experiments/moriarty-midnight-financial/custody/generate.mjs:326`; `experiments/moriarty-midnight-financial/custody/runtime.test.mjs:406`; `experiments/moriarty-midnight-financial/custody/runtime.test.mjs:435`.

Packet requires sufficient entry reserves, consistency with stored reserves, explicit intra-call deltas, and closure payouts of reserves. Numeric kernel counters are not physical token effects.

- swap checks entryB>=19743 and checkedAdd(entryA,10000)>=10000. The latter is an overflow check; neither ties entryA/entryB to kernelState.f0/f1.
- After valid initialization, a synthetic context with entryA=0 and entryB=19743 successfully swaps. Actual runtime effects request input10000A and output19743B; entry-plus-effects is10000A/0B, while returned/stored kernel reserves are1010000A/1980257B.
- With entryA=1000000 and entryB=1999999 the call also succeeds, but entry-plus-effects B=1980256 differs from stored1980257 by1. These reproduce the missing comparison independently of any future payer attribution.
- The author’s positive swap contexts supply only B=2000000 and leave A=0. They therefore already accept an A accounting discrepancy of1000000 while asserting the stored counter1010000. Later close tests manually supply1010000A, masking the discontinuity.
- After a normal fully funded swap, close accepts entry1010001A/1980258B, emits1010000A/1980257B, sets stored reserves0 and closes. Entry-plus-effects leaves1A/1B. No explicit surplus custody or recovery rule accounts for those units. This is a runtime counterexample, not evidence those contexts were reached on a real ledger.

**Correction:** Bind both entry reserve colors to the protected kernel reserve state and verify the exact entry-plus-input-minus-output relation to the returned state. Specify surplus handling explicitly (equality/rejection is sufficient for this fixed fixture), including close. Fund both colors in positive runtime contexts and carry synthetic balances by actual effect deltas rather than replacing them with expected counters. Preserve minOut1/19700 acceptance, isolated19744 rejection, fee30 retained inside gross10000A, and close-before-swap.

## KC01-R3 — Generation and build freshness guards do not cover all required source and mapping inputs

Paths: `experiments/moriarty-midnight-financial/custody/generate.mjs:69`; `experiments/moriarty-midnight-financial/custody/generate.mjs:80`; `experiments/moriarty-midnight-financial/custody/build.mjs:66`; `experiments/moriarty-midnight-financial/custody/build.mjs:78`.

Implementation prompt requires source/profile/program/metadata/kernel digest guards; the packet requires rejecting stale source/metadata/kernel mappings.

- generate hashes arithmetic, kernels, metadata and bound programs, but never reads loan.mori or swap.mori. It compares metadata.sourceHash to bindings.loanSource/swapSource without verifying those hashes against the source bytes.
- A read-only virtual-filesystem probe supplied invalid replacement source bytes for both .mori paths. Generator execution requested neither source file and produced wrappers byte-identical to the candidate. All output writes were captured in memory; no source files were changed.
- build directly checks arithmetic/kernel pins and wrapper substring presence (program/kernel hashes and transition0/transition1). It does not validate source/metadata/bound-program/profile bindings or require wrapper bytes to equal freshly generated output. Thus the supported build command bypasses much of the generator’s freshness gate.
- The current frozen inputs and generated wrappers all match their recorded hashes. This finding concerns the promised guard against stale future inputs, not a claim the present kernel bytes are stale.

**Correction:** Make a shared complete input/mapping validation routine mandatory for both commands, hash the actual pinned source bytes, and have build check exact generated-wrapper equality before invoking the compiler. Retain explicit output directories and no installs/network/proofs; use in-memory or isolated test doubles for changed-pin rejection controls.

## Preserved successes

- Both wrappers invoke transition0/transition1 from the byte-identical pinned kernels. Stage arithmetic and kernel files match source pins. Protected generated kernels remain pure; only wrapper entry circuits materialize custody effects.
- Role capability hashes use private random32byte in-memory secrets with role-domain/network/program separation. No secret is disclosed by source, written by review, or included in review output. Wrong-secret and other-role controls fail. Constructor payout addresses have no setters and outputs use their sealed mappings. Enrollment remains trusted and synthetic.
- Loan nominal arithmetic independently replays floor(5000000000*8*31/(100*365))=33972602, remainder27000. Accrue creates principal due500000000 and interest due33972602 with PR id4/IP id3, debtor2, creditor5 and denomination1. Principal4500000000 survives closure.
- The independent loan trace checked all9 kernel fields, complete two DueCreated and three settlement effect records, lastAccrue persistence, lastSettle, and lifetime/revision2/0→1/1→0/2. Final kernel balances19466027398/533972602 are labeled synthetic accounting, not payer evidence.
- Actual loan runtime effects mint20000000000 USD to the sealed borrower at init, emit no movement at accrue, and request input/output533972602 USD plus one lender recipient claim at settlement. Token color derives from the fixed USD domain and executing contract address.
- Actual swap init effects mint total1100000A and2000000B; claims allocate100000A to trader and1000000A/2000000B to contract. Swap effects request10000A input and19743B output to trader, with fee30A retained within gross input. On a consistent entry context, reserves1010000A/1980257B and work7/revision1 match.
- Close emits both pinned reserve colors to sealed provider, stores zero kernel reserves and closed1, and retains remaining6/revision2 after swap. Protected close-before-swap remains allowed and yields remaining7/revision1; late swap rejects. Surplus consistency is separately blocked by KC01-R2.
- Author tests preserve valid minOut19700 and1 and isolate19744 at minimum output not met. Explicit zero-uncertainty synthetic time exercises now, now+299, now+300, before-now and horizon boundaries. Independent Uint64 overflow control rejects before addition.
- 17author compiled-runtime tests passed independently on the fresh root-verify artifacts. Fifteen additional direct failed-call checks preserved complete decoded accepted ledger state and all9effect families; no financial output/state mutation was observed in those rejected calls.
- Exact embedded UTF8 stdout/stderr hashes and byte counts matched for10recorded streams. Old missing-artifact RED and root10pass/5fail diagnostic remain distinct and match their raw captures; missing historical argv/exit for the old RED is honestly recorded. GREEN raw captures also match. Root fresh-build/generation/test streams match freeze hashes.
- The completion changed the QueryContext.block test setup, not generate.mjs/loan.compact/swap.compact; these three files equal their preserved interrupted-pass counterparts. Fresh root build artifacts are separately pinned and were used for this review.

## Reproduction and evidence

The JSON report retains the exact read-only Node probe commands and outputs. All commands completed with exit0. The17test run yielded once and then completed. No source files, tests or compiled artifacts were changed.

- Exit0: ls/cat state binding, implementation prompt, approved packet/review, reports/README. 
- Exit0: Python initial SHA256 checks for candidate/source/runtime/artifact files; inspect freeze/test-evidence structured JSON. 
- Exit0: cat/sed/rg cold review of both wrappers, generator/build, bindings, tests, pinned kernels, stdlib and runtime types/compiled JS. 
- Exit0: MORIARTY_CUSTODY_ARTIFACTS=state/build/root-verify node --test --test-reporter=tap --test-timeout=90000 runtime.test.mjs. 17pass0fail; initial command yielded session50733; write_stdin poll completed
- Exit0: Independent Node program/reserve probe; exact full command and output retained in reproductions. 
- Exit0: Python exact UTF8 evidence/byte-count/raw/root-stream comparisons and safe receipt metadata inspection. 
- Exit0: Independent Node full loan/due trace and15direct complete state/effect rollback controls; exact command retained. 
- Exit0: Read-only virtual-source-drift generator probe with in-memory writes; exact command retained. 
- Exit0: Python candidate manifest recomputation and root generated-wrapper equality comparison. 
- Exit0: Python final all10owned/19input/4runtime/21artifact hashes and unchanged interrupted source checks; green raw comparison. 
- Exit0: Write only state/gpt6-result-review.json and state/gpt6-result-review.md. 

## Limits

- No actual ledger funding, consumed UTXO owner, payer, change, fee, finality, wallet, proof or network execution was performed. receiveUnshielded has no payer parameter; that known boundary is not a standalone blocker.
- Synthetic inconsistent reserve contexts demonstrate missing runtime constraints; they do not establish an exploit reachable on an actual deployed ledger.
- No compiler or writing generation command was run by the reviewer. The independent runtime probes use the root fresh-artifact hashes; reproducibility also has a read-only virtual generator run.
- Author rollback snapshots omit last result records and other ledger fields/effects; the independent15controls improve coverage but the retained suite should preserve equivalent checks.
- The malformed-source probe intercepts filesystem access in memory and does not execute a stale-source compiler build. Build freshness concerns are grounded in its actual source control flow.
- Full source corpus interpretation, native proof security and the accepted capability-route decision were not reopened. No private provider reasoning or raw worker.stdout was read.

Correct the three findings in a new frozen candidate, preserve all historical failures and packet constraints, regenerate/compile under separate existing admission, and obtain a fresh independent result review.

