[package]
name = "native-plonk"
version = "0.1.0"
edition = "2021"
[dependencies]
anyhow = "1"
libloading = "0.8"
native-plonk-packer = { path = "../polys-packer" }
midnight-cuda-packager = { path = "../../packager" }
[dev-dependencies]
rand\_chacha = "0.3"
rand\_core = "0.6"
ff = "0.13"
group = "0.13"
serialize = { path = "../../sources/midnight-ledger/serialize", package = "midnight-serialize" }
midnight-curves = { path = "../../third\_party/midnight-zk/curves" }
midnight-proofs = { path = "../../third\_party/midnight-zk/proofs" }
midnight-circuits = { path = "../../third\_party/midnight-zk/circuits" }
midnight-cuda-witness = { path = "../../witness" }
transient-crypto = { path = "../../sources/midnight-ledger/transient-crypto", package = "midnight-transient-crypto" }
blake2b\_simd = "1"
zkir = { path = "../../sources/midnight-ledger/zkir", package = "zkir" }
[patch."https://github.com/midnightntwrk/midnight-zk"]
midnight-curves = { path = "../../third\_party/midnight-zk/curves" }
midnight-proofs = { path = "../../third\_party/midnight-zk/proofs" }
midnight-circuits = { path = "../../third\_party/midnight-zk/circuits" }