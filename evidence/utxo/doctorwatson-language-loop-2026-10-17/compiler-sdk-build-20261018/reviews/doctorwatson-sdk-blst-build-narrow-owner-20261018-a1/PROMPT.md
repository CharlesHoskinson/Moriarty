Inspect only the concrete blst0.3.17 build.rs and Cargo.toml effects for proposed offline compiler-only stages test. Prior large1912file jobs timed out124without usable review; this request is deliberately one dependency, no retry of same broad scope. Last actual Opus4.8 rate rejection06:54:51UTC, no recovery; you are authorized Gemini HIGH security fallback, not Opus. Observed backend Cargo tree blst features=[default], curves=[default,serde,std]; workspace metadata portable is broader and must NOT be imported. Target proposed x86_64-unknown-linux-gnu, fresh owned target/tmp/CARGO_HOME, no HOME/inherited flags, ordinary Rust/linker public caches TCB still pending owner assessment. Describe exact env/tool/subprocess/native compilation/prebuilt fallback/CPU/assembly inputs of this one source, distinguish conditional paths from observed activation, identify finite unprovided helpers if material. Do not pronounce whole compiler dependency closure/no-I/O, proof security, or execute/admit anything. Correct the inventory if a concrete source fact is wrong; cite exact paths/branches. A narrow complete finding may retain explicit assumptions/unknown host-dependent branch choices. Do not demand proving every dependency correct as a prerequisite for mere source inventory.
Readonly packet /home/charl/moriarty-c2-opus55-restored-20261017/sdk-blst-build-narrow-packet. Return one whole six-field JSON with reviewToken doctorwatson-sdk-blst-build-narrow-owner-20261018-a1-whole-review. No commands, toolchain, runtime, build, SDK, network, private/config/auth/prover files or delegated agents. Only pinned public sources may be read.

--- FILE build.rs ---
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


--- FILE Cargo.toml ---
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


--- FILE SOURCE-NOTE.md ---
# Observed backend build surface

Root's sanitized offline Cargo queries succeeded; this lane ran no Cargo/metadata/rustc/package/build/proc-macro/source execution. The backend tree has726 rows and280 unique exact package identities. All280 join unambiguously to fullworkspace metadata, retaining full registry/Git/path namespaces and lock checksums. Metadata contains374 workspace packages; earlier manual lock adjacency contained381 identities. **None of these counts is a backend compilation-unit count.** The tree query requested mori-backend, Linux x86_64-unknown-linux-gnu and normal/build/dev edges; it did not select --test stages or produce a Cargo unit graph.

The joined observed surface contains33 custom-build candidate packages and22 proc-macro-library candidate packages (54 distinct packages; paste declares both). These are target declarations on packages present in the observed tree, not executed builds or observed compiler units. All280 manifests and33 root build sources were read from explicit repo/public-cache paths and hashed. No permitted read path is missing. Proc-macro library bodies were neither read nor executed; their declared source paths are reconstructed only.

## Observed feature strings and manual reconciliation

Raw feature strings are preserved per occurrence, including libc's two strings (default,extra_traits,std versus default,std). Twenty-eight identities differ between the textual tree feature union and fullworkspace metadata feature union. Fullworkspace resolve features are stored separately and are **not** imported as backend features. In particular:

- blst0.3.17 tree: default; workspace metadata additionally has portable.
- curves0.3.1 tree: default,serde,std; workspace additionally has portable.
- cc1.7.0 tree: parallel. This resolves a previously manual optional-feature unknown for this observed tree, without selecting a build branch/tool.
- aws-lc-rs1.18.1 tree: aws-lc-sys,prebuilt-nasm; aws-lc-sys0.45.0 tree: prebuilt-nasm. Defaults and broader workspace features must not be substituted for these exact strings.

Forty-two of the prior50 finite manifest identities appear in this tree. glob,hermit-abi,legacy curves0.2.1,halo2derive,syn1,ledger,zswap and onchain-runtime do not appear. All57 later-acquired registry frontier identities do appear. The three registry packets cover86 of the observed registry identities. Eight selected pinned SDK Git manifests and five workspace manifests are also read; the remaining181 manifest reads have current source hashes without new archive-byte verification. Onchain-runtime's Scheme/vendored branch remains real comparison source from the older inventory, but is absent from the actual observed backend tree.

## Reconstructed path provenance

Original query-owned CARGO_HOME roots were removed. START/RESULT/CLEANUP remain original; root supplied supplemental PUBLIC-CACHE-PATH-MAPPING.json records after the queries (`recordedAfterQuery=true`). Only those exact registry/git link prefixes were replaced, with suffixes preserved and traversal/other namespaces rejected. A reconstructed current path does not assert that a symlink still exists or prove current source bytes equal query-time bytes. SOURCE pins distinguish current public reads from the 86 manifest/14 build-source comparisons to prior checksum-verified archive copies and 8 matching pinned SDK Git blob comparisons. All those comparisons match. New public-cache manifests/build sources were hashed, not newly archive-verified or acquired.

## Concrete source references and unknowns

The33 root build sources yield533 lexical reference lines,134 literal environment references and eight literal tool occurrences (bindgen,clang,clang-cl,freebsd-version,nasm). These are source text and may occur in comments/tests/conditional branches; no actual call/branch/effect is established. Dynamic commands, compiler wrappers/paths, environment values, generated outputs and input files remain unresolved. No tool lookup or environment query was run by this lane.

Nine external build-module declarations were recorded without a recursive walk. Seven are in aws-lc-sys builder/main.rs (cc_builder,cmake_builder,fips_probe,nasm_builder,sys_bindgen,system_detect,system_library), alongside two other root-script module declarations listed exactly in the JSON. Their cfg/path conditions and helper bodies were not followed. AWS builder/native header/source/bindings paths, cc/compiler/archiver decisions and blst prebuilt/source/assembly/feature/host-dependent branches remain designated-owner source-review inputs. Proc-macro bodies likewise remain separate review input; manifest/target declarations alone do not establish behavior. This is a finite observed surface for small reviews, not a full source/effects closure or a broad blocker judgment.

The unrun compiler-command candidate pins the existing exact stages test with explicit Linux target and j1. It is a proposal for the separate designated-owner/resource decision; it grants no execution/retry/cache-warming/cap-change authority. Existing U0/U1/U2 financial/compiler/target acceptance gaps and all original U0–U7/M/MC/SP/G exits, failures, caps/counters and rosters stay unchanged.

## Verification and preservation

Both query stdout digests match their successful RESULT receipts. All313 current public source reads were rechecked for stable bytes. The original707-file,808-file and2124-file source packets and selected-build inventory are unchanged and their metadata/source hashes were reverified. Only compiler-observed-build-inventory was written. No network, archive acquisition, Git mutation, SDK/runtime/model/VM/proof/key/SRS operation or private/config/auth/provider/prover-cache read occurred. This packet supplies source facts and observed tooling data, with no security/no-I/O/resource judgment or execution admission.


--- FILE UNRUN-COMPILER-COMMAND-CANDIDATE.json ---
{
  "status": "UNRUN COMPILER-PATH TEST COMMAND CANDIDATE; NOT EXECUTION ADMISSION",
  "argv": [
    "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
    "test",
    "--frozen",
    "-j",
    "1",
    "-p",
    "mori-backend",
    "--target",
    "x86_64-unknown-linux-gnu",
    "--test",
    "stages",
    "legacy_counter_abi_and_payment_effects_match_existing_frozen_snapshots",
    "--",
    "--exact",
    "--test-threads=1"
  ],
  "working_directory": "/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017",
  "purpose": "Run the existing positive production-compiler Counter/Payment stages test; source/ABI/effect snapshot scope from prior compiler-local-check-proposal, not a new compiler qualification claim",
  "observed_dependency_query_alignment": {
    "package": "mori-backend",
    "target": "x86_64-unknown-linux-gnu",
    "edge_kinds_requested_by_observed_tree": [
      "normal",
      "build",
      "dev"
    ],
    "limits": "Observed tree was not a --test stages compilation-unit query. Explicit test selection here is a proposal. Exact test filter does not narrow compilation of stages.rs/library/dependencies."
  },
  "required_review_inputs": [
    "OBSERVED-BACKEND-IDENTITIES.json",
    "BUILD-PROC-MACRO-CANDIDATES.json",
    "SOURCE-INPUT-TOOL-ENV-REFERENCES.json",
    "CURRENT-SOURCE-PINS.json",
    "QUERY-PINS.json",
    "Prior frozen compiler-local-check-proposal runtime/financial/acceptance limitations"
  ],
  "pending_designated_owner_decisions": [
    "Root build scripts and nine untraversed build-module declarations; actual native/tool/builder branches remain unresolved",
    "Proc-macro library bodies and their executed expansion dependencies not inspected/executed in this lane",
    "Compiler library/SDK load runtime boundary and current source hash reconciliation remain separate from build metadata",
    "Actual Cargo units/host-target feature partitions, tool paths/environment/generated inputs and all resource/cap applicability remain unestablished here"
  ],
  "preserve_existing_caps_counters_failures": true,
  "new_resource_cap_or_retry_authority": false,
  "automatic_retries_or_cache_warming": false,
  "package_or_test_execution_performed": false,
  "security_or_resource_admission": false,
  "acceptance": "No original U0–U7/M/MC/SP/G exit closure. Existing snapshot test is scoped evidence only if eventually run and independently reviewed."
}

