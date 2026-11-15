# Slice C domain model: wrap, settlement and operations (for DDD.md)

Sender: Fable-C. Fable-A merges this into `architecture/DDD.md`. Terms follow TERMINOLOGY.md; a new term is marked (new). Citations use the short forms of the slice C specs (`branch@commit:path:line`).

## 1. Bounded contexts

| Context | Purpose | Kind | Capability specs | Code today |
|---|---|---|---|---|
| Wrap | Turn one `L_Fr` proof into the one BLS12-381 proof that the ledger reads; hold the wrap key and its limits (k <= 21, VK <= 50,000 bytes, 1,026 public inputs) | core | `sherlock-e2e-wrap` (WRP), reuses LW, GPW, RKY-003, RKY-004 | `tools/sherlock-wrap/` (`t10q-ntfix@95990a6d3`) |
| Ledger Verification | The stock ledger 9.1.0.1 verifier as an external system, and the local check that speaks its language | generic (external), with an anti-corruption layer | `sherlock-e2e-ledger-verifier` (LVR), reuses MID | `tools/sherlock-wrap/ledger-check/`, `validate_call` |
| Settlement | SherlockLedger contracts: run claims, collection roots, read entries; the argument map from a statement to `acceptRun` | core | `sherlock-e2e-ledger-contract` (SLC), `sherlock-e2e-batched-settlement` (BAT), reuses LDG, STL, ANC-012 to ANC-014 | `tools/sherlock-sdk/sherlock-ledger.compact` (`t20-batched@5dc1be2ee`), `tools/t23-suite/accept_args.py` (staged on `t23-args`) |
| Governance | Eras, the era table, the Fenrir operator roster and its verbs; sealing | supporting | `sherlock-e2e-governance` (EGV), reuses GOV, ERT, ELD, CRR | sketches only (`t22-governance@fe7207819`) |
| Network Operations | Preview deployment, the wallet, fees, the proof server, receipts, recovery | supporting | `sherlock-e2e-preview-deployment` (PVW), `sherlock-e2e-wallet` (WAL), `sherlock-e2e-sdk-cli` (SCL), `sherlock-e2e-observability` (OBS), `sherlock-e2e-recovery` (FRC) | `tools/sherlock-sdk/t10a-ledger-run.mjs`, `t17.sh`, `tools/t23-suite/t23.py` |
| Placement (settlement edge) | Which machine may hold what: case A only, the muninn check of box wraps, the wallet on muninn | supporting | `sherlock-e2e-helper-machines` (HLP), reuses ZK-001, ZK-009, ZK-018, PRD-002, PRD-003 | loop rules, BOXES.md, no code guard |

One context is not one deployable. Network Operations is four tools and one runbook today (decision D-085). The names muninn and OP-PV-WALLET belong to the factory rehearsal; the product rule is "the one machine that holds the wallet" and "the wallet pinned in the run configuration" (SCL-010).

## 2. Context map

| Upstream | Downstream | Relation | Exchanged meaning | Translation owner |
|---|---|---|---|---|
| Proving (slice B, LTR) | Wrap | customer-supplier; published language `LfrProofReady {l_fr_proof, l_fr_vk_prehash, public words, claim_pub}` | one `L_Fr` proof that is ready to wrap | Wrap (`wrapin.rs`) |
| Language (slice A, ERS) | Settlement | published language `StatementPublics {c_p, input_pub, outcome_pub, claim_pub}`; it is the result of the argument builder, not its input | the public face of one run statement | Settlement (the argument builder, SLC-006 to SLC-009). The builder input is the private `sherlock-accept-bundle/1` (preimages of program, input and outcome); it stays on the proving box (HLP-003, HLP-004). `acceptRun` takes 3 of the 4 words; the runner file adds `anchor` |
| Wrap | Ledger Verification | conformist: the wrap conforms to `MidnightVK::read` and `VerifierKey::verify`; nothing in the ledger changes | `WrapOutput` (3 files) | Wrap |
| Ledger Verification (Midnight ledger 9.1.0.1, external) | Settlement, Network Operations | anti-corruption layer: `sherlock-wrap-ledger-check`, `validate_call`, node error 1010/115 mapped to `refused-by-node` | accept or refuse | Network Operations |
| Wrap | Settlement | shared kernel, kept small: the call vector layout (1,026 lanes; lane 10 anchor, lanes 20 and 34 `claim_pub`, lane 39 `outcome_pub`; anchor slot 15) | lane map and anchor slot | both; a change is a rekey and a redeploy |
| Governance | Settlement and consumer contracts | customer-supplier, Governance upstream: the era table says which era accepts. Primary path: each wrap-keyed ledger entry calls `active(era_id)` at inclusion (ELD-003), once the wrap binds the era words (EGV-005). Fallback, only on a recorded failure of premise A2 or A3: the consumer checks at first use (EGV-006) | `era_id`, `active(era_id)`, `end_eff` (BAT-010, BAT-011) | Governance |
| Settlement | Consumer contracts (external) | open host service with published language: read entries `rootAccepted`, `runAccepted`; claims tree depth 20 | membership of a claim | Settlement |
| Settlement | Language (slice A, EUF) and Proving (slice B) | published language `SettlementReceipt`, `WrapVerdict` | what settled, and whether a box wrap is kept | Network Operations |
| Placement | Proving (slice B: BXF, PDM, ZKE) | partnership: B owns boxes, daemon and STARK-side zero knowledge; C owns the settlement edge (HLP) | `BoxResult {path, sha256, box}` in, `WrapVerdict` out | both |
| Fenrir (external) | Governance | conformist for the roster (rotated Fenrir operator keys); no Fenrir contract is called | operator keys | Governance |

Request direction and influence differ in one place: the ledger entry calls the era table (request from Settlement to Governance), and Governance is upstream because it decides which era accepts.

## 3. Aggregates and invariants

| Aggregate (root) | Context | Invariants (each bound by EARS) | Atomic scope |
|---|---|---|---|
| WrapKey (the one wrap VK of an era) | Wrap | k <= 21 (WRP-002); rows <= 1,782,573 (WRP-003, RKY-003); VK <= 50,000 bytes (WRP-004, RKY-004); 1,026 public inputs (LW-003); the LW checks pass on the final keys (WRP-012); anchor slot 15 is a key constant for the read key and the chunk write key (WRP-007, ANC-013, ANC-016); one key for every program and mode (WRP-010); GPU key = CPU key (WRP-011) | one keygen |
| WrapProof | Wrap | one proof for each ledger call (WRP-001); fresh blinders (WRP-006); verified by the stock verifier with tamper controls before use (LVR-001, LVR-004, HLP-001) | one prove |
| ProgramLedger (one SherlockLedger for each program; root = contract address) | Settlement | anchor = `c_p` of its program, sealed in slot 15 (SLC-002, SLC-005); a `claim_pub` settles at most once (LDG-002, STL-004); no live settle of an unbound statement (SLC-012); write entries hold the wrap key, read entries native keys (SLC-003, SLC-004); 4 entries only (SLC-001); sealed for life, with authority and entry keys read back (EGV-001, EGV-002) | one ledger transaction |
| ClaimEpoch (one collection of an operator; root = `claims_root`) | Settlement | at most one ledger write for each epoch (LDG-006, BAT-001, BAT-002); a claim is used by membership with no ledger write (LDG-004, BAT-007); the leaf list is available off chain (BAT-008); refused while the `L` collection form has no key (BAT-009); closed before an announced era end (BAT-010) | one epoch close |
| ChunkPlan | Settlement | `K_MAX` = 2 at every TTL (BAT-005); TTL 3,600 s (BAT-006) | plan time |
| EraTable (root = table address) | Governance | append-only; one era for each key set; one era accepts at a time; an end is set once (ERT-003 to ERT-007, EGV-004); verbs need the quorum of the roster in force (GOV-001); a key change is a new era (EGV-007) | one governance transaction |
| SettlementRun (one call from proof to receipt; root = `claim_pub` + state folder) | Network Operations | pinned chain and wallet (PVW-001, PVW-002, WAL-003); one sender at a time (WAL-004); a deploy is sent once (FRC-002); an unknown inclusion state is resolved before a new submission (FRC-013); saved artifacts are checked before reuse (FRC-014); a refusal needs a node error (PVW-007); secrets never in a message or receipt (WAL-002, WAL-005) | one runner step (each step resumable, FRC-001) |
| BoxResult acceptance | Placement | a box result is bound to its job by sha256 and call vector (HLP-008); a box wrap is kept only after the muninn check (HLP-001, HLP-002); private parts stay on the box (HLP-003, HLP-004); no host outside the user's roster (HLP-005, ZK-018) | one result |

Open consistency questions (carried as gaps): how ERT-003 "one ledger for each era" reads under one ledger for each program (T22 O8); whether a retry of one logical run keeps its salt, which CRR-005 needs across eras.

## 4. Domain events

| Event | Producer | Consumers | Carries |
|---|---|---|---|
| `LfrProofReady` | Proving (B) | Wrap | `L_Fr` proof, key pre-hash, public words, `claim_pub` |
| `WrapProved` | Wrap | Ledger Verification, Placement | `WrapOutput` = `wrap.vk`, `wrap.proof`, `wrap.public.json` (WRP-009) |
| `WrapVerdictIssued` | Ledger Verification on muninn | Proving (B), Settlement | `WrapVerdict {ok, key bytes, proof bytes, controls refused}` (LVR-004, HLP-001) |
| `AcceptArgsBuilt` | Settlement (builder, on the box) | Network Operations | the 4 public words (HLP-003) |
| `LedgerDeployed` | Network Operations | Settlement, Governance | address, block, installed keys, maintenance authority readback (SLC-003, EGV-002) |
| `RunSettled` | ProgramLedger | Language (A), Observability | `SettlementReceipt {contract, tx hash, block, status, bytes, fee, runs[claim_pub] = outcome_pub}` (OBS-001) |
| `CallRefused` | node, through the runner | Observability, Recovery | tx sha256, node JSON-RPC error (OBS-002, PVW-007) |
| `EpochClosed` | epoch operator | Settlement, submitters | `claims_root`, slot and path for each job, leaf list location (BAT-001, BAT-007, BAT-008) |
| `CollectionAccepted` | ProgramLedger | consumers | `claims_root` in `claim_roots` (LDG-003) |
| `EraAnnounced`, `EraActivated`, `SoundnessEnded`, `RosterRotated` | EraTable | Settlement, consumers, provers | era record (GOV-002, GOV-003, GOV-005, GOV-006) |
| `RunResumed`, `IncidentLogged` | Recovery | lead loop, wiki | step, state folder, cause (FRC-001, FRC-008, OBS-011) |

An event here is a named fact for the context map. It does not select an event-driven implementation: today each is a file or a runner state field.

## 5. Ubiquitous language (slice C terms)

| Term | Meaning in slice C | Note |
|---|---|---|
| wrap | the one BLS12-381 PLONK proof of a ledger call, and the circuit that makes it | TERMINOLOGY.md |
| call vector | the 1,026 public inputs of the wrap: `[binding_input, comm]`, anchor prologue, effect words | TERMINOLOGY.md |
| anchor | the sealed ledger cell that holds `c_p` of the ledger's program (slot 15 after T25) | "anchor" in Governance never means this |
| settle | include an `acceptRun` or `acceptCollection` call on chain | not "verify": the node verifies, the contract settles |
| refusal | a node JSON-RPC error on a submitted call; a timeout or a transport error is not a refusal | PVW-007, PVW-008 |
| epoch (claim epoch) | the batching window of one operator: 3,600 s or 2^20 claims | differs from a registry epoch (LDG-008) and from an era |
| era | one sealed set of ledgers with its own universal key set, listed in the era table | proposed name of T22, not yet in TERMINOLOGY.md (new) |
| sealed | deployed with an empty maintenance committee and threshold 1; no key change is possible | differs from Compact `sealed ledger` (a field only the constructor writes); both are used, always with the noun |
| receipt | the public record of a settled or refused call | never holds a private word |
| box | a machine the user owns that proves (muninn, huginn, odin) | case A |
| helper | a host the user does not own | case B, refused |
| KEEP | the lead's verdict that a box result enters the record after the muninn check | loop word |
