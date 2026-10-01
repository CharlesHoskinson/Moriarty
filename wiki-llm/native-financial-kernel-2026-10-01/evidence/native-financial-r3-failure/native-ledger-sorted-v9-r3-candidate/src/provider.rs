use crate::*;
use std::sync::{Arc, Mutex};
use transient_crypto::proofs::ProvingProvider;

pub struct Parameters { pub bytes: Vec<u8> }
impl ParamsProverProvider for Parameters {
    async fn get_params(&self, k: u8) -> io::Result<ParamsProver> {
        if k != 17 { return Err(io::Error::other("only frozen k17 parameters permitted")); }
        let mut c = Cursor::new(self.bytes.as_slice());
        let value = ParamsProver::read(&mut c)?;
        if c.position() != self.bytes.len() as u64 { return Err(io::Error::other("parameter trailing bytes")); }
        Ok(value)
    }
}
pub struct LocalResolver { pub material: ProvingKeyMaterial }
impl Resolver for LocalResolver {
    async fn resolve_key(&self, key: KeyLocation) -> io::Result<Option<ProvingKeyMaterial>> {
        if key.0.as_ref() != LOCATION { return Err(io::Error::other("unknown proof family; no auxiliary proof authority")); }
        Ok(Some(self.material.clone()))
    }
}
pub struct Record { pub proof: Proof, pub pis: Vec<Fr>, pub skips: Vec<Option<usize>>, pub binding: Fr }
#[derive(Default)]
pub struct Budget { attempts: usize, pub records: Vec<Record>, pub checked: Option<Vec<Option<usize>>> }
pub struct Provider {
    pub resolver: Arc<LocalResolver>,
    pub params: Arc<Parameters>,
    pub budget: Arc<Mutex<Budget>>,
}
impl ProvingProvider for Provider {
    async fn check(&self, preimage: &ProofPreimage) -> anyhow::Result<Vec<Option<usize>>> {
        let material = self.resolver.resolve_key(preimage.key_location.clone()).await?.ok_or_else(||anyhow::anyhow!("unresolved"))?;
        let mut c = Cursor::new(material.ir_source.as_slice());
        let ir = IrSource::load_ir_from_tagged(&mut c)?;
        anyhow::ensure!(c.position() == material.ir_source.len() as u64, "IR suffix");
        let skips = ir.check(preimage)?;
        self.budget.lock().map_err(|_|anyhow::anyhow!("budget poisoned"))?.checked = Some(skips.clone());
        Ok(skips)
    }
    async fn prove(self, preimage: &ProofPreimage, overwrite_binding_input: Option<Fr>) -> anyhow::Result<Proof> {
        // Only ledger finalization may supply this scalar. Do not accept the old binding0 caller.
        let binding = overwrite_binding_input.ok_or_else(||anyhow::anyhow!("finalized ledger binding absent"))?;
        anyhow::ensure!(binding != Fr::from(0u8), "zero finalized binding refused");
        let material = self.resolver.resolve_key(preimage.key_location.clone()).await?.ok_or_else(||anyhow::anyhow!("unresolved"))?;
        let mut c = Cursor::new(material.ir_source.as_slice());
        let ir = IrSource::load_ir_from_tagged(&mut c)?;
        anyhow::ensure!(c.position() == material.ir_source.len() as u64 && ir.k()==17, "IR identity/k");
        let mut c = Cursor::new(material.prover_key.as_slice());
        let pk = IrSource::load_prover_key_from_tagged(&mut c)?;
        anyhow::ensure!(c.position()==material.prover_key.len() as u64, "PK suffix");
        let vk: VerifierKey = decode(&material.verifier_key).map_err(|e|anyhow::anyhow!(e.to_string()))?;
        let mut p = preimage.clone(); p.binding_input = binding;
        let expected = ir.check(&p)?;
        {
            let mut budget=self.budget.lock().map_err(|_|anyhow::anyhow!("budget poisoned"))?;
            anyhow::ensure!(budget.attempts==0 && budget.checked.as_ref()==Some(&expected), "one-call proof budget/skips violated");
            budget.attempts+=1; // Irreversible attempt count, including failure.
        }
        let (proof,pis,skips)=ir.prove(OsRng,self.params.as_ref(),pk,&p).await?;
        anyhow::ensure!(skips==expected, "check/prove skip mismatch");
        let pv=verifier_parameters(&self.params.bytes).map_err(|e|anyhow::anyhow!(e.to_string()))?;
        vk.verify(&pv,&proof,pis.iter().copied())?;
        // Ledger uses embedded native verifier params too. Both must verify this same statement.
        vk.verify(&transient_crypto::proofs::PARAMS_VERIFIER,&proof,pis.iter().copied())?;
        self.budget.lock().map_err(|_|anyhow::anyhow!("budget poisoned"))?.records.push(Record{proof:proof.clone(),pis,skips,binding});
        Ok(proof)
    }
    fn split(&mut self) -> Self { Self{resolver:self.resolver.clone(),params:self.params.clone(),budget:self.budget.clone()} }
    fn resolver(&self) -> &impl Resolver { self.resolver.as_ref() }
}
