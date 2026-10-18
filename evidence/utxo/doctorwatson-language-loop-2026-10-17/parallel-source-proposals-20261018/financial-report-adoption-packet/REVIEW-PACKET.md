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
