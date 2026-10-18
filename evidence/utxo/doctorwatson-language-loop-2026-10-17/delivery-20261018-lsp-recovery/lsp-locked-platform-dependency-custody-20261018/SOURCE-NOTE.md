# Five exact locked public package sources

This finite packet contains only the five package identities requested by root, confirmed against actual I Cargo.lock. It supplies archives and complete public text source for a later root-operated metadata step. No package version was chosen or dependency graph resolved in this lane.

Predicate: every extracted source file equals its exact regular archive member, and each complete archive SHA-256 equals the corresponding current Cargo.lock checksum and fresh public crates.io version checksum. This predicate was checked with byte comparisons and recorded in `CUSTODY-VALIDATION.json`.

| Exact package | Text files | Text bytes |
|---|---:|---:|
| objc2 0.6.5 | 120 | 1,170,864 |
| rustls-platform-verifier-android 0.2.0 | 5 | 56,370 |
| wasm-bindgen-futures 0.4.79 | 9 | 32,085 |
| web-sys 0.3.106 | 1,734 | 11,236,554 |
| zerocopy-derive 0.8.61 | 175 | 1,931,228 |
| Total | 2,043 | 14,427,101 |

All five static-registry archive requests and ten exact version/dependency API requests returned HTTP 200 through Scrapling Fetcher. `RETRIEVALS.json` preserves requested/canonical URLs, UTC request/retrieval times, status, raw byte length and SHA-256. No credentials or retained cookies were supplied. The already documented static REST/raw-body pattern required no new site overlay.

`LOCK-PINS.json` preserves the exact five name/version/registry-namespace/checksum pins and actual lock copy hash. At capture, I HEAD was 74a909d69639df8823a527a4d608a7516ccd69a2; the actual lock contained 423 registry package identities and SHA-256 f47f1f3e04111a50134656dce0e9d278e19a6c162aaf06cdd5f79fceead68ff3. `INPUT-Cargo.lock` is a byte copy of that existing lock, not a generated or edited lockfile. The original lock hash remained unchanged at the end of acquisition.

Full archives are under `archives/`; all regular file members are copied under `source/`. No members were omitted: each was UTF-8 text without NUL bytes, no links/special members were followed, and extraction rejected traversal/absolute/duplicate names. Complete files include normalized/original manifests, Rust/build/proc-macro source, tests, license files and any published upstream lockfiles as inert text. The retained 30,000,000-byte source cap was not raised.

`PUBLIC-FILE-PROVENANCE.json` binds every source path/size/SHA to its archive member, archive SHA, actual lock checksum and fresh registry checksum. `PACKAGES.json` preserves exact registry version/yank/rust-version fields and literal package/lib/features/build/target declarations plus build.rs filenames. These declarations do not establish active Cargo features, selected targets/units or dependency effects.

No source acquisition gap, HTTP failure, checksum failure or extraction omission occurred for the requested five. No extra package closure or alternative version was acquired. This is not a completeness claim for all workspace/build inputs. The previously observed metadata exit 101 and previous custody packets remain unchanged; this packet does not repair or relabel their receipts.

No Cargo/resolver/compiler/build/proc-macro/package/server/SDK/runtime code was executed, and no global cache/index, vendor checksum record, lock, checkout or Git state was written. This source task supplies no security/resource judgment, metadata execution approval, native admission or original roadmap gate closure. Root retains the separate actual metadata step and integration.
