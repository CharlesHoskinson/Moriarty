[package]
name = "native-plonk-packer"
version = "0.1.0"
edition = "2021"
[dependencies]
anyhow = "1"
blake2b\_simd = "1"
rand\_chacha = "0.3"
rand\_core = "0.6"
ff = "0.13"
group = "0.13"
libloading = "0.8"
midnight-curves = { path = "../../third\_party/midnight-zk/curves" }
midnight-proofs = { path = "../../third\_party/midnight-zk/proofs" }
midnight-circuits = { path = "../../third\_party/midnight-zk/circuits" }
midnight-cuda-witness = { path = "../../witness" }
zkir = { path = "../../sources/midnight-ledger/zkir" }
serialize = { path = "../../sources/midnight-ledger/serialize", package = "midnight-serialize" }
transient-crypto = { path = "../../sources/midnight-ledger/transient-crypto", package = "midnight-transient-crypto" }
[dev-dependencies]
rand = "0.8"
midnight-cuda-packager = { path = "../../packager" }