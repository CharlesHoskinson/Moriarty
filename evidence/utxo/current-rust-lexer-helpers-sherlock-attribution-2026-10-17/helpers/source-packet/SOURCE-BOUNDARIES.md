# Lexer and helper source boundaries

Source inspection only. RuntimeReady=false; native authority0; full20-family, financial, recursion/PCD and real Preprod goal remains open. These analysis bytes are unreviewed/unpublished and need parent fresh Gemini4 Argon HIGH attribution review. No source was compiled,executed,imported or tested.

Main lexer pin: 8174e52975d8db9bf59636fea0fa338c0f47c855. Backend helper/lexer pin: 9c861d945fc8b706d5e2ee132aab3230490ff72e. Seven supplied lexer pairs were mechanically compared byte-for-byte; this does not establish semantic/runtime equivalence. Main/backend literal citations and hashes are paired in the JSON table. Caller-context read ranges are explicit in SOURCE-CHECKS.json.

The20 exact family IDs/names and existing UNLOCATED mappings remain by immutable reference to `core-frontend-source-observations/ADMISSION-SOURCE-TABLE.json#/familyInventory`, SHA-256 `9c9b32c9ba160eabd2d4b6acd8ac4a63f04c11efa8852c14b6d4d0adff3d813e`. No family mapping was added, and no inventory case path was followed.

## L01 SOURCE_DECLARATION

Constants declare source limit 65,536 bytes, token limit 8,192, identifier limit 64 bytes and decoded-string limit 1,024 UTF-8 bytes. lex checks source.len() before Lexer::run(false).

`MAIN-lexer-src-lib.rs:28–35`, SHA-256 `5cd5bd604e41fe57e754dd323a4fd86393d34c231a4dee16efe5fd13da5e28d7`:

```text
/// Largest accepted source file, in bytes.
pub const MAX_SOURCE_BYTES: usize = 65_536;
/// Most tokens in one file, including the end-of-file token.
pub const MAX_TOKENS: usize = 8_192;
/// Longest identifier or keyword, in bytes.
pub const MAX_IDENT_BYTES: usize = 64;
/// Longest decoded string, in UTF-8 bytes.
pub const MAX_STRING_BYTES: usize = 1_024;
```

`MAIN-lexer-src-lib.rs:47–53`, SHA-256 `5cd5bd604e41fe57e754dd323a4fd86393d34c231a4dee16efe5fd13da5e28d7`:

```text
/// Splits `source` into tokens and comments, or reports the first error.
pub fn lex(source: &str) -> Result<Tokens, MoriDiagnostic> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(diagnostics::source_too_large(source.len()));
    }
    Lexer::new(source).run(false)
}
```

## L09 CALL_EDGE

Core parse_profile and language call mori_lexer::lex, then recover token spans and decode String tokens. The inspected language body does not call first_token_text.

`CORE-parser.rs:14–31`, SHA-256 `83c8d4fabbd5837517c43353528545a783784e202aa913f7924a5d72c50abed2`:

```text
fn parse_profile(source: &str, staged: bool) -> Result<Program, Error> {
    let lexed = mori_lexer::lex(source).map_err(|e| Error::new("lex", e.to_string()))?;
    let tokens = (0..lexed.len())
        .map(|i| {
            let idx = TokenIdx::new(i);
            let span = lexed.span(idx, source);
            let raw = &source[span.start as usize..span.end as usize];
            let text = if lexed.kind(idx) == TokenKind::String {
                mori_lexer::decode_string(raw)
            } else {
                raw.to_owned()
            };
            Tok {
                text,
                kind: lexed.kind(idx),
                start: span.start,
            }
        })
```

`CORE-parser.rs:352–365`, SHA-256 `83c8d4fabbd5837517c43353528545a783784e202aa913f7924a5d72c50abed2`:

```text
pub fn language(source: &str) -> Result<&'static str, Error> {
    let lexed = mori_lexer::lex(source).map_err(|e| Error::new("lex", e.to_string()))?;
    let mut tokens = Vec::new();
    for i in 0..lexed.len() {
        let idx = TokenIdx::new(i);
        let span = lexed.span(idx, source);
        let raw = &source[span.start as usize..span.end as usize];
        let kind = lexed.kind(idx);
        tokens.push(Tok {
            text: if kind == TokenKind::String { mori_lexer::decode_string(raw) } else { raw.to_owned() },
            kind,
            start: span.start,
        });
    }
```

## L10 INFERENCE

Lexical decimal spelling and Core UInt parsing are separate source gates: lexer validates optional fraction syntax, whereas Core literal strips underscores and calls parse::<u128>(). This statement does not report executed acceptance or rejection.

`MAIN-lexer-src-number.rs:20–41`, SHA-256 `5826e09117b22505a74ab3144f4862445b513039bac275c88ace4d85331fad9a`:

```text
/// Checks the spelling of a number scanned by `scan::number_end`.
pub fn validate(raw: &str) -> Result<(), NumberError> {
    let mut dots = raw.match_indices('.').map(|(at, _)| at);
    let dot = dots.next();
    if let Some(at) = dots.next() {
        return Err(NumberError::ExtraDot { at });
    }

    let integer = &raw[..dot.unwrap_or(raw.len())];
    if integer.len() > 1 && integer.starts_with('0') {
        return Err(NumberError::LeadingZero);
    }
    check_underscores(integer, 0)?;

    if let Some(dot) = dot {
        let fraction = &raw[dot + 1..];
        if fraction.is_empty() {
            return Err(NumberError::MissingFraction);
        }
        check_underscores(fraction, dot + 1)?;
    }
    Ok(())
```

`CORE-parser.rs:224–242`, SHA-256 `83c8d4fabbd5837517c43353528545a783784e202aa913f7924a5d72c50abed2`:

```text
    fn literal(&mut self) -> Result<Value, Error> {
        if self.eat("true") {
            return Ok(Value::Bool(true));
        }
        if self.eat("false") {
            return Ok(Value::Bool(false));
        }
        if self.peek().kind != TokenKind::Number {
            return Err(self.error("syntax", "Expected an unsigned integer or boolean literal."));
        }
        let raw = self.peek().text.replace('_', "");
        let n = raw.parse::<u128>().map_err(|_| {
            self.error(
                "number",
                "UInt literal must fit u128; amounts use integral atomic units.",
            )
        })?;
        self.pos += 1;
        Ok(Value::UInt(UInt(n)))
```

## L14 SOURCE_DECLARATION

Public decode_string calls decode_with and expect("lexed strings are valid"). Its body does not return Result. first_token_text instead selects a bounded character-boundary prefix and calls run(true); run(true) stops after its first noncomment token and appends EOF.

`MAIN-lexer-src-lib.rs:37–63`, SHA-256 `5cd5bd604e41fe57e754dd323a4fd86393d34c231a4dee16efe5fd13da5e28d7`:

```text
/// The text of a string token, with escapes decoded.
///
/// `raw` must be the full text of a [`TokenKind::String`] token, quotes
/// included, as produced by [`lex`].
pub fn decode_string(raw: &str) -> String {
    let mut text = String::with_capacity(raw.len());
    string::decode_with(raw, |c| text.push(c)).expect("lexed strings are valid");
    text
}

/// Splits `source` into tokens and comments, or reports the first error.
pub fn lex(source: &str) -> Result<Tokens, MoriDiagnostic> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(diagnostics::source_too_large(source.len()));
    }
    Lexer::new(source).run(false)
}

/// Identify a leading profile token without requiring the remaining source to
/// lex successfully. Inspect only a bounded prefix; full parsing retains all
/// source and token limits.
pub fn first_token_text(source: &str) -> Result<&str, MoriDiagnostic> {
    let end = source.floor_char_boundary(source.len().min(MAX_SOURCE_BYTES));
    let prefix = &source[..end];
    let tokens = Lexer::new(prefix).run(true)?;
    Ok(tokens.span(TokenIdx::new(0), prefix).source_text(prefix))
}
```

`MAIN-lexer-src-lib.rs:85–108`, SHA-256 `5cd5bd604e41fe57e754dd323a4fd86393d34c231a4dee16efe5fd13da5e28d7`:

```text
    fn run(mut self, first_only: bool) -> Result<Tokens, MoriDiagnostic> {
        while let Some(&b) = self.bytes.get(self.pos) {
            let start = self.pos;
            match b {
                b' ' | b'\t' | b'\r' | b'\n' => self.pos += 1,
                b'/' if self.peek(1) == Some(b'/') => self.line_comment(start),
                b'/' if self.peek(1) == Some(b'*') => self.block_comment(start)?,
                b'"' => self.string(start)?,
                b'0'..=b'9' => self.number(start)?,
                b'A'..=b'Z' | b'a'..=b'z' => self.word(start)?,
                _ => match TokenKind::punctuation(b) {
                    Some(kind) => self.push(kind, start, start + 1)?,
                    None => return Err(self.unexpected_character(start)),
                },
            }
            if first_only && !self.tokens.is_empty() {
                break;
            }
        }
        // The end-of-file token is always pushed: the bound reserves room for it.
        self.tokens.push(Token {
            kind: TokenKind::Eof,
            start: offset(self.pos),
        });
```

## ID01 SOURCE_DECLARATION

CompilerBuild stores two Digest32 values. embedded delegates to embedded_build. checked rejects either zero digest, then compares self with recomputed embedded(), then returns self. No build routine was invoked by this inspection.

`HELPER-identity.rs:126–155`, SHA-256 `4dd88ceb42b98ad32a4c8502de9be8469508e710293810c6ff16eb59b423a661`:

```text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CompilerBuild {
    /// SHA-256 of the canonical `mori-r1cs-build-manifest/2` document of this crate's
    /// Cargo.toml and src/ (no annotation fields; review R-04).
    pub compiler_source_manifest_sha256: Digest32,
    /// Core closure digest under the unchanged closure rule (`CORE-CLOSURE-PINS.json`
    /// `closure_sha256`): the exact mori-core/lexer/diagnostics/span bytes embedded.
    pub core_closure_sha256: Digest32,
}

impl CompilerBuild {
    /// The embedded byte-derived identity (the only one `compile` accepts).
    pub fn embedded() -> Result<Self, CompileError> {
        crate::build_identity::embedded_build()
    }

    /// Refuses a zero digest, then any value other than the embedded identity: an
    /// arbitrary caller-supplied build digest is never taken as provenance.
    pub fn checked(self) -> Result<Self, CompileError> {
        if self.compiler_source_manifest_sha256.is_zero() || self.core_closure_sha256.is_zero() {
            return Err(CompileError::Identity(
                "compiler build digests must be nonzero; use CompilerBuild::embedded()".into(),
            ));
        }
        if self != Self::embedded()? {
            return Err(CompileError::Identity(
                "caller-supplied compiler build identity differs from the embedded byte-derived identity".into(),
            ));
        }
        Ok(self)
```

## ID02 SOURCE_DECLARATION

own! and core_file! expand include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),...)). Paths are literal source declarations; no include target was followed. This explicitly records the compile-time env! macro instead of treating the prose comment about environment variables as executable truth.

`HELPER-build_identity.rs:44–54`, SHA-256 `813b2b183309c6f8db8afa5e4a77e3fdc39c92fb8a5ea9eff6f76829157c7f3f`:

```text
macro_rules! own {
    ($p:literal) => {
        Embedded { path: $p, bytes: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $p)) }
    };
}

macro_rules! core_file {
    ($p:literal) => {
        Embedded { path: $p, bytes: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../moriarty/", $p)) }
    };
}
```

## S03 SOURCE_DECLARATION

build appends header fields in order relation version,program hash,profile digest,action index,shape digest,compiler build,logical work,optional public cap; then inputs; then pre followed by post states,cells,duty present/amount; then effects; then optional gross/fees per debit cell.

`HELPER-statement.rs:230–292`, SHA-256 `956b85251f1f9a8ed03617a58fff6e9ee6c388f3147dcb14b227f8555cd36df5`:

```text
impl StatementLayout {
    pub(crate) fn build(program: &ProgramIdentity, plan: &ActionPlan, build: CompilerBuild) -> Result<Self, CompileError> {
        use HeaderField as H;
        use SlotEncoding as E;
        let mut b = Builder { slots: Vec::new(), offset: 0 };
        b.push(SlotKind::Header(H::RelationVersion), E::Small, "header.relation_version".into());
        b.push(SlotKind::Header(H::ProgramHash), E::Digest, "header.program_hash".into());
        b.push(SlotKind::Header(H::ProfileDigest), E::Digest, "header.profile_digest".into());
        b.push(SlotKind::Header(H::ActionIndex), E::Small, "header.action_index".into());
        b.push(SlotKind::Header(H::ShapeDigest), E::Digest, "header.shape_digest".into());
        b.push(SlotKind::Header(H::CompilerBuild), E::Digest, "header.compiler_build".into());
        b.push(SlotKind::Header(H::LogicalWork), E::UInt, "header.logical_work".into());
        if plan.cap == CapBinding::PublicCap {
            b.push(SlotKind::Header(H::Cap), E::UInt, "header.cap".into());
        }
        for (i, s) in plan.inputs.iter().enumerate() {
            b.push(SlotKind::Input { index: i }, E::of(s.ty), format!("input.{}", s.name));
        }
        for (phase, post) in [("pre", false), ("post", true)] {
            for (i, s) in plan.states.iter().enumerate() {
                let kind = if post { SlotKind::PostState { index: i } } else { SlotKind::PreState { index: i } };
                b.push(kind, E::of(s.ty), format!("{phase}.state.{}", s.name));
            }
            for (i, c) in plan.cells.iter().enumerate() {
                let kind = if post { SlotKind::PostCell { index: i } } else { SlotKind::PreCell { index: i } };
                b.push(kind, E::UInt, format!("{phase}.cell.{}.{}", c.asset_name, c.account));
            }
            for (i, d) in plan.duties.iter().enumerate() {
                let (p, a) = if post {
                    (SlotKind::PostDutyPresent { index: i }, SlotKind::PostDutyAmount { index: i })
                } else {
                    (SlotKind::PreDutyPresent { index: i }, SlotKind::PreDutyAmount { index: i })
                };
                b.push(p, E::Bool, format!("{phase}.duty.{}.present", d.name));
                b.push(a, E::UInt, format!("{phase}.duty.{}.amount", d.name));
            }
        }
        for (i, e) in plan.effects.iter().enumerate() {
            let label = effect_label(i, e);
            match e {
                EffectShape::State { ty, .. } => {
                    b.push(SlotKind::Effect { index: i, role: EffectRole::StateBefore }, E::of(*ty), format!("{label}.before"));
                    b.push(SlotKind::Effect { index: i, role: EffectRole::StateAfter }, E::of(*ty), format!("{label}.after"));
                }
                EffectShape::Transfer { .. } | EffectShape::Fee { .. } => {
                    b.push(SlotKind::Effect { index: i, role: EffectRole::Amount }, E::UInt, format!("{label}.amount"));
                }
                EffectShape::Liability { .. } => {
                    b.push(SlotKind::Effect { index: i, role: EffectRole::DutyBefore }, E::UInt, format!("{label}.before"));
                    b.push(SlotKind::Effect { index: i, role: EffectRole::DutyAfter }, E::UInt, format!("{label}.after"));
                }
                EffectShape::Emit { ty, .. } => {
                    b.push(SlotKind::Effect { index: i, role: EffectRole::EmitValue }, E::of(*ty), format!("{label}.value"));
                }
            }
        }
        if plan.accounting == AccountingBinding::GrossDebitAndFees {
            for &cell in &plan.debit_cells {
                let c = &plan.cells[cell];
                b.push(SlotKind::GrossDebit { cell }, E::Wide, format!("accounting.{}.{}.gross_debit", c.asset_name, c.account));
                b.push(SlotKind::FeesPaid { cell }, E::Wide, format!("accounting.{}.{}.fees_paid", c.asset_name, c.account));
            }
        }
```

## A01 SOURCE_DECLARATION

StatementAdapter declares DirectFieldV0,ClaimantBoundAbiV1Pending,SettlementHashV2. admits returns Ok for DirectFieldV0,Unadopted for pending v1,or calls initial_ledger_target_admits for SettlementHashV2.

`HELPER-adapter.rs:31–46`, SHA-256 `24dcee35856ee8bf6ef3dcde8be39b56692af34e99fc5975bd1230dc07a91b1d`:

```text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum StatementAdapter {
    /// TEST-ONLY direct field statement. Not a claim ABI (see module docs).
    DirectFieldV0,
    /// Placeholder for the frozen, unreviewed claimant-bound ABI v1. Always refuses.
    ClaimantBoundAbiV1Pending,
    /// P3S1 PROPOSED (unreviewed, unadopted): MRG16-PVC ABI v2. Two public inputs, the
    /// big-endian halves of an in-circuit BLAKE2b-256 of the ABI v2 statement (constant
    /// header, free context bytes, every body slot bound to its wire). See [`crate::abi2`]
    /// and `candidate/docs/ABI-V2.md`. Compile options are fixed by
    /// [`crate::abi2::options_ok`]; whole-program Emit is refused by
    /// [`initial_ledger_target_admits`], which this variant calls (review F-10 wiring for
    /// this adapter only). Proving needs a [`crate::abi2::ContextV2`]
    /// ([`crate::CompiledRelation::assignment_v2`], [`crate::CompiledRelation::circuit_v2`]).
    SettlementHashV2,
}
```

`HELPER-adapter.rs:154–163`, SHA-256 `24dcee35856ee8bf6ef3dcde8be39b56692af34e99fc5975bd1230dc07a91b1d`:

```text
impl StatementAdapter {
    /// Adapter-level admission of a program. The direct test adapter admits every
    /// program the compiler lowers (Emit included); the pending ABI admits none.
    pub fn admits(&self, program: &Program) -> Result<(), AdapterError> {
        match self {
            Self::DirectFieldV0 => Ok(()),
            Self::ClaimantBoundAbiV1Pending => Err(AdapterError::Unadopted(ABI_V1_PENDING)),
            Self::SettlementHashV2 => initial_ledger_target_admits(program),
        }
    }
```

## A02 SOURCE_DECLARATION

initial_ledger_target_admits scans every action and every statement; the first Emit returns TargetRefusal containing action/ordinal/name; if no such branch is taken the function returns Ok. No call was executed.

`HELPER-adapter.rs:228–240`, SHA-256 `24dcee35856ee8bf6ef3dcde8be39b56692af34e99fc5975bd1230dc07a91b1d`:

```text
pub fn initial_ledger_target_admits(program: &Program) -> Result<(), AdapterError> {
    for action in &program.actions {
        for (ordinal, statement) in action.statements.iter().enumerate() {
            if let Statement::Emit { name, .. } = statement {
                return Err(AdapterError::TargetRefusal(format!(
                    "initial ledger target refuses whole-program Emit: action {:?} statement {ordinal} emits {name:?}",
                    action.name
                )));
            }
        }
    }
    Ok(())
}
```

## V201 SOURCE_DECLARATION

options_ok sequentially requires cap NotBound,accounting GrossDebitAndFees,frame TouchedOnly. The source error strings about target behavior are not independently verified here.

`HELPER-abi2.rs:83–96`, SHA-256 `51fa106305481cb6c48c9eeda0d2b24ae5e5f7b9816daa48b2b474b56e98debb`:

```text
pub fn options_ok(options: &CompileOptions) -> Result<(), CompileError> {
    if options.cap != CapBinding::NotBound {
        return Err(CompileError::Option("SettlementHashV2 requires CapBinding::NotBound".into()));
    }
    if options.accounting != AccountingBinding::GrossDebitAndFees {
        return Err(CompileError::Option("SettlementHashV2 requires AccountingBinding::GrossDebitAndFees".into()));
    }
    if options.frame != FrameCells::TouchedOnly {
        return Err(CompileError::Option(
            "SettlementHashV2 requires FrameCells::TouchedOnly; the target enforces the frame over the complete datum".into(),
        ));
    }
    Ok(())
}
```

## V204 CALL_EDGE

constants calls shape_bytes,then blake2b256(shape_bytes),then body_len converted to u32. body_len skips headers and uses UInt16/Bool1/Wide18 bytes, rejecting other nonheader encodings.

`HELPER-abi2.rs:208–228`, SHA-256 `51fa106305481cb6c48c9eeda0d2b24ae5e5f7b9816daa48b2b474b56e98debb`:

```text
/// Body length in bytes of a layout (every non-header slot).
pub fn body_len(layout: &StatementLayout) -> Result<usize, CompileError> {
    let mut n = 0usize;
    for slot in &layout.slots {
        n += match (slot.kind.is_header(), slot.encoding) {
            (true, _) => 0,
            (false, SlotEncoding::UInt) => 16,
            (false, SlotEncoding::Bool) => 1,
            (false, SlotEncoding::Wide) => 18,
            (false, other) => return Err(CompileError::Bound(format!("unexpected body slot encoding {other:?}"))),
        };
    }
    Ok(n)
}

pub(crate) fn constants(program: &Program, plan: &ActionPlan, layout: &StatementLayout, predicate: [u8; 32]) -> Result<AbiV2Constants, CompileError> {
    let shape_bytes = shape_bytes(program, plan)?;
    let shape_digest = blake2b::blake2b256(&shape_bytes);
    let body_len = u32::try_from(body_len(layout)?).map_err(|_| CompileError::Bound("ABI v2 body exceeds u32".into()))?;
    Ok(AbiV2Constants { predicate_digest: predicate, shape_bytes, shape_digest, body_len })
}
```

## ST01 SOURCE_DECLARATION

StagedAdmission stores StagedProgram and source digest. StagedLoweringPlan stores profile/format/hash fields,StoredCapParameter,actions and remaining strings. StoredCapParameter has only ExplicitPublicSlot; that enum label does not construct executable public slots.

`HELPER-staged.rs:33–49`, SHA-256 `f8ba260a0672a891f98b3af5823ded5ae23e514760ae58816eb29a75cc8d1fc6`:

```text
#[derive(Debug, Clone)]
pub struct StagedAdmission {
    staged: StagedProgram,
    source_sha256: Digest32,
}

/// D-13 (canonical reconstruction cap) is split between two recorded votes:
/// `Some(plan_total[a])` (pr54-stage-owner-tests-and-design DESIGN-VOTES D-13) and
/// `None` (pr54-owner-suite independent Opus review §9). This compiler selects
/// neither: a future staged relation carries the stored cap as a public, caller-chosen
/// slot and never fabricates a reconstruction default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum StoredCapParameter {
    /// The continuation's stored cap (`Option<UInt>`) is a public statement slot:
    /// a presence Bool and a UInt, absent exactly `(false, 0)`.
    ExplicitPublicSlot,
}
```

`HELPER-staged.rs:97–111`, SHA-256 `f8ba260a0672a891f98b3af5823ded5ae23e514760ae58816eb29a75cc8d1fc6`:

```text
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StagedLoweringPlan {
    pub language: String,
    pub semantics: String,
    pub logical_cost_profile: String,
    pub stage_plan_format: String,
    pub continuation_format: String,
    pub program_hash: Digest32,
    pub plan_hash: Digest32,
    pub source_sha256: Digest32,
    pub stored_cap: StoredCapParameter,
    pub actions: Vec<StagedActionPlan>,
    /// Required before any staged relation exists; none is waived.
    pub remaining: Vec<String>,
}
```

## ST04 INFERENCE

Observed caller returns Err(StagedNotLowered(plan)) for Staged2. The inspected lowering_plan body creates metadata,not CompiledRelation or constraint-system calls. This is a bounded source distinction,not a global impossibility claim or executable staged lowering result.

`S16-circuit.rs:266–271`, SHA-256 `3856551a3466e6790c0142df9b2025487d2330f9da20caa3796f1ff27bd5649c`:

```text
pub fn compile_source<F: PrimeField>(source: &str, options: CompileOptions, adapter: StatementAdapter) -> Result<CompiledRelation, CompileError> {
    match frontend::admit_source(source)? {
        Admitted::Core1(admitted) => compile::<F>(admitted, options, adapter),
        Admitted::Staged2(staged) => Err(CompileError::StagedNotLowered(Box::new(staged.lowering_plan()?))),
    }
}
```

`HELPER-staged.rs:156–244`, SHA-256 `f8ba260a0672a891f98b3af5823ded5ae23e514760ae58816eb29a75cc8d1fc6`:

```text
    pub fn lowering_plan(&self) -> Result<StagedLoweringPlan, CompileError> {
        let program = self.staged.program();
        let stage_plan = self.staged.plan();
        let mut actions = Vec::with_capacity(stage_plan.actions.len());
        for (index, (action, ap)) in program.actions.iter().zip(&stage_plan.actions).enumerate() {
            if ap.action != action.name || ap.segment_cost.len() != ap.sites.len() + 1 || ap.committed_before.len() != ap.sites.len() + 1 {
                return Err(CompileError::Bound(format!("stage plan for {:?} is not the admitted plan", action.name)));
            }
            let action_index = u16::try_from(index).map_err(|_| CompileError::Bound("action index exceeds u16".into()))?;
            let inputs: Vec<CaptureSlot> = action
                .inputs
                .iter()
                .enumerate()
                .map(|(i, input)| CaptureSlot { name: input.name.clone(), ty: input.ty, origin: mori_core::stage::Origin::Input, ordinal: i as u16 })
                .collect();
            let mut segments = Vec::with_capacity(ap.sites.len() + 1);
            for segment in 0..=ap.sites.len() {
                let (entry, start) = if segment == 0 {
                    (SegmentEntry::Start { inputs: inputs.clone() }, 0u16)
                } else {
                    let site = &ap.sites[segment - 1];
                    (
                        SegmentEntry::Resume {
                            site: site.site,
                            label: site.label.clone(),
                            guard_ordinal: site.ordinal,
                            captures: site.captures.clone(),
                            binders: site.binders.clone(),
                        },
                        site.ordinal,
                    )
                };
                let (exit, end) = match ap.sites.get(segment) {
                    Some(next) => (
                        SegmentExit::Await { site: next.site, label: next.label.clone(), ordinal: next.ordinal, captures: next.captures.clone() },
                        next.ordinal,
                    ),
                    None => (SegmentExit::Complete, ap.statement_count),
                };
                let statements = action
                    .statements
                    .get(usize::from(start)..usize::from(end))
                    .ok_or_else(|| CompileError::Bound("segment range outside the action".into()))?;
                let effect_kinds: Vec<String> = statements.iter().filter_map(effect_kind).collect();
                let has_emit = statements.iter().any(|s| matches!(s, Statement::Emit { .. }));
                let not_advanced_attempted = match statements.first() {
                    Some(guard @ Statement::Await { .. }) if segment > 0 => Some(UInt(plan::statement_cost(program, guard)?)),
                    _ => None,
                };
                let before = ap.committed_before[segment];
                let cost = ap.segment_cost[segment];
                let after = before.0.checked_add(cost.0).map(UInt).ok_or_else(|| CompileError::Bound("committed work exceeds u128".into()))?;
                segments.push(StagedSegmentShape {
                    segment: segment as u16,
                    entry,
                    statement_start: start,
                    statement_end: end,
                    exit,
                    segment_cost: cost,
                    committed_before: before,
                    committed_after: after,
                    not_advanced_attempted,
                    effect_kinds,
                    has_emit,
                });
            }
            actions.push(StagedActionPlan {
                action: action.name.clone(),
                action_index,
                statement_count: ap.statement_count,
                plan_total: ap.plan_total,
                segments,
            });
        }
        Ok(StagedLoweringPlan {
            language: program.language.clone(),
            semantics: program.semantics.clone(),
            logical_cost_profile: mori_core::stage::LOGICAL_COST_PROFILE_2.into(),
            stage_plan_format: mori_core::stage::STAGE_PLAN_FORMAT.into(),
            continuation_format: mori_core::stage::CONTINUATION_FORMAT.into(),
            program_hash: Digest32(self.staged.program_hash()),
            plan_hash: Digest32(self.staged.plan_hash()),
            source_sha256: self.source_sha256,
            stored_cap: StoredCapParameter::ExplicitPublicSlot,
            actions,
            remaining: STAGED_REMAINING.iter().map(|s| (*s).to_string()).collect(),
        })
    }
}
```

## U01 UNINSPECTED_BOUNDARY

Span,Soa/Soars and MoriDiagnostic implementations plus inherited Cargo versions are absent. Token-span and diagnostic-construction call sites are inspected,not these dependency bodies.

`MAIN-lexer-src-lib.rs:19–21`, SHA-256 `5cd5bd604e41fe57e754dd323a4fd86393d34c231a4dee16efe5fd13da5e28d7`:

```text
use mori_diagnostics::MoriDiagnostic;
use mori_span::Span;
use soa_rs::Soa;
```

`MAIN-lexer-src-token.rs:1–2`, SHA-256 `e7860a893c0827bbf362165438c67d621f229f0eacecb9175d959788687bc3db`:

```text
use mori_span::Span;
use soa_rs::{Soa, Soars};
```

`MAIN-lexer-src-token.rs:193–204`, SHA-256 `e7860a893c0827bbf362165438c67d621f229f0eacecb9175d959788687bc3db`:

```text
    /// The token's span, recovered by re-scanning `source` from its start.
    pub fn span(&self, idx: TokenIdx, source: &str) -> Span {
        let start = self.start(idx);
        Span::new(
            start,
            scan::token_end(self.kind(idx), source.as_bytes(), start),
        )
    }

    pub fn comment_span(&self, index: usize) -> Span {
        Span::new(self.comments.start()[index], self.comments.end()[index])
    }
```

`MAIN-lexer-Cargo.toml:5–19`, SHA-256 `1ae623c38bd4e5d83f691c01191e8f271a268c60913311a1ed62c4afeef88b68`:

```text
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true

[dependencies]
mori-diagnostics.workspace = true
mori-span.workspace = true
soa-rs.workspace = true

[dev-dependencies]
indoc.workspace = true
insta.workspace = true
mori-diagnostics = { workspace = true, features = ["fancy"] }
proptest.workspace = true
```

## U02 UNINSPECTED_BOUNDARY

Core intent::canonical/hash_bytes and every include_bytes target not separately supplied remain external to this packet. Source declarations and helper call order do not prove actual embedded contents,build execution,binary identity or cryptographic behavior.

`HELPER-identity.rs:80–99`, SHA-256 `4dd88ceb42b98ad32a4c8502de9be8469508e710293810c6ff16eb59b423a661`:

```text
/// SHA-256 of `intent::canonical(value)`, reusing Core's canonical encoder and hash.
pub fn digest_of<T: Serialize>(value: &T) -> Result<Digest32, CompileError> {
    let bytes = mori_core::intent::canonical(value).map_err(CompileError::Core)?;
    Ok(digest_bytes(&bytes))
}

/// SHA-256 of raw bytes through Core's `hash_bytes`.
pub fn digest_bytes(bytes: &[u8]) -> Digest32 {
    Digest32::from_hex(&mori_core::intent::hash_bytes(bytes)).expect("Core hash_bytes returns 64 lowercase hex characters")
}

/// Canonical typed-Program identity (equals `CoreSourceExportV1.program_hash` and the
/// owner-intent `program_hash` for the same Program).
pub fn program_hash(program: &Program) -> Result<Digest32, CompileError> {
    digest_of(program)
}

/// Raw source identity; never bound in-circuit by this compiler.
pub fn source_hash(source: &str) -> Digest32 {
    digest_bytes(source.as_bytes())
```

`HELPER-build_identity.rs:44–54`, SHA-256 `813b2b183309c6f8db8afa5e4a77e3fdc39c92fb8a5ea9eff6f76829157c7f3f`:

```text
macro_rules! own {
    ($p:literal) => {
        Embedded { path: $p, bytes: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $p)) }
    };
}

macro_rules! core_file {
    ($p:literal) => {
        Embedded { path: $p, bytes: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../moriarty/", $p)) }
    };
}
```

`HELPER-build_identity.rs:173–204`, SHA-256 `813b2b183309c6f8db8afa5e4a77e3fdc39c92fb8a5ea9eff6f76829157c7f3f`:

```text
/// Entries sorted by path bytewise; a repeated path is refused.
pub fn entries(files: &[Embedded]) -> Result<Vec<Entry>, CompileError> {
    let mut out: Vec<Entry> = files
        .iter()
        .map(|f| Entry { path: f.path.to_string(), sha256: identity::digest_bytes(f.bytes).hex(), bytes: f.bytes.len() as u64 })
        .collect();
    out.sort_by(|a, b| a.path.as_bytes().cmp(b.path.as_bytes()));
    if out.windows(2).any(|w| w[0].path == w[1].path) {
        return Err(CompileError::Identity("embedded build membership repeats a path".into()));
    }
    Ok(out)
}

/// Canonical bytes of the `mori-r1cs-build-manifest/2` document for `OWN_FILES`.
pub fn build_manifest_bytes() -> Result<Vec<u8>, CompileError> {
    mori_core::intent::canonical(&BuildManifestV2 { format: BUILD_MANIFEST_FORMAT, files: entries(OWN_FILES)? }).map_err(CompileError::Core)
}

/// SHA-256 of the canonical build manifest.
pub fn build_manifest_digest() -> Result<Digest32, CompileError> {
    Ok(identity::digest_bytes(&build_manifest_bytes()?))
}

/// The Core closure digest under the unchanged `/1` closure rule.
pub fn core_closure_digest() -> Result<Digest32, CompileError> {
    identity::digest_of(&entries(CORE_FILES)?)
}

/// The only `CompilerBuild` that `compile` accepts.
pub fn embedded_build() -> Result<CompilerBuild, CompileError> {
    Ok(CompilerBuild { compiler_source_manifest_sha256: build_manifest_digest()?, core_closure_sha256: core_closure_digest()? })
}
```

## U03 UNINSPECTED_BOUNDARY

blake2b256,blake2b256_gadget,alloc_bit,digest_half_lc,Cs/Lc/fe and UIntW/WideW implementation bodies are absent. Calls/types are recorded; no hash,field,proof,range or resource adequacy judgment is made.

`HELPER-abi2.rs:22–29`, SHA-256 `51fa106305481cb6c48c9eeda0d2b24ae5e5f7b9816daa48b2b474b56e98debb`:

```text
use crate::adapter::{AdapterError, BindError, SlotWire};
use crate::blake2b::{self, Bit};
use crate::error::{CompileError, MappingError};
use crate::gadgets::{UIntW, WideW};
use crate::plan::{AccountingBinding, ActionPlan, CapBinding, CompileOptions, DutyOrigin, EffectShape, FrameCells};
use crate::r1cs::{Cs, Lc, fe};
use crate::statement::{EffectValues, SlotEncoding, SlotKind, StatementLayout, TransitionValues, Wide, accounting_summary};
use ark_ff::PrimeField;
```

`HELPER-abi2.rs:297–304`, SHA-256 `51fa106305481cb6c48c9eeda0d2b24ae5e5f7b9816daa48b2b474b56e98debb`:

```text
/// The two public inputs `[int_be(d[0..16]), int_be(d[16..32])]`, d = BLAKE2b-256(stmt).
pub fn public_inputs<F: PrimeField>(statement: &[u8]) -> Vec<F> {
    let d = blake2b::blake2b256(statement);
    let mut hi = [0u8; 16];
    let mut lo = [0u8; 16];
    hi.copy_from_slice(&d[..16]);
    lo.copy_from_slice(&d[16..]);
    vec![fe(u128::from_be_bytes(hi)), fe(u128::from_be_bytes(lo))]
```

`HELPER-abi2.rs:343–354`, SHA-256 `51fa106305481cb6c48c9eeda0d2b24ae5e5f7b9816daa48b2b474b56e98debb`:

```text
    let mut sum = Lc::zero();
    for k in 0..n {
        for j in 0..8 {
            let val = honest.map(|h| (h[k] >> j) & 1 == 1);
            let bit = blake2b::alloc_bit(cs, "abi2.body", val)?;
            let weight = 8 * (n - 1 - k) + j;
            sum = sum.plus(&bit.lc.scaled(fe(1u128 << weight)));
            bits.push(bit);
        }
    }
    cs.enforce_eq(&sum, limb)?;
    Ok(())
```

`HELPER-abi2.rs:431–435`, SHA-256 `51fa106305481cb6c48c9eeda0d2b24ae5e5f7b9816daa48b2b474b56e98debb`:

```text
    let digest = blake2b::blake2b256_gadget(cs, &bits)?;
    for (k, from) in [0usize, 16].into_iter().enumerate() {
        let x = cs.input(claimed.map(|c| c[k]))?;
        cs.enforce_eq(&Lc::var(x), &blake2b::digest_half_lc(&digest, from))?;
    }
```

## U04 UNINSPECTED_BOUNDARY

plan::statement_cost and Core StagedProgram accessors/types are called from lowering_plan but their implementations are outside this packet. S16 plan_action/probe calls are also boundary references here. Prior33-file analysis remains separately immutable.

`HELPER-staged.rs:25–29`, SHA-256 `f8ba260a0672a891f98b3af5823ded5ae23e514760ae58816eb29a75cc8d1fc6`:

```text
use crate::error::CompileError;
use crate::identity::Digest32;
use crate::plan;
use mori_core::stage::{CaptureSlot, StageBinder, StagedProgram};
use mori_core::{Statement, UInt};
```

`HELPER-staged.rs:156–169`, SHA-256 `f8ba260a0672a891f98b3af5823ded5ae23e514760ae58816eb29a75cc8d1fc6`:

```text
    pub fn lowering_plan(&self) -> Result<StagedLoweringPlan, CompileError> {
        let program = self.staged.program();
        let stage_plan = self.staged.plan();
        let mut actions = Vec::with_capacity(stage_plan.actions.len());
        for (index, (action, ap)) in program.actions.iter().zip(&stage_plan.actions).enumerate() {
            if ap.action != action.name || ap.segment_cost.len() != ap.sites.len() + 1 || ap.committed_before.len() != ap.sites.len() + 1 {
                return Err(CompileError::Bound(format!("stage plan for {:?} is not the admitted plan", action.name)));
            }
            let action_index = u16::try_from(index).map_err(|_| CompileError::Bound("action index exceeds u16".into()))?;
            let inputs: Vec<CaptureSlot> = action
                .inputs
                .iter()
                .enumerate()
                .map(|(i, input)| CaptureSlot { name: input.name.clone(), ty: input.ty, origin: mori_core::stage::Origin::Input, ordinal: i as u16 })
```

`HELPER-staged.rs:199–207`, SHA-256 `f8ba260a0672a891f98b3af5823ded5ae23e514760ae58816eb29a75cc8d1fc6`:

```text
                let effect_kinds: Vec<String> = statements.iter().filter_map(effect_kind).collect();
                let has_emit = statements.iter().any(|s| matches!(s, Statement::Emit { .. }));
                let not_advanced_attempted = match statements.first() {
                    Some(guard @ Statement::Await { .. }) if segment > 0 => Some(UInt(plan::statement_cost(program, guard)?)),
                    _ => None,
                };
                let before = ap.committed_before[segment];
                let cost = ap.segment_cost[segment];
                let after = before.0.checked_add(cost.0).map(UInt).ok_or_else(|| CompileError::Bound("committed work exceeds u128".into()))?;
```

`S16-circuit.rs:225–226`, SHA-256 `3856551a3466e6790c0142df9b2025487d2330f9da20caa3796f1ff27bd5649c`:

```text
    let plan = plan::plan_action(admitted.program(), &options)?;
    let layout = StatementLayout::build(admitted.identity(), &plan, build)?;
```

`S16-circuit.rs:260–260`, SHA-256 `3856551a3466e6790c0142df9b2025487d2330f9da20caa3796f1ff27bd5649c`:

```text
    relation.probe = probe::<F>(&relation)?;
```

## U05 UNINSPECTED_BOUNDARY

Adapter TARGET_OBLIGATIONS,SETTLEMENT_V2_TARGET_DISCHARGE and STAGED_REMAINING are source string arrays. Their presence is recorded; the text is not evidence that listed obligations are discharged or implemented. No referenced ledger/source paths are followed.

`HELPER-adapter.rs:89–113`, SHA-256 `24dcee35856ee8bf6ef3dcde8be39b56692af34e99fc5975bd1230dc07a91b1d`:

```text
/// Obligations a pure transition proof does not discharge, whatever the adapter.
pub const TARGET_OBLIGATIONS: &[&str] = &[
    "Pre-state authenticity: pre states, cells and duties are statement inputs; a target must bind them to the consumed datum/UTxO (ScriptCustodied) or label them unauthenticated premises (KeyHeldExternal, projection.rs:415-430).",
    "Balance-row existence (premise, review F-04): Core refuses a scenario that lacks an explicit balance row for a touched account (runtime.rs:609-627), but the layout has no balance presence bit, so a claimed pre-balance of 0 for a missing row satisfies the relation. A target must authenticate the existence of every represented balance row, or a reviewed presence bit must be added; until then this is an unauthenticated premise, not a relation constant.",
    "Frame completeness: rows outside the represented cells are not in the statement; a target must represent them (FrameCells::Include) or establish that none exist, because Core's 65,536 trace bound counts every scenario row (runtime.rs:540-547). FrameCells::Include covers balance cells only: an unregistered incoming duty (no source Liability) cannot be represented at all (review F-05), so duty-frame completeness rests entirely on the target until a reviewed duty-frame option exists. No P3 or ABI mapping may claim a complete frame before then.",
    "Physical asset delta: Transfer/Fee effects are bookkeeping; payouts, deposits, custody totals, conservation and tagged outputs are separate target relations (ABI-V1 VOTES.md P3).",
    "Debit authority: authority for debited accounts and owner intent limits (intent::check_effects) are separate checks; the accounting summary only exposes gross debit and fees. The P3-7 scope is disputed and unreconciled: the later ABI review (f3ae8556, V6) requires authority for every debited account including custody-held balances, thread-advance authority and a signed intent in the statement, not only external (KeyHeldExternal) signers.",
    "Replay and consumption: one-time use of the consumed thread/claim, nonce and predecessor history.",
    "Ledger phases and failure outcomes: rejected executions have no success proof here; any charged failure transition needs its own relation.",
    "Setup provenance: the R1CS digest names the matrices; a VK is trusted only with a reviewed setup transcript over exactly those matrices.",
];

/// P3S1 PROPOSED: how the ABI v2 Cardano target (S2, `candidate/aiken/lib/pvc2/thread.ak`)
/// takes over each [`TARGET_OBLIGATIONS`] entry for [`StatementAdapter::SettlementHashV2`].
/// Source-level mapping only; nothing here is reviewed, compiled or executed.
pub const SETTLEMENT_V2_TARGET_DISCHARGE: &[(&str, &str)] = &[
    ("Pre-state authenticity", "S2 builds pre states, custody cells and duties from the spent thread datum; external cells use the canonical premise (gross debit in the step)."),
    ("Balance-row existence (F-04)", "S2 refuses a represented custody cell without a datum row; external cells have no row by design."),
    ("Frame completeness (F-05)", "The datum is the complete snapshot; S2 copies every unrepresented custody row and liability (registered or not) into the continuation and checks Core's scenario bounds on the whole snapshot."),
    ("Physical asset delta", "S2 checks exact custody value before and after, tagged or pinned-datum payouts per external credit, global output allocation, reserve top-ups and no own-policy mint."),
    ("Debit authority", "S2 requires a verified Core owner intent from every debited account (custody included), every accrual debtor, every discharge creditor and one action advancer; Core check_effects runs on the bound effects and the complete closing duties."),
    ("Replay and consumption", "The intent binds the consumed outref and full pre-snapshot; the claim names the outref; the ledger consumes it once; seq and the event chain advance."),
    ("Ledger phases and failure outcomes", "Cardano transactions are atomic; phase-2 failure consumes only key-locked collateral; a refusal leaves the datum (and every duty) unchanged. No charged failure transition is modelled."),
    ("Setup provenance", "Unchanged: the catalogue pins (predicate, vk) per action; VK provenance needs a reviewed setup transcript."),
];
```

`HELPER-staged.rs:113–127`, SHA-256 `f8ba260a0672a891f98b3af5823ded5ae23e514760ae58816eb29a75cc8d1fc6`:

```text
/// Remaining staged obligations. Listing them waives none.
pub const STAGED_REMAINING: &[&str] = &[
    "segment relations for every (action, segment) with program and plan hash bound in-circuit",
    "pre/post frame slots (Idle | Pending{site, captures}) in the exact capture layouts, captures prefix-monotone",
    "event binder slots and the resumed guard as a constrained Step against the unchanged boundary",
    "await entry charge and committed_before/committed_after/plan_total binding (logical-cost/2)",
    "stored cap as an explicit public slot; D-13 canonical reconstruction value stays split",
    "R7 canonical boundary predicates (states exact, balances strictly ascending, registry identities, trace bound)",
    "residual duties and present-zero versus absent across segments",
    "NotAdvanced (false guard) as a separate non-transaction relation with attempted work",
    "Emit and every effect kind in staged segments (the staged UNIT codec admits Emit)",
    "staged continuation codec correspondence (production codec 3b21... frozen; FREEZE-4 r2 result unadopted)",
    "authentication and one-time consumption of continuations and events (STAGE_LABELS: none/untracked)",
    "independent staged differential oracles and hostile witness mutations before any execution",
];
```

The first source-navigation gaps now have inspected local bodies. External/gadget boundaries stay explicit. In particular, metadata that names future staged slots/relations is distinct from executable lowering; this record grants no adoption,financial,cryptographic,proof or ledger conclusion.
