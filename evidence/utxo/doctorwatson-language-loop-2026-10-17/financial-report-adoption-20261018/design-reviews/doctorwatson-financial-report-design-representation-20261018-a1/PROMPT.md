Review exact additive financial-report.patch as a consequential observation-contract design, independently. Decide ADOPT / REVISE / ABSTAIN with substantive reasons. Current author is source-exposed, not an independent oracle; no runtime was run. Check full frame/statement ExprId vs typed amount/effect delta and prefix rules, zero/self effects and absent vs present-zero duty distinctions against genuine current DTO/runtime. The proposal must not change financial admission/arithmetic or invent Mint/Burn/Accrual semantics; keep those full obligations missing and retain full U/M/MC/SP/G exits. No financial-pair fullrawStep/consumer-byte/compiler/proof pass inferred. Parent baseline stays unchanged. Your role is ordinary observational representation/consumer design; do not decide conservation, authorization, soundness or security conditions, nor hostile tests. If exact patch is suitable for adoption, say so while noting its current unadopted status text would require explicit recorded adoption before it becomes the effective contract. No permissionless public developer gate. Include any necessary concrete wording revisions, exact source exposures and limits.
Readonly packet /home/charl/moriarty-c2-opus55-restored-20261017/financial-report-design-review-packet. Return one whole six-field JSON with reviewToken doctorwatson-financial-report-design-representation-20261018-a1-whole-review. No commands, toolchain, runtime, build, SDK, network, private/config/auth/prover files or delegated agents. Only pinned public sources may be read.

--- FILE REVIEW-PACKET.md ---
# Financial report observation adoption candidate

Status: **UNADOPTED; implementation-source-exposed; no runtime evidence**, 2026-10-18. Author lane: assigned GPT-6.1 Sol HIGH ordinary source preparation. Prepared for two fresh independent Gemini design/source reviews; neither reviewer was invoked by this lane. No votes or approvals are represented.

Review predicate: the additions specify complete observations of the existing Transfer/Fee/Liability/Discharge DTOs and effects, preserve outer typed result versus expression reference and absent versus present-zero liabilities, and preserve every baseline requirement without changing financial semantics or security conditions.

The exact unified candidate is `financial-report.patch`, against the source hashes in `SOURCE-IDENTITY.json`. It has additions only to two current checkout files; no existing baseline line is removed or replaced. Current source was reread after PR202's five-kind contract, but Git was not inspected and no commit identity is asserted. The patch is an outside-Git artifact and was not applied to the checkout. Changes to either baseline require rebuilding and rereviewing the exact candidate.

## Requested independent judgments

Each reviewer should independently return a substantive ADOPT / REVISE / ABSTAIN decision for this exact patch, identify each material ambiguity or mismatch with the included current DTO/runtime sources, and give concrete text where revision is needed. Review whether the proposed normative choices faithfully specify current report observations, including zero/self effects and the shared Liability effect variant; source agreement alone is not independent behavior proof. Check that no omitted mint/burn/dedicated-accrual obligation is waived and that addition does not acquire interest semantics. All security/conservation/authorization judgments are outside this review and remain with their designated owner. Two fresh substantive decisions are still missing; this packet establishes no adoption.

## Baseline retained exactly

The existing introductory scope, NR-01–06, Counter/empty requirements, positive work schedule, process/envelopes, original normal Transfer aggregate-byte compatibility and provenance paragraphs remain byte-for-byte. No new top-level roadmap ID, denominator or framework is added. The existing paragraphs continue to say the five-kind design is adopted and other financial statements retain their existing contracts. The new section is explicitly a separate unadopted candidate for four actual Core/1 kinds. UNI-018/020/021 retain their existing normative bodies and receive only the candidate child paragraphs below. Product contract, full finance and every U/M/MC/SP/G exit remain unchanged.

## Complete added documentation section

Destination: docs/CORE-EXECUTION-REPORTS.md, immediately before its existing Provenance and conformance heading.

## Proposed financial observation extension (unadopted)

This section is a proposed additive observation contract for normally accepted finite Rust `moriarty/1` / `moriarty-core/1` Transfer, Fee, Liability and Discharge statements. Its SHALL clauses remain unadopted until a separately recorded design decision. They do not replace NR-01–06, broaden the earlier five-kind adoption, or change typing, arithmetic, admission or financial transitions. The author inspected implementation source; this proposal is not independent behavior proof, a golden or an executed result. Core/2, other language profiles and target consumers retain separate contracts.

Current Core/1 has no Mint, Burn or dedicated accrual statement. Liability creates a named duty or adds an explicitly supplied amount. This addition supplies no interest rate, time basis, day-count, compounding or accrual schedule. Mint/burn and dedicated accrual remain missing language/financial obligations where required by the broader product; this report extension neither implements nor removes them.

Let S[-1] be the complete normalized Execution.pre and S[i] the complete snapshot immediately after statement i. Let q be the UInt result of that statement's amount expression evaluated immediately before its transition, using current state, action inputs, earlier captures and current liability reads under existing Core semantics. Let A be the exact declared AssetId denoted by its asset alias. For each completed financial statement, the Step SHALL have exactly index, statement, value, snapshot and effects. The numeric zero-based index SHALL be its ordinal in the selected action, including preceding nonfinancial statements. The exact admitted statement SHALL retain these members in addition to kind:

| kind | Statement members | Meaning |
| --- | --- | --- |
| Transfer | asset, from, to, amount | asset alias, account strings, amount ExprId |
| Fee | asset, from, to, amount | asset alias, account strings, amount ExprId |
| Liability | name, asset, debtor, creditor, amount | duty name, asset alias, party strings, amount ExprId |
| Discharge | name, amount | existing duty name, amount ExprId |

Statement.amount SHALL remain the numeric zero-based reference into that Program's expressions array. The outer Step.value SHALL instead contain the present typed UInt object with type "UInt" and value q's canonical unsigned decimal string, including zero. It SHALL report the moved, charged, added or discharged amount, respectively; neither the ExprId, a boolean success value, recipient balance nor closing duty amount shall replace it. Null or omission SHALL NOT replace the typed object for a completed statement of these kinds. The following table defines the report projection of existing accepted transitions; it creates no new admission premise:

| Statement | Complete post-statement snapshot relative to S[i-1] | Exact new Step.effects |
| --- | --- | --- |
| Transfer | Existing movement of q in A between the two explicit account rows; preserve all other balances, state and liabilities | one Transfer {asset:A,from,to,amount:q} |
| Fee | Existing movement of q in A between the two explicit account rows; preserve all other balances, state and liabilities | one Fee {asset:A,from,to,amount:q} |
| Liability, row absent | Insert the identified duty with amount q in name order; preserve all other duties, balances and state | one Liability {name,asset:A,debtor,creditor,before:0,after:q} |
| Liability, row present with amount b | Preserve duty identity; replace only its amount with the existing checked sum b+q; preserve other duties, balances and state | one Liability {name,asset:A,debtor,creditor,before:b,after:b+q} |
| Discharge, row present with amount b | Preserve duty identity; replace only its amount with the existing checked difference b-q, retaining zero; preserve other duties, balances and state | one Liability {name,asset:existing asset,debtor:existing debtor,creditor:existing creditor,before:b,after:b-q} |

The brace forms denote exact effect field sets plus kind carrying the named variant. Each effect AssetId SHALL contain network, issuer, reference and scale, rather than the statement asset alias. Effect amount, before and after SHALL be canonical decimal strings without a typed Value wrapper. Discharge SHALL use the Liability effect variant; no new Discharge effect is introduced. The exact statement and complete prior snapshot SHALL retain the distinction among creation, addition and discharge. A Liability effect with before zero alone cannot distinguish absent creation from a present-zero opening duty.

For an already admitted self-transfer or self-fee, the numeric balance SHALL remain unchanged while the amount effect remains present. For an already admitted zero amount, the effect SHALL remain present even where numeric amounts do not change. Zero-amount Liability creation SHALL insert a present-zero duty; zero or full Discharge SHALL retain the existing duty row. These clauses add no admission authority. Discharge SHALL remain a duty reduction; any payment is a separate statement and Step.

Each Step.snapshot SHALL equal S[i] in full, containing exactly state, balances and liabilities. State SHALL retain all declared names and typed values; inputs and let captures SHALL NOT become state fields. Balance rows SHALL retain exactly account, full asset and amount; liability rows exactly name, full asset, debtor, creditor and amount. Affected and preserved rows, including observer balances and incoming duties admitted by Core, SHALL remain present. Balances SHALL be ordered by exact (asset,account), AssetId by (network,issuer,reference,scale), and liabilities by name. Absent and present-zero liabilities SHALL remain distinct. Empty state SHALL be {}, and empty balance/liability collections SHALL be [], not omitted or null. This frame concerns the admitted local snapshot and supplies no authenticated-domain or ledger-acceptance claim.

Step.effects SHALL contain only the statement's new effect, rather than an accumulated prefix. With P[-1]=[] and P[i]=P[i-1] followed by Step[i].effects, Execution.effects SHALL equal the ordered concatenation of all statement deltas, including intervening nonfinancial effects, preserving duplicates and zero/self effects. P remains derived notation, not a wire field. Complete snapshots and effect histories SHALL both be retained; unchanged numeric state does not imply empty effects. For N>0, post SHALL equal the last Step.snapshot. For N=0, the existing complete pre=post, steps:[] and effects:[] requirements remain, without a synthetic Step. Existing Execution/envelope fields and logical-work schedule remain unchanged; no per-Step work member is introduced. Any claimed ordinary/metered agreement SHALL compare the same complete Execution and independently specified work under the existing schedule.

Design adoption, source/IR correspondence, independent expectations, actual runtime observations and acceptance SHALL remain separate. Expectations for complete statements/ExprIds, typed values, snapshots, deltas, prefixes, work and consumer-specific bytes SHALL be independently derived and frozen before authorized runs, without generating them from the implementation under comparison. Existing financial-pair coverage and the normal Transfer aggregate compatibility case SHALL retain their exact scope; no full raw Step.value/statement or ordinary simulate-byte coverage is inferred. Original proposals, freezes, failures, five-kind adoption and pending normal-report cases SHALL remain unchanged. All U0–U7, M1–M17, MC/SP/G, full finance, ACTUS/DeFi and composition obligations remain. This proposal decides no conservation, soundness, authorization, authentication, proof or security condition and supplies no hostile/security tests; those remain with the designated security owner. Internal review records remain separate from permissionless developer use.

## Complete added OpenSpec paragraphs

Destination: openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md. First addition is the last child paragraph under UNI-018, second under UNI-020, third under UNI-021.

**Proposed financial report observations (unadopted).** The [proposed financial section](../../../../../docs/CORE-EXECUTION-REPORTS.md#proposed-financial-observation-extension-unadopted) adds candidate observations only for normally accepted Core/1 Transfer, Fee, Liability and Discharge. Each Step SHALL retain its exact admitted IR statement and numeric zero-based action ordinal; statement.amount SHALL be an ExprId into that Program's expressions array, while outer Step.value SHALL be the present typed UInt evaluated amount, including zero, rather than the closing balance/duty or a reference. Each complete post-statement snapshot SHALL retain affected and preserved state, balance and liability rows, exact asset identities, canonical row order and absent versus present-zero duties. The detailed table SHALL project the existing movement, duty creation/addition and discharge transitions without changing admission or financial semantics. Mint, Burn and dedicated accrual remain absent Core/1 constructs and retain their broader missing obligations where required; explicit amount addition supplies no interest/accrual convention. These clauses remain unadopted until a separately recorded design decision and do not broaden the earlier NR-01–06 five-kind adoption.

**Proposed financial deltas (unadopted).** For those normally completed financial statements, Step.effects SHALL contain exactly one new Transfer, Fee or Liability effect as specified by the proposed table; Discharge SHALL use Liability with before/after amounts. Zero/self cases already admitted by existing semantics SHALL retain their explicit effects. Execution.effects SHALL equal the ordered concatenation of every Step delta, including intervening nonfinancial effects, preserving repetitions and order; prefixes remain derived data. A before-zero effect SHALL NOT erase the full-snapshot distinction between absent creation and a present-zero opening duty. Complete statement/ExprId/value/snapshot/delta/prefix/work observations and exact consumer-specific bytes SHALL require independently derived expectations frozen before the run. No evaluator output SHALL generate those expectations.

**Proposed financial evidence boundary (unadopted).** The financial observation extension SHALL remain design-only until independently adopted, and adoption SHALL remain separate from implementation and behavior. Its author inspected implementation source; source agreement supplies neither independent behavior proof nor goldens. Existing financial-pair and normal Transfer aggregate observations SHALL keep their exact scope without inferred complete raw Step.value/statement or ordinary simulate-byte coverage. Original proposals, freezes, failures, earlier five-kind adoption and pending normal-report cases SHALL remain unchanged. No parent acceptance, denominator or native/target result follows; all U0–U7, M1–M17, MC/SP/G and full financial/composition obligations remain. No conservation, soundness, authorization, authentication, proof or security condition is decided; security judgments and hostile/security tests remain with the designated security owner.

## Exact public DTO definitions from current source

The following excerpts are verbatim source, not replacement code or a fixture. Excerpt1: crates/mori-core/src/ir.rs lines1–52 (UInt canonical serialization, typed Value and ExprId). Excerpt2: lines104–270 (AssetId through Execution, complete current shared Statement/Effect definitions). Shared Statement includes Await for staged use; legacy Core/1 admission/decoding remains separate. Excerpt3: lines275–322 (frozen Core/1 statement wire, with no Await/Mint/Burn/Accrual). Whole-file source hashes are in SOURCE-IDENTITY.json. The proposed clauses change none of these types.

```rust
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const LANGUAGE: &str = "moriarty/1";
pub const SEMANTICS: &str = "moriarty-core/1";
pub const MAX_EXPRESSIONS: usize = 8192;
pub const MAX_STATEMENTS: usize = 1024;
pub const MAX_DECLARATIONS: usize = 256;
pub const MAX_TRACE_CELLS: usize = 65_536;

/// Unsigned integral atomic units; JSON uses canonical decimal strings.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UInt(pub u128);
impl Serialize for UInt {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}
impl<'de> Deserialize<'de> for UInt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if s.is_empty()
            || !s.bytes().all(|b| b.is_ascii_digit())
            || (s.len() > 1 && s.starts_with('0'))
        {
            return Err(serde::de::Error::custom(
                "expected canonical unsigned decimal string",
            ));
        }
        s.parse().map(UInt).map_err(serde::de::Error::custom)
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Type {
    UInt,
    Bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "value", deny_unknown_fields)]
pub enum Value {
    UInt(UInt),
    Bool(bool),
}
impl Value {
    pub fn ty(&self) -> Type {
        match self {
            Self::UInt(_) => Type::UInt,
            Self::Bool(_) => Type::Bool,
        }
    }
}
pub type ExprId = u16;
```

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct AssetId {
    pub network: String,
    pub issuer: String,
    pub reference: String,
    pub scale: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetDecl {
    pub name: String,
    pub identity: AssetId,
}
/// Serialize-only on the shared type: decoding always goes through the frozen
/// `v1` wire, so JSON can never yield `Await`.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Statement {
    Await {
        label: String,
        binders: Vec<InputDecl>,
        guard: ExprId,
    },
    Let {
        name: String,
        value: ExprId,
    },
    Requires {
        predicate: ExprId,
    },
    Ensures {
        predicate: ExprId,
    },
    Next {
        state: String,
        value: ExprId,
    },
    Emit {
        name: String,
        value: ExprId,
    },
    Transfer {
        asset: String,
        from: String,
        to: String,
        amount: ExprId,
    },
    Fee {
        asset: String,
        from: String,
        to: String,
        amount: ExprId,
    },
    Liability {
        name: String,
        asset: String,
        debtor: String,
        creditor: String,
        amount: ExprId,
    },
    Discharge {
        name: String,
        amount: ExprId,
    },
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub name: String,
    pub inputs: Vec<InputDecl>,
    pub statements: Vec<Statement>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Program {
    pub language: String,
    pub semantics: String,
    pub name: String,
    pub states: Vec<StateDecl>,
    pub assets: Vec<AssetDecl>,
    pub expressions: Vec<Expr>,
    pub actions: Vec<Action>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Balance {
    pub account: String,
    pub asset: AssetId,
    pub amount: UInt,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Liability {
    pub name: String,
    pub asset: AssetId,
    pub debtor: String,
    pub creditor: String,
    pub amount: UInt,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub state: BTreeMap<String, Value>,
    pub inputs: BTreeMap<String, Value>,
    pub balances: Vec<Balance>,
    pub liabilities: Vec<Liability>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub state: BTreeMap<String, Value>,
    pub balances: Vec<Balance>,
    pub liabilities: Vec<Liability>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Effect {
    State {
        name: String,
        before: Value,
        after: Value,
    },
    Transfer {
        asset: AssetId,
        from: String,
        to: String,
        amount: UInt,
    },
    Fee {
        asset: AssetId,
        from: String,
        to: String,
        amount: UInt,
    },
    Liability {
        name: String,
        asset: AssetId,
        debtor: String,
        creditor: String,
        before: UInt,
        after: UInt,
    },
    Emit {
        name: String,
        value: Value,
    },
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub index: u16,
    pub statement: Statement,
    pub value: Option<Value>,
    pub snapshot: Snapshot,
    pub effects: Vec<Effect>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Execution {
    pub semantics: String,
    pub action: String,
    pub pre: Snapshot,
    pub post: Snapshot,
    pub steps: Vec<Step>,
    pub effects: Vec<Effect>,
}
```

```rust
    use super::{AssetDecl, Effect, Expr, ExprId, InputDecl, Snapshot, StateDecl, Value};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(tag = "kind", deny_unknown_fields)]
    pub enum Statement {
        Let {
            name: String,
            value: ExprId,
        },
        Requires {
            predicate: ExprId,
        },
        Ensures {
            predicate: ExprId,
        },
        Next {
            state: String,
            value: ExprId,
        },
        Emit {
            name: String,
            value: ExprId,
        },
        Transfer {
            asset: String,
            from: String,
            to: String,
            amount: ExprId,
        },
        Fee {
            asset: String,
            from: String,
            to: String,
            amount: ExprId,
        },
        Liability {
            name: String,
            asset: String,
            debtor: String,
            creditor: String,
            amount: ExprId,
        },
        Discharge {
            name: String,
            amount: ExprId,
        },
    }
```

## Exact source witnesses for the report projection

These implementation excerpts expose the author's source dependency. They are source inspection, not tests, model evidence, goldens, or independently proved behavior. Current README lines43–71 separately describes atomic units, Transfer/Fee effects, self effects, no mint/burn/account creation, Liability addition, separate discharge/payment and present-zero duty reads.

Source-inspected runtime witnesses, with whole-file identity pinned: lines671–703 perform existing balance movement and leave a self-movement amount unchanged after existing premises; lines758–788 append one Transfer/Fee effect and return Some(Value::UInt(amount)); lines790–835 insert/update Liability by checked addition, append its full before/after effect and return the added amount; lines837–860 retain a discharged row, append its full before/after Liability effect and return the discharged amount. No branch defines Mint/Burn/Accrual. These statements are implementation-exposed source observations, not independent assertions or executed tests. The DTOs above and complete candidate tables state all report field names and proposed meanings.

crates/mori-core/src/runtime.rs lines883–917: exact ordinary per-statement delta, snapshot and Execution assembly.

```rust
    let mut env = current.state.clone();
    env.extend(scenario.inputs.clone());
    let mut steps = vec![];
    let mut all_effects = vec![];
    for (i, stmt) in selected.statements.iter().enumerate() {
        let mut effects = vec![];
        let result = execute_statement(
            program,
            stmt,
            &mut env,
            &mut current,
            &mut meter,
            &mut effects,
        );
        let value = result.map_err(|mut e| {
            e.step = Some(i as u16);
            e
        })?;
        all_effects.extend(effects.clone());
        steps.push(Step {
            index: i as u16,
            statement: stmt.clone(),
            value,
            snapshot: current.clone(),
            effects,
        });
    }
    Ok(Execution {
        semantics: SEMANTICS.into(),
        action: action.into(),
        pre,
        post: current,
        steps,
        effects: all_effects,
    })
```

Scenario normalization and row sorting are source-visible at runtime.rs lines531–670; the full public source hash is pinned. Existing financial_pair.rs lines107–154 compares full pre/post/snapshots/effects and step index/kind, but does not compare full raw statement/ExprIds or Step.value. The existing complete normal Transfer oracle at crates/mori/tests/core_test_report.rs lines108–123 belongs to its original aggregate consumer. Neither has been rerun, edited, regenerated or upgraded here.

## Provenance and limits

The earlier proposal directory financial-report-contract-proposal remains unchanged. This packet compacts its design into exact additive paragraphs; it supplies no numeric expected execution or public bytes generated from the evaluator. The author read current public IR, parser financial statement dispatch, runtime financial branches, snapshot/step assembly, README, adopted normal report document and current normative parent clauses. This exposure disqualifies relabeling the packet as an independent oracle. Independent expectations must be derived and frozen separately after design decisions and before authorized execution.

The development skill, AGENTS and product contract were refreshed and the required read-only develop status was run. Its historical SP01.6 stale binding/candidate inputs, missing accounting and unavailable live resource state remain untouched. No reviewers, Git operation, repository write, Cargo/rustc/build/package, Mori/runtime/SDK/proof/VM/key/wallet execution, network, scheduler, Foreman, private/config/prover/operational-cache read or excluded validation-tree traversal occurred. Only the mandated cached skill instructions were read. All writes are in this new adoption-packet directory. Validation here is restricted to packet size, exact additive patch reconstruction, included-source identity and Markdown/JSON integrity; it is no behavior evidence.


--- FILE financial-report.patch ---
--- a/docs/CORE-EXECUTION-REPORTS.md
+++ b/docs/CORE-EXECUTION-REPORTS.md
@@ -92,2 +92,37 @@
 
+## Proposed financial observation extension (unadopted)
+
+This section is a proposed additive observation contract for normally accepted finite Rust `moriarty/1` / `moriarty-core/1` Transfer, Fee, Liability and Discharge statements. Its SHALL clauses remain unadopted until a separately recorded design decision. They do not replace NR-01–06, broaden the earlier five-kind adoption, or change typing, arithmetic, admission or financial transitions. The author inspected implementation source; this proposal is not independent behavior proof, a golden or an executed result. Core/2, other language profiles and target consumers retain separate contracts.
+
+Current Core/1 has no Mint, Burn or dedicated accrual statement. Liability creates a named duty or adds an explicitly supplied amount. This addition supplies no interest rate, time basis, day-count, compounding or accrual schedule. Mint/burn and dedicated accrual remain missing language/financial obligations where required by the broader product; this report extension neither implements nor removes them.
+
+Let S[-1] be the complete normalized Execution.pre and S[i] the complete snapshot immediately after statement i. Let q be the UInt result of that statement's amount expression evaluated immediately before its transition, using current state, action inputs, earlier captures and current liability reads under existing Core semantics. Let A be the exact declared AssetId denoted by its asset alias. For each completed financial statement, the Step SHALL have exactly index, statement, value, snapshot and effects. The numeric zero-based index SHALL be its ordinal in the selected action, including preceding nonfinancial statements. The exact admitted statement SHALL retain these members in addition to kind:
+
+| kind | Statement members | Meaning |
+| --- | --- | --- |
+| Transfer | asset, from, to, amount | asset alias, account strings, amount ExprId |
+| Fee | asset, from, to, amount | asset alias, account strings, amount ExprId |
+| Liability | name, asset, debtor, creditor, amount | duty name, asset alias, party strings, amount ExprId |
+| Discharge | name, amount | existing duty name, amount ExprId |
+
+Statement.amount SHALL remain the numeric zero-based reference into that Program's expressions array. The outer Step.value SHALL instead contain the present typed UInt object with type "UInt" and value q's canonical unsigned decimal string, including zero. It SHALL report the moved, charged, added or discharged amount, respectively; neither the ExprId, a boolean success value, recipient balance nor closing duty amount shall replace it. Null or omission SHALL NOT replace the typed object for a completed statement of these kinds. The following table defines the report projection of existing accepted transitions; it creates no new admission premise:
+
+| Statement | Complete post-statement snapshot relative to S[i-1] | Exact new Step.effects |
+| --- | --- | --- |
+| Transfer | Existing movement of q in A between the two explicit account rows; preserve all other balances, state and liabilities | one Transfer {asset:A,from,to,amount:q} |
+| Fee | Existing movement of q in A between the two explicit account rows; preserve all other balances, state and liabilities | one Fee {asset:A,from,to,amount:q} |
+| Liability, row absent | Insert the identified duty with amount q in name order; preserve all other duties, balances and state | one Liability {name,asset:A,debtor,creditor,before:0,after:q} |
+| Liability, row present with amount b | Preserve duty identity; replace only its amount with the existing checked sum b+q; preserve other duties, balances and state | one Liability {name,asset:A,debtor,creditor,before:b,after:b+q} |
+| Discharge, row present with amount b | Preserve duty identity; replace only its amount with the existing checked difference b-q, retaining zero; preserve other duties, balances and state | one Liability {name,asset:existing asset,debtor:existing debtor,creditor:existing creditor,before:b,after:b-q} |
+
+The brace forms denote exact effect field sets plus kind carrying the named variant. Each effect AssetId SHALL contain network, issuer, reference and scale, rather than the statement asset alias. Effect amount, before and after SHALL be canonical decimal strings without a typed Value wrapper. Discharge SHALL use the Liability effect variant; no new Discharge effect is introduced. The exact statement and complete prior snapshot SHALL retain the distinction among creation, addition and discharge. A Liability effect with before zero alone cannot distinguish absent creation from a present-zero opening duty.
+
+For an already admitted self-transfer or self-fee, the numeric balance SHALL remain unchanged while the amount effect remains present. For an already admitted zero amount, the effect SHALL remain present even where numeric amounts do not change. Zero-amount Liability creation SHALL insert a present-zero duty; zero or full Discharge SHALL retain the existing duty row. These clauses add no admission authority. Discharge SHALL remain a duty reduction; any payment is a separate statement and Step.
+
+Each Step.snapshot SHALL equal S[i] in full, containing exactly state, balances and liabilities. State SHALL retain all declared names and typed values; inputs and let captures SHALL NOT become state fields. Balance rows SHALL retain exactly account, full asset and amount; liability rows exactly name, full asset, debtor, creditor and amount. Affected and preserved rows, including observer balances and incoming duties admitted by Core, SHALL remain present. Balances SHALL be ordered by exact (asset,account), AssetId by (network,issuer,reference,scale), and liabilities by name. Absent and present-zero liabilities SHALL remain distinct. Empty state SHALL be {}, and empty balance/liability collections SHALL be [], not omitted or null. This frame concerns the admitted local snapshot and supplies no authenticated-domain or ledger-acceptance claim.
+
+Step.effects SHALL contain only the statement's new effect, rather than an accumulated prefix. With P[-1]=[] and P[i]=P[i-1] followed by Step[i].effects, Execution.effects SHALL equal the ordered concatenation of all statement deltas, including intervening nonfinancial effects, preserving duplicates and zero/self effects. P remains derived notation, not a wire field. Complete snapshots and effect histories SHALL both be retained; unchanged numeric state does not imply empty effects. For N>0, post SHALL equal the last Step.snapshot. For N=0, the existing complete pre=post, steps:[] and effects:[] requirements remain, without a synthetic Step. Existing Execution/envelope fields and logical-work schedule remain unchanged; no per-Step work member is introduced. Any claimed ordinary/metered agreement SHALL compare the same complete Execution and independently specified work under the existing schedule.
+
+Design adoption, source/IR correspondence, independent expectations, actual runtime observations and acceptance SHALL remain separate. Expectations for complete statements/ExprIds, typed values, snapshots, deltas, prefixes, work and consumer-specific bytes SHALL be independently derived and frozen before authorized runs, without generating them from the implementation under comparison. Existing financial-pair coverage and the normal Transfer aggregate compatibility case SHALL retain their exact scope; no full raw Step.value/statement or ordinary simulate-byte coverage is inferred. Original proposals, freezes, failures, five-kind adoption and pending normal-report cases SHALL remain unchanged. All U0–U7, M1–M17, MC/SP/G, full finance, ACTUS/DeFi and composition obligations remain. This proposal decides no conservation, soundness, authorization, authentication, proof or security condition and supplies no hostile/security tests; those remain with the designated security owner. Internal review records remain separate from permissionless developer use.
+
 ## Provenance and conformance
--- a/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md
+++ b/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md
@@ -208,2 +208,4 @@
 
+**Proposed financial report observations (unadopted).** The [proposed financial section](../../../../../docs/CORE-EXECUTION-REPORTS.md#proposed-financial-observation-extension-unadopted) adds candidate observations only for normally accepted Core/1 Transfer, Fee, Liability and Discharge. Each Step SHALL retain its exact admitted IR statement and numeric zero-based action ordinal; statement.amount SHALL be an ExprId into that Program's expressions array, while outer Step.value SHALL be the present typed UInt evaluated amount, including zero, rather than the closing balance/duty or a reference. Each complete post-statement snapshot SHALL retain affected and preserved state, balance and liability rows, exact asset identities, canonical row order and absent versus present-zero duties. The detailed table SHALL project the existing movement, duty creation/addition and discharge transitions without changing admission or financial semantics. Mint, Burn and dedicated accrual remain absent Core/1 constructs and retain their broader missing obligations where required; explicit amount addition supplies no interest/accrual convention. These clauses remain unadopted until a separately recorded design decision and do not broaden the earlier NR-01–06 five-kind adoption.
+
 ### Requirement: UNI-019 Scoped lowering comparison
@@ -234,2 +236,4 @@
 
+**Proposed financial deltas (unadopted).** For those normally completed financial statements, Step.effects SHALL contain exactly one new Transfer, Fee or Liability effect as specified by the proposed table; Discharge SHALL use Liability with before/after amounts. Zero/self cases already admitted by existing semantics SHALL retain their explicit effects. Execution.effects SHALL equal the ordered concatenation of every Step delta, including intervening nonfinancial effects, preserving repetitions and order; prefixes remain derived data. A before-zero effect SHALL NOT erase the full-snapshot distinction between absent creation and a present-zero opening duty. Complete statement/ExprId/value/snapshot/delta/prefix/work observations and exact consumer-specific bytes SHALL require independently derived expectations frozen before the run. No evaluator output SHALL generate those expectations.
+
 ### Requirement: UNI-021 Capability evidence and parent acceptance
@@ -247,2 +251,4 @@
 
+**Proposed financial evidence boundary (unadopted).** The financial observation extension SHALL remain design-only until independently adopted, and adoption SHALL remain separate from implementation and behavior. Its author inspected implementation source; source agreement supplies neither independent behavior proof nor goldens. Existing financial-pair and normal Transfer aggregate observations SHALL keep their exact scope without inferred complete raw Step.value/statement or ordinary simulate-byte coverage. Original proposals, freezes, failures, earlier five-kind adoption and pending normal-report cases SHALL remain unchanged. No parent acceptance, denominator or native/target result follows; all U0–U7, M1–M17, MC/SP/G and full financial/composition obligations remain. No conservation, soundness, authorization, authentication, proof or security condition is decided; security judgments and hostile/security tests remain with the designated security owner.
+
 ### Requirement: UNI-022 Target-separated requirement reconciliation


--- FILE SOURCE-IDENTITY.json ---
{
  "status": "unadopted-source-exposed-design-candidate",
  "checkout": "/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017",
  "date": "2026-10-18",
  "git_identity": "not-inspected; no Git invocation",
  "base_sources": [
    {
      "path": "AGENTS.md",
      "bytes": 20644,
      "sha256": "f058c13f2ff40192dc55a7ef729dfcd02291494c649797bdc132ed4cc8ee163a"
    },
    {
      "path": "docs/MORIARTY-PRODUCT-CONTRACT.md",
      "bytes": 12394,
      "sha256": "d5674c650741103284e0aa8cded4c0e8a0a1be38faa0c65aedb043710caa4396"
    },
    {
      "path": "RUST_GUIDELINES.md",
      "bytes": 16438,
      "sha256": "efe1561f2584da23b9d2b00f82876170ed7d3643dc0e6ff1da165a81e9ded46f"
    },
    {
      "path": "ROADMAP.md",
      "bytes": 32564,
      "sha256": "a28cc6383047e4ee4c850534a1077fc0444aebc6556de357922ee14a578b0054"
    },
    {
      "path": "docs/CORE-EXECUTION-REPORTS.md",
      "bytes": 10951,
      "sha256": "b5f6d32235151daae8037e53c091301a8914d698459960cd4a6da0d95e97de95"
    },
    {
      "path": "openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md",
      "bytes": 23955,
      "sha256": "5d3a579e29f2a03eb81d03c1c36da4e8ab8ca1f3e8c199d88838513ac0b34653"
    },
    {
      "path": "crates/mori-core/README.md",
      "bytes": 11482,
      "sha256": "0d91facda4bcf06e545af2f9ced7c72b2b76220a9b53eb46a5a4caea66ebc4dc"
    },
    {
      "path": "crates/mori-core/src/ir.rs",
      "bytes": 12823,
      "sha256": "0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d"
    },
    {
      "path": "crates/mori-core/src/parser.rs",
      "bytes": 15275,
      "sha256": "587e8b024ae16d513c5e6d6ef4929e4140bbfc7d2a716b5d38781645cb7c7275"
    },
    {
      "path": "crates/mori-core/src/runtime.rs",
      "bytes": 37871,
      "sha256": "eceac176f673180e83533ce55a99afa5bdf3f97a548bbb1c463dac3f84c83930"
    },
    {
      "path": "crates/mori-core/tests/financial_pair.rs",
      "bytes": 8563,
      "sha256": "a595526d917ff8cd45845bc1214ae7cbc25b428797ab7bb6ca92161b11af44a4"
    },
    {
      "path": "crates/mori/tests/core_test_report.rs",
      "bytes": 10521,
      "sha256": "74e021ed57129647354f3ec226d5d3acdee314091d9ed77870b67c3b76ac2387"
    }
  ],
  "patch_sha256": "03f868a9702c456fe2992c9268f75d43e2bd6ae61db9c5c5be4cef632ead03fc",
  "candidate_files": [
    {
      "path": "docs/CORE-EXECUTION-REPORTS.md",
      "base_sha256": "b5f6d32235151daae8037e53c091301a8914d698459960cd4a6da0d95e97de95",
      "candidate_sha256": "c1a05aadfd41410f81b9e2142e88e288d5b87b4be2a7bb3f213f178cbe404c80",
      "added_bytes": 7963
    },
    {
      "path": "openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md",
      "base_sha256": "5d3a579e29f2a03eb81d03c1c36da4e8ab8ca1f3e8c199d88838513ac0b34653",
      "candidate_sha256": "fe3900625f664dcb37d3fa4d294ee65c49b9528893adc94059a623674fe7ec1b",
      "added_bytes": 3151
    }
  ]
}


--- FILE baseline/crates/mori-core/src/runtime.rs ---
use crate::policy::{CostPlane, Meter};
use crate::*;
use std::collections::{BTreeMap, BTreeSet};

fn failure(code: &str, message: impl Into<String>) -> Error {
    Error::new(code, message)
}
pub(crate) fn identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    matches!(bytes.next(), Some(b'A'..=b'Z' | b'a'..=b'z'))
        && name.len() <= 64
        && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
        && !matches!(
            name,
            "language"
                | "agreement"
                | "state"
                | "asset"
                | "action"
                | "input"
                | "let"
                | "next"
                | "requires"
                | "ensures"
                | "emit"
                | "transfer"
                | "fee"
                | "liability"
                | "discharge"
                | "true"
                | "false"
                | "and"
                | "or"
                | "not"
                | "UInt"
                | "Bool"
        )
}
fn names<'a>(values: impl Iterator<Item = &'a str>, kind: &str) -> Result<(), Error> {
    let mut seen = BTreeSet::new();
    for name in values {
        if !identifier(name) || !seen.insert(name) {
            return Err(failure(
                "duplicate",
                format!("Invalid or duplicate {kind} name {name:?}."),
            ));
        }
    }
    Ok(())
}
pub(crate) fn expr_type(
    program: &Program,
    id: ExprId,
    env: &BTreeMap<String, Type>,
    depth: usize,
) -> Result<Type, Error> {
    type_cached(
        program,
        id,
        env,
        depth,
        &mut vec![None; program.expressions.len()],
    )
}
fn type_cached(
    program: &Program,
    id: ExprId,
    env: &BTreeMap<String, Type>,
    depth: usize,
    cache: &mut [Option<Type>],
) -> Result<Type, Error> {
    if depth > 64 {
        return Err(failure("bound", "Expression depth exceeds 64."));
    }
    let index = usize::from(id);
    let node = program
        .expressions
        .get(index)
        .ok_or_else(|| failure("expression", "Expression index out of bounds."))?;
    if let Some(ty) = cache[index] {
        return Ok(ty);
    }
    let result = match node {
        Expr::LiabilityPresent { .. } => Ok(Type::Bool),
        Expr::LiabilityAmount { .. } => Ok(Type::UInt),
        Expr::Literal { value } => Ok(value.ty()),
        Expr::Name { name } => env
            .get(name)
            .copied()
            .ok_or_else(|| failure("name", format!("Unknown or forward name {name:?}."))),
        Expr::Not { value } => {
            if type_cached(program, *value, env, depth + 1, cache)? != Type::Bool {
                return Err(failure("type", "not requires Bool."));
            }
            Ok(Type::Bool)
        }
        Expr::Binary { op, lhs, rhs } => {
            let a = type_cached(program, *lhs, env, depth + 1, cache)?;
            let b = type_cached(program, *rhs, env, depth + 1, cache)?;
            let expected = match op {
                BinaryOp::And | BinaryOp::Or => Type::Bool,
                _ => Type::UInt,
            };
            if a != b || (!matches!(op, BinaryOp::Eq | BinaryOp::Ne) && a != expected) {
                return Err(failure("type", "Binary operands have incompatible types."));
            }
            Ok(
                if matches!(op, BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul) {
                    Type::UInt
                } else {
                    Type::Bool
                },
            )
        }
    }?;
    cache[index] = Some(result);
    Ok(result)
}
/// Validates even manually constructed/deserialized IR before interpretation.
pub fn validate(program: &Program) -> Result<(), Error> {
    if program.language != LANGUAGE || program.semantics != SEMANTICS {
        return Err(failure(
            "profile",
            "Unsupported language or semantics identity.",
        ));
    }
    if !identifier(&program.name) {
        return Err(failure(
            "name",
            "Agreement name must be a non-reserved ASCII identifier of at most 64 bytes.",
        ));
    }
    if program.actions.is_empty() {
        return Err(failure("program", "Program needs at least one action."));
    }
    if program.states.len() + program.assets.len() + program.actions.len() > MAX_DECLARATIONS
        || program.expressions.len() > MAX_EXPRESSIONS
    {
        return Err(failure(
            "bound",
            "Program exceeds finite declaration/expression bounds.",
        ));
    }
    names(
        program
            .states
            .iter()
            .map(|s| s.name.as_str())
            .chain(program.assets.iter().map(|a| a.name.as_str())),
        "declaration",
    )?;
    names(program.actions.iter().map(|a| a.name.as_str()), "action")?;
    let mut identities = BTreeSet::new();
    for asset in &program.assets {
        let id = &asset.identity;
        if id.network.is_empty()
            || id.issuer.is_empty()
            || id.reference.is_empty()
            || id.network.len() > 1024
            || id.issuer.len() > 1024
            || id.reference.len() > 1024
            || id.scale > 38
            || !identities.insert(id)
        {
            return Err(failure(
                "asset",
                "Asset identities must be complete, unique and use scale 0..38.",
            ));
        }
        if program.assets.iter().any(|other| {
            other.identity.network == id.network
                && other.identity.issuer == id.issuer
                && other.identity.reference == id.reference
                && other.identity.scale != id.scale
        }) {
            return Err(failure(
                "asset",
                "Same asset identity cannot have conflicting scales.",
            ));
        }
    }
    let mut depths = Vec::with_capacity(program.expressions.len());
    for (i, e) in program.expressions.iter().enumerate() {
        let refs = match e {
            Expr::Binary { lhs, rhs, .. } => vec![*lhs, *rhs],
            Expr::Not { value } => vec![*value],
            _ => vec![],
        };
        let mut depth = 0;
        for r in refs {
            if usize::from(r) >= i {
                return Err(failure(
                    "expression",
                    "Expression references must point to earlier nodes; cycles are forbidden.",
                ));
            }
            depth = depth.max(depths[usize::from(r)] + 1);
        }
        if depth > 64 {
            return Err(failure("bound", "Expression depth exceeds 64."));
        }
        depths.push(depth);
    }
    let state_types: BTreeMap<_, _> = program
        .states
        .iter()
        .map(|s| (s.name.clone(), s.ty))
        .collect();
    for state in &program.states {
        if state.initial.ty() != state.ty {
            return Err(failure(
                "type",
                format!("Initial state {:?} has wrong type.", state.name),
            ));
        }
    }
    for action in &program.actions {
        if action.inputs.len() > MAX_DECLARATIONS || action.statements.len() > MAX_STATEMENTS {
            return Err(failure(
                "bound",
                "Action exceeds input or statement bounds.",
            ));
        }
        names(action.inputs.iter().map(|i| i.name.as_str()), "input")?;
        let mut env = state_types.clone();
        for input in &action.inputs {
            if env.insert(input.name.clone(), input.ty).is_some()
                || program.assets.iter().any(|a| a.name == input.name)
            {
                return Err(failure("duplicate", "Input cannot shadow a declaration."));
            }
        }
        for statement in &action.statements {
            match statement {
                // Same identity as `stage::require_core1`; only an in-memory
                // program can carry Await here, since `/1` JSON cannot.
                Statement::Await { .. } => {
                    return Err(failure("profile", "Await requires moriarty-core/2."));
                }
                Statement::Let { name, value } => {
                    let ty = expr_type(program, *value, &env, 0)?;
                    if !identifier(name)
                        || env.insert(name.clone(), ty).is_some()
                        || program.assets.iter().any(|a| a.name == *name)
                    {
                        return Err(failure("duplicate", "let cannot shadow an existing name."));
                    }
                }
                Statement::Requires { predicate } | Statement::Ensures { predicate } => {
                    if expr_type(program, *predicate, &env, 0)? != Type::Bool {
                        return Err(failure("type", "Predicate must have type Bool."));
                    }
                }
                Statement::Next { state, value } => {
                    let expected = state_types
                        .get(state)
                        .ok_or_else(|| failure("name", format!("Unknown state {state:?}.")))?;
                    if expr_type(program, *value, &env, 0)? != *expected {
                        return Err(failure("type", "State update has wrong type."));
                    }
                }
                Statement::Emit { name, value } => {
                    if name.is_empty() || name.len() > 1024 {
                        return Err(failure("name", "Event name must contain 1..1024 bytes."));
                    }
                    expr_type(program, *value, &env, 0)?;
                }
                Statement::Transfer {
                    asset,
                    from,
                    to,
                    amount,
                }
                | Statement::Fee {
                    asset,
                    from,
                    to,
                    amount,
                } => {
                    validate_asset(program, asset)?;
                    if from.is_empty() || to.is_empty() || from.len() > 1024 || to.len() > 1024 {
                        return Err(failure("account", "Account identities cannot be empty."));
                    }
                    require_uint(program, *amount, &env)?;
                }
                Statement::Liability {
                    name,
                    asset,
                    debtor,
                    creditor,
                    amount,
                } => {
                    validate_asset(program, asset)?;
                    if name.is_empty()
                        || debtor.is_empty()
                        || creditor.is_empty()
                        || name.len() > 1024
                        || debtor.len() > 1024
                        || creditor.len() > 1024
                    {
                        return Err(failure(
                            "liability",
                            "Liability identities cannot be empty.",
                        ));
                    }
                    require_uint(program, *amount, &env)?;
                }
                Statement::Discharge { name, amount } => {
                    if name.is_empty() || name.len() > 1024 {
                        return Err(failure(
                            "liability",
                            "Liability name must contain 1..1024 bytes.",
                        ));
                    }
                    require_uint(program, *amount, &env)?;
                }
            }
        }
    }
    validate_liability_reads(program)?;
    Ok(())
}
// New reads require a source-fixed identity. Legacy Core external duties and
// discharge-only programs keep their existing validation rules.
pub(crate) fn validate_liability_reads(p: &Program) -> Result<(), Error> {
    let mut reads = BTreeMap::new();
    for expr in &p.expressions {
        if let Expr::LiabilityPresent { name } | Expr::LiabilityAmount { name } = expr {
            if name.is_empty() || name.len() > 1024 {
                return Err(failure(
                    "liability",
                    "Liability read name must contain 1..1024 bytes.",
                ));
            }
            reads.insert(name.as_str(), None);
        }
    }
    for stmt in p.actions.iter().flat_map(|a| &a.statements) {
        if let Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            ..
        } = stmt
            && let Some(found) = reads.get_mut(name.as_str())
        {
            let identity = (
                validate_asset(p, asset)?,
                debtor.as_str(),
                creditor.as_str(),
            );
            if found.is_some_and(|previous| previous != identity) {
                return Err(failure(
                    "liability",
                    format!("Conflicting source liability identity for {name:?}."),
                ));
            }
            *found = Some(identity);
        }
    }
    for (name, identity) in reads {
        if identity.is_none() {
            return Err(failure(
                "liability",
                format!("Liability read {name:?} has no source-derived identity."),
            ));
        }
    }
    Ok(())
}

pub(crate) fn require_uint(
    p: &Program,
    id: ExprId,
    env: &BTreeMap<String, Type>,
) -> Result<(), Error> {
    if expr_type(p, id, env, 0)? != Type::UInt {
        Err(failure(
            "type",
            "Financial amount requires UInt atomic units.",
        ))
    } else {
        Ok(())
    }
}
pub(crate) fn validate_asset<'a>(p: &'a Program, name: &str) -> Result<&'a AssetId, Error> {
    p.assets
        .iter()
        .find(|a| a.name == name)
        .map(|a| &a.identity)
        .ok_or_else(|| failure("asset", format!("Unknown asset {name:?}.")))
}

fn evaluate(
    p: &Program,
    id: ExprId,
    env: &BTreeMap<String, Value>,
    snapshot: &Snapshot,
    meter: &mut Option<&mut Meter>,
) -> Result<Value, Error> {
    eval_cached(
        p,
        id,
        env,
        snapshot,
        &mut vec![None; p.expressions.len()],
        meter,
    )
}
fn charge_logical(meter: &mut Option<&mut Meter>) -> Result<(), Error> {
    if let Some(meter) = meter.as_deref_mut() {
        meter.charge(CostPlane::Logical, UInt(1))?;
    }
    Ok(())
}
fn eval_cached(
    p: &Program,
    id: ExprId,
    env: &BTreeMap<String, Value>,
    snapshot: &Snapshot,
    cache: &mut [Option<Value>],
    meter: &mut Option<&mut Meter>,
) -> Result<Value, Error> {
    let i = usize::from(id);
    if let Some(value) = &cache[i] {
        return Ok(value.clone());
    }
    // Charge the attempted node before its children or operation. Cache hits
    // above remain free, and evaluate creates a new cache for each statement.
    charge_logical(meter)?;
    let result = match &p.expressions[i] {
        Expr::LiabilityPresent { name } | Expr::LiabilityAmount { name } => {
            let row = snapshot
                .liabilities
                .binary_search_by(|l| l.name.as_str().cmp(name.as_str()))
                .ok()
                .map(|index| &snapshot.liabilities[index]);
            if matches!(&p.expressions[i], Expr::LiabilityPresent { .. }) {
                Value::Bool(row.is_some())
            } else {
                Value::UInt(row.map_or(UInt(0), |l| l.amount))
            }
        }
        Expr::Literal { value } => value.clone(),
        Expr::Name { name } => env
            .get(name)
            .cloned()
            .ok_or_else(|| failure("name", format!("Unknown name {name:?}.")))?,
        Expr::Not { value } => match eval_cached(p, *value, env, snapshot, cache, meter)? {
            Value::Bool(b) => Value::Bool(!b),
            _ => return Err(failure("type", "not expects Bool.")),
        },
        Expr::Binary { op, lhs, rhs } => {
            let a = eval_cached(p, *lhs, env, snapshot, cache, meter)?;
            let b = eval_cached(p, *rhs, env, snapshot, cache, meter)?;
            match (op, a, b) {
                (BinaryOp::Add, Value::UInt(UInt(a)), Value::UInt(UInt(b))) => {
                    Value::UInt(UInt(a.checked_add(b).ok_or_else(|| {
                        failure("overflow", "UInt addition exceeds u128.")
                    })?))
                }
                (BinaryOp::Sub, Value::UInt(UInt(a)), Value::UInt(UInt(b))) => {
                    Value::UInt(UInt(a.checked_sub(b).ok_or_else(|| {
                        failure("underflow", "UInt subtraction is negative.")
                    })?))
                }
                (BinaryOp::Mul, Value::UInt(UInt(a)), Value::UInt(UInt(b))) => {
                    Value::UInt(UInt(a.checked_mul(b).ok_or_else(|| {
                        failure("overflow", "UInt multiplication exceeds u128.")
                    })?))
                }
                (BinaryOp::Eq, a, b) => Value::Bool(a == b),
                (BinaryOp::Ne, a, b) => Value::Bool(a != b),
                (BinaryOp::Lt, Value::UInt(a), Value::UInt(b)) => Value::Bool(a < b),
                (BinaryOp::Le, Value::UInt(a), Value::UInt(b)) => Value::Bool(a <= b),
                (BinaryOp::Gt, Value::UInt(a), Value::UInt(b)) => Value::Bool(a > b),
                (BinaryOp::Ge, Value::UInt(a), Value::UInt(b)) => Value::Bool(a >= b),
                (BinaryOp::And, Value::Bool(a), Value::Bool(b)) => Value::Bool(a && b),
                (BinaryOp::Or, Value::Bool(a), Value::Bool(b)) => Value::Bool(a || b),
                _ => return Err(failure("type", "Invalid operand types.")),
            }
        }
    };
    cache[i] = Some(result.clone());
    Ok(result)
}
fn amount(
    p: &Program,
    id: ExprId,
    env: &BTreeMap<String, Value>,
    snapshot: &Snapshot,
    meter: &mut Option<&mut Meter>,
) -> Result<UInt, Error> {
    match evaluate(p, id, env, snapshot, meter)? {
        Value::UInt(v) => Ok(v),
        _ => Err(failure("type", "Expected UInt amount.")),
    }
}
// Core also supports incoming duties without a source declaration. Only names
// declared by Liability statements have a source-fixed identity; do not impose
// the backend's closed, sixteen-entry compiler profile on general Core input.
fn liability_identities(p: &Program) -> Result<BTreeMap<&str, (&AssetId, &str, &str)>, Error> {
    let mut identities = BTreeMap::new();
    for stmt in p.actions.iter().flat_map(|action| &action.statements) {
        if let Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            ..
        } = stmt
        {
            let identity = (
                validate_asset(p, asset)?,
                debtor.as_str(),
                creditor.as_str(),
            );
            if let Some(existing) = identities.insert(name.as_str(), identity)
                && existing != identity
            {
                return Err(failure(
                    "liability",
                    format!("Conflicting source liability identity for {name:?}."),
                ));
            }
        }
    }
    Ok(identities)
}
pub(crate) fn scenario_snapshot(p: &Program, a: &Action, s: &Scenario) -> Result<Snapshot, Error> {
    if s.balances.len() > 4096 || s.liabilities.len() > 1024 {
        return Err(failure(
            "bound",
            "Scenario exceeds 4,096 balances or 1,024 liabilities.",
        ));
    }
    let added_liabilities = a
        .statements
        .iter()
        .filter(|stmt| matches!(stmt, Statement::Liability { .. }))
        .count();
    let trace_records =
        p.states.len() + s.balances.len() + s.liabilities.len() + added_liabilities + 1;
    if trace_records.saturating_mul(a.statements.len() + 2) > MAX_TRACE_CELLS {
        return Err(failure(
            "bound",
            "Scenario and action exceed 65,536 trace snapshot records.",
        ));
    }
    if !s.state.is_empty() && s.state.len() != p.states.len() {
        return Err(failure(
            "state",
            "Provide either initial state {} or every declared state value.",
        ));
    }
    let mut state = BTreeMap::new();
    for d in &p.states {
        let value = if s.state.is_empty() {
            d.initial.clone()
        } else {
            s.state
                .get(&d.name)
                .ok_or_else(|| failure("state", format!("Missing state {:?}.", d.name)))?
                .clone()
        };
        if value.ty() != d.ty {
            return Err(failure(
                "state",
                format!("Wrong type for state {:?}.", d.name),
            ));
        }
        state.insert(d.name.clone(), value);
    }
    if s.inputs.len() != a.inputs.len() {
        return Err(failure(
            "input",
            "Scenario must provide exactly the selected action's inputs.",
        ));
    }
    for input in &a.inputs {
        if s.inputs.get(&input.name).map(Value::ty) != Some(input.ty) {
            return Err(failure(
                "input",
                format!("Missing or wrong type for input {:?}.", input.name),
            ));
        }
    }
    let ids: BTreeSet<_> = p.assets.iter().map(|a| a.identity.clone()).collect();
    let mut keys = BTreeSet::new();
    let mut balances = s.balances.clone();
    for b in &balances {
        if !ids.contains(&b.asset) {
            return Err(failure(
                "asset",
                format!(
                    "Undeclared exact asset identity for account {:?}.",
                    b.account
                ),
            ));
        }
        if b.account.is_empty()
            || b.account.len() > 1024
            || !keys.insert((b.asset.clone(), b.account.clone()))
        {
            return Err(failure(
                "balance",
                "Duplicate or invalid balance account/asset.",
            ));
        }
    }
    for stmt in &a.statements {
        if let Statement::Transfer {
            asset, from, to, ..
        }
        | Statement::Fee {
            asset, from, to, ..
        } = stmt
        {
            let id = validate_asset(p, asset)?;
            for account in [from, to] {
                if !keys.contains(&(id.clone(), account.clone())) {
                    return Err(failure(
                        "balance",
                        format!("Missing explicit balance for {account:?} and asset {asset:?}."),
                    ));
                }
            }
        }
    }
    balances.sort_by(|a, b| (&a.asset, &a.account).cmp(&(&b.asset, &b.account)));
    let identities = liability_identities(p)?;
    let mut liabilities = s.liabilities.clone();
    let mut names = BTreeSet::new();
    for l in &liabilities {
        if !ids.contains(&l.asset) {
            return Err(failure(
                "asset",
                "Liability uses undeclared exact asset identity.",
            ));
        }
        if l.name.is_empty()
            || l.name.len() > 1024
            || l.debtor.is_empty()
            || l.creditor.is_empty()
            || l.debtor.len() > 1024
            || l.creditor.len() > 1024
            || !names.insert(l.name.clone())
        {
            return Err(failure(
                "liability",
                "Invalid or duplicate liability identity.",
            ));
        }
        if let Some(identity) = identities.get(l.name.as_str())
            && *identity != (&l.asset, l.debtor.as_str(), l.creditor.as_str())
        {
            return Err(failure(
                "liability",
                format!("Existing liability identity differs for {:?}.", l.name),
            ));
        }
    }
    liabilities.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(Snapshot {
        state,
        balances,
        liabilities,
    })
}
fn move_funds(
    snapshot: &mut Snapshot,
    id: &AssetId,
    from: &str,
    to: &str,
    amount: UInt,
) -> Result<(), Error> {
    let f = snapshot
        .balances
        .iter()
        .position(|b| b.asset == *id && b.account == from)
        .ok_or_else(|| failure("balance", "Missing source balance."))?;
    let t = snapshot
        .balances
        .iter()
        .position(|b| b.asset == *id && b.account == to)
        .ok_or_else(|| failure("balance", "Missing recipient balance."))?;
    let debit = snapshot.balances[f]
        .amount
        .0
        .checked_sub(amount.0)
        .ok_or_else(|| failure("balance", format!("Insufficient balance for {from:?}.")))?;
    if f != t {
        let credit = snapshot.balances[t]
            .amount
            .0
            .checked_add(amount.0)
            .ok_or_else(|| failure("overflow", "Recipient balance exceeds u128."))?;
        snapshot.balances[f].amount = UInt(debit);
        snapshot.balances[t].amount = UInt(credit);
    }
    Ok(())
}
/// Shared scalar statement execution for legacy actions and admitted segments.
fn execute_statement(
    program: &Program,
    stmt: &Statement,
    env: &mut BTreeMap<String, Value>,
    current: &mut Snapshot,
    meter: &mut Option<&mut Meter>,
    effects: &mut Vec<Effect>,
) -> Result<Option<Value>, Error> {
    charge_logical(meter)?;
    match stmt {
        Statement::Await { guard, .. } => {
            let v = evaluate(program, *guard, env, current, meter)?;
            Ok(Some(v))
        }
        Statement::Let { name, value } => {
            let v = evaluate(program, *value, env, current, meter)?;
            env.insert(name.clone(), v.clone());
            Ok(Some(v))
        }
        Statement::Requires { predicate } | Statement::Ensures { predicate } => {
            let v = evaluate(program, *predicate, env, current, meter)?;
            if v != Value::Bool(true) {
                return Err(failure(
                    if matches!(stmt, Statement::Requires { .. }) {
                        "requires"
                    } else {
                        "ensures"
                    },
                    "Action predicate evaluated to false.",
                ));
            }
            Ok(Some(v))
        }
        Statement::Next { state, value } => {
            let after = evaluate(program, *value, env, current, meter)?;
            let before = current.state[state].clone();
            current.state.insert(state.clone(), after.clone());
            env.insert(state.clone(), after.clone());
            effects.push(Effect::State {
                name: state.clone(),
                before,
                after: after.clone(),
            });
            Ok(Some(after))
        }
        Statement::Emit { name, value } => {
            let v = evaluate(program, *value, env, current, meter)?;
            effects.push(Effect::Emit {
                name: name.clone(),
                value: v.clone(),
            });
            Ok(Some(v))
        }
        Statement::Transfer {
            asset,
            from,
            to,
            amount: expr,
        }
        | Statement::Fee {
            asset,
            from,
            to,
            amount: expr,
        } => {
            let amount = amount(program, *expr, env, current, meter)?;
            let id = validate_asset(program, asset)?.clone();
            move_funds(current, &id, from, to, amount)?;
            effects.push(if matches!(stmt, Statement::Fee { .. }) {
                Effect::Fee {
                    asset: id,
                    from: from.clone(),
                    to: to.clone(),
                    amount,
                }
            } else {
                Effect::Transfer {
                    asset: id,
                    from: from.clone(),
                    to: to.clone(),
                    amount,
                }
            });
            Ok(Some(Value::UInt(amount)))
        }
        Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            amount: expr,
        } => {
            let amount = amount(program, *expr, env, current, meter)?;
            let id = validate_asset(program, asset)?.clone();
            let found = current.liabilities.iter().position(|l| l.name == *name);
            let before = if let Some(index) = found {
                let l = &current.liabilities[index];
                if l.asset != id || l.debtor != *debtor || l.creditor != *creditor {
                    return Err(failure("liability", "Existing liability identity differs."));
                }
                l.amount
            } else {
                UInt(0)
            };
            let after = UInt(
                before
                    .0
                    .checked_add(amount.0)
                    .ok_or_else(|| failure("overflow", "Liability exceeds u128."))?,
            );
            if let Some(index) = found {
                current.liabilities[index].amount = after;
            } else {
                current.liabilities.push(Liability {
                    name: name.clone(),
                    asset: id.clone(),
                    debtor: debtor.clone(),
                    creditor: creditor.clone(),
                    amount: after,
                });
                current.liabilities.sort_by(|a, b| a.name.cmp(&b.name));
            }
            effects.push(Effect::Liability {
                name: name.clone(),
                asset: id,
                debtor: debtor.clone(),
                creditor: creditor.clone(),
                before,
                after,
            });
            Ok(Some(Value::UInt(amount)))
        }
        Statement::Discharge { name, amount: expr } => {
            let amount = amount(program, *expr, env, current, meter)?;
            let l = current
                .liabilities
                .iter_mut()
                .find(|l| l.name == *name)
                .ok_or_else(|| {
                    failure("liability", format!("Unknown residual liability {name:?}."))
                })?;
            let before = l.amount;
            let after =
                UInt(before.0.checked_sub(amount.0).ok_or_else(|| {
                    failure("liability", "Discharge exceeds residual liability.")
                })?);
            l.amount = after;
            effects.push(Effect::Liability {
                name: name.clone(),
                asset: l.asset.clone(),
                debtor: l.debtor.clone(),
                creditor: l.creditor.clone(),
                before,
                after,
            });
            Ok(Some(Value::UInt(amount)))
        }
    }
}

/// Executes one finite action on a clone. A refusal exposes no committed effects.
pub fn simulate(program: &Program, action: &str, scenario: &Scenario) -> Result<Execution, Error> {
    simulate_with_meter(program, action, scenario, None)
}
pub(crate) fn simulate_with_meter(
    program: &Program,
    action: &str,
    scenario: &Scenario,
    mut meter: Option<&mut Meter>,
) -> Result<Execution, Error> {
    validate(program)?;
    let selected = program
        .actions
        .iter()
        .find(|a| a.name == action)
        .ok_or_else(|| failure("action", format!("Unknown action {action:?}.")))?;
    let pre = scenario_snapshot(program, selected, scenario)?;
    let mut current = pre.clone();
    let mut env = current.state.clone();
    env.extend(scenario.inputs.clone());
    let mut steps = vec![];
    let mut all_effects = vec![];
    for (i, stmt) in selected.statements.iter().enumerate() {
        let mut effects = vec![];
        let result = execute_statement(
            program,
            stmt,
            &mut env,
            &mut current,
            &mut meter,
            &mut effects,
        );
        let value = result.map_err(|mut e| {
            e.step = Some(i as u16);
            e
        })?;
        all_effects.extend(effects.clone());
        steps.push(Step {
            index: i as u16,
            statement: stmt.clone(),
            value,
            snapshot: current.clone(),
            effects,
        });
    }
    Ok(Execution {
        semantics: SEMANTICS.into(),
        action: action.into(),
        pre,
        post: current,
        steps,
        effects: all_effects,
    })
}
/// Environment seed supplied only by stage admission.
pub enum SegmentEntry<'a> {
    Start {
        inputs: &'a [crate::stage::NamedValue],
    },
    Resume {
        site: u16,
        captures: &'a [crate::stage::NamedValue],
        event: &'a [crate::stage::NamedValue],
    },
}

pub enum SegmentRun {
    NotAdvanced,
    Ran {
        execution: Box<Execution>,
        exit_site: u16,
        captures_out: Vec<crate::stage::NamedValue>,
    },
}

/// Runs an already-admitted finite source segment with the same scalar executor
/// as legacy simulation. The caller owns admission, publication and work policy.
pub(crate) fn run_segment(
    sp: &crate::stage::StagedProgram,
    action: usize,
    entry: SegmentEntry<'_>,
    boundary: Snapshot,
    meter: &mut Meter,
) -> Result<SegmentRun, Error> {
    let program = sp.program();
    let selected = &program.actions[action];
    let plan = &sp.plan().actions[action];
    let pre = boundary;
    let mut current = pre.clone();
    let mut env = pre.state.clone();
    let mut meter = Some(meter);
    let mut steps = Vec::new();
    let mut all_effects = Vec::new();
    let begin = match entry {
        SegmentEntry::Start { inputs } => {
            env.extend(inputs.iter().map(|v| (v.name.clone(), v.value.clone())));
            0
        }
        SegmentEntry::Resume {
            site,
            captures,
            event,
        } => {
            env.extend(
                captures
                    .iter()
                    .chain(event)
                    .map(|v| (v.name.clone(), v.value.clone())),
            );
            let ordinal = usize::from(plan.sites[usize::from(site) - 1].ordinal);
            let statement = &selected.statements[ordinal];
            let mut effects = Vec::new();
            // execute_statement charges dispatch once, then visits the guard.
            let value = execute_statement(
                program,
                statement,
                &mut env,
                &mut current,
                &mut meter,
                &mut effects,
            )
            .map_err(|mut error| {
                error.step = Some(ordinal as u16);
                error
            })?;
            if value != Some(Value::Bool(true)) {
                return Ok(SegmentRun::NotAdvanced);
            }
            steps.push(Step {
                index: ordinal as u16,
                statement: statement.clone(),
                value,
                snapshot: pre.clone(),
                effects,
            });
            ordinal + 1
        }
    };
    let mut exit_site = 0;
    let mut captures_out = Vec::new();
    for (ordinal, statement) in selected.statements.iter().enumerate().skip(begin) {
        if matches!(statement, Statement::Await { .. }) {
            // Entry charge is attempted now; guard and its Step belong to resume.
            charge_logical(&mut meter).map_err(|mut error| {
                error.step = Some(ordinal as u16);
                error
            })?;
            let site = plan
                .sites
                .iter()
                .find(|s| usize::from(s.ordinal) == ordinal)
                .ok_or_else(|| failure("stage_site", "Await site is absent from stage plan."))?;
            exit_site = site.site;
            for slot in &site.captures {
                let value = env
                    .get(&slot.name)
                    .cloned()
                    .ok_or_else(|| failure("name", format!("Unknown name {:?}.", slot.name)))?;
                captures_out.push(crate::stage::NamedValue {
                    name: slot.name.clone(),
                    value,
                });
            }
            break;
        }
        let mut effects = Vec::new();
        let value = execute_statement(
            program,
            statement,
            &mut env,
            &mut current,
            &mut meter,
            &mut effects,
        )
        .map_err(|mut error| {
            error.step = Some(ordinal as u16);
            error
        })?;
        all_effects.extend(effects.clone());
        steps.push(Step {
            index: ordinal as u16,
            statement: statement.clone(),
            value,
            snapshot: current.clone(),
            effects,
        });
    }
    Ok(SegmentRun::Ran {
        execution: Box::new(Execution {
            semantics: program.semantics.clone(),
            action: selected.name.clone(),
            pre,
            post: current,
            steps,
            effects: all_effects,
        }),
        exit_site,
        captures_out,
    })
}

/// Recomputes all steps and rejects at the first deterministic divergence.
pub fn replay(
    program: &Program,
    action: &str,
    scenario: &Scenario,
    expected: &Execution,
) -> Result<(), Error> {
    let actual = simulate(program, action, scenario)?;
    if actual.semantics != expected.semantics
        || actual.action != expected.action
        || actual.pre != expected.pre
    {
        return Err(failure(
            "replay",
            "Execution identity or predecessor differs.",
        ));
    }
    for i in 0..actual.steps.len().max(expected.steps.len()) {
        if actual.steps.get(i) != expected.steps.get(i) {
            let mut e = failure("replay", format!("First divergence at step {i}."));
            e.step = Some(i as u16);
            return Err(e);
        }
    }
    if actual.effects != expected.effects || actual.post != expected.post {
        return Err(failure(
            "replay",
            "Complete effect sequence or poststate differs.",
        ));
    }
    Ok(())
}

