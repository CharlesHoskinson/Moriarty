//! B2 state replay of an FSYN S0 corpus with the in-repository I1 oracle.
//!
//! Usage: `i1_replay CORPUS`. Replays every header of `CORPUS/i1/blocks` from
//! genesis, compares each result to `CORPUS/i1/expected`, then binds the batch
//! chain of `CORPUS/i1/batches.json` to the replayed states and record hashes.
//! Exit 0 only when every check passes; the first failure goes to stderr.
use mori_recursion::rollup::i1::{
    oracle::{Oracle, validate_batches},
    types::{BatchStatement, Expected, Genesis, Header},
};
use std::{fs, path::Path, process::ExitCode};

fn read<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let raw = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_slice(&raw).map_err(|e| format!("{}: {e}", path.display()))
}

fn replay(root: &Path) -> Result<(), String> {
    let i1 = root.join("i1");
    let genesis: Genesis = read(&i1.join("genesis.json"))?;
    let mut oracle = Oracle::from_genesis(&genesis).map_err(|e| format!("oracle genesis: {e}"))?;
    let mut names: Vec<String> = fs::read_dir(i1.join("blocks"))
        .map_err(|e| format!("i1/blocks: {e}"))?
        .map(|entry| entry.map(|e| e.file_name().to_string_lossy().into_owned()))
        .collect::<Result<_, _>>()
        .map_err(|e| format!("i1/blocks: {e}"))?;
    names.sort();
    let mut headers = Vec::new();
    let mut expected = Vec::new();
    let mut transactions = 0usize;
    for (i, name) in names.iter().enumerate() {
        if *name != format!("{:06}.json", i + 1) {
            return Err(format!(
                "i1/blocks is not a contiguous header sequence at {name}"
            ));
        }
        let header: Header = read(&i1.join("blocks").join(name))?;
        let want: Expected = read(&i1.join("expected").join(name))?;
        let got = oracle
            .apply_header(&header)
            .map_err(|e| format!("oracle header {}: {e}", header.height))?;
        if got != want {
            return Err(format!("oracle disagrees with expected/{name}"));
        }
        transactions += header.txs.len();
        headers.push(header);
        expected.push(want);
    }
    println!(
        "PASS i1 oracle replay ({} headers, {transactions} transactions)",
        headers.len()
    );
    let batches: Vec<BatchStatement> = read(&i1.join("batches.json"))?;
    validate_batches(&batches).map_err(|e| format!("oracle batch chain: {e}"))?;
    for b in &batches {
        let range = (b.h_first as usize)
            .checked_sub(1)
            .and_then(|first| expected.get(first..b.h_last as usize))
            .filter(|r| !r.is_empty())
            .ok_or_else(|| format!("batch {} heights outside the replay", b.batch_no))?;
        let records: Vec<_> = headers[b.h_first as usize - 1..b.h_last as usize]
            .iter()
            .flat_map(|h| h.record_hashes.iter().copied())
            .collect();
        let (first, last) = (&range[0], &range[range.len() - 1]);
        if b.state_pre != first.state_before
            || b.state_post != last.state_after
            || b.dep_acc_to != last.dep_acc_after
            || records != b.record_hashes
        {
            return Err(format!(
                "batch {} disagrees with the replayed states",
                b.batch_no
            ));
        }
    }
    let last = batches.last().map(|b| b.h_last).unwrap_or(0);
    if last != headers.len() as u64 {
        return Err(format!("batches cover {last} of {} headers", headers.len()));
    }
    println!(
        "PASS batch chain: {} batches bound to the replayed states and record hashes",
        batches.len()
    );
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: i1_replay CORPUS");
        return ExitCode::from(2);
    }
    match replay(Path::new(&args[1])) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
