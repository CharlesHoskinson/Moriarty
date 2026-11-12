//! SRS loading, row accounting and file helpers.

use std::{fs, io::Cursor, path::Path};

use midnight_curves::Bls12;
use midnight_proofs::{poly::kzg::params::ParamsKZG, utils::SerdeFormat};

/// The Midnight ceremony SRS file `bls_midnight_2p<k>` from `~/.cache/midnight/zk-params`
/// (or `$E5_SRS_DIR`), downsized to `k`.
pub fn srs(k: u32) -> ParamsKZG<Bls12> {
    let dir = std::env::var("E5_SRS_DIR").unwrap_or_else(|_| {
        format!("{}/.cache/midnight/zk-params", std::env::var("HOME").unwrap())
    });
    let bytes = fs::read(Path::new(&dir).join(format!("bls_midnight_2p{k}"))).expect("SRS file");
    let mut p = ParamsKZG::<Bls12>::read_custom(&mut Cursor::new(bytes), SerdeFormat::RawBytesUnchecked)
        .expect("SRS");
    p.downsize(k);
    p
}

/// The k = 21 row gate of the E5 row (`2^21 - 7` usable rows at 85 percent).
pub const K21_GATE: usize = 1_782_573;
