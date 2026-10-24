use anyhow::Result;
use midnight\_transient\_crypto::curve::{FR\_BYTES, Fr};
use prover\_cuda::CudaProver;
use rand::{CryptoRng, RngCore};
/// A preprocessed witness bundle for CUDA proving.
#[derive(Clone, Debug)]
pub struct ZkirPreprocessed {
/// The circuit memory after witness generation.
pub memory: Vec,
/// The public input vector (statement).
pub pis: Vec,
/// The skip sequence for public inputs.
pub pi\_skips: Vec>,
/// The binding input committed into the statement.
pub binding\_input: Fr,
/// The communications commitment and its randomness, when present.
pub communications\_commitment: Option<(Fr, Fr)>,
}
impl ZkirPreprocessed {
/// Packs the preprocessed witness into the stable CUDA layout.
pub fn pack\_cuda(&self) -> Result> {
let memory\_len = u32::try\_from(self.memory.len())?;
let pis\_len = u32::try\_from(self.pis.len())?;
let pi\_skips\_len = u32::try\_from(self.pi\_skips.len())?;
let has\_comm = self.communications\_commitment.is\_some() as u8;
let mut out = Vec::with\_capacity(
4 + 4 + 4 + 1
+ (self.memory.len() + self.pis.len() + 1 + 2 \* (has\_comm as usize)) \* FR\_BYTES
+ self.pi\_skips.len() \* 4,
);
out.extend\_from\_slice(&memory\_len.to\_le\_bytes());
out.extend\_from\_slice(&pis\_len.to\_le\_bytes());
out.extend\_from\_slice(&pi\_skips\_len.to\_le\_bytes());
out.push(has\_comm);
for value in &self.memory {
out.extend\_from\_slice(&value.as\_le\_bytes());
}
for value in &self.pis {
out.extend\_from\_slice(&value.as\_le\_bytes());
}
for skip in &self.pi\_skips {
let encoded = match skip {
Some(value) => u32::try\_from(\*value)?,
None => u32::MAX,
};
out.extend\_from\_slice(&encoded.to\_le\_bytes());
}
out.extend\_from\_slice(&self.binding\_input.as\_le\_bytes());
if let Some((comm, rand)) = self.communications\_commitment {
out.extend\_from\_slice(&comm.as\_le\_bytes());
out.extend\_from\_slice(&rand.as\_le\_bytes());
}
Ok(out)
}
}
/// CUDA backend wrapper that returns raw proof bytes.
pub struct CudaProverBackend {
prover: CudaProver,
}
impl CudaProverBackend {
/// Initializes the CUDA backend.
pub fn new() -> Result {
Ok(Self {
prover: CudaProver::new()?,
})
}
/// Proves using already-serialized inputs.
pub fn prove\_raw(
&self,
params: &[u8],
prover\_key: &[u8],
preprocessed: &[u8],
rng\_seed: &[u8; 32],
) -> Result> {
self.prover
.prove(params, prover\_key, preprocessed, rng\_seed)
}
/// Proves using a preprocessed witness bundle.
pub fn prove\_preprocessed(
&self,
params: &[u8],
prover\_key: &[u8],
preprocessed: &ZkirPreprocessed,
mut rng: R,
) -> Result> {
let packed = preprocessed.pack\_cuda()?;
let mut rng\_seed = [0u8; 32];
rng.fill\_bytes(&mut rng\_seed);
self.prove\_raw(params, prover\_key, &packed, &rng\_seed)
}
}