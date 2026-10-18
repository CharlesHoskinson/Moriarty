//! L4 ordered scalar bytes only. No statement binding, digest or native ABI.
use super::{rows::*, limits::{self, Locator, Result}};

fn count_bytes(capacity: u128) -> usize {
    let bits=(128-capacity.leading_zeros()).max(1);
    ((bits+7)/8) as usize
}
fn id_bytes(capacity: u128) -> usize { count_bytes(capacity.saturating_sub(1)) }
struct Writer { bytes: Vec<u8> }
impl Writer {
    fn unsigned(&mut self,n: u128,width: usize) -> Result<()> {
        let loc=Locator::field("ProgramBlock byte expansion");
        let le=n.to_le_bytes();
        if le[width..].iter().any(|&b|b!=0) {return Err(limits::host(loc));}
        self.bytes.try_reserve(width).map_err(|_|limits::host(loc))?;
        self.bytes.extend_from_slice(&le[..width]);Ok(())
    }
    fn byte(&mut self,n: u8) -> Result<()> {self.unsigned(n as u128,1)}
    fn id(&mut self,n: usize,cap: u128) -> Result<()> {self.unsigned(n as u128,id_bytes(cap))}
    fn count(&mut self,n: usize,cap: u128) -> Result<()> {self.unsigned(n as u128,count_bytes(cap))}
    fn slice(&mut self,s: Slice,cap: u128) -> Result<()> {self.count(s.start,cap)?;self.count(s.len,cap)}
    fn value(&mut self,v: ValueRef,cap: u128) -> Result<()> {self.byte(u8::from(v.space))?;self.id(v.index,cap)}
}
impl PreparedA {
    /// Expand all sixteen header columns and fifteen padded tables in L4 order.
    /// Inactive variant lanes are already zero: they are not dereferenced or
    /// range-checked against an empty active table (notably Entry functionIndex).
    pub fn program_bytes(&self) -> Result<Vec<u8>> {
        let c=&self.capacities.tables;let b=&self.block;let mut w=Writer{bytes:Vec::new()};
        let cap=|id:TableId| c[id as usize];
        let value_cap=cap(TableId::Constants).max(self.capacities.data);
        let path_cap=cap(TableId::ExprChildren).max(cap(TableId::Arms).checked_add(1)
            .ok_or_else(||limits::host(Locator::field("OccurrencePaths width")))?);
        w.id(b.header.entry_expr_id,cap(TableId::Expressions))?;
        for (i,n) in b.header.active_lengths.iter().enumerate() {w.count(*n,c[i])?;}
        for r in &b.names.rows {w.slice(r.bytes,cap(TableId::NameBytes))?;}
        for r in &b.name_bytes.rows {w.byte(*r)?;}
        for r in &b.occurrences.rows {w.byte(r.owner_tag)?;w.id(r.function_index,cap(TableId::Functions))?;w.slice(r.path,cap(TableId::OccurrencePaths))?;}
        for r in &b.occurrence_paths.rows {w.count(*r,path_cap)?;}
        for r in &b.expressions.rows {
            w.byte(r.tag)?;w.id(r.occurrence_id,cap(TableId::Occurrences))?;w.value(r.value,value_cap)?;
            w.id(r.name_id,cap(TableId::Names))?;w.byte(r.binary_op)?;w.byte(r.unary_op)?;
            w.slice(r.children,cap(TableId::ExprChildren))?;w.slice(r.arms,cap(TableId::Arms))?;
        }
        for r in &b.expr_children.rows {w.id(*r,cap(TableId::Expressions))?;}
        for r in &b.functions.rows {
            w.id(r.source_table_index,cap(TableId::Functions))?;w.id(r.name_id,cap(TableId::Names))?;
            w.slice(r.parameters,cap(TableId::Parameters))?;w.id(r.return_type as usize,cap(TableId::Types))?;w.id(r.body_expr_id,cap(TableId::Expressions))?;
        }
        for r in &b.parameters.rows {w.id(r.name_id,cap(TableId::Names))?;w.id(r.type_id as usize,cap(TableId::Types))?;}
        for r in &b.patterns.rows {w.byte(r.tag)?;w.id(r.name0,cap(TableId::Names))?;w.id(r.name1,cap(TableId::Names))?;}
        for r in &b.arms.rows {w.id(r.pattern_id,cap(TableId::Patterns))?;w.id(r.body_expr_id,cap(TableId::Expressions))?;}
        for r in &b.types.rows {w.byte(r.kind)?;w.slice(r.constructors,cap(TableId::Constructors))?;}
        for r in &b.constructors.rows {w.id(r.owning_type as usize,cap(TableId::Types))?;w.count(r.source_order,cap(TableId::Constructors))?;w.slice(r.fields,cap(TableId::ConstructorFieldTypes))?;}
        for r in &b.constructor_field_types.rows {w.id(*r as usize,cap(TableId::Types))?;}
        for r in &b.constants.rows {
            w.byte(r.kind)?;w.id(r.type_id as usize,cap(TableId::Types))?;w.id(r.ctor_id,cap(TableId::Constructors))?;
            for byte in r.uint_bytes {w.byte(byte)?;}w.byte(u8::from(r.bool_value))?;w.slice(r.fields,cap(TableId::ConstantFields))?;
        }
        for r in &b.constant_fields.rows {w.value(*r,value_cap)?;}
        Ok(w.bytes)
    }
}
