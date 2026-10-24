//! Scalar/2 typed-value adapter. Every ledger decision remains in Moriarty.
use super::*;
use crate::sherlock::helper_machine::shared::owner_ext_t3_uses::ExtValue;
use crate::sherlock::helper_machine::shared::owner_ext_type_builder::TypeExprSrc;
use crate::sherlock::helper_machine::shared::{CoreValue, i128_class::KIntC};
use crate::sherlock::nominal_ast_inputs::{NominalAstArgument, NominalAstBindingRequest};
use crate::sherlock_compile_artifact::nominal_body_compile::compile_nominal_source_host_bounded_with_ast_inputs;

pub const LEDGER_AST_ADAPTER_PROFILE: &str = "sherlock-selected-note-ledger-ast/1";
pub struct PreparedSelectedLedgerAst {
    original: Value,
    typed: SelectedLedgerInput,
    library: String,
    document_sha256: [u8; 32],
}
#[derive(Debug)]
pub enum SelectedLedgerAstCompileError {
    Input(SelectedLedgerInputError),
    Compile(NominalBodyCompileFailure),
}
/// Representation-only preparation; no seed planner, validity predicate,
/// evaluator or source interpolation. Source is bounded-parsed during compile.
pub fn prepare_selected_ledger_ast(
    original: Value,
    library: &str,
) -> Result<PreparedSelectedLedgerAst, SelectedLedgerInputError> {
    let typed = decode_selected_ledger(original.clone())?;
    let bytes = serde_json::to_vec(&original).map_err(|e| SelectedLedgerInputError::Shape {
        detail: e.to_string(),
    })?;
    Ok(PreparedSelectedLedgerAst {
        original,
        typed,
        library: library.into(),
        document_sha256: Sha256::digest(&bytes).into(),
    })
}
impl PreparedSelectedLedgerAst {
    pub fn original(&self) -> &Value {
        &self.original
    }
    pub fn typed(&self) -> &SelectedLedgerInput {
        &self.typed
    }
    pub fn library_source(&self) -> &str {
        &self.library
    }
    pub fn document_sha256(&self) -> &[u8; 32] {
        &self.document_sha256
    }
    /// Full visible constants, not an expected answer or fixed-key private input.
    pub fn binding_request(&self) -> Result<NominalAstBindingRequest, SelectedLedgerInputError> {
        Ok(NominalAstBindingRequest {
            function_name: "check_ledger".into(),
            arguments: vec![NominalAstArgument {
                name: "input".into(),
                value: self.typed.ast()?,
            }],
        })
    }
    pub fn compile(
        &self,
        budget: DraftParseBudget,
        config: ExtCompileConfig,
        locator: Option<LocatorC>,
    ) -> Result<NominalBodyCompileArtifact, SelectedLedgerAstCompileError> {
        let request = self
            .binding_request()
            .map_err(SelectedLedgerAstCompileError::Input)?;
        compile_nominal_source_host_bounded_with_ast_inputs(
            &self.library,
            budget,
            config,
            locator,
            request,
        )
        .map_err(SelectedLedgerAstCompileError::Compile)
    }
}
trait Ast {
    fn ast(&self) -> Result<ExtValue, SelectedLedgerInputError>;
}
fn ty(name: &str) -> TypeExprSrc {
    match name {
        "UInt" => TypeExprSrc::UInt,
        "Bool" => TypeExprSrc::Bool,
        _ => TypeExprSrc::Named { name: name.into() },
    }
}
fn ctor(ty: TypeExprSrc, name: &str, fields: Vec<ExtValue>) -> ExtValue {
    ExtValue::Constructor {
        ty,
        ctor: name.into(),
        fields,
    }
}
fn record(name: &str, fields: &[&dyn Ast]) -> Result<ExtValue, SelectedLedgerInputError> {
    Ok(ctor(
        ty(name),
        name,
        fields
            .iter()
            .map(|x| x.ast())
            .collect::<Result<Vec<_>, _>>()?,
    ))
}
impl Ast for u128 {
    fn ast(&self) -> Result<ExtValue, SelectedLedgerInputError> {
        Ok(ExtValue::Core {
            value: CoreValue::UInt {
                n: KIntC::InRange(*self),
            },
        })
    }
}
impl Ast for bool {
    fn ast(&self) -> Result<ExtValue, SelectedLedgerInputError> {
        Ok(ExtValue::Core {
            value: CoreValue::Bool { b: *self },
        })
    }
}
impl Ast for String {
    fn ast(&self) -> Result<ExtValue, SelectedLedgerInputError> {
        let (len, w) = packed_text(self)?;
        Ok(ctor(
            ty("Text"),
            "Utf8",
            vec![
                (len as u128).ast()?,
                w[0].ast()?,
                w[1].ast()?,
                w[2].ast()?,
                w[3].ast()?,
            ],
        ))
    }
}
impl Ast for Option<u128> {
    fn ast(&self) -> Result<ExtValue, SelectedLedgerInputError> {
        let t = TypeExprSrc::Apply {
            name: "Maybe".into(),
            args: vec![TypeExprSrc::UInt],
        };
        Ok(match self {
            Some(n) => ctor(t, "Some", vec![n.ast()?]),
            None => ctor(t, "None", vec![]),
        })
    }
}
impl<T: Ast> Ast for Seq<'_, T> {
    fn ast(&self) -> Result<ExtValue, SelectedLedgerInputError> {
        let seq = || TypeExprSrc::Apply {
            name: "Seq".into(),
            args: vec![ty(self.0)],
        };
        let mut result = ctor(seq(), "Empty", vec![]);
        for value in self.1.iter().rev() {
            result = ctor(seq(), "Link", vec![value.ast()?, result]);
        }
        Ok(result)
    }
}
macro_rules! ast_record {
    ($ty:ty,$name:literal,$($field:ident),+)=>{impl Ast for $ty {
        fn ast(&self)->Result<ExtValue,SelectedLedgerInputError>{record($name,&[$(&self.$field),+])}
    }};
}
ast_record!(Asset, "Asset", network, issuer, reference, scale);
ast_record!(
    Config,
    "Config",
    seed,
    headers,
    batch_headers,
    tx_per_batch,
    shoppers,
    merchants,
    genesis_notes_per_shopper
);
ast_record!(
    Counts,
    "Counts",
    cm_count,
    nf_count,
    rh_count,
    wd_count,
    height,
    tx_count,
    dep_consumed,
    fee_total
);
ast_record!(
    Note,
    "Note",
    reference,
    owner,
    value,
    created_height,
    asset,
    spent
);
ast_record!(
    Account,
    "Account",
    asset,
    native_token_id,
    genesis,
    deposited,
    withdrawn,
    fees,
    unspent
);
impl Ast for Transaction {
    fn ast(&self) -> Result<ExtValue, SelectedLedgerInputError> {
        record(
            "Tx",
            &[
                &self.id,
                &self.batch,
                &self.height,
                &self.slot,
                &self.pos,
                &self.kind,
                &self.kind_code,
                &self.asset,
                &self.spender,
                &Seq("Text", &self.inputs),
                &Seq("Note", &self.input_notes),
                &Seq("Note", &self.outputs),
                &self.sum_inputs,
                &self.sum_outputs,
                &self.fee,
                &self.deposit,
                &self.withdrawal,
                &self.withdrawal_destination,
                &self.state_before,
                &self.state_after,
            ],
        )
    }
}
impl Ast for Header {
    fn ast(&self) -> Result<ExtValue, SelectedLedgerInputError> {
        record(
            "Header",
            &[
                &self.height,
                &self.batch,
                &self.state_before,
                &self.state_after_transactions,
                &self.state_after,
                &Seq("Text", &self.tx_ids),
            ],
        )
    }
}
struct KindCountAst {
    name: String,
    count: u128,
}
ast_record!(KindCountAst, "KindCount", name, count);
impl Ast for Batch {
    fn ast(&self) -> Result<ExtValue, SelectedLedgerInputError> {
        let k = &self.kind_counts;
        let values = [
            k.checkout_change,
            k.checkout_exact,
            k.p2p,
            k.sweep,
            k.deposit,
            k.withdrawal,
        ];
        let kinds: Vec<_> = KINDS
            .iter()
            .zip(values)
            .map(|(name, count)| KindCountAst {
                name: (*name).into(),
                count,
            })
            .collect();
        record(
            "Batch",
            &[
                &self.batch_no,
                &self.h_first,
                &self.h_last,
                &self.state_pre,
                &self.state_post,
                &Seq("Text", &self.ordered_ids),
                &Seq("Text", &self.ordered_kinds),
                &Seq("KindCount", &kinds),
                &self.fee_sum,
                &self.deposit_sum,
                &self.withdrawal_sum,
            ],
        )
    }
}
impl Ast for SelectedLedgerInput {
    fn ast(&self) -> Result<ExtValue, SelectedLedgerInputError> {
        record(
            "LedgerInput",
            &[
                &self.config,
                &self.asset,
                &Seq("Note", &self.genesis_notes),
                &self.genesis_state,
                &Seq("Tx", &self.transactions),
                &Seq("Header", &self.headers),
                &Seq("Batch", &self.batches),
                &Seq("Note", &self.final_notes),
                &self.final_state,
                &Seq("Account", &self.accounting),
            ],
        )
    }
}
