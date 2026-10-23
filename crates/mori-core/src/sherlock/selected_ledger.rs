//! Typed scalar/2 data -> ordinary nominal/1 source specialization.
//!
//! This adapter computes no ledger validity or expected effect. Every decision
//! is made by check_ledger in the supplied source, then existing body/owner APIs.
//! Source constants are public; native roots, signatures and finance are absent.
use crate::sherlock_compile_artifact::ext_compile::ExtCompileConfig;
use crate::sherlock_compile_artifact::nominal_body_compile::{
    NominalBodyCompileArtifact, NominalBodyCompileFailure,
    compile_nominal_source_host_bounded,
};
use crate::sherlock::helper_machine::shared::LocatorC;
use crate::sherlock_nominal_source::{
    parse_nominal_source_bounded, DraftError, DraftParseBudget, SourceOwner,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

pub const SCALAR_INPUT_SCHEMA: &str = "sherlock-fenrir-derived-public-scalar/2";
pub const LEDGER_ADAPTER_PROFILE: &str = "sherlock-selected-note-ledger-source/1";
const ROOT_REASON: &str = "Cryptographic root requires independent field-hash/tree oracle and native witness adaptation; no SHA-256 substitute.";
const KINDS: [&str;6] = ["checkout_change","checkout_exact","p2p","sweep","deposit","withdrawal"];

#[derive(Debug)]
pub enum SelectedLedgerInputError {
    Profile { field: &'static str, found: String },
    Shape { detail: String },
    RootMarker { field: &'static str },
    TextTooLong { bytes: usize, limit: usize },
    SourceBytes { observed: usize, limit: usize },
    LibraryParse(DraftError),
    EntrySpan,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Asset { pub network:String, pub issuer:String, pub reference:String, pub scale:u128 }
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub seed:u128, pub headers:u128, pub batch_headers:u128, pub tx_per_batch:u128,
    pub shoppers:u128, pub merchants:u128, pub genesis_notes_per_shopper:u128,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootMarker { pub value:Value, pub status:String, pub reason:String }
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Counts {
    pub cm_count:u128, pub nf_count:u128, pub rh_count:u128, pub wd_count:u128,
    pub height:u128, pub tx_count:u128, pub dep_consumed:u128, pub fee_total:u128,
    pub cm_root:RootMarker, pub nf_root:RootMarker, pub rh_root:RootMarker, pub wd_root:RootMarker,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Note {
    #[serde(rename="ref")] pub reference:String,
    pub owner:u128, pub value:u128, pub created_height:u128, pub asset:Asset, pub spent:bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transaction {
    pub id:String,pub batch:u128,pub height:u128,pub slot:u128,pub pos:u128,
    pub kind:String,pub kind_code:u128,pub asset:Asset,
    #[serde(deserialize_with="required_optional")] pub spender:Option<u128>,
    pub inputs:Vec<String>,pub input_notes:Vec<Note>,pub outputs:Vec<Note>,
    pub sum_inputs:u128,pub sum_outputs:u128,pub fee:u128,pub deposit:u128,
    pub withdrawal:u128,
    #[serde(deserialize_with="required_optional")] pub withdrawal_destination:Option<u128>,
    pub state_before:Counts,pub state_after:Counts,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Header {
    pub height:u128,pub batch:u128,pub state_before:Counts,
    pub state_after_transactions:Counts,pub state_after:Counts,pub tx_ids:Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KindCounts {
    pub checkout_change:u128,pub checkout_exact:u128,pub p2p:u128,
    pub sweep:u128,pub deposit:u128,pub withdrawal:u128,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Batch {
    pub batch_no:u128,pub h_first:u128,pub h_last:u128,pub state_pre:Counts,pub state_post:Counts,
    pub ordered_ids:Vec<String>,pub ordered_kinds:Vec<String>,pub kind_counts:KindCounts,
    pub fee_sum:u128,pub deposit_sum:u128,pub withdrawal_sum:u128,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Account {
    pub asset:Asset,pub native_token_id:u128,pub genesis:u128,pub deposited:u128,
    pub withdrawn:u128,pub fees:u128,pub unspent:u128,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectedLedgerInput {
    pub config:Config,pub asset:Asset,pub genesis_notes:Vec<Note>,pub genesis_state:Counts,
    pub transactions:Vec<Transaction>,pub headers:Vec<Header>,pub batches:Vec<Batch>,
    pub final_notes:Vec<Note>,pub final_state:Counts,pub accounting:Vec<Account>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request { schema:String,config:Config,asset:Asset }

/// Decode representation only. Negative semantic cases remain in the input.
/// Top-level provenance is retained by PreparedSelectedLedger, not certified.
pub fn decode_selected_ledger(document:Value)->Result<SelectedLedgerInput,SelectedLedgerInputError> {
    let profile=|field:&'static str,expected:&str,v:&Value| {
        if v.as_str()==Some(expected) { Ok(()) } else {
            Err(SelectedLedgerInputError::Profile {field,found:v.to_string()})
        }
    };
    profile("schema",SCALAR_INPUT_SCHEMA,&document["schema"])?;
    let request:Request=decode(&document,"request")?;
    profile("request.schema","fsyn-scalar-request/2",&Value::String(request.schema))?;
    let input=SelectedLedgerInput {
        config:request.config,asset:request.asset,genesis_notes:decode(&document,"genesis_notes")?,
        genesis_state:decode(&document,"genesis_state")?,transactions:decode(&document,"transactions")?,
        headers:decode(&document,"headers")?,batches:decode(&document,"batches")?,
        final_notes:decode(&document,"final_notes")?,final_state:decode(&document,"final_state")?,
        accounting:decode(&document,"accounting")?,
    };
    for state in std::iter::once(&input.genesis_state).chain(std::iter::once(&input.final_state))
        .chain(input.transactions.iter().flat_map(|tx| [&tx.state_before,&tx.state_after]))
        .chain(input.headers.iter().flat_map(|h| [&h.state_before,&h.state_after_transactions,&h.state_after]))
        .chain(input.batches.iter().flat_map(|b| [&b.state_pre,&b.state_post])) {
        for (field,root) in [("cm_root",&state.cm_root),("nf_root",&state.nf_root),("rh_root",&state.rh_root),("wd_root",&state.wd_root)] {
            if !root.value.is_null() || root.status!="unavailable" || root.reason!=ROOT_REASON {
                return Err(SelectedLedgerInputError::RootMarker { field });
            }
        }
    }
    Ok(input)
}
fn required_optional<'de,D:serde::Deserializer<'de>>(d:D)->Result<Option<u128>,D::Error> {
    Option::<u128>::deserialize(d)
}
fn decode<T:serde::de::DeserializeOwned>(v:&Value,field:&str)->Result<T,SelectedLedgerInputError> {
    serde_json::from_value(v.get(field).cloned().ok_or_else(||SelectedLedgerInputError::Shape {
        detail:format!("missing {field}"),
    })?).map_err(|e|SelectedLedgerInputError::Shape {detail:format!("{field}: {e}")})
}

pub struct PreparedSelectedLedger {
    original:Value,typed:SelectedLedgerInput,source:String,document_sha256:[u8;32],
}
impl PreparedSelectedLedger {
    pub fn original(&self)->&Value {&self.original}
    pub fn typed(&self)->&SelectedLedgerInput {&self.typed}
    pub fn source(&self)->&str {&self.source}
    /// SHA256 of serde_json compact document transport, not raw input byte identity.
    pub fn document_sha256(&self)->&[u8;32] {&self.document_sha256}
    /// Existing public route, exact caller layout/domain/locator and owner payload.
    pub fn compile(&self,budget:DraftParseBudget,config:ExtCompileConfig,locator:Option<LocatorC>)
        ->Result<NominalBodyCompileArtifact,NominalBodyCompileFailure>
    { compile_nominal_source_host_bounded(&self.source,budget,config,locator) }
}

/// Replace only the real parser's root Entry expression. All library functions
/// and declarations remain identical. The resulting source is parsed/bodychecked
/// by the existing compiler; arbitrary source is never called a trusted checker.
pub fn prepare_selected_ledger_source(original:Value,library:&str,budget:DraftParseBudget)
    ->Result<PreparedSelectedLedger,SelectedLedgerInputError>
{
    let typed=decode_selected_ledger(original.clone())?;
    let parsed=parse_nominal_source_bounded(library,budget)
        .map_err(SelectedLedgerInputError::LibraryParse)?;
    let roots:Vec<_>=parsed.provenance.expressions.iter()
        .filter(|x|x.owner==SourceOwner::Entry && x.child_path.is_empty()).collect();
    let [root]=roots.as_slice() else {return Err(SelectedLedgerInputError::EntrySpan)};
    let start=root.span.start as usize;let end=root.span.end as usize;
    let prefix=library.get(..start).ok_or(SelectedLedgerInputError::EntrySpan)?;
    let suffix=library.get(end..).ok_or(SelectedLedgerInputError::EntrySpan)?;
    let mut emit=Emitter {out:String::new(),limit:budget.max_source_bytes};
    emit.put(prefix)?;emit.put("check_ledger(")?;typed.emit(&mut emit)?;
    emit.put(")")?;emit.put(suffix)?;
    let bytes=serde_json::to_vec(&original).map_err(|e|SelectedLedgerInputError::Shape{detail:e.to_string()})?;
    let document_sha256=Sha256::digest(&bytes).into();
    Ok(PreparedSelectedLedger {original,typed,source:emit.out,document_sha256})
}

/// Complete UTF8 identity; no truncation, hash alias or host-generated identifier.
pub fn packed_text(value:&str)->Result<(usize,[u128;4]),SelectedLedgerInputError> {
    let bytes=value.as_bytes();
    if bytes.len()>64 {return Err(SelectedLedgerInputError::TextTooLong {bytes:bytes.len(),limit:64})}
    let mut words=[0u128;4];
    for (index,&byte) in bytes.iter().enumerate() {words[index/16]|=u128::from(byte)<<((index%16)*8);}
    Ok((bytes.len(),words))
}
struct Emitter {out:String,limit:usize}
impl Emitter {
    fn put(&mut self,s:&str)->Result<(),SelectedLedgerInputError> {
        let observed=self.out.len().checked_add(s.len()).unwrap_or(usize::MAX);
        if observed>self.limit {return Err(SelectedLedgerInputError::SourceBytes {observed,limit:self.limit})}
        self.out.push_str(s);Ok(())
    }
    fn ctor(&mut self,ty:&str,name:&str,fields:&[&dyn Emit])->Result<(),SelectedLedgerInputError> {
        self.put("construct ")?;self.put(ty)?;self.put("::")?;self.put(name)?;self.put("(")?;
        for (i,f) in fields.iter().enumerate() {if i>0 {self.put(",")?;} f.emit(self)?;}
        self.put(")")
    }
}
trait Emit {fn emit(&self,e:&mut Emitter)->Result<(),SelectedLedgerInputError>;}
impl Emit for u128 {fn emit(&self,e:&mut Emitter)->Result<(),SelectedLedgerInputError>{e.put(&self.to_string())}}
impl Emit for bool {fn emit(&self,e:&mut Emitter)->Result<(),SelectedLedgerInputError>{e.put(if *self {"true"}else{"false"})}}
impl Emit for String {fn emit(&self,e:&mut Emitter)->Result<(),SelectedLedgerInputError>{
    let (len,w)=packed_text(self)?;e.ctor("Text","Utf8",&[&(len as u128),&w[0],&w[1],&w[2],&w[3]])
}}
impl Emit for Option<u128> {fn emit(&self,e:&mut Emitter)->Result<(),SelectedLedgerInputError>{match self{
    Some(v)=>e.ctor("Maybe<UInt>","Some",&[v]),None=>e.ctor("Maybe<UInt>","None",&[]),
}}}
struct Seq<'a,T>(&'static str,&'a [T]);
impl<T:Emit> Emit for Seq<'_,T> {fn emit(&self,e:&mut Emitter)->Result<(),SelectedLedgerInputError>{
    let ty=format!("Seq<{}>",self.0);
    for value in self.1 {e.put("construct ")?;e.put(&ty)?;e.put("::Link(")?;value.emit(e)?;e.put(",")?;}
    e.ctor(&ty,"Empty",&[])?;
    for _ in self.1 {e.put(")")?;}Ok(())
}}
macro_rules! record_emit {
    ($ty:ty,$name:literal,$($field:ident),+)=>{impl Emit for $ty {
        fn emit(&self,e:&mut Emitter)->Result<(),SelectedLedgerInputError>{e.ctor($name,$name,&[$(&self.$field),+])}
    }};
}
record_emit!(Asset,"Asset",network,issuer,reference,scale);
record_emit!(Config,"Config",seed,headers,batch_headers,tx_per_batch,shoppers,merchants,genesis_notes_per_shopper);
record_emit!(Counts,"Counts",cm_count,nf_count,rh_count,wd_count,height,tx_count,dep_consumed,fee_total);
record_emit!(Note,"Note",reference,owner,value,created_height,asset,spent);
record_emit!(Account,"Account",asset,native_token_id,genesis,deposited,withdrawn,fees,unspent);
impl Emit for Transaction {fn emit(&self,e:&mut Emitter)->Result<(),SelectedLedgerInputError>{
    e.ctor("Tx","Tx",&[&self.id,&self.batch,&self.height,&self.slot,&self.pos,&self.kind,&self.kind_code,
        &self.asset,&self.spender,&Seq("Text",&self.inputs),&Seq("Note",&self.input_notes),
        &Seq("Note",&self.outputs),&self.sum_inputs,&self.sum_outputs,&self.fee,&self.deposit,
        &self.withdrawal,&self.withdrawal_destination,&self.state_before,&self.state_after])
}}
impl Emit for Header {fn emit(&self,e:&mut Emitter)->Result<(),SelectedLedgerInputError>{
    e.ctor("Header","Header",&[&self.height,&self.batch,&self.state_before,&self.state_after_transactions,
        &self.state_after,&Seq("Text",&self.tx_ids)])
}}
impl Emit for Batch {fn emit(&self,e:&mut Emitter)->Result<(),SelectedLedgerInputError>{
    let k=&self.kind_counts;let values=[k.checkout_change,k.checkout_exact,k.p2p,k.sweep,k.deposit,k.withdrawal];
    e.put("construct Batch::Batch(")?;
    let first:[&dyn Emit;7]=[&self.batch_no,&self.h_first,&self.h_last,&self.state_pre,&self.state_post,
        &Seq("Text",&self.ordered_ids),&Seq("Text",&self.ordered_kinds)];
    for f in first {f.emit(e)?;e.put(",")?;}
    for (name,value) in KINDS.iter().zip(values) {
        e.put("construct Seq<KindCount>::Link(")?;
        e.ctor("KindCount","KindCount",&[&name.to_string(),&value])?;e.put(",")?;
    }
    e.ctor("Seq<KindCount>","Empty",&[])?;for _ in KINDS {e.put(")")?;}
    e.put(",")?;self.fee_sum.emit(e)?;e.put(",")?;self.deposit_sum.emit(e)?;
    e.put(",")?;self.withdrawal_sum.emit(e)?;e.put(")")
}}
impl Emit for SelectedLedgerInput {fn emit(&self,e:&mut Emitter)->Result<(),SelectedLedgerInputError>{
    e.ctor("LedgerInput","LedgerInput",&[&self.config,&self.asset,&Seq("Note",&self.genesis_notes),
        &self.genesis_state,&Seq("Tx",&self.transactions),&Seq("Header",&self.headers),
        &Seq("Batch",&self.batches),&Seq("Note",&self.final_notes),&self.final_state,&Seq("Account",&self.accounting)])
}}

// Explicit typed AST data route; original source-expansion API remains distinct.
#[path="selected_ledger_ast.rs"]
mod ast;
pub use ast::{prepare_selected_ledger_ast,PreparedSelectedLedgerAst,SelectedLedgerAstCompileError,LEDGER_AST_ADAPTER_PROFILE};
