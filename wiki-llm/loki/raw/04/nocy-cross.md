use midnight\_circuits::compact\_std\_lib::{prove, setup\_pk, setup\_vk, verify, MidnightCircuit};
use midnight\_curves::Bls12;
use midnight\_proofs::poly::kzg::params::ParamsKZG;
use midnight\_proofs::utils::SerdeFormat;
use native\_plonk::NativePlonkApi;
use rand\_chacha::ChaCha20Rng;
use rand\_core::SeedableRng;
use serialize::tagged\_serialize;
use transient\_crypto::curve::Fr;
use transient\_crypto::proofs::{KeyLocation, ProverKey as TaggedProverKey, ProofPreimage};
use zkir::{Instruction as I, IrSource};
#[test]
fn gpu\_proof\_verifies\_with\_cpu\_verifier() {
if std::env::var("MIDNIGHT\_CUDA\_TESTS").ok().as\_deref() != Some("1") {
return;
}
let api = NativePlonkApi::load().expect("native PLONK library should load");
let ir = IrSource {
num\_inputs: 1,
do\_communications\_commitment: false,
instructions: std::sync::Arc::new(vec![I::Assert { cond: 0 }]),
};
let k = MidnightCircuit::from\_relation(&ir).min\_k();
let params = ParamsKZG::::unsafe\_setup(k, ChaCha20Rng::from\_seed([7u8; 32]));
let vk = setup\_vk(¶ms, &ir);
let pk = setup\_pk(&ir, &vk);
let preimage = ProofPreimage {
inputs: vec![Fr::from(1)],
private\_transcript: vec![],
public\_transcript\_inputs: vec![],
public\_transcript\_outputs: vec![],
binding\_input: Fr::from(42),
communications\_commitment: None,
key\_location: KeyLocation("test".into()),
};
let preproc = midnight\_cuda\_witness::preprocess\_ir(&ir, &preimage).unwrap();
let packed\_preproc = midnight\_cuda\_witness::pack\_preprocessed(&preproc).unwrap();
let seed = [9u8; 32];
eprintln!("debug: cpu\_prove\_start");
let cpu\_proof = prove::<\_, Blake2bState>(
¶ms,
&pk,
&ir,
&preproc.pis,
preproc.clone(),
ChaCha20Rng::from\_seed(seed),
)
.expect("cpu proof should succeed");
eprintln!("cpu proof len: {}", cpu\_proof.len());
let params\_verifier = params.verifier\_params();
verify::(¶ms\_verifier, &vk, &preproc.pis, &cpu\_proof)
.expect("cpu proof should verify");
let mut params\_bytes = Vec::new();
params
.write\_custom(&mut params\_bytes, SerdeFormat::RawBytesUnchecked)
.unwrap();
let tagged\_pk: TaggedProverKey = TaggedProverKey::from(pk);
let mut pk\_bytes = Vec::new();
tagged\_serialize(&tagged\_pk, &mut pk\_bytes).unwrap();
let mut out = vec![0u8; 4 \* 1024 \* 1024];
eprintln!("debug: gpu\_prove\_start");
let proof\_len = api
.prove(¶ms\_bytes, &pk\_bytes, &packed\_preproc, &seed, &mut out)
.expect("gpu proof should succeed");
let gpu\_proof = &out[..proof\_len];
eprintln!("gpu proof len: {}", gpu\_proof.len());
verify::(¶ms\_verifier, &vk, &preproc.pis, gpu\_proof)
.expect("gpu proof should verify");
}
use blake2b\_simd::State as Blake2bState;