//! Nonrecursive finite numerical transfer precursor. No owner authentication,
//! byte hashing, source correspondence, replay/head semantics or ledger acceptance.
use ff::PrimeField;
use midnight_curves::Fq;
use midnight_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Constraints, Error, Instance, Selector},
    poly::Rotation,
};
use serde_json::Value as Json;
pub const PUBLIC_VERSION: &str = "moriarty-transfer-numerical/1";
pub const PUBLIC_FIELDS: [&str; 28] = [
    "version",
    "sender_pre",
    "sender_post",
    "recipient_pre",
    "recipient_post",
    "fee_recipient_pre",
    "fee_recipient_post",
    "amount",
    "fee",
    "gross",
    "gross_cap",
    "fee_cap",
    "net_floor",
    "round",
    "not_before",
    "not_after",
    "work_remaining_pre",
    "work_remaining_post",
    "work_spent_pre",
    "work_spent_post",
    "allowance_remaining_pre",
    "allowance_remaining_post",
    "allowance_spent_pre",
    "allowance_spent_post",
    "effect_debit",
    "effect_recipient_credit",
    "effect_fee_credit",
    "effect_allowance_use",
];
const N: usize = PUBLIC_FIELDS.len();
const SLACKS: usize = 8;
/// Exact 128-bit integer embedded in the scalar field. All arithmetic equality
/// operands are independently range constrained, preventing field wraparound.
pub fn field(value: u128) -> Fq {
    Fq::from(value as u64) + Fq::from((value >> 64) as u64) * (Fq::from(u64::MAX) + Fq::from(1))
}
#[derive(Clone, Debug)]
pub struct TransferConfig {
    relation: [Column<Advice>; N + SLACKS],
    relation_selector: Selector,
    instance: Column<Instance>,
    accumulator: Column<Advice>,
    bits: [Column<Advice>; 8],
    range_step: Selector,
    range_start: Selector,
    signed_start: Selector,
}
#[derive(Clone, Debug)]
pub struct TransferCircuit {
    values: [Value<Fq>; N + SLACKS],
}
impl TransferCircuit {
    pub fn new(values: [u128; N]) -> Self {
        let mut all = [Value::unknown(); N + SLACKS];
        for (i, value) in values.into_iter().enumerate() {
            all[i] = Value::known(field(value));
        }
        // These are untrusted witness hints; gates and ranges establish inequalities.
        for (i, (a, b)) in [(10, 9), (11, 8), (7, 12), (13, 14), (15, 13)]
            .into_iter()
            .enumerate()
        {
            all[N + i] = Value::known(field(values[a].saturating_sub(values[b])));
        }
        all[N + 5] = Value::known(field(values[16].saturating_add(values[18])));
        all[N + 6] = Value::known(field(values[20].saturating_add(values[22])));
        all[N + 7] = Value::known(field(values[7].saturating_sub(1)));
        Self { values: all }
    }
    /// Public layout contains numerical claims only, never a success flag.
    pub fn instances(&self) -> Vec<Fq> {
        self.values[..N]
            .iter()
            .map(|v| {
                let mut x = Fq::from(0);
                v.map(|known| {
                    x = known;
                    known
                });
                x
            })
            .collect()
    }
    /// Fault injection and external field-layout adapter. Out-of-range values
    /// remain witnesses and must fail the circuit, rather than a host predicate.
    pub fn replace_witness_field(&mut self, index: usize, value: Fq) {
        self.values[index] = Value::known(value);
    }
}
impl Circuit<Fq> for TransferCircuit {
    type Config = TransferConfig;
    type FloorPlanner = SimpleFloorPlanner;
    fn without_witnesses(&self) -> Self {
        Self {
            values: [Value::unknown(); N + SLACKS],
        }
    }
    fn configure(meta: &mut ConstraintSystem<Fq>) -> TransferConfig {
        let relation = std::array::from_fn(|_| {
            let c = meta.advice_column();
            meta.enable_equality(c);
            c
        });
        let instance = meta.instance_column();
        meta.enable_equality(instance);
        let relation_selector = meta.selector();
        meta.create_gate("complete finite numerical transfer", |m| {
            let v: Vec<_> = relation
                .iter()
                .map(|c| m.query_advice(*c, Rotation::cur()))
                .collect();
            let one = midnight_proofs::plonk::Expression::Constant(Fq::from(1));
            Constraints::with_selector(
                relation_selector,
                vec![
                    v[0].clone() - one.clone(),
                    v[N + 7].clone() + one.clone() - v[7].clone(),
                    v[7].clone() + v[8].clone() - v[9].clone(),
                    v[2].clone() + v[9].clone() - v[1].clone(),
                    v[3].clone() + v[7].clone() - v[4].clone(),
                    v[5].clone() + v[8].clone() - v[6].clone(),
                    v[17].clone() + one.clone() - v[16].clone(),
                    v[18].clone() + one - v[19].clone(),
                    v[21].clone() + v[9].clone() - v[20].clone(),
                    v[22].clone() + v[9].clone() - v[23].clone(),
                    v[9].clone() + v[N].clone() - v[10].clone(),
                    v[8].clone() + v[N + 1].clone() - v[11].clone(),
                    v[12].clone() + v[N + 2].clone() - v[7].clone(),
                    v[14].clone() + v[N + 3].clone() - v[13].clone(),
                    v[13].clone() + v[N + 4].clone() - v[15].clone(),
                    v[16].clone() + v[18].clone() - v[N + 5].clone(),
                    v[20].clone() + v[22].clone() - v[N + 6].clone(),
                    v[24].clone() - v[9].clone(),
                    v[25].clone() - v[7].clone(),
                    v[26].clone() - v[8].clone(),
                    v[27].clone() - v[9].clone(),
                ],
            )
        });
        let accumulator = meta.advice_column();
        meta.enable_equality(accumulator);
        let bits = std::array::from_fn(|_| meta.advice_column());
        let range_step = meta.selector();
        let range_start = meta.selector();
        let signed_start = meta.selector();
        meta.create_gate("UInt128 byte decomposition", |m| {
            let a = m.query_advice(accumulator, Rotation::cur());
            let next = m.query_advice(accumulator, Rotation::next());
            let mut byte = midnight_proofs::plonk::Expression::Constant(Fq::from(0));
            let mut gates = Vec::new();
            for (i, c) in bits.iter().enumerate() {
                let b = m.query_advice(*c, Rotation::cur());
                gates.push(
                    b.clone()
                        * (b.clone() - midnight_proofs::plonk::Expression::Constant(Fq::from(1))),
                );
                byte = byte + b * midnight_proofs::plonk::Expression::Constant(Fq::from(1u64 << i));
            }
            gates.push(
                a * midnight_proofs::plonk::Expression::Constant(Fq::from(256)) + byte - next,
            );
            Constraints::with_selector(range_step, gates)
        });
        meta.create_gate("UInt128 starts at zero", |m| {
            Constraints::with_selector(
                range_start,
                vec![m.query_advice(accumulator, Rotation::cur())],
            )
        });
        meta.create_gate("nonnegative signed127 amount/cap", |m| {
            Constraints::with_selector(signed_start, vec![m.query_advice(bits[7], Rotation::cur())])
        });
        TransferConfig {
            relation,
            relation_selector,
            instance,
            accumulator,
            bits,
            range_step,
            range_start,
            signed_start,
        }
    }
    fn synthesize(&self, c: TransferConfig, mut l: impl Layouter<Fq>) -> Result<(), Error> {
        let values = l.assign_region(
            || "numerical relation",
            |mut r| {
                c.relation_selector.enable(&mut r, 0)?;
                let mut assigned = Vec::new();
                for (i, column) in c.relation.iter().enumerate() {
                    assigned.push(r.assign_advice(|| "value", *column, 0, || self.values[i])?);
                }
                Ok(assigned)
            },
        )?;
        for (i, cell) in values[..N].iter().enumerate() {
            l.constrain_instance(cell.cell(), c.instance, i)?;
        }
        for (i, cell) in values.iter().enumerate() {
            l.assign_region(
                || format!("128-bit range {i}"),
                |mut r| {
                    c.range_start.enable(&mut r, 0)?;
                    if (7..=12).contains(&i) {
                        c.signed_start.enable(&mut r, 0)?;
                    }
                    let mut accumulator = Value::known(Fq::from(0));
                    r.assign_advice(|| "initial zero", c.accumulator, 0, || accumulator)?;
                    for row in 0..16 {
                        c.range_step.enable(&mut r, row)?;
                        // Fq canonical representation is little endian. High bytes
                        // are excluded; final equality rejects any value >=2^128.
                        let byte = self.values[i].map(|v| v.to_repr().as_ref()[15 - row]);
                        for bit in 0..8 {
                            r.assign_advice(
                                || "bit",
                                c.bits[bit],
                                row,
                                || byte.map(|b| Fq::from(((b >> bit) & 1) as u64)),
                            )?;
                        }
                        accumulator = accumulator
                            .zip(byte)
                            .map(|(a, b)| a * Fq::from(256) + Fq::from(b as u64));
                        let last =
                            r.assign_advice(|| "prefix", c.accumulator, row + 1, || accumulator)?;
                        if row == 15 {
                            r.constrain_equal(last.cell(), cell.cell())?;
                        }
                    }
                    Ok(())
                },
            )?;
        }
        Ok(())
    }
}
/// Adapter for frozen exported Core5 transfer fixtures. Exact account/effect
/// identities are checked here, outside the circuit: no source binding claim.
pub fn fixture_public_values(f: &Json) -> Result<[u128; N], String> {
    fn num(v: &Json) -> Result<u128, String> {
        let s = v.as_str().ok_or("expected decimal string")?;
        if s.is_empty()
            || !s.bytes().all(|b| b.is_ascii_digit())
            || (s.len() > 1 && s.starts_with('0'))
        {
            return Err("noncanonical UInt128".into());
        }
        s.parse().map_err(|_| "UInt128 overflow".into())
    }
    fn balance(s: &Json, id: &str) -> Result<u128, String> {
        let rows = s["balances"].as_array().ok_or("balances")?;
        let found: Vec<_> = rows.iter().filter(|x| x["account"] == id).collect();
        if found.len() != 1 {
            return Err("duplicate/missing balance".into());
        }
        num(&found[0]["amount"])
    }
    let intent = &f["intent_statement"]["lowered_intent"];
    let pre = &f["statement"]["lowered"]["state"];
    let post = &f["expected_post"];
    if pre["core"] != "moriarty-core/5"
        || post["core"] != "moriarty-core/5"
        || intent["kind"] != "Transfer"
    {
        return Err("Core5 Transfer fixture required".into());
    }
    let owner = intent["signer"].as_str().ok_or("signer")?;
    let recipient = intent["recipient"].as_str().ok_or("recipient")?;
    let fee = intent["feeRecipient"].as_str().ok_or("feeRecipient")?;
    if owner == recipient || owner == fee || recipient == fee {
        return Err("distinct-account numerical relation only".into());
    }
    for state in [pre, post] {
        let rows = state["balances"].as_array().ok_or("balances")?;
        if rows.len() != 3
            || rows[0]["account"] != owner
            || rows[1]["account"] != recipient
            || rows[2]["account"] != fee
        {
            return Err("exact ordered three-account transfer state required".into());
        }
    }
    let mut v = [0; N];
    v[0] = 1;
    for (i, s, id) in [
        (1, pre, owner),
        (2, post, owner),
        (3, pre, recipient),
        (4, post, recipient),
        (5, pre, fee),
        (6, post, fee),
    ] {
        v[i] = balance(s, id)?;
    }
    for (i, key) in [
        (7, "amount"),
        (8, "fee"),
        (10, "grossCap"),
        (11, "feeCap"),
        (12, "netFloor"),
        (14, "notBefore"),
        (15, "notAfter"),
    ] {
        v[i] = num(&intent[key])?;
    }
    v[9] = v[7].checked_add(v[8]).ok_or("gross overflow")?;
    v[13] = num(&pre["round"])?;
    for (i, s, key) in [
        (16, pre, "workRemaining"),
        (17, post, "workRemaining"),
        (18, pre, "workSpent"),
        (19, post, "workSpent"),
    ] {
        v[i] = num(&s[key])?;
    }
    for (s, i) in [(pre, 20), (post, 21)] {
        let a = s["allowances"].as_array().ok_or("allowances")?;
        if a.len() != 1 || a[0]["owner"] != owner {
            return Err("exact single owner allowance required".into());
        }
        v[i] = num(&a[0]["remaining"])?;
        v[i + 2] = num(&a[0]["spent"])?;
    }
    let e = f["expected_effects"].as_array().ok_or("effects")?;
    let fee_present = v[8] != 0;
    if e.len() != if fee_present { 6 } else { 5 } {
        return Err("exact ordered effect count required".into());
    }
    for (pos, kind, id) in [(0, "Debit", owner), (1, "Credit", recipient)] {
        if e[pos]["kind"] != kind || e[pos]["account"] != id || e[pos]["asset"] != intent["asset"] {
            return Err("effect identity/order mismatch".into());
        }
        v[24 + pos] = num(&e[pos]["amount"])?;
    }
    let allowance_pos = if fee_present {
        if e[2]["kind"] != "Credit" || e[2]["account"] != fee || e[2]["asset"] != intent["asset"] {
            return Err("fee effect mismatch".into());
        }
        v[26] = num(&e[2]["amount"])?;
        3
    } else {
        2
    };
    if e[allowance_pos]["kind"] != "UseAllowance"
        || e[allowance_pos]["owner"] != owner
        || e[allowance_pos + 1]["kind"] != "UseReplay"
        || e[allowance_pos + 2]["kind"] != "AdvanceHead"
    {
        return Err("effect order mismatch".into());
    }
    v[27] = num(&e[allowance_pos]["amount"])?;
    Ok(v)
}
