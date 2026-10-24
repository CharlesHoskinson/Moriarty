//! GPU-keygen byte-identity lock.
//!
//! Pins the GPU keygen body
//! (`halo2\_axiom\_gpu::plonk::{keygen\_vk\_custom, keygen\_pk, keygen\_pk2}`): runs
//! CPU and GPU keygen against the SAME SRS and circuit and asserts the resulting
//! verifying/proving keys are byte-identical. The GPU path diverges from CPU
//! only in the per-column fixed/selector MSM commitments and the fixed-column
//! iFFT, so any differing byte localizes to that GPU code.
//!
//! ONE SRS feeds both sides: a fresh `setup`/`new` per side would draw a
//! different toxic `s`, so the GPU `ParamsKZG` is serialized and read back into
//! a `halo2-axiom` `ParamsKZG` — the bytes round-trip is load-bearing.
//!
//! Covered canonical->GPU bridge paths: a `Selector`-gated custom gate (so
//! `compress\_selectors` has a selector to compress), a permutation/copy
//! constraint, a lookup argument, and a global constant.
//!
//! Run for both `compress\_selectors = false` and `true`.
use halo2\_axiom\_gpu::circuit::{Layouter, SimpleFloorPlanner, Value};
use halo2\_axiom\_gpu::plonk::{
Advice, Circuit, Column, ConstraintSystem, Error, Fixed, Instance, Selector, TableColumn,
};
use halo2\_axiom\_gpu::poly::Rotation;
use halo2curves::bn256::{Bn256, Fr};
use rand\_core::OsRng;
/// `1 << K` rows. K >= 14 clears `GPU\_MSM\_THRESHOLD` so keygen's fixed/selector
/// commitments run on the real GPU device-MSM path (not the CPU fallback).
const K: u32 = 14;
/// Size of the lookup table assigned during synthesis.
const TABLE\_SIZE: usize = 8;
/// A circuit exercising the four canonical->GPU keygen bridge paths at once: a
/// `Selector`-gated custom gate, a permutation copy from the instance column, a
/// lookup argument, and a global constant.
#[derive(Clone)]
struct RichCircuit {
/// Public input; tied to advice `a` at row 0 (copy constraint).
public: Value,
/// Private multiplicand.
b: Value,
}
#[derive(Clone)]
struct RichConfig {
a: Column,
b: Column,
c: Column,
/// Advice cell pinned to a global constant.
d: Column,
/// Simple selector gating the multiplication gate (compressible).
s: Selector,
/// Fixed "selector" for the lookup; a plain fixed column, since `lookup`
/// rejects inputs containing a simple selector.
ql: Column,
instance: Column,
/// Lookup table column.
table: TableColumn,
}
impl Circuit for RichCircuit {
type Config = RichConfig;
type FloorPlanner = SimpleFloorPlanner;
#[cfg(feature = "circuit-params")]
type Params = ();
fn without\_witnesses(&self) -> Self {
Self {
public: Value::unknown(),
b: Value::unknown(),
}
}
fn configure(meta: &mut ConstraintSystem) -> RichConfig {
let a = meta.advice\_column();
let b = meta.advice\_column();
let c = meta.advice\_column();
let d = meta.advice\_column();
let s = meta.selector();
let ql = meta.fixed\_column();
let constant = meta.fixed\_column();
let instance = meta.instance\_column();
let table = meta.lookup\_table\_column();
meta.enable\_equality(a);
meta.enable\_equality(c);
meta.enable\_equality(d);
meta.enable\_equality(instance);
// Adds `constant` to the permutation and marks it the constants column.
meta.enable\_constant(constant);
// Custom gate, degree 3: exercises selector compression.
meta.create\_gate("mul", |meta| {
let a = meta.query\_advice(a, Rotation::cur());
let b = meta.query\_advice(b, Rotation::cur());
let c = meta.query\_advice(c, Rotation::cur());
let s = meta.query\_selector(s);
vec![s \* (a \* b - c)]
});
// Lookup argument: `ql \* a` must appear in `table`.
meta.lookup("range", |meta| {
let a = meta.query\_advice(a, Rotation::cur());
let ql = meta.query\_fixed(ql, Rotation::cur());
vec![(ql \* a, table)]
});
RichConfig {
a,
b,
c,
d,
s,
ql,
instance,
table,
}
}
fn synthesize(&self, config: RichConfig, mut layouter: impl Layouter) -> Result<(), Error> {
// Lookup table fixed column (committed in the vk).
layouter.assign\_table(
|| "range table",
|mut table| {
for i in 0..TABLE\_SIZE {
table.assign\_cell(
|| "table cell",
config.table,
i,
|| Value::known(Fr::from(i as u64)),
)?;
}
Ok(())
},
)?;
layouter.assign\_region(
|| "main",
|mut region| {
config.s.enable(&mut region, 0)?;
region.assign\_fixed(config.ql, 0, Fr::from(1));
// Copy constraint: advice `a[0]` <- public `instance[0]`.
region.assign\_advice\_from\_instance(
|| "a = public",
config.instance,
0,
config.a,
0,
)?;
region.assign\_advice(config.b, 0, self.b);
let c\_val = self.public.zip(self.b).map(|(a, b)| a \* b);
region.assign\_advice(config.c, 0, c\_val);
// Global constant: pin advice `d[0]` to a fixed constant,
// assigning the constants fixed column and a permutation copy.
region.assign\_advice\_from\_constant(|| "d const", config.d, 0, Fr::from(5))?;
Ok(())
},
)
}
}
/// Asserts CPU and GPU keygen produce byte-identical verifying and proving keys
/// for `circuit` under `compress\_selectors`, across all three entry points:
/// `keygen\_vk\_custom`, `keygen\_pk`, and `keygen\_pk2`.
fn assert\_keygen\_byte\_identity(
gpu\_params: &halo2\_axiom\_gpu::poly::kzg::commitment::ParamsKZG,
cpu\_params: &halo2\_axiom::poly::kzg::commitment::ParamsKZG,
circuit: &RichCircuit,
compress\_selectors: bool,
) {
let fmt = halo2\_axiom::SerdeFormat::RawBytes;
let gpu\_vk = halo2\_axiom\_gpu::plonk::keygen\_vk\_custom(gpu\_params, circuit, compress\_selectors)
.expect("gpu keygen\_vk\_custom");
let cpu\_vk = halo2\_axiom::plonk::keygen\_vk\_custom(cpu\_params, circuit, compress\_selectors)
.expect("cpu keygen\_vk\_custom");
assert\_eq!(
gpu\_vk.to\_bytes(fmt),
cpu\_vk.to\_bytes(fmt),
"GPU keygen\_vk\_custom must be byte-identical to CPU (compress\_selectors = {compress\_selectors})"
);
let gpu\_pk =
halo2\_axiom\_gpu::plonk::keygen\_pk(gpu\_params, gpu\_vk, circuit).expect("gpu keygen\_pk");
let cpu\_pk = halo2\_axiom::plonk::keygen\_pk(cpu\_params, cpu\_vk, circuit).expect("cpu keygen\_pk");
assert\_eq!(
gpu\_pk.to\_bytes(fmt),
cpu\_pk.to\_bytes(fmt),
"GPU keygen\_pk must be byte-identical to CPU (compress\_selectors = {compress\_selectors})"
);
let gpu\_pk2 = halo2\_axiom\_gpu::plonk::keygen\_pk2(gpu\_params, circuit, compress\_selectors)
.expect("gpu keygen\_pk2");
let cpu\_pk2 = halo2\_axiom::plonk::keygen\_pk2(cpu\_params, circuit, compress\_selectors)
.expect("cpu keygen\_pk2");
assert\_eq!(
gpu\_pk2.to\_bytes(fmt),
cpu\_pk2.to\_bytes(fmt),
"GPU keygen\_pk2 must be byte-identical to CPU (compress\_selectors = {compress\_selectors})"
);
assert\_eq!(
gpu\_pk2.to\_bytes(fmt),
gpu\_pk.to\_bytes(fmt),
"GPU keygen\_pk2 must match GPU keygen\_pk (compress\_selectors = {compress\_selectors})"
);
}
#[test]
fn gpu\_keygen\_bytes\_match\_cpu() {
// ONE SRS: generate on the GPU side, serialize, read back into halo2-axiom.
let gpu\_params = halo2\_axiom\_gpu::poly::kzg::commitment::ParamsKZG::::setup(K, OsRng);
let mut srs\_bytes = Vec::new();
gpu\_params
.write\_custom(
&mut srs\_bytes,
halo2\_axiom\_gpu::SerdeFormat::RawBytesUnchecked,
)
.expect("write shared SRS");
let cpu\_params = halo2\_axiom::poly::kzg::commitment::ParamsKZG::::read\_custom(
&mut &srs\_bytes[..],
halo2\_axiom::SerdeFormat::RawBytesUnchecked,
)
.expect("read shared SRS into halo2-axiom ParamsKZG");
// Witness-free circuit: keygen ignores advice values.
let circuit = RichCircuit {
public: Value::unknown(),
b: Value::unknown(),
};
for compress\_selectors in [false, true] {
assert\_keygen\_byte\_identity(&gpu\_params, &cpu\_params, &circuit, compress\_selectors);
}
}