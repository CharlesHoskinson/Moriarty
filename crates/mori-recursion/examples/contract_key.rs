//! Read-only extraction of an installed public verifier from canonical contract-state bytes.
use midnight_onchain_state::state::{ContractState, EntryPointBuf};
use midnight_storage::db::InMemoryDB;
use mori_recursion::util::{sha256_hex, vk_from_bytes};
use std::{error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    if args.len() != 3 {
        return Err("usage: contract_key STATE ENTRY_POINT RAW_VK_OUTPUT".into());
    }
    let state_path = PathBuf::from(&args[0]);
    if fs::metadata(&state_path)?.len() > 16 * 1024 * 1024 {
        return Err("contract-state input exceeds 16 MiB".into());
    }
    let raw = fs::read(state_path)?;
    let state: ContractState<InMemoryDB> = midnight_serialize::tagged_deserialize(&raw[..])?;
    let mut roundtrip = Vec::new();
    midnight_serialize::tagged_serialize(&state, &mut roundtrip)?;
    if roundtrip != raw {
        return Err("contract-state input is not canonical".into());
    }
    let name = args[1].to_str().ok_or("entry point must be UTF-8")?;
    let entry = EntryPointBuf::from(name.as_bytes());
    let operation = state
        .operations
        .get(&entry)
        .ok_or("entry point is absent")?;
    let verifier = operation.latest().ok_or("entry point has no v3 verifier")?;
    let bytes = verifier.original_bytes();
    let vk = vk_from_bytes(&bytes)?;
    let mut output = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    std::io::Write::write_all(&mut output, &bytes)?;
    println!(
        "{}",
        serde_json::json!({
            "entryPoint": name, "stateSHA256": sha256_hex(&raw), "vkSHA256": sha256_hex(&bytes),
            "vkBytes": bytes.len(), "k": vk.k(), "hasIr": operation.ir.is_some(),
        })
    );
    Ok(())
}
