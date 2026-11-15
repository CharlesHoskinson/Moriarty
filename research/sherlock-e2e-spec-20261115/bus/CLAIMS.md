sherlock-e2e-wrap | WRP | Fable-C | the one BLS12-381 wrap at the settlement edge: k <= 21, VK <= 50,000 bytes, key export, muninn check (refs LW, GPW, T25)
sherlock-e2e-ledger-verifier | LVR | Fable-C | unchanged ledger 9.1.0.1 verifier, version pin, refusals (refs MID)
sherlock-e2e-ledger-contract | SLC | Fable-C | SherlockLedger, acceptRun, statement to acceptRun argument map (T23 Part A, t23-args, slot 15) (refs LDG, STL)
sherlock-e2e-batched-settlement | BAT | Fable-C | acceptCollection per epoch, claims_root membership, K_MAX = 2, tx counts (T20)
sherlock-e2e-governance | EGV | Fable-C | Fenrir operator keys, HFC eras, era registry, sealed contracts (T22; refs GOV, ERT, ELD, CRR)
sherlock-e2e-preview-deployment | PVW | Fable-C | Preview deploy, receipts, runbook, Mainnet gate (T17, T23 settle)
sherlock-e2e-wallet | WAL | Fable-C | wallet handling: never logged, flock, stays on muninn
sherlock-e2e-sdk-cli | SCL | Fable-C | SDK and CLI surface for prove, wrap, submit, status (settlement side)
sherlock-e2e-observability | OBS | Fable-C | run records, receipts, logs, privacy scans, timing records
sherlock-e2e-recovery | FRC | Fable-C | failure and recovery: refusals, resubmit, crash recovery, durable runs
sherlock-e2e-helper-machines | HLP | Fable-C | helper machines: case A only, no plain witness to helpers, muninn check of box wraps; case B deferred to Veritee
sherlock-e2e-compact-compiler | ECC | Fable-A | Compact source, stock compactc ZKIR, Mycroft compile to R_SL, refusals, suite coverage
sherlock-e2e-rsl-statement | ERS | Fable-A | R_SL statement, rsl-call/1 call vector, witness tape, W words, comm; published type RslStatement for slices B and C
sherlock-e2e-rsl-kits | EKT | Fable-A | kit interfaces and native equality at app level: SHA-256, keccak256, jubjub, secp256k1, Poseidon, ALU (AIR keys and leaf bucket stay with slice B)
sherlock-e2e-universal-interpreter | EUI | Fable-A | universal interpreter: execution semantics, reference run, GP-1 at the execution layer
sherlock-e2e-compact-suite | ECS | Fable-A | Compact test suite, T23 Part B: per-call compile, execute, compare with native; suite table
sherlock-e2e-user-flow | EUF | Fable-A | end-to-end flow: developer writes Compact, user proves on own boxes, settles on Preview; cross-slice hand-offs
sherlock-e2e-leaf-proving | LFP | Fable-B | app proof of an R_SL run under the universal leaf: 6-or-9 AIR bucket, fixed-size shards, prog_d in the leaf statement, small-field STARK (refs RSL, CPR-002, modular-leaf chips; statement and kits stay with slice A)
sherlock-e2e-l-transcript | LTR | Fable-B | L, L_Fr, the L_Fr tape (typed MEM_BUS), transcript and public words; publishes LfrProofReady to the wrap (refs LW, FLG)
sherlock-e2e-universal-keys | UKS | Fable-B | one universal key set, key_root, keys_d, the single rekey T25 (refs RKY-001, RKY-002, RKY-005, CPR-003; wrap rows and VK bytes RKY-003, RKY-004 stay with WRP)
sherlock-e2e-node-aggregation | NAG | Fable-B | N1, N2, N3, every AIR required, FP-2 node statement, MERGE-WORK 5,400 columns, PCD join (refs NODE, SHP)
sherlock-e2e-gp-hooks | GPH | Fable-B | GP-1 to GP-4 at the proving layer: universal VM leaf, GP-2 composition, GP-3 foreign proofs (X8, X14), GP-4 PCD (X7) (refs GP, CPR-005, LPC-001..003)
sherlock-e2e-call-binding | CLB | Fable-B | FP-3 L binding of call lanes, anchor slot 15 on the iface and L side, x_j lock form, chunk count k (refs ANC-013..019; decode.rs and SherlockLedger stay with WRP and SLC; the Mycroft refusal stays with slice A)
sherlock-e2e-elastic-proving | ELA | Fable-B | T16: schedules from the profile, scale up, scale down, across hardware, one claim_pub under one key set, two-host join (refs SPL, LPC-006, LPC-007)
sherlock-e2e-setup-mode | STM | Fable-B | setup mode and sherlock-profile/1, replaces F-3; brings ST, PF, SC of SETUP-SPEC.md into OpenSpec by reference
sherlock-e2e-box-fleet | BXF | Fable-B | proving boxes muninn, huginn, odin: heavy-slot, gpu.lock, box bus, file moves with sha256, per-box baselines (the muninn ledger check of box wraps stays with HLP)
sherlock-e2e-gpu-proving | GPV | Fable-B | GPU STARK paths (CUDA backend for leaf, nodes, L_Fr), CPU fallback, byte-equal publics (the GPU wrap GPW stays with WRP)
sherlock-e2e-prover-daemon | PDM | Fable-B | resident prover daemon and warm path: keys and contexts loaded, socket serve, slot reuse, T14 and T14b timing (refs PRD)
sherlock-e2e-zero-knowledge | ZKE | Fable-B | zero knowledge end to end at the proving layer: r_P, salt, test-seed guard, EQ-ZK, GPU half, no plain witness off the owned boxes (refs ZK-001..019)
sherlock-e2e-stark-security | SSG | Fable-B | 100-bit security, D3 on every key, the T12 gate G1 to G4 and FP-8 on the T25 keys, Quint lane and counterexample coverage T13 (refs SQG, DG, FPR)
