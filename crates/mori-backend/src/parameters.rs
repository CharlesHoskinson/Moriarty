//! Offline SRS provenance. Hash the bounded immutable snapshot before upstream
//! RawBytesUnchecked parsing; the provider returns only this decoded snapshot.
use crate::{Error, error};
use midnight_transient_crypto::proofs::{ParamsProver, ParamsProverProvider};
use sha2::{Digest, Sha256};
use std::{
    io::{self, Cursor, Read},
    path::Path,
};

const MAX_PARAMETER_BYTES: u64 = 128 * 1024 * 1024;
// Copied as data from ledger revision 04c9c5d9bcebb8d4427d8589fb54d58a55599c14,
// base-crypto/src/data_provider.rs EXPECTED_DATA. Never trust a supplied digest.
const PINNED_HASHES: [&str; 20] = [
    "59b30b3114a34ccbbfb599376e178fb8d9b3366cae2174c2f1da20e75847f823",
    "bbe04fe3c70d0c138447cb086b4baddc30cb8bb2a004114bc02e6f739516280e",
    "80e15568fa1a0117db893239be7fa5e34a6bcc3a8c3bfa7709534b9cb88eb6c1",
    "4be827a6472193df80d8f08b4b25a85baef436fdd1965d89b6af89f4ec4e99e2",
    "232f401fad10c7ddf8828d2aa4c85c6506c5da09795998cecaeb9f75fc8f6ada",
    "0a1c9229f315fc1868ff25f668fb83aec4d09f4f23a706b5197c692c619d72c6",
    "cf2ad6be7d0fedf5bec2aaa35f6be4aca33053d74268fdf5aa54fcb2891ea6df",
    "e82ae890c080188355f37feaffe91372584cd810615082d9143d4dec0453fd9d",
    "909b707551eaaea79828e883cde6fc46ab15986c3b1d791bed462c9e2805c933",
    "b9009f1098bcefffec3c461ab3a5e3a17f7e5599f0f08c70fcdc55a89227bcbd",
    "46b2290933cbed4c378889e4ba971f1a92888331ffb09466acd4ff61a1e2cb42",
    "9901589d7956ff58be0d85569b2f455b77b58c3758026ffb5bbe4807000b96d1",
    "ef08eb3fcf62df8f72c515cffa027e681808b530cb016eea104115545ef6d5c8",
    "d3324910969c4cc54143b8045b649e5c3a4bd5fb7b8f85fe1b770f640ce1c803",
    "fc253016885ec830e97808c9ec920bb5cab5c21af590380a6cb5eb0538e2b244",
    "724c7c3d779148bb113c7ee9c034b2f27db16e6bdf315fde90105a9bad00b1de",
    "09c877216d6589b370263e18af40a030a901b41a7a7c37ef58c9901db41f05c6",
    "4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74",
    "e8436dc5d8b598f169c127c745135d889744007e6d384ff126df8d1332522f86",
    "8e8dc15c4362f05c912f1e770559a3945db3e58a374def416ed5d3e65ad5b10e",
];
pub(crate) struct OfflineParameters {
    k: u8,
    params: ParamsProver,
}
impl OfflineParameters {
    pub(crate) fn load(directory: &Path, k: u8) -> Result<Self, Error> {
        let expected = PINNED_HASHES
            .get(k as usize)
            .ok_or_else(|| error("parameters", "No pinned SRS for the circuit degree."))?;
        let path = directory.join(format!("bls_midnight_2p{k}"));
        // Nonblocking open avoids waiting for a FIFO writer before we can
        // inspect the opened descriptor. A pre-open stat alone has a race.
        #[cfg(unix)]
        let opened = {
            use std::os::unix::fs::OpenOptionsExt;
            std::fs::OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_NONBLOCK)
                .open(&path)
        };
        #[cfg(not(unix))]
        let opened = std::fs::File::open(&path);
        let file = opened
            .map_err(|e| error("parameters", format!("Cannot open explicit local SRS: {e}")))?;
        if !file
            .metadata()
            .map_err(|e| error("parameters", e.to_string()))?
            .is_file()
        {
            return Err(error("parameters", "SRS must be a regular file."));
        }
        let mut bytes = Vec::new();
        file.take(MAX_PARAMETER_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| error("parameters", e.to_string()))?;
        if bytes.len() as u64 > MAX_PARAMETER_BYTES {
            return Err(error("parameters", "SRS exceeds the 128 MiB byte cap."));
        }
        let actual = crate::proving::hex(&Sha256::digest(&bytes));
        if actual != *expected {
            return Err(error(
                "parameters",
                "Local SRS does not match the pinned upstream SHA-256.",
            ));
        }
        // Upstream read_custom begins with a little-endian u32 degree. Verify it
        // before the unchecked decoder performs any degree-dependent allocation.
        if bytes.get(..4) != Some(u32::from(k).to_le_bytes().as_slice()) {
            return Err(error(
                "parameters",
                "Pinned SRS degree disagrees with the circuit.",
            ));
        }
        let mut snapshot = Cursor::new(bytes);
        let params =
            ParamsProver::read(&mut snapshot).map_err(|e| error("parameters", e.to_string()))?;
        if snapshot.position() != snapshot.get_ref().len() as u64 {
            return Err(error(
                "parameters",
                "SRS decoder did not consume the pinned snapshot.",
            ));
        }
        Ok(Self { k, params })
    }
}
impl ParamsProverProvider for OfflineParameters {
    async fn get_params(&self, k: u8) -> io::Result<ParamsProver> {
        if k != self.k {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Offline provider refuses a different circuit degree.",
            ));
        }
        Ok(self.params.clone())
    }
}
