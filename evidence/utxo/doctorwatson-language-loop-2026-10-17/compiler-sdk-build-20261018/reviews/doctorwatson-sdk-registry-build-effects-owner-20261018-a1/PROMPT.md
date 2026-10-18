Designated security OWNER HIGH, explicitly authorized Gemini4ArgonHigh outage fallback after Opus4.8 rate rejection06:54:51UTC/no recovery. SOURCE ONLY publicpacket /home/charl/moriarty-c2-opus55-restored-20261017/compiler-sdk-source-inspection/registry-owner-packet. No commands/source execution/Cargo/build/SDK/runtime/proof/SRS/key/VM/network/privatefiles/delegatedagents. Return one whole sixfield object matching schema token doctorwatson-sdk-registry-build-effects-owner-20261018-a1-whole-review. This is bounded source inspection, not runtime/command/resource/native admission. Never treat broad source collections or lock adjacency as selected Cargo feature/target graph. Preserve prior timeout/whole reviews as history. Read additional public files only as necessary, not the entire1912file corpus. Primary goal: BUILD-TIME side effects and exact unresolved prerequisites for future -p mori-backend --test stages exact compiler/IrSource::load test. Inspect registry build/proc-macro declarations and actual relevant build.rs (especially blst/libc/Serde), plus SDK build.rs. Distinguish declared subprocess/env/file/tool lookups from runtime proof/key operations. Determine whether onchain-runtime/scheme is actually reachable from backend dependencies; do not assign unrelated workspace scripts to backend. Manual lock closure is conservative, not selected Cargo graph. Exact21+13registryarchives are included, not every transitive package. ffi/SOURCE-INPUT-REFERENCES.json is available for targetedRead, not inlined due its size; omission fromprimary is explicit, no gap suppressed. Identify exact missing active SDKregistry manifest/build/proc source and generated inputs. State smallest substantive next source/resource preconditions for compiler-only command under owned coldtarget/offline/-j1/noHOME/publiccaches/noSRS/prover reads, but do NOT approve command/budget or infer hosttools/config/CPU featureselection. Source acquisition is not execution authority. Cite file/line/function and honest actualreadlimits; unverified leaves remain open. Primary inventories and scripts follow.

--- FILE moriarty/crates/mori-backend/Cargo.toml ---
[package]
name = "mori-backend"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true

[dependencies]
mori-core.workspace = true
midnight-zkir-v3.workspace = true
midnight-transient-crypto.workspace = true
midnight-curves.workspace = true
serde.workspace = true
serde_json.workspace = true
sha2.workspace = true
thiserror.workspace = true
futures.workspace = true
rand.workspace = true

[dev-dependencies]
insta.workspace = true
proptest.workspace = true

[target.'cfg(unix)'.dependencies]
libc.workspace = true


--- FILE sdk-public-code/zkir-v3/Cargo.toml ---
[package]
name = "midnight-zkir-v3"
version = "3.0.0-rc.2"
edition = "2024"
license.workspace = true
repository.workspace = true
publish = false

[[bin]]
name = "zkir"
path = "src/main.rs"
required-features = ["binary"]

[features]
binary = [
  "dep:log",
  "dep:tracing-subscriber",
  "dep:clap",
  "dep:env_logger",
  "dep:tokio",
  "dep:indicatif",
  "dep:console",
  "dep:futures-executor",
]
proptest = [
  "dep:pastey",
  "dep:proptest",
  "dep:proptest-derive",
  "base-crypto/proptest",
  "transient-crypto/proptest",
  "serialize/proptest",
]

[dependencies]
base-crypto = { version = "1.1.0", path = "../base-crypto", package = "midnight-base-crypto" }
serialize = { version = "1.0.0", path = "../serialize", package = "midnight-serialize" }
transient-crypto = { version = "3.0.0", path = "../transient-crypto", package = "midnight-transient-crypto", features = [
  "cli",
] }

midnight-curves = { version = "^0.3.1" }
midnight-proofs = { version = "^0.8.1" }
midnight-circuits = { version = "^7.2.2" }
midnight-zk-stdlib = { version = "^2.3.3" }

group = "^0.13.0"
const-hex = "^1.14.0"
serde = { version = "^1.0.0", features = ["derive", "rc"] }
serde_bytes = { version = "^0.11.0" }
serde_json = { version = "^1.0.0" }
serde_repr = "^0.1.0"
anyhow = "^1.0.102"
hex = "^0.4.3"
sha2 = "^0.11.0"
sha3 = "^0.11.0"
# rand 0.9.0 is not yet supported by some of our dependencies, we will
# stick with 0.8.0 for now.
rand = "^0.8.0"
rand_chacha = "^0.3.1"
tracing = { version = "^0.1.41" }

log = { version = "^0.4.28", optional = true }
tracing-subscriber = { version = "^0.3.19", optional = true }
clap = { version = "^4.5.40", features = ["derive"], optional = true }
env_logger = { version = "^0.11.8", optional = true }
tokio = { version = "^1.46.1", features = [
  "rt",
  "macros",
  "rt-multi-thread",
], optional = true }
indicatif = { version = "^0.18.4", optional = true }
console = { version = "^0.16.3", optional = true }
futures-executor = { version = "^0.3.32", optional = true }
num-bigint = { version = "0.4" }
num-traits = "0.2"

pastey = { version = "0.2.1", optional = true }
proptest = { version = "1.4.0", optional = true }
proptest-derive = { version = "0.8", optional = true }

[dev-dependencies]
midnight-zkir-v3 = { path = ".", features = ["proptest"] }
actix-rt = "^2.9.0"


--- FILE sdk-public-code/transient-crypto/Cargo.toml ---
[package]
name = "midnight-transient-crypto"
version = "3.0.0"
edition = "2024"
license.workspace = true
repository.workspace = true
description = "Cryptographic primitives for Midnight that may change over time."

[features]
default = []
cli = ["base-crypto/cli"]
proptest = [
  "dep:proptest",
  "dep:proptest-derive",
  "base-crypto/proptest",
  "serialize/proptest",
  "storage-core/proptest",
]
mock-verify = []
public-internal-structure = []

[dependencies]
base-crypto = { version = "1.1.0", path = "../base-crypto", package = "midnight-base-crypto" }
derive = { version = "1.0.0", path = "../base-crypto-derive", package = "midnight-base-crypto-derive" }
serialize = { version = "1.0.0", path = "../serialize", package = "midnight-serialize" }
storage-core = { version = "1.0.0", path = "../storage-core", package = "midnight-storage-core" }

ff = "^0.13.1"
group = "^0.13.0"
midnight-curves = { version = "^0.3.1" }
midnight-proofs = { version = "^0.8.1" }
midnight-circuits = { version = "^7.2.2" }
midnight-zk-stdlib = { version = "^2.3.3" }
anyhow = { version = "^1.0.102" }
lru = "0.18.0"

# rand 0.9.0 is not yet supported by some of our dependencies, we will
# stick with 0.8.0 for now.
rand = { version = "^0.8.4", features = ["getrandom"] }
lazy_static = "^1.5.0"
tracing = { version = "^0.1.41" }
const-hex = "^1.14.1"
sha2 = "^0.11.0"
k256 = "^0.13.4"
signature = "^2.2.0"
zeroize = "^1.8.0"

derive-where = "^1.6.1"
serde = { version = "^1.0.219", features = ["derive", "rc"] }
serde_bytes = { version = "^0.11.17" }
serde_json = { version = "^1.0.140" }
fake = { version = "^2.10.0", features = ["derive"] }
pastey = "0.2.1"
# proptest 1.7.0+ rely on rand 0.9.0, which we are not yet updating to.
proptest = { version = "~1.6.0", optional = true }
proptest-derive = { version = "0.8", optional = true }
blake2b_simd = "1.0.3"

futures = { version = "^0.3.31" }

[dev-dependencies]
midnight-transient-crypto = { path = ".", features = ["proptest"] }
actix-rt = "^2.10.0"
rand_chacha = "^0.3.1"
tracing-subscriber = "0.3.19"
criterion = { version = "^0.5.1" }
rayon = "^1.10.0"

[[bench]]
name = "benchmarking"
harness = false

[[bench]]
name = "merkle_deserialize"
harness = false

[[example]]
name = "calibrate"
path = "examples/calibrate.rs"
required-features = ["mock-verify"]


--- FILE sdk-public-code/onchain-runtime/Cargo.toml ---
[package]
name = "midnight-onchain-runtime"
version = "4.0.0"
edition = "2024"
license.workspace = true
repository.workspace = true
description = "The full smart contract runtime for Midnight's ledger."

[features]
default = ["vendored"]
vendored = []
proptest = ["dep:proptest", "dep:proptest-derive", "coin-structure/proptest", "onchain-runtime-state/proptest", "onchain-vm/proptest"]

[dependencies]
coin-structure = { version = "3.0.0", path = "../coin-structure", package = "midnight-coin-structure" }
onchain-runtime-state = { version = "4.0.0", path = "../onchain-state", package = "midnight-onchain-state" }
onchain-vm = { version = "4.0.0", path = "../onchain-vm", package = "midnight-onchain-vm" }
storage = { version = "2.0.0", path = "../storage", package = "midnight-storage" }
serialize = { version = "1.0.0", path = "../serialize", package = "midnight-serialize" }
base-crypto = { version = "1.1.0", path = "../base-crypto", package = "midnight-base-crypto" }
transient-crypto = { version = "3.0.0", path = "../transient-crypto", package = "midnight-transient-crypto" }

derive-where = "^1.6.1"
enum_index = "^0.2.0"
enum_index_derive = "^0.2.0"

tracing = { version = "^0.1.41" }
lazy_static = "^1.5.0"
hex = "^0.4.3"
konst = "0.4.3"
# rand 0.9.0 is not yet supported by some of our dependencies, we will
# stick with 0.8.0 for now.
rand = { version = "^0.8.5", features = ["getrandom"] }

fake = { version = "^2.10.0", features = ["derive"] }
serde = { version = "^1.0.219", features = ["derive"] }
serde_bytes = { version = "^0.11.17" }
# proptest 1.7.0+ rely on rand 0.9.0, which we are not yet updating to.
proptest = { version = "~1.6.0", optional = true }
proptest-derive = { version = "0.8", optional = true }

[dev-dependencies]
midnight-onchain-runtime = { path = ".", features = ["proptest"] }
rand = { version = "^0.8.5", features = ["getrandom"] }
criterion = "0.5"
pastey = "0.2.1"
serde_json = { version = "^1.0", features = ["std"] }
itertools = "0.14"

[[bench]]
name = "benchmarking"
harness = false


--- FILE sdk-public-code/onchain-runtime/build.rs ---
// This file is part of midnight-ledger.
// Copyright (C) Midnight Foundation
// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;
use std::fs::File;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../flake.lock");
    println!("cargo:rerun-if-changed=generate-rust-macros.ss");

    let vendored: String = env::var("CARGO_FEATURE_VENDORED").unwrap_or("0".into());
    let is_vendored = vendored == "1";

    if !is_vendored {
        let macro_out = Path::new(&"./vendored").join("program_fragments.rs");
        assert!(
            Command::new("scheme")
                .arg("--script")
                .arg("generate-rust-macros.ss")
                .stdout(File::create(macro_out).unwrap())
                .status()
                .unwrap()
                .success()
        );
    }
}


--- FILE registry/BUILD-PROC-MACRO-INVENTORY.json ---
{
  "status": "SOURCE-ONLY DECLARATION INVENTORY; NO MACRO/BUILD EXECUTION OR SECURITY JUDGMENT",
  "packages": [
    {
      "package": "const-hex-1.19.3",
      "root_manifest": "public-code/const-hex-1.19.3/Cargo.toml",
      "root_manifest_sha256": "1a2345244020dc12756ac47afa1f147e56615b748dc8d48fc4758401d8016a07",
      "package_build_declaration": false,
      "effective_source_build_path": null,
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "cfg-if": {
          "version": "1"
        },
        "proptest": {
          "version": "1.4",
          "optional": true,
          "default-features": false
        },
        "serde_core": {
          "version": "1.0",
          "optional": true,
          "default-features": false
        }
      },
      "target_normal_dependency_declarations": {
        "cfg(any(target_arch = \"x86\", target_arch = \"x86_64\"))": {
          "cpufeatures": {
            "version": "0.2"
          }
        }
      },
      "lock_dependencies": [
        "cfg-if",
        "cpufeatures 0.2.17",
        "proptest",
        "serde_core"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "ff-0.13.1",
      "root_manifest": "public-code/ff-0.13.1/Cargo.toml",
      "root_manifest_sha256": "3dc57e64db6bcffb9ee83ecd1e732d0b7be93a1ab2c881ff83220748ed0f704f",
      "package_build_declaration": null,
      "effective_source_build_path": null,
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "bitvec": {
          "version": "1",
          "optional": true,
          "default-features": false
        },
        "byteorder": {
          "version": "1",
          "optional": true,
          "default-features": false
        },
        "ff_derive": {
          "version": "0.13.1",
          "optional": true
        },
        "rand_core": {
          "version": "0.6",
          "default-features": false
        },
        "subtle": {
          "version": "2.2.1",
          "features": [
            "i128"
          ],
          "default-features": false
        }
      },
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [
        "bitvec",
        "rand_core 0.6.4",
        "subtle"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "midnight-curves-0.2.1",
      "root_manifest": "public-code/midnight-curves-0.2.1/Cargo.toml",
      "root_manifest_sha256": "d4d02d1b2c69e8817cb5f94a315508cfd42a846b93ed350cbaf5767d25f7ddb0",
      "package_build_declaration": false,
      "effective_source_build_path": null,
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "bitvec": {
          "version": "1.0.1"
        },
        "blst": {
          "version": "0.3.15",
          "default-features": true
        },
        "byte-slice-cast": {
          "version": "1.2.3"
        },
        "digest": {
          "version": "0.10"
        },
        "ff": {
          "version": "0.13"
        },
        "group": {
          "version": "0.13",
          "features": [
            "tests"
          ]
        },
        "halo2derive": {
          "version": "0.2.0"
        },
        "hex": {
          "version": "0.4.3",
          "features": [
            "serde"
          ],
          "optional": true
        },
        "lazy_static": {
          "version": "1.4"
        },
        "num-bigint": {
          "version": "0.4.6"
        },
        "pairing": {
          "version": "0.23"
        },
        "paste": {
          "version": "1.0"
        },
        "rand_core": {
          "version": "0.6",
          "default-features": false
        },
        "rayon": {
          "version": "1.10"
        },
        "serde": {
          "version": "1.0",
          "features": [
            "derive"
          ],
          "optional": true
        },
        "serde_arrays": {
          "version": "0.2.0",
          "optional": true
        },
        "sha2": {
          "version": "0.10"
        },
        "subtle": {
          "version": "2.6"
        }
      },
      "target_normal_dependency_declarations": {
        "cfg(all(target_arch = \"wasm32\", target_os = \"unknown\"))": {
          "getrandom": {
            "version": "0.2",
            "features": [
              "js"
            ]
          }
        }
      },
      "lock_dependencies": [
        "bitvec",
        "blst",
        "byte-slice-cast",
        "digest 0.10.7",
        "ff",
        "getrandom 0.2.17",
        "group",
        "halo2derive",
        "hex",
        "lazy_static",
        "num-bigint",
        "pairing",
        "paste",
        "rand_core 0.6.4",
        "rayon",
        "serde",
        "serde_arrays",
        "sha2 0.10.9",
        "subtle"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "midnight-curves-0.3.1",
      "root_manifest": "public-code/midnight-curves-0.3.1/Cargo.toml",
      "root_manifest_sha256": "a0fe142ec9ad49ec635b9f53dfa4a4b259f5fb8cedeabef1a76a5f8ce0f3aee7",
      "package_build_declaration": false,
      "effective_source_build_path": null,
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "bitvec": {
          "version": "1.0.1"
        },
        "blst": {
          "version": "0.3.15",
          "default-features": true
        },
        "byte-slice-cast": {
          "version": "1.2.3"
        },
        "curve25519-dalek": {
          "version": "4.1.3",
          "features": [
            "group"
          ]
        },
        "digest": {
          "version": "0.10"
        },
        "ff": {
          "version": "0.13"
        },
        "group": {
          "version": "0.13",
          "features": [
            "tests"
          ]
        },
        "halo2derive": {
          "version": "0.2.0",
          "optional": true
        },
        "hex": {
          "version": "0.4.3",
          "features": [
            "serde"
          ],
          "optional": true
        },
        "k256": {
          "version": "0.13.4",
          "features": [
            "expose-field",
            "bits",
            "alloc"
          ],
          "default-features": false
        },
        "lazy_static": {
          "version": "1.4"
        },
        "num-bigint": {
          "version": "0.4.6"
        },
        "p256": {
          "version": "0.13.2",
          "features": [
            "expose-field",
            "bits",
            "alloc",
            "arithmetic"
          ],
          "default-features": false
        },
        "pairing": {
          "version": "0.23"
        },
        "paste": {
          "version": "1.0"
        },
        "primeorder": {
          "version": "0.13"
        },
        "rand_core": {
          "version": "0.6",
          "default-features": false
        },
        "rayon": {
          "version": "1.10"
        },
        "serde": {
          "version": "1.0",
          "features": [
            "derive"
          ],
          "optional": true
        },
        "serde_arrays": {
          "version": "0.2.0",
          "optional": true
        },
        "sha2": {
          "version": "0.10"
        },
        "subtle": {
          "version": "2.6"
        }
      },
      "target_normal_dependency_declarations": {
        "cfg(all(target_arch = \"wasm32\", target_os = \"unknown\"))": {
          "getrandom": {
            "version": "0.2",
            "features": [
              "js"
            ]
          }
        }
      },
      "lock_dependencies": [
        "bitvec",
        "blst",
        "byte-slice-cast",
        "curve25519-dalek",
        "digest 0.10.7",
        "ff",
        "getrandom 0.2.17",
        "group",
        "hex",
        "k256",
        "lazy_static",
        "num-bigint",
        "p256",
        "pairing",
        "paste",
        "primeorder",
        "rand_core 0.6.4",
        "rayon",
        "serde",
        "serde_arrays",
        "sha2 0.10.9",
        "subtle"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "serde-1.0.229",
      "root_manifest": "public-code/serde-1.0.229/Cargo.toml",
      "root_manifest_sha256": "6883caefd46e86eb15d22d3a6d274ad304c5e7934ab70eb58d81b7c4a29ef9db",
      "package_build_declaration": "build.rs",
      "effective_source_build_path": "build.rs",
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [
        "public-code/serde-1.0.229/build.rs"
      ],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "serde_core": {
          "version": "=1.0.229",
          "features": [
            "result"
          ],
          "default-features": false
        },
        "serde_derive": {
          "version": "1",
          "optional": true
        }
      },
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [
        "serde_core",
        "serde_derive"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "serde_core-1.0.229",
      "root_manifest": "public-code/serde_core-1.0.229/Cargo.toml",
      "root_manifest_sha256": "c6049e170e4f7c2b48749fefab59db583bfac4e466cde6d0c53e4f49bc5044f7",
      "package_build_declaration": "build.rs",
      "effective_source_build_path": "build.rs",
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [
        "public-code/serde_core-1.0.229/build.rs"
      ],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {},
      "target_normal_dependency_declarations": {
        "cfg(any())": {
          "serde_derive": {
            "version": "=1.0.229"
          }
        }
      },
      "lock_dependencies": [
        "serde_derive"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "serde_derive-1.0.229",
      "root_manifest": "public-code/serde_derive-1.0.229/Cargo.toml",
      "root_manifest_sha256": "d2581160ffa61baa386f882e08b0db20051dc3c782a82268d6473998b61c926e",
      "package_build_declaration": false,
      "effective_source_build_path": null,
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [],
      "lib_proc_macro_declaration": true,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [
        {
          "file": "public-code/serde_derive-1.0.229/src/lib.rs",
          "line": 113,
          "declaration_start": "#[proc_macro_derive(Serialize, attributes(serde))]",
          "kind": "source-text attribute occurrence; no expansion/evaluation"
        },
        {
          "file": "public-code/serde_derive-1.0.229/src/lib.rs",
          "line": 121,
          "declaration_start": "#[proc_macro_derive(Deserialize, attributes(serde))]",
          "kind": "source-text attribute occurrence; no expansion/evaluation"
        }
      ],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "proc-macro2": {
          "version": "1.0.74",
          "features": [
            "proc-macro"
          ],
          "default-features": false
        },
        "quote": {
          "version": "1.0.35",
          "features": [
            "proc-macro"
          ],
          "default-features": false
        },
        "syn": {
          "version": "3",
          "features": [
            "clone-impls",
            "derive",
            "parsing",
            "printing",
            "proc-macro"
          ],
          "default-features": false
        }
      },
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [
        "proc-macro2",
        "quote 1.0.47",
        "syn 3.0.6"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "serde_json-1.0.151",
      "root_manifest": "public-code/serde_json-1.0.151/Cargo.toml",
      "root_manifest_sha256": "00b920255c320a278bf25b962b491263a19d8c90e12e4a91c1d823db71eabb2f",
      "package_build_declaration": "build.rs",
      "effective_source_build_path": "build.rs",
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [
        "public-code/serde_json-1.0.151/build.rs"
      ],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "indexmap": {
          "version": "2.2.3",
          "optional": true
        },
        "itoa": {
          "version": "1.0"
        },
        "memchr": {
          "version": "2",
          "default-features": false
        },
        "serde_core": {
          "version": "1.0.220",
          "default-features": false
        },
        "zmij": {
          "version": "1.0"
        }
      },
      "target_normal_dependency_declarations": {
        "cfg(any())": {
          "serde": {
            "version": "1.0.220",
            "default-features": false
          }
        }
      },
      "lock_dependencies": [
        "itoa",
        "memchr",
        "serde",
        "serde_core",
        "zmij"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "serde_repr-0.1.21",
      "root_manifest": "public-code/serde_repr-0.1.21/Cargo.toml",
      "root_manifest_sha256": "dfa3951971fcf5409988c1be9dcd45848022a7c8aa4aac683fe8200511f798d5",
      "package_build_declaration": false,
      "effective_source_build_path": null,
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [],
      "lib_proc_macro_declaration": true,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [
        {
          "file": "public-code/serde_repr-0.1.21/src/lib.rs",
          "line": 50,
          "declaration_start": "#[proc_macro_derive(Serialize_repr)]",
          "kind": "source-text attribute occurrence; no expansion/evaluation"
        },
        {
          "file": "public-code/serde_repr-0.1.21/src/lib.rs",
          "line": 80,
          "declaration_start": "#[proc_macro_derive(Deserialize_repr, attributes(serde))]",
          "kind": "source-text attribute occurrence; no expansion/evaluation"
        }
      ],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "proc-macro2": {
          "version": "1.0.74"
        },
        "quote": {
          "version": "1.0.35"
        },
        "syn": {
          "version": "3"
        }
      },
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [
        "proc-macro2",
        "quote 1.0.47",
        "syn 3.0.6"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "subtle-2.6.1",
      "root_manifest": "public-code/subtle-2.6.1/Cargo.toml",
      "root_manifest_sha256": "a3b9aefa358b0cd742de62e880ea080414ead838847a95892acaa58575dfc4cf",
      "package_build_declaration": false,
      "effective_source_build_path": null,
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {},
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "autocfg-1.5.1",
      "root_manifest": "public-code/autocfg-1.5.1/Cargo.toml",
      "root_manifest_sha256": "ed7c832b4250c3be736e040d68688c2b83a5b164a93ef533e61532cbe0d44e99",
      "package_build_declaration": false,
      "effective_source_build_path": null,
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {},
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "halo2derive-0.2.0",
      "root_manifest": "public-code/halo2derive-0.2.0/Cargo.toml",
      "root_manifest_sha256": "a22f655a7a01036e45ea672a509f4adde78053498f84d0948b25b86f78c1a8b8",
      "package_build_declaration": null,
      "effective_source_build_path": null,
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [],
      "lib_proc_macro_declaration": true,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [
        {
          "file": "public-code/halo2derive-0.2.0/src/lib.rs",
          "line": 6,
          "declaration_start": "#[proc_macro]",
          "kind": "source-text attribute occurrence; no expansion/evaluation"
        }
      ],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "num-bigint": {
          "version": "0.4",
          "default-features": false
        },
        "num-integer": {
          "version": "0.1",
          "default-features": false
        },
        "num-traits": {
          "version": "0.2",
          "default-features": false
        },
        "proc-macro2": {
          "version": "1.0",
          "default-features": false
        },
        "quote": {
          "version": "1.0",
          "default-features": false
        },
        "syn": {
          "version": "1.0",
          "features": [
            "full"
          ]
        }
      },
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [
        "num-bigint",
        "num-integer",
        "num-traits",
        "proc-macro2",
        "quote 1.0.47",
        "syn 1.0.109"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "num-bigint-0.4.8",
      "root_manifest": "public-code/num-bigint-0.4.8/Cargo.toml",
      "root_manifest_sha256": "fcb19e142df64dc58a459ba33e1e22c1551497962f8037ed98df124d749f28f1",
      "package_build_declaration": false,
      "effective_source_build_path": null,
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "arbitrary": {
          "version": "1",
          "optional": true,
          "default-features": false
        },
        "num-integer": {
          "version": "0.1.46",
          "features": [
            "i128"
          ],
          "default-features": false
        },
        "num-traits": {
          "version": "0.2.18",
          "features": [
            "i128"
          ],
          "default-features": false
        },
        "quickcheck": {
          "version": "1",
          "optional": true,
          "default-features": false
        },
        "rand": {
          "version": "0.8",
          "optional": true,
          "default-features": false
        },
        "serde": {
          "version": "1.0",
          "optional": true,
          "default-features": false
        }
      },
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [
        "num-integer",
        "num-traits",
        "rand 0.8.8"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "num-integer-0.1.47",
      "root_manifest": "public-code/num-integer-0.1.47/Cargo.toml",
      "root_manifest_sha256": "a0fc4aee99b0663bb80ce2fc8052491b7e28872765711266faddb2ed0474cb96",
      "package_build_declaration": false,
      "effective_source_build_path": null,
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "num-traits": {
          "version": "0.2.11",
          "features": [
            "i128"
          ],
          "default-features": false
        }
      },
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [
        "num-traits"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "num-traits-0.2.19",
      "root_manifest": "public-code/num-traits-0.2.19/Cargo.toml",
      "root_manifest_sha256": "dd701a0da6cce7e6e047cc944b2330f3c5e48cd64329e10942ea7d9299d435be",
      "package_build_declaration": "build.rs",
      "effective_source_build_path": "build.rs",
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [
        "public-code/num-traits-0.2.19/build.rs"
      ],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {
        "autocfg": {
          "version": "1"
        }
      },
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "libm": {
          "version": "0.2.0",
          "optional": true
        }
      },
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [
        "autocfg"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "paste-1.0.15",
      "root_manifest": "public-code/paste-1.0.15/Cargo.toml",
      "root_manifest_sha256": "b639e399e32159609ed573ffdb1a4edf06fba8aee208d9259e9dba031834aaaa",
      "package_build_declaration": "build.rs",
      "effective_source_build_path": "build.rs",
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [
        "public-code/paste-1.0.15/build.rs"
      ],
      "lib_proc_macro_declaration": true,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [
        {
          "file": "public-code/paste-1.0.15/src/lib.rs",
          "line": 164,
          "declaration_start": "#[proc_macro]",
          "kind": "source-text attribute occurrence; no expansion/evaluation"
        },
        {
          "file": "public-code/paste-1.0.15/src/lib.rs",
          "line": 185,
          "declaration_start": "#[proc_macro]",
          "kind": "source-text attribute occurrence; no expansion/evaluation"
        },
        {
          "file": "public-code/paste-1.0.15/src/lib.rs",
          "line": 191,
          "declaration_start": "#[proc_macro]",
          "kind": "source-text attribute occurrence; no expansion/evaluation"
        }
      ],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {},
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "proc-macro2-1.0.107",
      "root_manifest": "public-code/proc-macro2-1.0.107/Cargo.toml",
      "root_manifest_sha256": "6bdbe54a7f052e552dcfca90351764476e2767237ec3f954df8bd9f602861056",
      "package_build_declaration": "build.rs",
      "effective_source_build_path": "build.rs",
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [
        "public-code/proc-macro2-1.0.107/build.rs"
      ],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "unicode-ident": {
          "version": "1.0"
        }
      },
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [
        "unicode-ident"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "quote-1.0.47",
      "root_manifest": "public-code/quote-1.0.47/Cargo.toml",
      "root_manifest_sha256": "13d8c50837f14aff48cd622349f98fb2c46350b7a0c08fd574cf7de4def27d06",
      "package_build_declaration": "build.rs",
      "effective_source_build_path": "build.rs",
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [
        "public-code/quote-1.0.47/build.rs"
      ],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "proc-macro2": {
          "version": "1.0.80",
          "default-features": false
        }
      },
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [
        "proc-macro2"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "syn-1.0.109",
      "root_manifest": "public-code/syn-1.0.109/Cargo.toml",
      "root_manifest_sha256": "139a7e42e4c95f9730d4dfe05dd5e09ee59f713eacc310921a61d9fdfc0bdce0",
      "package_build_declaration": null,
      "effective_source_build_path": "build.rs",
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [
        "public-code/syn-1.0.109/build.rs"
      ],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "proc-macro2": {
          "version": "1.0.46",
          "default-features": false
        },
        "quote": {
          "version": "1.0",
          "optional": true,
          "default-features": false
        },
        "unicode-ident": {
          "version": "1.0"
        }
      },
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [
        "proc-macro2",
        "quote 1.0.47",
        "unicode-ident"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "syn-3.0.6",
      "root_manifest": "public-code/syn-3.0.6/Cargo.toml",
      "root_manifest_sha256": "69b7b5a4f75d37bac9745ced65bf9b715ddc4ecb32308df7d796c55e9cbae40f",
      "package_build_declaration": false,
      "effective_source_build_path": null,
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {
        "proc-macro2": {
          "version": "1.0.91",
          "default-features": false
        },
        "quote": {
          "version": "1.0.35",
          "optional": true,
          "default-features": false
        },
        "unicode-ident": {
          "version": "1"
        }
      },
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [
        "proc-macro2",
        "quote 1.0.47",
        "unicode-ident"
      ],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    },
    {
      "package": "unicode-ident-1.0.26",
      "root_manifest": "public-code/unicode-ident-1.0.26/Cargo.toml",
      "root_manifest_sha256": "f70dabc0fe831788a202762e62cc76c19941680960e2e2710d1ee9e79877d9af",
      "package_build_declaration": false,
      "effective_source_build_path": null,
      "effective_source_build_copied": true,
      "all_build_rs_source_members": [],
      "lib_proc_macro_declaration": false,
      "lib_path": "src/lib.rs",
      "proc_macro_attribute_occurrences": [],
      "build_dependency_declarations": {},
      "target_build_dependency_declarations": {},
      "normal_dependency_declarations": {},
      "target_normal_dependency_declarations": {},
      "lock_dependencies": [],
      "scope": "source declarations only; no selected feature/target resolution and no build/proc-macro behavior judgment"
    }
  ]
}


--- FILE ffi/BUILD-PROC-MACRO-INVENTORY.json ---
{
  "status": "SOURCE DECLARATIONS ONLY, not executed or feature/target resolved",
  "packages": [
    {
      "package": "blst-0.3.17",
      "manifest_path": "public-code/blst-0.3.17/Cargo.toml",
      "package_build_declaration": "build.rs",
      "effective_root_build_source": "build.rs",
      "effective_build_source_copied": true,
      "all_build_rs_members": [
        "build.rs"
      ],
      "proc_macro_library": false,
      "proc_macro_attribute_declarations": [],
      "manifest_dependency_declarations": [
        {
          "target": null,
          "kind": "dependencies",
          "alias": "serde",
          "declaration": {
            "version": "1.0.152",
            "optional": true
          }
        },
        {
          "target": null,
          "kind": "dependencies",
          "alias": "zeroize",
          "declaration": {
            "version": "^1.1",
            "features": [
              "zeroize_derive"
            ]
          }
        },
        {
          "target": null,
          "kind": "build-dependencies",
          "alias": "cc",
          "declaration": {
            "version": "1.0"
          }
        },
        {
          "target": null,
          "kind": "dev-dependencies",
          "alias": "rand_chacha",
          "declaration": {
            "version": "0.3"
          }
        },
        {
          "target": null,
          "kind": "dev-dependencies",
          "alias": "rand_core",
          "declaration": {
            "version": "0.6",
            "features": [
              "std"
            ]
          }
        },
        {
          "target": null,
          "kind": "dev-dependencies",
          "alias": "rmp-serde",
          "declaration": {
            "version": "1.1.1"
          }
        },
        {
          "target": "cfg(any(unix, windows))",
          "kind": "dev-dependencies",
          "alias": "criterion",
          "declaration": {
            "version": "0.3"
          }
        },
        {
          "target": "cfg(not(any(target_arch=\"wasm32\", target_os=\"none\", target_os=\"unknown\", target_os=\"uefi\")))",
          "kind": "dependencies",
          "alias": "threadpool",
          "declaration": {
            "version": "^1.8.1"
          }
        },
        {
          "target": "cfg(target_env = \"msvc\")",
          "kind": "build-dependencies",
          "alias": "glob",
          "declaration": {
            "version": "0.3"
          }
        }
      ],
      "lock_dependency_entries": [
        {
          "lock_dependency": "cc",
          "locked_identities": [
            {
              "name": "cc",
              "version": "1.7.0",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "50a649af8a827553c29fb0cb4bd4a6f1a0dd695bd3232b9bc98bd9c8a3ffbb8b",
              "source_packet": "this extension"
            }
          ]
        },
        {
          "lock_dependency": "glob",
          "locked_identities": [
            {
              "name": "glob",
              "version": "0.3.4",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "e4eba85ea1d0a966a983acd07deee566e67395d2d96b6fb39e62b5a833f1eb0b",
              "source_packet": "this extension"
            }
          ]
        },
        {
          "lock_dependency": "threadpool",
          "locked_identities": [
            {
              "name": "threadpool",
              "version": "1.8.1",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "d050e60b33d41c19108b32cea32164033a9013fe3b46cbd4457559bfbf77afaa",
              "source_packet": "this extension"
            }
          ]
        },
        {
          "lock_dependency": "zeroize",
          "locked_identities": [
            {
              "name": "zeroize",
              "version": "1.9.1",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "e13084392c5e4bc371903e2935a5eaeed24905a7511356b883835e18a78f6879",
              "source_packet": "this extension"
            }
          ]
        }
      ],
      "features_as_declared": {
        "default": [],
        "force-adx": [],
        "no-threads": [],
        "portable": [],
        "serde-secret": [
          "serde"
        ]
      }
    },
    {
      "package": "cc-1.7.0",
      "manifest_path": "public-code/cc-1.7.0/Cargo.toml",
      "package_build_declaration": false,
      "effective_root_build_source": null,
      "effective_build_source_copied": null,
      "all_build_rs_members": [],
      "proc_macro_library": false,
      "proc_macro_attribute_declarations": [],
      "manifest_dependency_declarations": [
        {
          "target": null,
          "kind": "dependencies",
          "alias": "find-msvc-tools",
          "declaration": {
            "version": "0.1.14"
          }
        },
        {
          "target": null,
          "kind": "dependencies",
          "alias": "jobserver",
          "declaration": {
            "version": "0.1.32",
            "optional": true,
            "default-features": false
          }
        },
        {
          "target": null,
          "kind": "dependencies",
          "alias": "shlex",
          "declaration": {
            "version": "2.0.1"
          }
        },
        {
          "target": null,
          "kind": "dev-dependencies",
          "alias": "tempfile",
          "declaration": {
            "version": "3.0.4"
          }
        },
        {
          "target": "cfg(unix)",
          "kind": "dependencies",
          "alias": "libc",
          "declaration": {
            "version": "0.2.168",
            "optional": true,
            "default-features": false
          }
        }
      ],
      "lock_dependency_entries": [
        {
          "lock_dependency": "find-msvc-tools",
          "locked_identities": [
            {
              "name": "find-msvc-tools",
              "version": "0.1.14",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "aedcfb3409746eddb02b9e19ebda1c3394f759a152e48ee875a0844d1b955484",
              "source_packet": "this extension"
            }
          ]
        },
        {
          "lock_dependency": "jobserver",
          "locked_identities": [
            {
              "name": "jobserver",
              "version": "0.1.35",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "1c00acbd29eabad4a2392fa0e921c874934dbbf4194312ad20f04a0ed67a3cb3",
              "source_packet": "this extension"
            }
          ]
        },
        {
          "lock_dependency": "libc",
          "locked_identities": [
            {
              "name": "libc",
              "version": "0.2.190",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "ce5d3ddc6d3fa000eb1536d85e147bfe31aacaba692ed6a876f95cb7c855be78",
              "source_packet": "this extension"
            }
          ]
        },
        {
          "lock_dependency": "shlex",
          "locked_identities": [
            {
              "name": "shlex",
              "version": "2.0.1",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "f8fadd59c855ef2080decdef8ff161eb6661b86933c9d82e5ba29dc602a55aba",
              "source_packet": "this extension"
            }
          ]
        }
      ],
      "features_as_declared": {
        "jobserver": [],
        "parallel": [
          "dep:jobserver",
          "dep:libc"
        ]
      }
    },
    {
      "package": "find-msvc-tools-0.1.14",
      "manifest_path": "public-code/find-msvc-tools-0.1.14/Cargo.toml",
      "package_build_declaration": false,
      "effective_root_build_source": null,
      "effective_build_source_copied": null,
      "all_build_rs_members": [],
      "proc_macro_library": false,
      "proc_macro_attribute_declarations": [],
      "manifest_dependency_declarations": [],
      "lock_dependency_entries": [],
      "features_as_declared": {}
    },
    {
      "package": "glob-0.3.4",
      "manifest_path": "public-code/glob-0.3.4/Cargo.toml",
      "package_build_declaration": false,
      "effective_root_build_source": null,
      "effective_build_source_copied": null,
      "all_build_rs_members": [],
      "proc_macro_library": false,
      "proc_macro_attribute_declarations": [],
      "manifest_dependency_declarations": [
        {
          "target": null,
          "kind": "dev-dependencies",
          "alias": "doc-comment",
          "declaration": {
            "version": "0.3"
          }
        },
        {
          "target": null,
          "kind": "dev-dependencies",
          "alias": "tempfile",
          "declaration": {
            "version": "3"
          }
        }
      ],
      "lock_dependency_entries": [],
      "features_as_declared": {}
    },
    {
      "package": "hermit-abi-0.5.3",
      "manifest_path": "public-code/hermit-abi-0.5.3/Cargo.toml",
      "package_build_declaration": false,
      "effective_root_build_source": null,
      "effective_build_source_copied": null,
      "all_build_rs_members": [],
      "proc_macro_library": false,
      "proc_macro_attribute_declarations": [],
      "manifest_dependency_declarations": [
        {
          "target": null,
          "kind": "dependencies",
          "alias": "alloc",
          "declaration": {
            "version": "1.0.0",
            "optional": true,
            "package": "rustc-std-workspace-alloc"
          }
        },
        {
          "target": null,
          "kind": "dependencies",
          "alias": "core",
          "declaration": {
            "version": "1.0.0",
            "optional": true,
            "package": "rustc-std-workspace-core"
          }
        }
      ],
      "lock_dependency_entries": [],
      "features_as_declared": {
        "default": [],
        "rustc-dep-of-std": [
          "core",
          "alloc"
        ]
      }
    },
    {
      "package": "jobserver-0.1.35",
      "manifest_path": "public-code/jobserver-0.1.35/Cargo.toml",
      "package_build_declaration": false,
      "effective_root_build_source": null,
      "effective_build_source_copied": null,
      "all_build_rs_members": [],
      "proc_macro_library": false,
      "proc_macro_attribute_declarations": [],
      "manifest_dependency_declarations": [
        {
          "target": null,
          "kind": "dev-dependencies",
          "alias": "tempfile",
          "declaration": {
            "version": "3.10.1"
          }
        },
        {
          "target": "cfg(unix)",
          "kind": "dependencies",
          "alias": "libc",
          "declaration": {
            "version": "0.2.171"
          }
        },
        {
          "target": "cfg(unix)",
          "kind": "dev-dependencies",
          "alias": "nix",
          "declaration": {
            "version": "0.31.3",
            "features": [
              "fs"
            ]
          }
        },
        {
          "target": "cfg(windows)",
          "kind": "dependencies",
          "alias": "getrandom",
          "declaration": {
            "version": "0.4",
            "features": [
              "std"
            ]
          }
        }
      ],
      "lock_dependency_entries": [
        {
          "lock_dependency": "getrandom 0.4.3",
          "locked_identities": [
            {
              "name": "getrandom",
              "version": "0.4.3",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "300e883d756b2e4ec94e02791f39b04b522276138852cfc41d9fb7e904106099",
              "source_packet": "not copied in these packets"
            }
          ]
        },
        {
          "lock_dependency": "libc",
          "locked_identities": [
            {
              "name": "libc",
              "version": "0.2.190",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "ce5d3ddc6d3fa000eb1536d85e147bfe31aacaba692ed6a876f95cb7c855be78",
              "source_packet": "this extension"
            }
          ]
        }
      ],
      "features_as_declared": {}
    },
    {
      "package": "libc-0.2.190",
      "manifest_path": "public-code/libc-0.2.190/Cargo.toml",
      "package_build_declaration": "build.rs",
      "effective_root_build_source": "build.rs",
      "effective_build_source_copied": true,
      "all_build_rs_members": [
        "build.rs"
      ],
      "proc_macro_library": false,
      "proc_macro_attribute_declarations": [],
      "manifest_dependency_declarations": [
        {
          "target": null,
          "kind": "dependencies",
          "alias": "rustc-std-workspace-core",
          "declaration": {
            "version": "1.0.1",
            "optional": true
          }
        }
      ],
      "lock_dependency_entries": [],
      "features_as_declared": {
        "align": [],
        "const-extern-fn": [],
        "default": [
          "std"
        ],
        "extra_traits": [],
        "rustc-dep-of-std": [
          "align",
          "rustc-std-workspace-core"
        ],
        "std": [],
        "use_std": [
          "std"
        ]
      }
    },
    {
      "package": "num_cpus-1.17.0",
      "manifest_path": "public-code/num_cpus-1.17.0/Cargo.toml",
      "package_build_declaration": false,
      "effective_root_build_source": null,
      "effective_build_source_copied": null,
      "all_build_rs_members": [],
      "proc_macro_library": false,
      "proc_macro_attribute_declarations": [],
      "manifest_dependency_declarations": [
        {
          "target": "cfg(not(windows))",
          "kind": "dependencies",
          "alias": "libc",
          "declaration": {
            "version": "0.2.26"
          }
        },
        {
          "target": "cfg(target_os = \"hermit\")",
          "kind": "dependencies",
          "alias": "hermit-abi",
          "declaration": {
            "version": "0.5.0"
          }
        }
      ],
      "lock_dependency_entries": [
        {
          "lock_dependency": "hermit-abi",
          "locked_identities": [
            {
              "name": "hermit-abi",
              "version": "0.5.3",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "e17592d60ebacc7d5e169f4663c5f84f9161cc90328abcfe8456f41e4dfcb284",
              "source_packet": "this extension"
            }
          ]
        },
        {
          "lock_dependency": "libc",
          "locked_identities": [
            {
              "name": "libc",
              "version": "0.2.190",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "ce5d3ddc6d3fa000eb1536d85e147bfe31aacaba692ed6a876f95cb7c855be78",
              "source_packet": "this extension"
            }
          ]
        }
      ],
      "features_as_declared": {}
    },
    {
      "package": "shlex-2.0.1",
      "manifest_path": "public-code/shlex-2.0.1/Cargo.toml",
      "package_build_declaration": false,
      "effective_root_build_source": null,
      "effective_build_source_copied": null,
      "all_build_rs_members": [],
      "proc_macro_library": false,
      "proc_macro_attribute_declarations": [],
      "manifest_dependency_declarations": [],
      "lock_dependency_entries": [],
      "features_as_declared": {
        "default": [
          "std"
        ],
        "std": []
      }
    },
    {
      "package": "syn-2.0.119",
      "manifest_path": "public-code/syn-2.0.119/Cargo.toml",
      "package_build_declaration": false,
      "effective_root_build_source": null,
      "effective_build_source_copied": null,
      "all_build_rs_members": [],
      "proc_macro_library": false,
      "proc_macro_attribute_declarations": [
        {
          "file": "src/error.rs",
          "line": 45,
          "text": "/// #[proc_macro_attribute]"
        },
        {
          "file": "src/error.rs",
          "line": 85,
          "text": "/// #[proc_macro_derive(MyDerive)]"
        },
        {
          "file": "src/error.rs",
          "line": 251,
          "text": "/// #[proc_macro_derive(MyTrait)]"
        },
        {
          "file": "src/lib.rs",
          "line": 80,
          "text": "//! #[proc_macro_derive(MyMacro)]"
        },
        {
          "file": "src/meta.rs",
          "line": 13,
          "text": "/// `#[proc_macro_attribute]` macro."
        },
        {
          "file": "src/meta.rs",
          "line": 48,
          "text": "/// #[proc_macro_attribute]"
        },
        {
          "file": "src/meta.rs",
          "line": 103,
          "text": "/// #[proc_macro_attribute]"
        },
        {
          "file": "src/parse.rs",
          "line": 81,
          "text": "//! #[proc_macro]"
        },
        {
          "file": "src/parse.rs",
          "line": 1351,
          "text": "/// #[proc_macro_attribute]"
        },
        {
          "file": "src/parse_macro_input.rs",
          "line": 37,
          "text": "/// #[proc_macro]"
        },
        {
          "file": "src/parse_macro_input.rs",
          "line": 73,
          "text": "/// #[proc_macro]"
        },
        {
          "file": "src/spanned.rs",
          "line": 32,
          "text": "//! #[proc_macro_derive(MyMacro)]"
        }
      ],
      "manifest_dependency_declarations": [
        {
          "target": null,
          "kind": "dependencies",
          "alias": "proc-macro2",
          "declaration": {
            "version": "1.0.91",
            "default-features": false
          }
        },
        {
          "target": null,
          "kind": "dependencies",
          "alias": "quote",
          "declaration": {
            "version": "1.0.35",
            "optional": true,
            "default-features": false
          }
        },
        {
          "target": null,
          "kind": "dependencies",
          "alias": "unicode-ident",
          "declaration": {
            "version": "1"
          }
        },
        {
          "target": null,
          "kind": "dev-dependencies",
          "alias": "anyhow",
          "declaration": {
            "version": "1"
          }
        },
        {
          "target": null,
          "kind": "dev-dependencies",
          "alias": "automod",
          "declaration": {
            "version": "1"
          }
        },
        {
          "target": null,
          "kind": "dev-dependencies",
          "alias": "insta",
          "declaration": {
            "version": "1"
          }
        },
        {
          "target": null,
          "kind": "dev-dependencies",
          "alias": "ref-cast",
          "declaration": {
            "version": "1"
          }
        },
        {
          "target": null,
          "kind": "dev-dependencies",
          "alias": "rustversion",
          "declaration": {
            "version": "1"
          }
        },
        {
          "target": null,
          "kind": "dev-dependencies",
          "alias": "syn-test-suite",
          "declaration": {
            "version": "0"
          }
        },
        {
          "target": null,
          "kind": "dev-dependencies",
          "alias": "termcolor",
          "declaration": {
            "version": "1"
          }
        },
        {
          "target": "cfg(not(miri))",
          "kind": "dev-dependencies",
          "alias": "flate2",
          "declaration": {
            "version": "1"
          }
        },
        {
          "target": "cfg(not(miri))",
          "kind": "dev-dependencies",
          "alias": "rayon",
          "declaration": {
            "version": "1"
          }
        },
        {
          "target": "cfg(not(miri))",
          "kind": "dev-dependencies",
          "alias": "reqwest",
          "declaration": {
            "version": "0.13",
            "features": [
              "blocking"
            ]
          }
        },
        {
          "target": "cfg(not(miri))",
          "kind": "dev-dependencies",
          "alias": "tar",
          "declaration": {
            "version": "0.4.16"
          }
        },
        {
          "target": "cfg(not(miri))",
          "kind": "dev-dependencies",
          "alias": "walkdir",
          "declaration": {
            "version": "2.3.2"
          }
        }
      ],
      "lock_dependency_entries": [
        {
          "lock_dependency": "proc-macro2",
          "locked_identities": [
            {
              "name": "proc-macro2",
              "version": "1.0.107",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "985e7ec9bb745e6ce6535b544d84d6cd6f7ad8bd711c398938ae983b91a766d9",
              "source_packet": "original frozen packet"
            }
          ]
        },
        {
          "lock_dependency": "quote 1.0.47",
          "locked_identities": [
            {
              "name": "quote",
              "version": "1.0.47",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "1fbf4db142a473a8d80c26bbf18454ed458bf8d26c8219c331daecfdbd079001",
              "source_packet": "original frozen packet"
            }
          ]
        },
        {
          "lock_dependency": "unicode-ident",
          "locked_identities": [
            {
              "name": "unicode-ident",
              "version": "1.0.26",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "d245f478577f809a851594d02313b640fb437e0bb33866753cff937863096954",
              "source_packet": "original frozen packet"
            }
          ]
        }
      ],
      "features_as_declared": {
        "clone-impls": [],
        "default": [
          "derive",
          "parsing",
          "printing",
          "clone-impls",
          "proc-macro"
        ],
        "derive": [],
        "extra-traits": [],
        "fold": [],
        "full": [],
        "parsing": [],
        "printing": [
          "dep:quote"
        ],
        "proc-macro": [
          "proc-macro2/proc-macro",
          "quote?/proc-macro"
        ],
        "test": [
          "syn-test-suite/all-features"
        ],
        "visit": [],
        "visit-mut": []
      }
    },
    {
      "package": "threadpool-1.8.1",
      "manifest_path": "public-code/threadpool-1.8.1/Cargo.toml",
      "package_build_declaration": null,
      "effective_root_build_source": null,
      "effective_build_source_copied": null,
      "all_build_rs_members": [],
      "proc_macro_library": false,
      "proc_macro_attribute_declarations": [],
      "manifest_dependency_declarations": [
        {
          "target": null,
          "kind": "dependencies",
          "alias": "num_cpus",
          "declaration": {
            "version": "1.13"
          }
        }
      ],
      "lock_dependency_entries": [
        {
          "lock_dependency": "num_cpus",
          "locked_identities": [
            {
              "name": "num_cpus",
              "version": "1.17.0",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "91df4bbde75afed763b708b7eee1e8e7651e02d97f6d5dd763e89367e957b23b",
              "source_packet": "this extension"
            }
          ]
        }
      ],
      "features_as_declared": {}
    },
    {
      "package": "zeroize-1.9.1",
      "manifest_path": "public-code/zeroize-1.9.1/Cargo.toml",
      "package_build_declaration": false,
      "effective_root_build_source": null,
      "effective_build_source_copied": null,
      "all_build_rs_members": [],
      "proc_macro_library": false,
      "proc_macro_attribute_declarations": [],
      "manifest_dependency_declarations": [
        {
          "target": null,
          "kind": "dependencies",
          "alias": "serde",
          "declaration": {
            "version": "1",
            "optional": true,
            "default-features": false
          }
        },
        {
          "target": null,
          "kind": "dependencies",
          "alias": "zeroize_derive",
          "declaration": {
            "version": "1.5",
            "optional": true
          }
        }
      ],
      "lock_dependency_entries": [
        {
          "lock_dependency": "zeroize_derive",
          "locked_identities": [
            {
              "name": "zeroize_derive",
              "version": "1.5.0",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "3c50655cbb0fe3fc43170059e702f1ce5e19b84cec58dc87b037a09935c2f328",
              "source_packet": "this extension"
            }
          ]
        }
      ],
      "features_as_declared": {
        "aarch64": [],
        "alloc": [],
        "default": [
          "alloc"
        ],
        "derive": [
          "zeroize_derive"
        ],
        "simd": [],
        "std": [
          "alloc"
        ]
      }
    },
    {
      "package": "zeroize_derive-1.5.0",
      "manifest_path": "public-code/zeroize_derive-1.5.0/Cargo.toml",
      "package_build_declaration": false,
      "effective_root_build_source": null,
      "effective_build_source_copied": null,
      "all_build_rs_members": [],
      "proc_macro_library": true,
      "proc_macro_attribute_declarations": [
        {
          "file": "src/lib.rs",
          "line": 33,
          "text": "#[proc_macro_derive(Zeroize, attributes(zeroize))]"
        },
        {
          "file": "src/lib.rs",
          "line": 94,
          "text": "#[proc_macro_derive(ZeroizeOnDrop, attributes(zeroize))]"
        }
      ],
      "manifest_dependency_declarations": [
        {
          "target": null,
          "kind": "dependencies",
          "alias": "proc-macro2",
          "declaration": {
            "version": "1"
          }
        },
        {
          "target": null,
          "kind": "dependencies",
          "alias": "quote",
          "declaration": {
            "version": "1"
          }
        },
        {
          "target": null,
          "kind": "dependencies",
          "alias": "syn",
          "declaration": {
            "version": "2",
            "features": [
              "full",
              "extra-traits",
              "visit"
            ]
          }
        }
      ],
      "lock_dependency_entries": [
        {
          "lock_dependency": "proc-macro2",
          "locked_identities": [
            {
              "name": "proc-macro2",
              "version": "1.0.107",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "985e7ec9bb745e6ce6535b544d84d6cd6f7ad8bd711c398938ae983b91a766d9",
              "source_packet": "original frozen packet"
            }
          ]
        },
        {
          "lock_dependency": "quote 1.0.47",
          "locked_identities": [
            {
              "name": "quote",
              "version": "1.0.47",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "1fbf4db142a473a8d80c26bbf18454ed458bf8d26c8219c331daecfdbd079001",
              "source_packet": "original frozen packet"
            }
          ]
        },
        {
          "lock_dependency": "syn 2.0.119",
          "locked_identities": [
            {
              "name": "syn",
              "version": "2.0.119",
              "source": "registry+https://github.com/rust-lang/crates.io-index",
              "checksum": "872831b642d1a07999a962a351ed35b955ea2cfc8f3862091e2a240a84f17297",
              "source_packet": "this extension"
            }
          ]
        }
      ],
      "features_as_declared": {}
    }
  ],
  "root_build_source_input_inventory": [
    {
      "package": "blst-0.3.17",
      "build_source": "build.rs",
      "source_input": "blst/src/server.c",
      "status": "copied",
      "notes": "cc files source; transitive C/header text copied without compiler preprocessing"
    },
    {
      "package": "blst-0.3.17",
      "build_source": "build.rs",
      "source_input": "blst/build/assembly.S",
      "status": "copied",
      "notes": "assembly() non-MSVC branch; no target selection claim"
    },
    {
      "package": "blst-0.3.17",
      "build_source": "build.rs",
      "source_input": "blst/build/win64/*-{x86_64,armv8,unknown}.asm",
      "status": "archive matching normal assembly sources copied",
      "notes": "MSVC glob constructed from architecture; unknown pattern may match nothing; never evaluated by package code"
    },
    {
      "package": "blst-0.3.17",
      "build_source": "build.rs",
      "source_input": "blst/src and blst/build",
      "status": "all allowlisted archive source files copied; excluded members in provenance",
      "notes": "rerun-if-changed directories; filesystem never probed by package code"
    },
    {
      "package": "blst-0.3.17",
      "build_source": "build.rs",
      "source_input": "libblst.a",
      "status": "not copied; absent from archive member inventory",
      "notes": "optional prebuilt archive existence branch; binary excluded by policy; no filesystem lookup performed"
    },
    {
      "package": "blst-0.3.17",
      "build_source": "build.rs",
      "source_input": "CARGO_MANIFEST_DIR/../.. fallback blst repository root",
      "status": "not copied or inspected",
      "notes": "dynamic fallback if packaged blst directory absent; outside archive source namespace; no external root resolution performed"
    },
    {
      "package": "blst-0.3.17",
      "build_source": "build.rs",
      "source_input": "target/compiler/CC/CARGO_* environment, compiler tools and cc::Build generated probes/output",
      "status": "not acquired/resolved/executed",
      "notes": "dynamic build inputs, not static archive source files; full source review remains with designated owner"
    },
    {
      "package": "libc-0.2.190",
      "build_source": "build.rs",
      "source_input": "build.rs",
      "status": "copied",
      "notes": "literal rerun-if-changed input"
    },
    {
      "package": "libc-0.2.190",
      "build_source": "build.rs",
      "source_input": "RUSTC/RUSTC_WRAPPER, target/config environment, command outputs from version queries",
      "status": "not acquired/resolved/executed",
      "notes": "dynamic inputs; no additional literal external source-file reads observed in this root build script; no build/side-effect classification"
    }
  ],
  "source_reference_lexical_inventory_file": "SOURCE-INPUT-REFERENCES.json",
  "limits": "Every root build.rs/proc-macro declaration in selected packages inventoried. Lexical reference inventory is a review aid, not a complete semantic dependency analysis; dynamic generated paths, external tools, system headers and all execution effects remain unestablished."
}


--- FILE ffi/public-code/blst-0.3.17/build.rs ---
#![allow(unused_imports)]

extern crate cc;

use std::env;
use std::path::{Path, PathBuf};

fn assembly(
    file_vec: &mut Vec<PathBuf>,
    base_dir: &Path,
    _arch: &str,
    _is_msvc: bool,
) {
    #[cfg(target_env = "msvc")]
    if _is_msvc {
        let sfx = match _arch {
            "x86_64" => "x86_64",
            "aarch64" => "armv8",
            _ => "unknown",
        };
        let files =
            glob::glob(&format!("{}/win64/*-{}.asm", base_dir.display(), sfx))
                .expect("unable to collect assembly files");
        for file in files {
            file_vec.push(file.unwrap());
        }
        return;
    }

    file_vec.push(base_dir.join("assembly.S"));
}

fn main() {
    if env::var("CARGO_FEATURE_SERDE_SECRET").is_ok() {
        println!(
            "cargo:warning=blst: non-production feature serde-secret enabled"
        );
    }

    // account for cross-compilation [by examining environment variables]
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();

    let target_no_std = target_os.eq("none")
        || (target_os.eq("unknown") && target_arch.eq("wasm32"))
        || target_os.eq("uefi")
        || env::var("BLST_TEST_NO_STD").is_ok();

    if !target_no_std {
        println!("cargo:rustc-cfg=feature=\"std\"");
        if target_arch.eq("wasm32") || target_os.eq("unknown") {
            println!("cargo:rustc-cfg=feature=\"no-threads\"");
        }
    }
    println!("cargo:rerun-if-env-changed=BLST_TEST_NO_STD");

    /*
     * Use pre-built libblst.a if there is one. This is primarily
     * for trouble-shooting purposes. Idea is that libblst.a can be
     * compiled with flags independent from cargo defaults, e.g.
     * '../../build.sh -O1 ...'.
     */
    if Path::new("libblst.a").exists() {
        println!("cargo:rustc-link-search=.");
        println!("cargo:rustc-link-lib=blst");
        println!("cargo:rerun-if-changed=libblst.a");
        return;
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut blst_base_dir = manifest_dir.join("blst");
    if !blst_base_dir.exists() {
        // Reach out to ../.., which is the root of the blst repo.
        // Use an absolute path to avoid issues with relative paths
        // being treated as strings by `cc` and getting concatenated
        // in ways that reach out of the OUT_DIR.
        blst_base_dir = manifest_dir
            .parent()
            .and_then(|dir| dir.parent())
            .expect("can't access parent of parent of current directory")
            .into();
    }
    println!("Using blst source directory {}", blst_base_dir.display());

    // Set CC environment variable to choose alternative C compiler.
    // Optimization level depends on whether or not --release is passed
    // or implied.

    if target_os.eq("uefi") && env::var("CC").is_err() {
        match std::process::Command::new("clang")
            .arg("--version")
            .output()
        {
            Ok(_) => env::set_var("CC", "clang"),
            Err(_) => { /* no clang in sight, just ignore the error */ }
        }
    }

    if target_env.eq("sgx") && env::var("CC").is_err() {
        match std::process::Command::new("clang")
            .arg("--version")
            .output()
        {
            Ok(out) => {
                let version = String::from_utf8(out.stdout)
                    .unwrap_or("unintelligible".to_string());
                if let Some(x) = version.find("clang version ") {
                    let x = x + 14;
                    let y = version[x..].find('.').unwrap_or(0);
                    if version[x..x + y].parse::<i32>().unwrap_or(0) >= 11 {
                        env::set_var("CC", "clang");
                    }
                }
            }
            Err(_) => { /* no clang in sight, just ignore the error */ }
        }
    }

    if target_env.eq("msvc")
        && env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap().eq("32")
        && env::var("CC").is_err()
    {
        match std::process::Command::new("clang-cl")
            .args(["-m32", "--version"])
            .output()
        {
            Ok(out) => {
                if String::from_utf8(out.stdout)
                    .unwrap_or("unintelligible".to_string())
                    .contains("Target: i386-pc-windows-msvc")
                {
                    env::set_var("CC", "clang-cl");
                }
            }
            Err(_) => { /* no clang-cl in sight, just ignore the error */ }
        }
    }

    let mut cc = cc::Build::new();

    let c_src_dir = blst_base_dir.join("src");
    println!("cargo:rerun-if-changed={}", c_src_dir.display());
    let mut file_vec = vec![c_src_dir.join("server.c")];

    if target_arch.eq("x86_64") || target_arch.eq("aarch64") {
        let asm_dir = blst_base_dir.join("build");
        println!("cargo:rerun-if-changed={}", asm_dir.display());
        assembly(
            &mut file_vec,
            &asm_dir,
            &target_arch,
            cc.get_compiler().is_like_msvc(),
        );
    } else {
        cc.define("__BLST_NO_ASM__", None);
    }
    match (cfg!(feature = "portable"), cfg!(feature = "force-adx")) {
        (true, false) => {
            if target_arch.eq("x86_64") && target_env.eq("sgx") {
                panic!("'portable' is not supported on SGX target");
            }
            println!("Compiling in portable mode without ISA extensions");
            cc.define("__BLST_PORTABLE__", None);
        }
        (false, true) => {
            if target_arch.eq("x86_64") {
                println!("Enabling ADX support via `force-adx` feature");
                cc.define("__ADX__", None);
            } else {
                println!("`force-adx` is ignored for non-x86_64 targets");
            }
        }
        (false, false) => {
            if target_arch.eq("x86_64") {
                if target_env.eq("sgx") {
                    println!("Enabling ADX for Intel SGX target");
                    cc.define("__ADX__", None);
                } else if env::var("CARGO_ENCODED_RUSTFLAGS")
                    .unwrap_or_default()
                    .contains("target-cpu=")
                {
                    // If target-cpu is specified on the rustc command line,
                    // then obey the resulting target-features.
                    let feat_list = env::var("CARGO_CFG_TARGET_FEATURE")
                        .unwrap_or_default();
                    let features: Vec<_> = feat_list.split(',').collect();
                    if !features.contains(&"ssse3") {
                        println!(
                            "Compiling in portable mode without ISA extensions"
                        );
                        cc.define("__BLST_PORTABLE__", None);
                    } else if features.contains(&"adx") {
                        println!(
                            "Enabling ADX because it was set as target-feature"
                        );
                        cc.define("__ADX__", None);
                    }
                } else {
                    #[cfg(target_arch = "x86_64")]
                    if std::is_x86_feature_detected!("adx") {
                        println!(
                            "Enabling ADX because it was detected on the host"
                        );
                        cc.define("__ADX__", None);
                    }
                }
            }
        }
        (true, true) => panic!(
            "Cannot compile with both `portable` and `force-adx` features"
        ),
    }
    if target_env.eq("msvc") && cc.get_compiler().is_like_msvc() {
        cc.flag("-Zl");
    }
    cc.flag_if_supported("-mno-avx") // avoid costly transitions
        .flag_if_supported("-fno-builtin")
        .flag_if_supported("-Wno-unused-function")
        .flag_if_supported("-Wno-unused-command-line-argument");
    if target_arch.eq("wasm32") || target_family.is_empty() {
        cc.flag("-ffreestanding");
    }
    if target_arch.eq("wasm32") || target_no_std {
        cc.define("SCRATCH_LIMIT", "(45 * 1024)");
    }
    if target_env.eq("sgx") {
        cc.flag_if_supported("-mlvi-hardening");
        cc.define("__SGX_LVI_HARDENING__", None);
        cc.define("__BLST_NO_CPUID__", None);
        cc.define("__ELF__", None);
        cc.define("SCRATCH_LIMIT", "(45 * 1024)");
    }
    if !cfg!(debug_assertions) {
        cc.opt_level(2);
    }
    cc.files(&file_vec).compile("blst");

    // pass some DEP_BLST_* variables to dependents
    println!(
        "cargo:BINDINGS={}",
        blst_base_dir.join("bindings").to_string_lossy()
    );
    println!("cargo:C_SRC={}", c_src_dir.to_string_lossy());
}


--- FILE ffi/public-code/blst-0.3.17/Cargo.toml ---
# THIS FILE IS AUTOMATICALLY GENERATED BY CARGO
#
# When uploading crates to the registry Cargo will automatically
# "normalize" Cargo.toml files for maximal compatibility
# with all versions of Cargo and also rewrite `path` dependencies
# to registry (e.g., crates.io) dependencies.
#
# If you are reading this file be aware that the original Cargo.toml
# will likely look very different (and much more reasonable).
# See Cargo.toml.orig for the original contents.

[package]
edition = "2018"
name = "blst"
version = "0.3.17"
authors = ["sean-sn <sean@supranational.net>"]
build = "build.rs"
links = "blst"
include = [
    "**/*.rs",
    "/Cargo.toml",
    "/README.md",
    "/rustfmt.toml",
    "/blst/src/*.c",
    "/blst/src/*.h*",
    "/blst/build/**",
    "/blst/bindings/blst.h",
    "/blst/bindings/blst_aux.h",
    "/blst/bindings/blst.hpp",
]
autolib = false
autobins = false
autoexamples = false
autotests = false
autobenches = false
description = "Bindings for blst BLS12-381 library"
readme = "README.md"
keywords = [
    "crypto",
    "bls",
    "signature",
    "asm",
    "wasm",
]
categories = ["cryptography"]
license = "Apache-2.0"
repository = "https://github.com/supranational/blst"

[badges.maintenance]
status = "actively-developed"

[features]
default = []
force-adx = []
no-threads = []
portable = []
serde-secret = ["serde"]

[lib]
name = "blst"
path = "src/lib.rs"

[[bench]]
name = "blst_benches"
path = "benches/blst_benches.rs"
harness = false

[dependencies.serde]
version = "1.0.152"
optional = true

[dependencies.zeroize]
version = "^1.1"
features = ["zeroize_derive"]

[dev-dependencies.rand_chacha]
version = "0.3"

[dev-dependencies.rand_core]
version = "0.6"
features = ["std"]

[dev-dependencies.rmp-serde]
version = "1.1.1"

[build-dependencies.cc]
version = "1.0"

[target."cfg(any(unix, windows))".dev-dependencies.criterion]
version = "0.3"

[target.'cfg(not(any(target_arch="wasm32", target_os="none", target_os="unknown", target_os="uefi")))'.dependencies.threadpool]
version = "^1.8.1"

[target.'cfg(target_env = "msvc")'.build-dependencies.glob]
version = "0.3"

[profile.release]


--- FILE registry/public-code/midnight-curves-0.3.1/Cargo.toml ---
# THIS FILE IS AUTOMATICALLY GENERATED BY CARGO
#
# When uploading crates to the registry Cargo will automatically
# "normalize" Cargo.toml files for maximal compatibility
# with all versions of Cargo and also rewrite `path` dependencies
# to registry (e.g., crates.io) dependencies.
#
# If you are reading this file be aware that the original Cargo.toml
# will likely look very different (and much more reasonable).
# See Cargo.toml.orig for the original contents.

[package]
edition = "2018"
name = "midnight-curves"
version = "0.3.1"
authors = [
    "dignifiedquire <me@dignifiedquire.com>",
    "Midnight team",
]
build = false
autolib = false
autobins = false
autoexamples = false
autotests = false
autobenches = false
description = "Implementation of BLS12 381 and Jubjub curves."
readme = "README.md"
categories = [
    "cryptography",
    "algorithms",
]
license = "MIT/Apache-2.0"
repository = "https://github.com/midnightntwrk/midnight-zk/"
resolver = "2"

[package.metadata.docs.rs]
rustdoc-args = [
    "--html-in-header",
    "katex-header.html",
]

[features]
__private_bench = []
default = [
    "serde",
    "std",
]
dev-curves = ["dep:halo2derive"]
portable = ["blst/portable"]
serde = [
    "dep:serde",
    "dep:serde_arrays",
    "dep:hex",
]
std = []

[lib]
name = "midnight_curves"
path = "src/lib.rs"

[[bench]]
name = "ec"
path = "benches/ec.rs"
harness = false

[[bench]]
name = "field_arith"
path = "benches/field_arith.rs"
harness = false

[[bench]]
name = "msm"
path = "benches/msm.rs"
harness = false

[[bench]]
name = "pairing"
path = "benches/pairing.rs"
harness = false

[dependencies.bitvec]
version = "1.0.1"

[dependencies.blst]
version = "0.3.15"
default-features = true

[dependencies.byte-slice-cast]
version = "1.2.3"

[dependencies.curve25519-dalek]
version = "4.1.3"
features = ["group"]

[dependencies.digest]
version = "0.10"

[dependencies.ff]
version = "0.13"

[dependencies.group]
version = "0.13"
features = ["tests"]

[dependencies.halo2derive]
version = "0.2.0"
optional = true

[dependencies.hex]
version = "0.4.3"
features = ["serde"]
optional = true

[dependencies.k256]
version = "0.13.4"
features = [
    "expose-field",
    "bits",
    "alloc",
]
default-features = false

[dependencies.lazy_static]
version = "1.4"

[dependencies.num-bigint]
version = "0.4.6"

[dependencies.p256]
version = "0.13.2"
features = [
    "expose-field",
    "bits",
    "alloc",
    "arithmetic",
]
default-features = false

[dependencies.pairing]
version = "0.23"

[dependencies.paste]
version = "1.0"

[dependencies.primeorder]
version = "0.13"

[dependencies.rand_core]
version = "0.6"
default-features = false

[dependencies.rayon]
version = "1.10"

[dependencies.serde]
version = "1.0"
features = ["derive"]
optional = true

[dependencies.serde_arrays]
version = "0.2.0"
optional = true

[dependencies.sha2]
version = "0.10"

[dependencies.subtle]
version = "2.6"

[dev-dependencies.ark-std]
version = "0.4"

[dev-dependencies.bincode]
version = "1.3"

[dev-dependencies.criterion]
version = "0.7"
features = ["html_reports"]

[dev-dependencies.halo2derive]
version = "0.2.0"

[dev-dependencies.hex]
version = "0.4.3"

[dev-dependencies.rand]
version = "0.8"

[dev-dependencies.rand_xorshift]
version = "0.3.0"

[dev-dependencies.serde_json]
version = "1.0.143"

[target.'cfg(all(target_arch = "wasm32", target_os = "unknown"))'.dependencies.getrandom]
version = "0.2"
features = ["js"]

