//! Offline standard SRS admission for source-derived local calls.
//! Hash the bounded immutable snapshot before upstream unchecked decoding.
use midnight_transient_crypto::proofs::{ParamsProver, ParamsProverProvider};
use sha2::{Digest, Sha256};
use std::{
    io::{self, Cursor, Read},
    path::Path,
};

const MAX_PARAMETER_BYTES: u64 = 128 * 1024 * 1024;
// Standard parameters from SDK revision 04c9c5d9bcebb8d4427d8589fb54d58a55599c14,
// base-crypto/src/data_provider.rs EXPECTED_DATA. Preserve the existing ledger
// consumer's 0..=19 envelope; caller-supplied hashes cannot configure trust.
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

#[derive(Clone)]
pub(crate) struct PinnedCallParameters {
    k: u8,
    params: ParamsProver,
    digest: &'static str,
    snapshot_bytes: u64,
}

impl PinnedCallParameters {
    pub(crate) fn load(
        directory: &Path,
        degree: u8,
        max_snapshot_bytes: u64,
    ) -> crate::Result<Self> {
        let expected = expected_digest(degree).ok_or_else(|| error("parameter-degree"))?;
        if max_snapshot_bytes == 0 || max_snapshot_bytes > MAX_PARAMETER_BYTES {
            return Err(error("parameter-byte-limit"));
        }
        let path = directory.join(format!("bls_midnight_2p{degree}"));
        // A pre-open stat can race. Nonblocking open also avoids waiting for a
        // FIFO writer before the opened descriptor can be rejected below.
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
        let file = opened.map_err(|_| error("parameter-open"))?;
        if !file
            .metadata()
            .map_err(|_| error("parameter-metadata"))?
            .is_file()
        {
            return Err(error("parameter-file-type"));
        }
        let mut bytes = Vec::new();
        // The cap has already been checked, so the sentinel addition cannot
        // overflow. File growth cannot make this snapshot exceed cap + 1.
        file.take(max_snapshot_bytes + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| error("parameter-read"))?;
        let snapshot_bytes = bytes.len() as u64;
        if snapshot_bytes > max_snapshot_bytes {
            return Err(error("parameter-size"));
        }
        if crate::codec::hex(&Sha256::digest(&bytes)) != expected {
            return Err(error("parameter-digest"));
        }
        check_degree_header(&bytes, degree)?;
        let mut snapshot = Cursor::new(bytes);
        let params = ParamsProver::read(&mut snapshot).map_err(|_| error("parameter-decode"))?;
        if snapshot.position() != snapshot_bytes {
            return Err(error("parameter-trailing-bytes"));
        }
        Ok(Self {
            k: degree,
            params,
            digest: expected,
            snapshot_bytes,
        })
    }

    pub(crate) fn params(&self) -> ParamsProver {
        self.params.clone()
    }

    pub(crate) fn degree(&self) -> u8 {
        self.k
    }

    pub(crate) fn digest(&self) -> &str {
        self.digest
    }

    pub(crate) fn snapshot_bytes(&self) -> u64 {
        self.snapshot_bytes
    }
}

impl ParamsProverProvider for PinnedCallParameters {
    async fn get_params(&self, k: u8) -> io::Result<ParamsProver> {
        if k != self.k {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Pinned call parameters refuse a different degree.",
            ));
        }
        Ok(self.params.clone())
    }
}

pub(crate) fn expected_digest(degree: u8) -> Option<&'static str> {
    PINNED_HASHES.get(usize::from(degree)).copied()
}

fn check_degree_header(bytes: &[u8], degree: u8) -> crate::Result<()> {
    // Upstream read_custom starts with a little-endian u32 degree, which
    // controls allocations during unchecked decoding. Check it first.
    if bytes.get(..4) != Some(u32::from(degree).to_le_bytes().as_slice()) {
        return Err(error("parameter-degree-header"));
    }
    Ok(())
}

fn error(code: &'static str) -> crate::Error {
    crate::Error::new("parameters", code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };

    static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TemporaryDirectory(PathBuf);

    impl TemporaryDirectory {
        fn new() -> Self {
            loop {
                let path = std::env::temp_dir().join(format!(
                    "mori-ledger-parameter-refusal-{}-{}",
                    std::process::id(),
                    NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed),
                ));
                match fs::create_dir(&path) {
                    Ok(()) => return Self(path),
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                    Err(error) => panic!("cannot create temporary test directory: {error}"),
                }
            }
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn write(&self, degree: u8, bytes: &[u8]) {
            fs::write(self.path().join(format!("bls_midnight_2p{degree}")), bytes).unwrap();
        }
    }

    impl Drop for TemporaryDirectory {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    fn refused(directory: &Path, degree: u8, max_bytes: u64, code: &'static str) {
        match PinnedCallParameters::load(directory, degree, max_bytes) {
            Err(error) => assert_eq!(error, crate::Error::new("parameters", code)),
            Ok(_) => panic!("invalid temporary parameters were admitted"),
        }
    }

    #[test]
    fn pinned_digest_range_cannot_expand() {
        assert_eq!(
            expected_digest(0),
            Some("59b30b3114a34ccbbfb599376e178fb8d9b3366cae2174c2f1da20e75847f823")
        );
        assert_eq!(
            expected_digest(19),
            Some("8e8dc15c4362f05c912f1e770559a3945db3e58a374def416ed5d3e65ad5b10e")
        );
        for degree in 0..=19 {
            let digest = expected_digest(degree).unwrap();
            assert_eq!(digest.len(), 64);
            assert!(
                digest
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            );
        }
        for degree in 20..=u8::MAX {
            assert_eq!(expected_digest(degree), None);
        }
    }

    #[test]
    fn unsupported_degree_refuses_before_file_open() {
        let directory = TemporaryDirectory::new();
        for degree in 20..=u8::MAX {
            refused(
                &directory.path().join("missing"),
                degree,
                4,
                "parameter-degree",
            );
        }
    }

    #[test]
    fn zero_snapshot_limit_refuses_before_file_open() {
        let directory = TemporaryDirectory::new();
        refused(
            &directory.path().join("missing"),
            4,
            0,
            "parameter-byte-limit",
        );
    }

    #[test]
    fn snapshot_limit_above_hard_cap_refuses_before_file_open() {
        let directory = TemporaryDirectory::new();
        for max_bytes in [MAX_PARAMETER_BYTES + 1, u64::MAX] {
            refused(
                &directory.path().join("missing"),
                4,
                max_bytes,
                "parameter-byte-limit",
            );
        }
    }

    #[test]
    fn missing_supported_degree_has_no_fallback() {
        let directory = TemporaryDirectory::new();
        refused(directory.path(), 19, MAX_PARAMETER_BYTES, "parameter-open");
    }

    #[test]
    fn different_degree_filename_is_not_considered() {
        let directory = TemporaryDirectory::new();
        directory.write(5, &5_u32.to_le_bytes());
        refused(directory.path(), 4, 4, "parameter-open");
    }

    #[cfg(unix)]
    #[test]
    fn opened_directory_is_refused_as_nonregular() {
        let directory = TemporaryDirectory::new();
        fs::create_dir(directory.path().join("bls_midnight_2p4")).unwrap();
        refused(directory.path(), 4, 4, "parameter-file-type");
    }

    #[cfg(unix)]
    #[test]
    fn fifo_without_writer_is_refused_without_waiting() {
        use std::{ffi::CString, os::unix::ffi::OsStrExt};
        let directory = TemporaryDirectory::new();
        let path = directory.path().join("bls_midnight_2p4");
        let path = CString::new(path.as_os_str().as_bytes()).unwrap();
        // SAFETY: path is NUL terminated and is an owned, nonexistent temporary leaf.
        assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
        refused(directory.path(), 4, 4, "parameter-file-type");
    }

    #[test]
    fn oversized_tiny_snapshot_refuses_before_digest() {
        let directory = TemporaryDirectory::new();
        directory.write(4, &[0; 5]);
        refused(directory.path(), 4, 4, "parameter-size");
    }

    #[test]
    fn exact_byte_bound_still_checks_pinned_digest() {
        let directory = TemporaryDirectory::new();
        directory.write(4, &[0; 4]);
        refused(directory.path(), 4, 4, "parameter-digest");
    }

    #[test]
    fn untrusted_digest_refuses_even_with_valid_degree_header() {
        let directory = TemporaryDirectory::new();
        directory.write(4, &4_u32.to_le_bytes());
        refused(directory.path(), 4, 4, "parameter-digest");
    }

    #[test]
    fn invalid_degree_header_is_refused_without_decoding() {
        for bytes in [
            &[][..],
            &[4][..],
            &[4, 0][..],
            &[4, 0, 0][..],
            &[5, 0, 0, 0][..],
            &[0, 0, 0, 4][..],
            &[255; 4][..],
        ] {
            assert_eq!(
                check_degree_header(bytes, 4),
                Err(crate::Error::new("parameters", "parameter-degree-header"))
            );
        }
    }

    #[test]
    fn valid_little_endian_degree_header_is_checked_without_decoding() {
        for degree in 0..=19 {
            let mut bytes = u32::from(degree).to_le_bytes().to_vec();
            assert_eq!(check_degree_header(&bytes, degree), Ok(()));
            bytes.push(255);
            assert_eq!(check_degree_header(&bytes, degree), Ok(()));
        }
    }
}
