# Experimental Rust Moriarty / Cardano source

This package exposes a frozen Rust `moriarty/1` → `moriarty-core/1` → `SettlementHashV2` R1CS path through the `mori-k2` executable. The ordinary workflow compiles a selected action and inspects a proposed transition against its fixed constraint matrices. It is an experimental source delivery, with captured local observations; it is not a qualified release of the whole language or a demonstrated Cardano deployment.

Any developer may build and use the supported subset in their own environment. No Moriarty maintainer approval, campaign record, hosted service, reviewer identity or project registry is a toolchain prerequisite. Asset-owner authorization and target validity remain separate objective obligations.

## Package and provenance

Keep this layout intact:

```text
moriarty/                    frozen 70-file Rust Core dependency closure
r1cs/source/mori-r1cs/        compiler library, including embedded source identity
r1cs/fixtures/               source programs and scenario fixtures
r1cs/expectations/           frozen ABI expectations
r1cs/EXPECTED-PACKET*/       additional source fixtures and expectations
k2/                          standalone driver crate, requests, gold and tests
k2/Cargo.lock                captured dependency lock
SOURCE-DELIVERY.json          path, size, digest and original candidate/receipt lineage
DELIVERY-INVENTORY.json       commands, evidence pins and delivery limits
```

The source manifest has 156 non-lock files plus one lock: Core 70, compiler/fixtures/expectations 60, driver 26 and lock 1. These are frozen experiment bytes, not a copy of current public main. The initial public `utxo` tree lacked the 70 frozen Core paths. The old JavaScript CLI uses Source/6 and TypeScript Core/5; invoking that CLI does not exercise this Rust path. The package does not supply the Rust `mori` CLI, LSP or MCP.

The base candidate manifest is `0df562aa3e3aa54cb6e6825352bc240fead652bed601af955db57325f0f7989e`. K4H overlays eight driver files (four new, four replacements); all 60 compiler and 70 Core paths remain unchanged. The driver package version remains `0.1.0-k2s1`; its declared driver identity is `mori-k2-driver/0.1.0-k2s1+k4h1`, outside the compiler build identity. Compiler identity embeds its manifest/source members and all 70 Core files. Moving or editing those dependency bytes can change the compiled relation identity. Source identity does not identify a running binary.

Historical `UNADOPTED`, `never built` and `never run` labels in source headers and `k2/README.md` record their authors' original state. They are preserved verbatim. The dated observations below describe subsequent captured executions. This documentation task performed no new package build or execution.

## Build and ordinary reproduction

Use Rust **1.99.0**, Cargo and the ordinary host linker/build environment for that toolchain. `moriarty/rust-toolchain.toml` pins `1.99.0`; both standalone manifests require Rust 1.99 and edition 2024. The explicit `+1.99.0` below matters because the toolchain file is in the sibling Core directory. If that toolchain is unavailable in your environment, reproduction stops there; substituting another version is a different build.

Start in this package's `k2/` directory. Cargo may need ordinary registry access for locked dependencies on a first build; no registry cache is shipped. `--offline` is usable only after the required dependencies already exist locally. The published observation used Linux x86_64 and a populated offline registry; a clean external installation has not yet been observed.

```bash
cd experiments/moriarty-cardano-rust/k2
cargo +1.99.0 build --locked --release --no-default-features --bin mori-k2
```

From an extracted standalone package, use `cd k2` instead. Use the resulting binary explicitly. If `CARGO_TARGET_DIR` is configured, substitute its `release/mori-k2` path for `./target/release/mori-k2` below. Building the enclosing Moriarty workspace is not the reproduction entrypoint.

Two contrasting fixture programs use the same compiler and selector profile:

```bash
./target/release/mori-k2 compile \
  --source ../r1cs/fixtures/reused/installment-invoice.mori \
  --selector requests/invoice-partial.selector.json

./target/release/mori-k2 compile \
  --source ../r1cs/fixtures/reused/conditional-reserve.mori \
  --selector requests/reserve-allocate.selector.json

./target/release/mori-k2 inspect \
  --source ../r1cs/fixtures/reused/installment-invoice.mori \
  --request requests/invoice-partial.request.json \
  --vk-digest aade9e4ec92bb00754e7704107b33f60036035bc8dc8454a54bd9068bacc39f0

./target/release/mori-k2 inspect \
  --source ../r1cs/fixtures/reused/conditional-reserve.mori \
  --request requests/reserve-allocate.request.json \
  --vk-digest fea88a909b0322b7dea60426325ae76aec926bd4905704145a1dabcac9182d9a
```

These VK digests and request contexts are synthetic, caller-declared fixture values. They are not keys or authenticated chain observations. `compile` prints a relation descriptor after a setup-mode constraint synthesis; that mode does not generate a cryptographic setup or key. `inspect` prints the statement bytes, digest, two public inputs, decoded ordered body slots and the fixed matrices' decision on one full assignment. Both commands write only stdout/stderr and use no randomness. They can consume substantial memory: the captured two inspections peaked at about 0.85 and 1.14 GiB RSS. Those measurements are observations, not bounds for new programs.

Read `fixedMatrix.violated`, `fixedMatrix.proveMatrixDigestEqual`, the matrix digest and the decoded financial slots. An `inspect` exit status of 0 means it produced a receipt; it can report violated rows in that receipt. `equalsDescriptor` compares values produced by the same binary and is not an independent anchor.

| Captured fixture | `partial(35)` | `allocate(true,23,7,2)` |
|---|---:|---:|
| Constraints | 308790 | 414211 |
| Witness variables | 306452 | 411068 |
| Body / total statement bytes | 378 / 721 | 609 / 952 |
| Fixed rows violated | 0 | 0 |
| Logical work | 28 | 57 |

Captured matrix digests are `e458153e583e0a817fbab08f24f4ba16465e459969c1f8cd4e13b801e82694c0` (partial) and `61bea6fd6639cedb8f48a807c8c825a61882d0829f34a75052f6d5af2daef2d0` (allocate). Logical work is a source profile quantity, separate from constraint count, memory, fees and ledger costs.

Ordinary test targets, selected explicitly to retain the default keyless feature set:

```bash
cargo +1.99.0 test --locked --release --no-default-features --test codec_tier -- --test-threads=1
cargo +1.99.0 test --locked --release --no-default-features --test compile_tier -- --test-threads=1
cargo +1.99.0 test --locked --release --no-default-features --test hostile_tier -- --test-threads=1 --show-output
```

The last command reproduces the K4H target without the maintainer sandbox wrapper; it is not a reproduction of the wrapper's isolation or resource enforcement. Captured K4H took about 125 seconds and 1.43 GiB peak RSS on its host. Hardware, dependency availability and new programs can differ.

## Supported subset and refusals

The driver admits one selected action of an accepted `moriarty/1` program under `moriarty-core/1`, using `SettlementHashV2`, `NotBound`, `GrossDebitAndFees` and `TouchedOnly`. Use the supplied selector as a template, change `action` to an action in your program, and pass your own source path. Optional `expectedSourceSha256` and `expectedProgramHash` pins must match the new program. The pipeline selects constructs, not fixture names. A new external developer program has not yet been demonstrated through the full proof and ledger path.

The implemented source surface includes `UInt` (checked unsigned 128-bit arithmetic), `Bool`, named state/inputs/lets, arithmetic and comparisons, strict Boolean operations, positional `requires`/`ensures`, state updates, transfers, fees and source-registered liability accrual/discharge and presence/amount reads. `ensures` reads the state at its position in the action. Present-zero liability differs from absence. The source coverage table is `r1cs/source/mori-r1cs/src/coverage.rs`; its target premises and obligations remain explicit.

The driver refuses other selector profiles. Core admission failures, unknown actions, invalid pins, external duties lacking a source identity, constant impossible actions and represented trace bounds can refuse compilation. Core can admit `moriarty/2`, but this compiler returns `StagedNotLowered`; `await` is not lowered. The `SettlementHashV2` adapter refuses a program containing `emit` in any action. Other library adapters/options are not additional driver support. `TouchedOnly` represents touched balance cells; it does not establish an authenticated complete ledger frame. `NotBound` has no cap slot.

Source files must be UTF-8 and at most 256 KiB. JSON is limited to 1 MiB, depth 32 and 65,536 nodes, with duplicate keys, trailing data and unknown DTO fields refused. Requests use `moriarty-k2-prove-request/1`, a selector, either `CoreReference` scenario proposals or explicit `Records`, and context premises. An invalid reference scenario can refuse before a receipt exists.

CLI arguments are `--flag value` pairs only. Unknown/duplicate flags, missing values and positional arguments are usage errors. There is no separate `--help` handler; an empty invocation prints usage and exits 2. Commands exit 1 on refusal and 0 on successful command completion.

## Proof API boundary

The CLI also contains these separate entrypoints; they are source interfaces, beyond the observed ordinary reproduction above:

```text
mori-k2 setup --source FILE --selector FILE --out-record FILE --out-proving-key FILE
mori-k2 prove --source FILE --request FILE --setup-record FILE --proving-key FILE --out-proof FILE
mori-k2 verify --setup-record FILE --proof FILE [--source FILE]
```

`setup` and `prove` draw `OsRng`; `verify` performs cryptographic verification and does not generate keys. Setup/prove create new output paths and refuse overwrites. The Unix proving-key output mode is 0600. Setup records and proof exports are JSON; the driver emits no Plutus Data/CBOR or transaction. The `crypto-authority` Cargo feature exposes separately ignored cryptographic tests; its name is an internal experiment label, not a permission required of a public developer.

No genuine current-language Groth16 setup/key, proof, keyed verification or Moriarty Preprod transaction is established by this delivery. Do not infer such a result from compiled APIs, specimen bytes, a synthesis mode named `Setup`, or a passing ordinary target.

## Captured evidence and remaining work

As of this documentation candidate, the October 16 K4H root records report 10 passing test groups, 107 frozen cases, two descriptors, two ordinary statement receipts and a `PASS` from the independent-expectation comparison gate. This is a captured result from the exact frozen experiment layout. Source/effect review preceded that run; fresh independent **actual-result review remains pending**, as does the independent packaging audit. See `DELIVERY-INVENTORY.json` for original evidence paths and SHA-256 pins. These records are evidence references, never build inputs or required developer admission records.

The routed source/effect review retains these interpretation limits: semantic cases labelled `UNSAT` reject the canonical completion against fixed rows; single-variable sweeps do not prove arbitrary-witness soundness or uniqueness; the carry cases do not isolate Booleanity; receipt self-comparison is not an independent anchor; large `wide144` receipt values can print as `top*2^128+lo` rather than decimal. The two fixture financial checks use frozen hand expectations. The gate is a local comparison program, not a ledger verifier. Security conclusions belong to the routed security reviews; this guide supplies no independent security approval.

The first unobserved capability after ordinary synthesis/inspection is genuine setup, followed by real proving and keyed verification, consumer/target correspondence and a developer-owned ledger transaction. A clean external installation and independent reproduction are also unobserved. Cardano context/pre-state/frame authentication and VK-to-matrix provenance remain open consumer obligations. The Cardano experiment does not replace Moriarty's required Midnight ZKIRv3 target or establish Preview/Fenrir qualification.

All 20 UTxO families, ACTUS (277 fixtures, 18 executable types, 32 taxonomy dispositions, DS01–07), DeFi (72 rows, DA01–24, held-outs, intent and regression requirements), general financial obligations, recursion, private handoff, multi-parent composition and all U0–U7/M1–M17/MC/SP/G exits remain open. Existing failure evidence and counters remain historical facts; this package closes none of those exits.
