use midnight_zkir::IrSource;
use serialize::tagged_deserialize;
use std::{fs, io::Cursor, path::PathBuf};
use transient_crypto::proofs::ProofPreimage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 2 { return Err("usage: beta-public-preimage-check PAY_ZKIR FIXTURE_DIRECTORY".into()); }
    let ir = IrSource::load(&fs::read(&args[0])?[..])?;
    let fixture = PathBuf::from(&args[1]);
    let names = ["good", "forged-output", "forged-frame", "forged-signature", "forged-head-read", "forged-recipient-post-read"];
    for name in names {
        let bytes = fs::read(fixture.join(format!("{name}.preimage")))?;
        let mut cursor = Cursor::new(bytes.as_slice());
        let preimage: ProofPreimage = tagged_deserialize(&mut cursor)?;
        if cursor.position() != bytes.len() as u64 { return Err(format!("{name}: trailing preimage bytes").into()); }
        // Projection identity checks are host fixture guards, not IR predicates.
        const LOCATION: &str = "local-public-fixture/pay/c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae";
        if preimage.key_location.0.as_ref() != LOCATION || preimage.binding_input != 0u8.into() {
            return Err(format!("{name}: host fixture identity mismatch (not relation refusal)").into());
        }
        let result = preimage.check(&ir);
        if name == "good" {
            let skips = result.map_err(|e| format!("actual public fixture failed source check: {e:?}"))?;
            println!("good: SOURCE_CHECK_OK; pi_skips={skips:?}; no proof/SRS/ledger acceptance");
        } else {
            match result {
                Err(error) => println!("{name}: SOURCE_CHECK_REFUSED {error:?}"),
                Ok(skips) => return Err(format!("{name}: forged fixture unexpectedly checked; skips={skips:?}").into()),
            }
        }
    }
    Ok(())
}
