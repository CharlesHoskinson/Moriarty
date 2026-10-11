# Native owner intent Task 2 report

Scope: the integrated signed compiler relation, shared host reconstruction and
the production signed-intent/proof CLI. Task base
`bec2f88d815a555c9845efa2a7fec878eca9c86c`. Controller preservation commit of the
partial backend: `7760b7c20a028a13502064ec82812072512a139d` (tree
`797aa8a6…65fd`). Author: Claude Opus 5.5, model `claude-opus-5-5`, one
continuous implementation session `717f9469-a57e-4c58-9f6d-c35f7835dcf8`,
resumed after a provider session limit. No other author, no delegation and no
spawned agents. Independent check pending: a fresh GPT-6.1 Sol HIGH full Task 2
review of the frozen final bytes. Task 2 is not native qualification; Tasks 3–4
remain.

## Evidence scope

Everything below is ordinary host/reference compilation, simulation,
reconstruction and the off-circuit ZKIR interpreter (`ProofPreimage::check`),
plus CLI runs against empty or nonexistent parameter directories. No SRS was
decoded and no key generation, native proving, native verification, MockProver,
ignored native test or resource experiment ran. The production `prove` path
computes the existing `ir.k()` admission value before parameters; honest signed
inputs reached the missing-parameters diagnostic under `--max-k 19`, which is an
incidental pass of the existing admission check, not a k measurement. No circuit
k, row count or max-shape figure is reported; those are Task 3.

A checker refusal is not a native proof failure. Host SHA256/JSON
reconstruction is not computed in circuit. Host attachment acceptance shows only
that the supplied FENS signature satisfies the equation for the reconstructed
message and expected key; it does not authenticate the account label, key
custody, nonce consumption, predecessor history, ledger time or deployment.

## Production behavior

### Compiler (`crates/mori-backend/src/lib.rs`, `signed_relation.rs`)

- All artifact/ABI/relation surfaces are revision 3 and carry an explicit
  `authorization_mode` (`unsigned` or `moriarty-jubjub-owner-intent/1`).
  Revision 1/2 artifacts and bundles refuse with a recompile diagnostic.
- `Target.signed_policy: Option<SignedPolicy>` is omitted when absent; explicit
  null and unknown policy fields refuse. With a policy, network/anchor/policy
  labels follow the strict label rule. `Target::from_json` now uses Core's
  bounded duplicate-key parser.
- `compile` (unsigned) refuses a signed Target; `compile_signed` refuses an
  unsigned one. Both use the same `compile_mode` staging.
- Source-wide signed admission (`signed_relation::admit`) runs before the duty
  registry and before any layout allocation, over every action, selected or not:
  at most 16 assets, distinct full AssetIds, every transfer/fee payer and every
  liability debtor equal to the policy owner, and the signed label rule on
  program, state, input, local, action, asset name and identity parts, account,
  debtor, creditor, duty (liability and discharge) and event names. A control
  label refuses as `mori::backend::signed`; 17 assets or duties refuse as
  `mori::backend::limit`. Unsigned compilation keeps Core's own rules.
- Staging: financial positions 0/1 are emitted as placeholder context checks;
  after the whole financial layout and its effect/balance/liability frames
  exist, the 18-element context array is hashed (constant-free
  `{position,role,name,ty,asset}` projection), and the two LE128 limbs are
  patched into positions 0/1 and their binding constants. Unsigned v3 uses the
  same staging with mode `unsigned`.
- Signed circuits then append the N+11+4A authorization inputs in the specified
  order (UInt ranged, key/message Native), each unconditionally Impact-published;
  typed private `public_key: Point<Jubjub>`, `R: Point<Jubjub>`,
  `response: Native` (raw `[PK.x,PK.y,R.x,R.y,s]`); PK coordinates equal the
  public key; PK and R nonidentity (x ≠ 0 for typed subgroup points); MOI1 over
  the actual financial public wires with source-fixed tags, counts, asset limbs;
  computed message equals the public message; FENS challenge
  `[FENS,R.x,R.y,PK.x,PK.y,msg]` reduced to a Jubjub scalar; canonical response
  by native→scalar→native roundtrip; unconditional `Assert(s·B == R + e·PK)`;
  `not_before <= at <= not_after` with `at` public but outside MOI1; checked
  gross/fee/net/closing-duty sums over actual ordered effect and closing-duty
  wires, compared with the same cap wires that enter MOI1.
- Artifact decoding (`TryFrom<ArtifactFields>`) checks the whole artifact: mode
  equals the Target's mode, program/context digests are canonical, context
  positions 0/1 are canonical UInt constants, and each circuit has exactly the
  unsigned (no descriptors) or signed (exact 11+4A names/types/positions,
  strictly ascending cap assets, exact three private descriptors) shape.
  `PublicBinding.constant/asset`, every nullable `EffectDescriptor` member and the
  nullable `Resources` k fields must be present (explicit null); `PublicBinding`'s
  financial type is unchanged.

### Shared reconstruction and host acceptance (`signed_relation.rs`)

- `reconstruct` performs one parse, compile, duty-slot check, simulation and
  financial reconstruction for every consumer, then (signed) compares the
  separately supplied `ExpectedAuthority` with the trusted Target, its hash and
  policy, checks the interval at the caller's `at`, rebuilds the exact payload
  from raw source bytes, Program, scenario, Target, action and authority,
  applies `UnsignedPayload::validate_assets` to the actual source asset set,
  refuses a payload whose eventual wrapper (canonical 192-hex attachment, +223
  bytes) would exceed 65,536 bytes, computes the payload/predecessor/context
  limbs, the 75-element-style MOI1 message and the authorization public values,
  and finally checks the host caps (checked UInt128 sums; refunds never reduce
  gross; owner fees reduce net once).
- `accept` (verify-intent, prove, verify): strict wrapper validation, exact
  canonical payload equality with the reconstruction, strict host FENS on the
  attached bytes. It builds no private witness.
- `witness` (prove and verify-intent only): raw `[PK.x,PK.y,R.x,R.y,s]` from the
  accepted attachment, checked against the compiled relation by the ordinary
  ZKIR checker.
- Public API: `prepare_intent` (returns `PreparedIntent`, validated and
  preflighted through Core's bounded exporter), `verify_intent` (returns the
  complete `Execution`), and `accept_intent` (same acceptance, plus the
  reconstructed payload hash, context hash, message and `at`; no attachment).

### Proof surfaces (`proving.rs`)

- v3 Claim: the previous fields plus `authorization_mode`, `context_hash`,
  `financial_bindings`, `authorization_bindings`, `public_values` (32LE
  NativeHex per position) and `authorization` (explicit null, or exactly
  `{payload_hash,message_hex,public_key_hex,at}`). No attachment, R or s.
  `ProofBundle` is that Claim plus `proof_hex`; decoding validates digests,
  NativeHex, key encoding, mode/authorization consistency, the exact descriptor
  shape and statement length, all as `mori::backend::claim`.
- `prove`/`verify` (unsigned) refuse a signed Target. `prove_signed` and
  `verify_signed` take the brief's signatures. Preparation modes: `Unsigned`,
  `Verify` (public acceptance only, empty private vector) and `Prove` (adds the
  witness). Verification compares the canonical reconstructed Claim with the
  candidate minus `proof_hex`, then derives its own circuit and VK.
- Key location `moriarty-local-native-transition/3/<mode>`. Signed receipts use
  qualification `local-native-transition-proof-with-existential-owner-signature`
  and list key/account association, nonce consumption and predecessor history as
  open gates.

### CLI (`crates/mori/src/commands/`)

- `mori compile`: the trusted Target selects `compile_signed` or `compile`; a
  refused signed admission creates no output directory.
- `mori intent … --authority A --profile moriarty-jubjub-owner-intent/1
  --target T --at N [--review|--json]`: signed preparation, printing the exact
  `PreparedIntent`. `--review` prints escaped identities, interval and the note
  that `at` is not signed, per-asset caps, context and payload hashes, the MOI1
  message, the existential native scope and the unverified key/nonce/history
  gates. It contains no signer, key custody or approval control. The legacy
  `--scheme/--public-key/--framing` group remains required together without
  `--profile`, and is refused with it; `--target/--at` without `--profile`
  refuse.
- `mori verify-intent … --signature W --authority A --at N --profile …
  --target T`: strict signed acceptance with the ordinary relation check;
  prints `accepted` with profile, qualification
  `local-signed-owner-intent-host-and-relation-check`, payload/context hashes,
  message, `at` and the complete execution, or `rejected` with the backend error
  and exit 1. Profile without Target, or Target without profile, refuse.
- `mori prove|verify … --target T [--signature W --authority A --at N]`: a
  signed Target requires all three and an unsigned Target refuses any of them,
  before the bundle or parameters are read. Envelope and authority files are
  capped at 1 MiB before parsing.
- `docs/NATIVE-CORE.md` names the v3 revisions and modes.

## Sol backend preflight findings and repairs

Report `sol-owner-task2-backend-preflight.md` (SHA256 `c7642e60…bca59f5`).

- **I1 (labels):** fixed as above; admission moved before the registry so a
  control-bearing discharge-only name refuses as a label, while a legal Unicode
  discharge-only name still refuses as an unknown duty. All twelve independent
  source cases (copied byte-for-byte to
  `crates/mori-backend/tests/programs/signed-admission/`, hashes checked against
  the manifest in the test) give the manifest outcome.
- **I2 (schema/mode):** required explicit nulls, whole-artifact mode/Target/shape
  validation and canonical Claim encodings, with deletion, mode and encoding
  tests. The two independent descriptor deletions refuse both standalone and
  embedded in a generated golden artifact and a generated golden signed Claim.
- **M1 (bounds):** `prepare_intent` preflights the complete report through the
  bounded exporter (an escaped-Unicode 120-balance review now refuses as
  `limit`), and the wrapper budget is exact: a generated 65,313-byte payload
  prepares and its placeholder wrapper is 65,536 bytes; 65,314 refuses before any
  attachment is requested.
- **M2 (witness):** verification builds no private witness; tested that `Verify`
  preparation has an empty private vector and the same public values/binding as
  `Prove`, that the alternative attachment reaches parameters with the same
  Claim, and that a flipped attachment refuses as `signature`.
- **M3 (identified refusals and payment):** hostile response/point controls now
  assert the refusal kind (`Failed direct assertion`, `Equality constraint
  failed`, `Failed to decode … JubjubPoint`, Bool/bit range) and, where
  meaningful, ablate exactly the identified constraint in a copy of the IR to
  show the same hostile witness then passes: wrong tag/order → FENS assert;
  `s+r` → roundtrip equality; identity R with a forged equation-satisfying
  response → R nonidentity; identity PK with `s=k` → PK nonidentity. The
  payment test asserts `paid=70` and the exact ordered effects
  Transfer70 / Fee2 / Liability payment 50→30 / State 0→70 / Emit 70.

## Checked requirements (Task 2 brief)

- Golden source/Target: context `4d84b3a3…24d6`, 28 financial descriptors with
  roles/names/types/assets/constants and values, 15 authorization positions,
  three typed private descriptors (five raw inputs), the 75-field MOI1 with
  direct financial operands, unconditional Impact for all 43 positions, exact
  payload bytes/hash, message and projections all match the unchanged golden
  vector.
- Golden attachment and the independent alternative (nonce 23) both accept;
  a one-bit change refuses; `at` 10/15/20 accept and 9/21 refuse.
- Second payer, second fee payer and second debtor in an unexecuted action, 17
  assets and 17 duties refuse; 16 assets with 16 duties and a zero-asset pure
  state action compile (11+4·16 and 11 authorization positions).
- Independent payment case (bytes equal to the runtime reference): exact caps
  72/2/68/35 prepare and a freshly QA-signed statement verifies; each of
  71/1/69/34 refuses in host preparation and verification naming the cap; a
  freshly re-signed tighter statement refuses in the ordinary checker at the cap
  assertion while its attachment is valid.
- Authority/Target/payload substitutions (network, anchor, address, owner,
  beneficiary, nonce, predecessor, interval, target hash, key, caps, asset
  scale, raw source bytes) refuse as `authorization`; unsigned Targets on signed
  entry points and signed Targets on unsigned entry points refuse as `target`.
- Self-beneficiary transfer gives gross 10/net 0; a nonzero owner fee
  underflows net; checked sums overflow refuses.
- CLI: compile/intent/verify-intent/prove/verify signed paths, partial and
  mixed groups, escaped Unicode review, control-label Target refusal, exclusive
  output preservation, FIFO parameters refusing without waiting (k 1..19
  pipes), stale v2 bundle refusal and no publication on refusal.

## Commands and results

All Rust commands used `RUSTUP_TOOLCHAIN=1.99.0`,
`CARGO_TARGET_DIR=/home/charl/Moriarty/.worktrees/readiness-20261010/target`,
`--locked --offline`. Captures are under
`/home/charl/Moriarty/.moriarty-dev/runtime/native-liabilities-20261011/native-owner-task2/`.

Final current bytes:
`cargo test -p mori-core -p mori-backend -p mori --locked --offline`
(`check-22-final-suites.log`, byte manifest `check-22-final-bytes.manifest`):
exit 0, **129 passed, 0 failed, 3 ignored** (the pre-existing genuine native
tests; not run), no compiler warnings. Per binary: CLI unit 4, `intent_review`
1, `native_proofs` 8 (+2 ignored), `signed_intents` 9, `verified_review` 1;
backend lib 13, `authorization` 18, `compile` 13, `liabilities` 8, `proving` 10
(+1 ignored), `signed_relation` 14; Core lib 1, `authority` 5, `intents` 4,
`json_export` 2, `language` 18.
`cargo fmt -p mori-backend -p mori --check` exit 0; `git diff --check` clean;
`cargo clippy -p mori-backend -p mori --all-targets` exit 0 with seven remaining
warnings, all on lines that predate Task 2 (`liabilities.rs:90`,
`proving.rs` proof-hex/effect-tuple helpers, test helpers in `liabilities.rs` and
Task 1 `authorization.rs`).

Earlier session captures, kept unchanged:

| Capture | Result |
| --- | --- |
| `baseline.log` | Base bytes: all ordinary suites passed |
| `red-backend-signed.log` | RED: missing `compile_signed`, `signed_relation`, `prove_signed`, policy and v3 fields |
| `green-attempt1-backend-signed.log` | 9/10; alias fixture was parser-refused (test fixed) |
| `green-attempt2-admission.log` | 1/1 |
| `green-attempt3-checker-units.log` | Misnamed: records compile errors, not success |
| `green-attempt4-lib.log` | 10/11; in-interval `at` expectation was wrong (test fixed) |
| `green-attempt5-checker-units.log` | 5/5 |
| `red-existing-v2-pins*.log` | Expected v2 pins failing, then bundle-shape compile failure |
| `green-attempt6-existing.log` | liabilities 8/8, proving 10/10 (1 ignored) |
| `stalled-build-original.log`, `self-matching-build-wait-diagnosis.json` | The earlier `pgrep -f` wait matched its own shell; no cargo was running |

This resumed session:

| Capture | Result |
| --- | --- |
| `check-07-red-preflight-repairs-integration.log` | RED 10/14: control labels admitted; deletion and mode swap decoded; exporter test fixture bug |
| `check-08…` | 13/14: unsigned cross-check expectation wrong for discharge-only files (test fixed) |
| `check-09-label-cases.log` | 1/1 |
| `check-10-lib-units.log` | Compile error (accept return type) |
| `check-11-lib-units.log` | 12/13: Bool range refusal not in the allowed kinds (test fixed) |
| `check-12`, `check-13` | 1/1 each |
| `check-14-red-cli.log` | RED 1/9: signed CLI arguments absent (group test vacuous at RED) |
| `check-15-cli.log` | 2/9: test guard dropped its temp directory (test fixed) |
| `check-16-cli.log` | 9/9 |
| `check-17-fmt.log`, `check-20…` | fmt check exit 0 |
| `check-18-final-suites.log` | 129 passed, 0 failed, 3 ignored, on the bytes before the clippy fix |
| `check-19-clippy.log`, `check-21-clippy.log` | exit 0; one Task 2 warning fixed between them |

## Remaining limits and concerns

- No native evidence of any kind. Circuit k, rows, max-shape cost and the
  native positive/negative controls are Tasks 3–4 with their own resource votes.
  The CLI default `--max-k 9` predates this task; signed tests pass `--max-k 19`.
- `mori verify` for a signed bundle is exercised only to stale/group refusals at
  the CLI and to the missing-parameters diagnostic in backend tests; no genuine
  signed bundle exists until native proving is authorized.
- Checker evidence is the interpreter's off-circuit semantics, which may differ
  from in-circuit constraints (for example typed point assignment). Ablation
  identifies the interpreter's refusal site only.
- Nonidentity uses x ≠ 0 on typed subgroup points; it relies on the typed
  assignment's subgroup constraint in circuit, which only native work can show.
- The wrapper budget check also runs on verification paths, where
  `SignedIntent::validate` already bounds the actual wrapper.
- Signed admission validates more identifier kinds (program, state, input,
  local) than the finding named; Core already restricts these by grammar, so
  this adds no observed refusal for parsed source.
- Input files named by the CLI, including envelopes, are opened with ordinary
  blocking reads; only the SRS path has the nonblocking FIFO refusal.
- Commit attribution: under the user's rule no Claude attribution is added to
  the Task 2 commit. Separately, twelve historical commits already reachable from
  this branch and from `main` (for example `63dbfb67`, `2eda1f14`) carry
  `Co-Authored-By: Claude Opus 5.5` trailers. The pre-push guard would therefore
  refuse any push of this branch. They were not changed because the brief
  forbids rewriting commits; the user must decide.
- Open after this slice: authenticated ledger frame/phases, network/anchor/address
  qualification, ledger clock, key ownership, nonce consumption, revocation,
  predecessor history, creditor consent, cumulative limits, recursion, fraud
  cost and the wider U0–U7/M1–M17 gates.
