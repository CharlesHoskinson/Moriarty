# Actual pinned source excerpts

## Selected actual test

Source: `/home/charl/moriarty-c2-opus55-restored-20261017/compiler-one-check-candidate/full-originals/repository/crates/mori-backend/tests/stages.rs`; SHA256 `10b757f45bbba0a2e68d88fbc4c3ca776b01ee4fb954e5923a3514f4246dd491`; lines 373-399 of 399. Full original copied under source/.

```rust
373: #[test]
374: fn legacy_counter_abi_and_payment_effects_match_existing_frozen_snapshots() {
375:     let counter = mori_core::parse(include_str!("programs/counter.mori")).unwrap();
376:     let a = mori_backend::compile(&counter, &Target::ledger9("local", "counter-anchor")).unwrap();
377:     let actual = a
378:         .circuits
379:         .iter()
380:         .map(|c| {
381:             (
382:                 &c.action,
383:                 c.public_inputs
384:                     .iter()
385:                     .map(|p| (&p.role, &p.name))
386:                     .collect::<Vec<_>>(),
387:             )
388:         })
389:         .collect::<Vec<_>>();
390:     let snapshot = include_str!("snapshots/compile__authored_counter_abi.snap");
391:     let expected = snapshot.split_once("\n---\n").unwrap().1.trim_end();
392:     assert_eq!(format!("{actual:#?}"), expected);
393: 
394:     let payment = mori_core::parse(include_str!("programs/payment.mori")).unwrap();
395:     let a = mori_backend::compile(&payment, &Target::ledger9("local", "payment-anchor")).unwrap();
396:     let snapshot = include_str!("snapshots/compile__authored_payment_effects.snap");
397:     let expected = snapshot.split_once("\n---\n").unwrap().1.trim_end();
398:     assert_eq!(format!("{:#?}", a.circuits[0].effect_frame), expected);
399: }
```

## Actual Target::ledger9 definition and constants

Source: `/home/charl/moriarty-c2-opus55-restored-20261017/compiler-one-check-candidate/full-originals/repository/crates/mori-backend/src/lib.rs`; SHA256 `3ad0862058adbdc9b9a080f644fec7aa1721d70f115cdf8a08f5ef45cbdf2259`; lines 37-76 of 1255. Full original copied under source/.

```rust
37: 
38: pub const LEDGER_REVISION: &str = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14";
39: pub const ZKIR_VERSION: &str = "3.0.0-rc.2";
40: #[derive(Debug, Clone, Serialize, Deserialize)]
41: #[serde(deny_unknown_fields)]
42: pub struct Target {
43:     pub profile: String,
44:     pub network: String,
45:     pub anchor: String,
46:     pub ledger_revision: String,
47:     pub zkir_version: String,
48:     pub max_instructions: u32,
49:     pub max_artifact_bytes: u32,
50:     /// Absent (omitted) for unsigned targets; explicit null refuses.
51:     #[serde(
52:         default,
53:         skip_serializing_if = "Option::is_none",
54:         deserialize_with = "present_policy"
55:     )]
56:     pub signed_policy: Option<SignedPolicy>,
57: }
58: fn present_policy<'de, D: serde::Deserializer<'de>>(
59:     d: D,
60: ) -> Result<Option<SignedPolicy>, D::Error> {
61:     SignedPolicy::deserialize(d).map(Some)
62: }
63: impl Target {
64:     pub fn ledger9(network: impl Into<String>, anchor: impl Into<String>) -> Self {
65:         Self {
66:             profile: "ledger-9".into(),
67:             network: network.into(),
68:             anchor: anchor.into(),
69:             ledger_revision: LEDGER_REVISION.into(),
70:             zkir_version: ZKIR_VERSION.into(),
71:             max_instructions: 100_000,
72:             max_artifact_bytes: 16_777_216,
73:             signed_policy: None,
74:         }
75:     }
76:     pub fn from_json(text: &str) -> Result<Self, Error> {
```

## Actual unsigned compile and compile_mode bodies

Source: `/home/charl/moriarty-c2-opus55-restored-20261017/compiler-one-check-candidate/full-originals/repository/crates/mori-backend/src/lib.rs`; SHA256 `3ad0862058adbdc9b9a080f644fec7aa1721d70f115cdf8a08f5ef45cbdf2259`; lines 358-501 of 1255. Full original copied under source/.

```rust
358: fn digest<T: Serialize>(value: &T) -> Result<[u8; 32], Error> {
359:     let bytes =
360:         mori_core::intent::canonical(value).map_err(|e| error("serialization", e.to_string()))?;
361:     Ok(Sha256::digest(&bytes).into())
362: }
363: /// Lower a validated program. Financial relations constrain the declared touched
364: /// balance frame. An authenticated ledger adapter must preserve all accounts
365: /// outside it and bind the actual predecessor; that adapter is an open gate.
366: /// This unsigned entry point refuses a signed Target rather than dropping it.
367: pub fn compile(program: &Program, target: &Target) -> Result<Artifact, Error> {
368:     stage::require_core1(program)?;
369:     if target.signed_policy.is_some() {
370:         return Err(error(
371:             "target",
372:             "A signed Target requires the signed compiler; unsigned compilation refuses it.",
373:         ));
374:     }
375:     compile_mode(program, target)
376: }
377: /// Lower every action with the strict owner signature and checked caps in the
378: /// same circuit. Source-wide signed admission precedes any layout allocation.
379: pub fn compile_signed(program: &Program, target: &Target) -> Result<Artifact, Error> {
380:     stage::require_core1(program)?;
381:     if target.signed_policy.is_none() {
382:         return Err(error(
383:             "target",
384:             "Signed compilation requires a Target signed_policy.",
385:         ));
386:     }
387:     compile_mode(program, target)
388: }
389: pub(crate) fn compile_mode(program: &Program, target: &Target) -> Result<Artifact, Error> {
390:     stage::require_core1(program)?;
391:     target.validate()?;
392:     mori_core::validate(program).map_err(|e| error("program", e.to_string()))?;
393:     if program
394:         .assets
395:         .iter()
396:         .any(|asset| asset.identity.network != target.network)
397:     {
398:         return Err(error(
399:             "target",
400:             "Asset network differs from the target domain; cross-domain assets are unsupported.",
401:         ));
402:     }
403:     // Signed label/owner admission precedes the registry's own refusals.
404:     let signed = match &target.signed_policy {
405:         Some(policy) => Some(signed_relation::admit(program, policy)?),
406:         None => None,
407:     };
408:     let registry = liabilities::registry(program)?;
409:     let hash = digest(program)?;
410:     let program_hash = mori_core::intent::hex(&hash);
411:     let mut circuits = Vec::with_capacity(program.actions.len());
412:     let mut resources = Resources {
413:         instructions: 0,
414:         zkir_bytes: 0,
415:         artifact_bytes: 0,
416:         public_values: 0,
417:         max_replay_steps: 0,
418:         circuit_k: None,
419:         fraud_max_k: None,
420:         cost_measured: false,
421:         liability_registry_entries: registry.len() as u32,
422:         max_liability_registry_entries: MAX_LIABILITY_REGISTRY_ENTRIES as u32,
423:     };
424:     for action in &program.actions {
425:         let circuit = lower_action(program, action, target, &registry, signed.as_ref())?;
426:         resources.instructions = resources
427:             .instructions
428:             .checked_add(circuit.instruction_count)
429:             .ok_or_else(|| error("limit", "Instruction count overflow."))?;
430:         resources.zkir_bytes = resources
431:             .zkir_bytes
432:             .checked_add(
433:                 u32::try_from(circuit.zkir.len())
434:                     .map_err(|_| error("limit", "Artifact too large."))?,
435:             )
436:             .ok_or_else(|| error("limit", "Artifact size overflow."))?;
437:         resources.public_values = resources
438:             .public_values
439:             .checked_add(
440:                 (circuit.public_inputs.len() + circuit.authorization_bindings.len()) as u32,
441:             )
442:             .ok_or_else(|| error("limit", "Public ABI size overflow."))?;
443:         resources.max_replay_steps = resources.max_replay_steps.max(circuit.replay_steps);
444:         if resources.instructions > target.max_instructions
445:             || resources.zkir_bytes > target.max_artifact_bytes
446:         {
447:             return Err(error(
448:                 "limit",
449:                 "Program exceeds target instruction or ZKIR byte limit.",
450:             ));
451:         }
452:         circuits.push(circuit);
453:     }
454:     let mut artifact = Artifact {
455:         format: ARTIFACT_FORMAT.into(),
456:         abi: PUBLIC_ABI_VERSION.into(),
457:         relation: RELATION_VERSION.into(),
458:         authorization_mode: target.authorization_mode().into(),
459:         program_hash,
460:         semantics: program.semantics.clone(),
461:         target: target.clone(),
462:         circuits,
463:         resources,
464:         gates: [
465:             "native-proof-generation",
466:             "circuit-cost-and-k-measurement",
467:             "fraud-proof-cost-and-k-measurement",
468:             "authenticated-ledger-frame-and-phase-correspondence",
469:             "signed-intent-and-predecessor-history-verification",
470:             "live-ledger-deployment",
471:         ]
472:         .into_iter()
473:         .map(|p| Gate {
474:             predicate: p.into(),
475:             status: "unperformed".into(),
476:         })
477:         .collect(),
478:     };
479:     // The measured envelope includes its own decimal byte count. Reach the
480:     // fixed point (at most ten digit changes for u32), using the CLI's exact
481:     // pretty JSON serialization. ABI and frame metadata count toward the cap.
482:     for _ in 0..12 {
483:         let bytes = serde_json::to_vec_pretty(&artifact)
484:             .map_err(|e| error("serialization", e.to_string()))?;
485:         let count = u32::try_from(bytes.len())
486:             .map_err(|_| error("limit", "Artifact envelope exceeds u32 byte count."))?;
487:         if count > target.max_artifact_bytes {
488:             return Err(error(
489:                 "limit",
490:                 "Artifact envelope exceeds target byte limit.",
491:             ));
492:         }
493:         if count == artifact.resources.artifact_bytes {
494:             return Ok(artifact);
495:         }
496:         artifact.resources.artifact_bytes = count;
497:     }
498:     Err(error(
499:         "serialization",
500:         "Artifact size count did not stabilize.",
501:     ))
```

## Actual scalar immediate helper

Source: `/home/charl/moriarty-c2-opus55-restored-20261017/compiler-one-check-candidate/full-originals/repository/crates/mori-backend/src/lib.rs`; SHA256 `3ad0862058adbdc9b9a080f644fec7aa1721d70f115cdf8a08f5ef45cbdf2259`; lines 508-510 of 1255. Full original copied under source/.

```rust
508: fn immediate(n: u128) -> Operand {
509:     Operand::Immediate(Fr::from(n))
510: }
```

## Actual lower_action entry

Source: `/home/charl/moriarty-c2-opus55-restored-20261017/compiler-one-check-candidate/full-originals/repository/crates/mori-backend/src/lib.rs`; SHA256 `3ad0862058adbdc9b9a080f644fec7aa1721d70f115cdf8a08f5ef45cbdf2259`; lines 766-797 of 1255. Full original copied under source/.

```rust
766: fn lower_action(
767:     program: &Program,
768:     action: &Action,
769:     target: &Target,
770:     registry: &[LiabilityIdentity],
771:     signed: Option<&signed_relation::Layout>,
772: ) -> Result<CircuitArtifact, Error> {
773:     lower_action_segment(program, action, target, registry, signed, None)
774:         .map(|(circuit, _)| circuit)
775: }
776: 
777: // The full-range legacy path and every continuation segment use the same
778: // expression and financial statement lowerer. None preserves the /3 layout.
779: fn lower_action_segment(
780:     program: &Program,
781:     action: &Action,
782:     target: &Target,
783:     registry: &[LiabilityIdentity],
784:     signed: Option<&signed_relation::Layout>,
785:     stage: Option<(
786:         &mori_core::stage::StagedProgram,
787:         &stage::frame::SegmentLayout,
788:     )>,
789: ) -> Result<(CircuitArtifact, Option<stage::PendingPositions>), Error> {
790:     let mut builder = Builder::default();
791:     // Stage positions 0/1 with placeholders. Their constants are the hash of
792:     // the frozen financial layout, inserted only after that layout exists.
793:     let mut context_checks = Vec::with_capacity(2);
794:     for i in 0..2 {
795:         builder.public(
796:             "context",
797:             &format!("program_target_action.{i}"),
```

## Actual IR construction, serde, load and equality

Source: `/home/charl/moriarty-c2-opus55-restored-20261017/compiler-one-check-candidate/full-originals/repository/crates/mori-backend/src/lib.rs`; SHA256 `3ad0862058adbdc9b9a080f644fec7aa1721d70f115cdf8a08f5ef45cbdf2259`; lines 1215-1255 of 1255. Full original copied under source/.

```rust
1215:     };
1216:     if builder.instructions.len() > target.max_instructions as usize {
1217:         return Err(error("limit", "Action exceeds target instruction limit."));
1218:     }
1219:     let output_types = vec![IrType::Native; outputs.len()];
1220:     builder
1221:         .instructions
1222:         .push(Instruction::Output { vals: outputs });
1223:     let instruction_count = u32::try_from(builder.instructions.len())
1224:         .map_err(|_| error("limit", "Instruction count overflow."))?;
1225:     let ir = IrSource {
1226:         version: Default::default(),
1227:         inputs: Vec::new(),
1228:         outputs: output_types,
1229:         do_communications_commitment: false,
1230:         instructions: Arc::new(builder.instructions),
1231:     };
1232:     let mut disk = serde_json::to_value(&ir).map_err(|e| error("serialization", e.to_string()))?;
1233:     disk["version"] = serde_json::json!({"major":3,"minor":0});
1234:     let zkir = serde_json::to_string(&disk).map_err(|e| error("serialization", e.to_string()))?;
1235:     let parsed = IrSource::load(zkir.as_bytes()).map_err(|e| error("zkir", e.to_string()))?;
1236:     if parsed != ir {
1237:         return Err(error("zkir", "Pinned upstream round-trip changed the IR."));
1238:     }
1239:     Ok((
1240:         CircuitArtifact {
1241:             action: action.name.clone(),
1242:             zkir,
1243:             context_hash: proving::hex(&context),
1244:             public_inputs: builder.bindings,
1245:             authorization_bindings,
1246:             private_inputs,
1247:             instruction_count,
1248:             effect_frame,
1249:             balance_frame,
1250:             liability_frame,
1251:             replay_steps,
1252:         },
1253:         pending_positions,
1254:     ))
1255: }
```

## Actual SDK IrSource::load body (adjacent model/prove omitted)

Source: `/home/charl/moriarty-c2-opus55-restored-20261017/compiler-one-check-candidate/full-originals/pinned-sdk/zkir-v3/src/ir.rs`; SHA256 `b0dd55776696f829a9b228ef7558b2b0cc4c4e1ebf4b2e4b23533eb428c01ec8`; lines 954-989 of 1019. Full original copied under source/.

```rust
954:     /// Attempts to parse an arbitrary input as IR.
955:     pub fn load<R: Read>(reader: R) -> io::Result<Self> {
956:         let value: serde_json::Value = serde_json::from_reader(reader)?;
957:         match value {
958:             serde_json::Value::Object(mut obj) => {
959:                 let ver = serde_json::from_value(
960:                     obj.get("version")
961:                         .ok_or(io::Error::new(
962:                             io::ErrorKind::InvalidData,
963:                             "Expected a version entry",
964:                         ))?
965:                         .clone(),
966:                 )?;
967:                 match ver {
968:                     SerdeVersion {
969:                         major: 3,
970:                         minor: 0..=0,
971:                     } => {
972:                         obj.insert(
973:                             "version".into(),
974:                             serde_json::Value::Number(ver.minor.into()),
975:                         );
976:                         Ok(serde_json::from_value(serde_json::Value::Object(obj))?)
977:                     }
978:                     SerdeVersion { major, minor } => Err(io::Error::new(
979:                         io::ErrorKind::InvalidData,
980:                         format!("Unhandled version: {major}.{minor}"),
981:                     )),
982:                 }
983:             }
984:             _ => Err(io::Error::new(
985:                 io::ErrorKind::InvalidData,
986:                 "Expected a JSON object",
987:             )),
988:         }
989:     }
```

## Actual parser entry

Source: `/home/charl/moriarty-c2-opus55-restored-20261017/compiler-one-check-candidate/full-originals/repository/crates/mori-core/src/parser.rs`; SHA256 `587e8b024ae16d513c5e6d6ef4929e4140bbfc7d2a716b5d38781645cb7c7275`; lines 1-17 of 407. Full original copied under source/.

```rust
1: use crate::*;
2: use mori_lexer::{TokenIdx, TokenKind};
3: 
4: /// Parses the explicitly versioned core profile; beta grammar is untouched.
5: pub fn parse(source: &str) -> Result<Program, Error> {
6:     parse_profile(source, false)
7: }
8: 
9: /// Syntax-only parser; only stage::load admits the resulting source.
10: pub(crate) fn parse_v2(source: &str) -> Result<Program, Error> {
11:     parse_profile(source, true)
12: }
13: 
14: fn parse_profile(source: &str, staged: bool) -> Result<Program, Error> {
15:     let lexed = mori_lexer::lex(source).map_err(|e| Error::new("lex", e.to_string()))?;
16:     let tokens = (0..lexed.len())
17:         .map(|i| {
```

## Actual parser return path

Source: `/home/charl/moriarty-c2-opus55-restored-20261017/compiler-one-check-candidate/full-originals/repository/crates/mori-core/src/parser.rs`; SHA256 `587e8b024ae16d513c5e6d6ef4929e4140bbfc7d2a716b5d38781645cb7c7275`; lines 162-174 of 407. Full original copied under source/.

```rust
162:         if parser.program.states.len() + parser.program.assets.len() + parser.program.actions.len()
163:             > MAX_DECLARATIONS
164:         {
165:             return Err(parser.error("bound", "Program exceeds 256 declarations."));
166:         }
167:     }
168:     if parser.peek().kind != TokenKind::Eof {
169:         return Err(parser.error("syntax", "Unexpected content after agreement."));
170:     }
171:     if !staged {
172:         validate(&parser.program)?;
173:     }
174:     Ok(parser.program)
```
