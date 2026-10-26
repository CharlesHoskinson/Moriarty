# Mopro performance benchmarks
> Source: https://zkmopro.org/docs/performance/
> Collected: 2026-10-26
> Published: Unknown

Performance and Benchmarks | Mopro


[Skip to main content](#__docusaurus_skipToContent_fallback)

[![Mopro Logo](/img/logo_title.svg)![Mopro Logo](/img/logo_title.svg)](/)

[0.3](/docs/intro)

* [0.3](/docs/performance)
* [0.2](/docs/0.2/performance)
* [0.1](/docs/0.1/performance)
* [0.0.1](/docs/0.0.1/performance)

[Docs](/docs/intro)[Blog](/blog)

[Github](https://github.com/zkmopro/mopro)![](/img/link_arrow.svg)

Search

* [Introduction](/docs/intro)
* [Prerequisites](/docs/prerequisites)
* [Getting Started](/docs/getting-started)
* [Projects](/docs/projects)
* [Architectures](/docs/architectures)
* [Setup](/docs/setup/rust-setup)
* [Mobile SDKs](/docs/sdk/overview)
* [Supported Adapters](/docs/adapters/overview)
* [Rust Crates](/docs/crates/mopro-ffi)
* [Performance and Benchmarks](/docs/performance)
* [Community and Talks](/docs/community)
* [FAQ](/docs/FAQ)

* Performance and Benchmarks
Version: 0.3

On this page

Performance and Benchmarks
==========================

Circom[​](#circom "Direct link to Circom")
------------------------------------------

In summary:   
Both native witness generation and proof generation are generally faster than `snarkjs` in the browser, with potential speed improvements of up to **20 times**.   
However, performance varies across different circuits.
We *recommend* developers benchmark their custom circuits before selecting tools for app development.

warning

* [circom-witness-rs](https://github.com/philsippl/circom-witness-rs) is not fully compatible with circom circuits. See: [zkmopro/mopro#32](https://github.com/zkmopro/mopro/issues/32).
* [wasmer](https://github.com/arkworks-rs/circom-compat) doesn't work in iOS. See: [zkmopro/mopro#109](https://github.com/zkmopro/mopro/issues/109).

### iOS[​](#ios "Direct link to iOS")

Benchmarks on an iPhone 16 Pro (2024).

| Witness generation | [witnesscalc](https://github.com/0xPolygonID/witnesscalc) | [circom-witnesscalc](https://github.com/iden3/circom-witnesscalc) | [wasmer](https://github.com/arkworks-rs/circom-compat) | [w2c](https://github.com/vimwitch/rust-witness) | [snarkjs](https://github.com/iden3/snarkjs) |
| --- | --- | --- | --- | --- | --- |
| [Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256) | 142.1 ms (~1x) | 75.4 ms (**~2x**) | 287.7 ms (slower) | 140 ms (~1x) | 147.1 ms |
| [SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256) | 41 ms (**~2x**) | 51.3 ms (~1.7x) | 171.3 ms (slower) | 93.9 ms (~1x) | 91.8 ms |
| [RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa) | 153 ms (**~19x**) | - | 2937.5 ms (~1x) | 2312.3 ms (~1.2x) | 2979.5 ms |
| [Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32) | 22 ms (~3.5x) | 14.6 ms (**~5.3x**) | 266.5 ms (slower) | 38.9 ms (~2x) | 77.6 ms |
| [Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar) | 285.1 ms | - | 3284.7 ms | 1490.8 ms | - |

| Proof generation | [rapidsnark](https://github.com/iden3/rapidsnark) | [ark-works](https://github.com/arkworks-rs/circom-compat) | [snarkjs](https://github.com/iden3/snarkjs) |
| --- | --- | --- | --- |
| [Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256) | 630.3 ms (**~8.2x**) | 956.9 ms (~5.4x) | 5182.1 ms |
| [SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256) | 186.7 ms (**~8.2x**) | 498.6 ms (~3x) | 1487 ms |
| [RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa) | 749.1 ms (**~8.8x**) | 2250.8 ms (~3x) | 6604.5 ms |
| [Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32) | 143.3 ms (**~6.9x**) | 151.4 ms (~6.6x) | 1001.6 ms |
| [Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar) | 3131.7 ms | 10681.6 ms | - |

info

**Details:** [Spreadsheet of Circom benchmark (iOS)](https://docs.google.com/spreadsheets/d/1MFABmsYSUsWDmhbjleqhBXk7nkYwhu589yK-CHtRkNI/edit?usp=sharing)

### Android[​](#android "Direct link to Android")

Benchmarks on an Samsung S23 Ultra (2023).

| Witness generation | [witnesscalc](https://github.com/0xPolygonID/witnesscalc) | [circom-witnesscalc](https://github.com/iden3/circom-witnesscalc) | [wasmer](https://github.com/arkworks-rs/circom-compat) | [w2c](https://github.com/vimwitch/rust-witness) | [snarkjs](https://github.com/iden3/snarkjs) |
| --- | --- | --- | --- | --- | --- |
| [Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256) | 101.4 ms (~3x) | 71 ms (**~4x**) | 507.3 ms (slower) | 210.5 ms (~1.3x) | 292.3 ms |
| [SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256) | 29 ms (**~5x**) | 44 ms (~3.5x) | 271.6 ms (slower) | 106.9 ms (~1.4x) | 157.9 ms |
| [RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa) | 155 ms (**~25x**) | - | 4723 ms (slower) | 3751 ms (~1x) | 3958 ms |
| [Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32) | 10.3 ms (**~7x**) | 14.7 ms (~5x) | 416.9 ms (slower) | 32.8 ms (~2x) | 74.1 ms |
| [Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar) | 365.1 ms (**~8.7x**) | - | 5359.6 ms (slower) | 2716.4 ms (~1.1x) | 3207.5 ms |

| Proof generation | [rapidsnark](https://github.com/iden3/rapidsnark) | [ark-works](https://github.com/arkworks-rs/circom-compat) | [snarkjs](https://github.com/iden3/snarkjs) |
| --- | --- | --- | --- |
| [Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256) | 743.7 ms (**~14x**) | 2330.4 ms (~4.7x) | 11096.4 ms |
| [SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256) | 228.4 ms (**~15x**) | 1575.2 ms (~2x) | 3514.8 ms |
| [RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa) | 950 ms (**~14x**) | 5839 ms (~2.3x) | 13442 ms |
| [Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32) | 165.8 ms (**~13x**) | 276.9 ms (~7.7x) | 2146 ms |
| [Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar) | 3394.5 ms (**~15x**) | 33239.2 ms (~1.5x) | 51546.3 ms |

info

**Details:** [Spreadsheet of Circom benchmark (Android)](https://docs.google.com/spreadsheets/d/1TDgL2NXxYl8UH-RZPWfWdawY0tjxf3c6l8B11_FG7Kg/edit?usp=sharing)

### macOS[​](#macos "Direct link to macOS")

Benchmarks on an Macbook Pro M1 Max (2021).

| Witness generation | [witnesscalc](https://github.com/0xPolygonID/witnesscalc) | [circom-witnesscalc](https://github.com/iden3/circom-witnesscalc) | [wasmer](https://github.com/arkworks-rs/circom-compat) | [w2c](https://github.com/vimwitch/rust-witness) | [snarkjs](https://github.com/iden3/snarkjs) |
| --- | --- | --- | --- | --- | --- |
| [Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256) | 63.9 ms (**~5x**) | 69.6 ms (~5x) | 507.7 ms (slower) | 214.6 ms (~1.6x) | 348.7 ms |
| [SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256) | 22 ms (**~10x**) | 32 ms (~7x) | 272 ms (slower) | 112 ms (~2x) | 225 ms |
| [RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa) | 164 ms (**~29x**) | - | 5326 ms (slower) | 4796 ms (slower) | 4777 ms |
| [Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32) | 7.1 ms (~18x) | 5 ms (**~26x**) | 287 ms (slower) | 34.9 ms (~3.7x) | 130 ms |
| [Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar) | 324 ms (**~10x**) | - | 5369 ms (slower) | 2902 ms (~1.2x) | 3437 ms |

| Proof generation | [rapidsnark](https://github.com/iden3/rapidsnark) | [ark-works](https://github.com/arkworks-rs/circom-compat) | [snarkjs](https://github.com/iden3/snarkjs) |
| --- | --- | --- | --- |
| [Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256) | 528 ms (**~7x**) | 1161 ms (~3.3x) | 3873 ms |
| [SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256) | 158 ms (**~9x**) | 779 ms (~2x) | 1462 ms |
| [RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa) | 676 ms (**~8x**) | 3005 ms (~1.8x) | 5553 ms |
| [Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32) | 112 ms (~7.8x) | 84 ms (**~10x**) | 877 ms |
| [Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar) | 2421 ms (**~8x**) | 14142 ms (~1.3x) | 19794 ms |

info

**Details:** [Spreadsheet of Circom benchmark (host)](https://docs.google.com/spreadsheets/d/1irKg_TOP-yXms8igwCN_3OjVrtFe5gTHkuF0RbrVuho/edit?usp=sharing)

### Web[​](#web "Direct link to Web")

We have enabled `wasm-bindgen-rayon` for multithreading in the browser. Below is a benchmark comparing arkworks with `wasm-bindgen-rayon` against `snarkjs`.

* **iPhone 16 Pro**

  | Proof generation | [ark-works with `rayon`](https://github.com/RReverser/wasm-bindgen-rayon) | [snarkjs](https://github.com/iden3/snarkjs) |
  | --- | --- | --- |
  | [Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256) | 1717.81 ms (**~3x**) | 5166.02 ms |
  | [SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256) | 601.32 ms | 380.61 ms (**~1.5x**) |
  | [RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa) | 7152.85 ms (**~1.1x**) | 8473.58 ms |
  | [Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32) | 652.07 ms (**~1.4x**) | 919.54 ms |
  | [Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar) | - ms | - ms |
* **Android Samsung S23U**

  | Proof generation | [ark-works with `rayon`](https://github.com/RReverser/wasm-bindgen-rayon) | [snarkjs](https://github.com/iden3/snarkjs) |
  | --- | --- | --- |
  | [Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256) | 2733.15 ms (**~2.7x**) | 7412.39 ms |
  | [SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256) | 980.64 ms (**~2.4x**) | 2379.58 ms |
  | [RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa) | 9313.07 ms (**~1.1x**) | 10725.49 ms |
  | [Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32) | 792.87 ms (**~1.3x**) | 1045.34 ms |
  | [Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar) | - ms | - ms |

info

**Details:** [zkmopro/mopro#290](https://github.com/zkmopro/mopro/issues/290)

Halo2[​](#halo2 "Direct link to Halo2")
---------------------------------------

In summary:   
The performance of the Mopro build is comparable to that of native Halo2 build.

The below tests were run on a Macbook Pro M1 Pro (2021) as well as an iPhone 15 Pro (2023).

| [Keccak256](https://github.com/ElusAegis/halo2-keccak-stable.git) | Prove Time (s) | Verify Time (s) |
| --- | --- | --- |
| Native (M1 Pro) | 10.3 s | 0.15 s |
| iPhone 15 Pro | 11.0 s | 0.12 s |
| Pixel 6 Pro | 23.9 s | 0.51 s |
| Web | 26.6 s | 0.55 s |

| [RSA](https://github.com/KimiWu123/halo2-rsa-mopro.git) | Prove Time (s) | Verify Time (s) |
| --- | --- | --- |
| Native (M1 Pro) | 26.9 s | 4.0 s |
| iPhone 15 Pro | crashes | crashes |
| Pixel 6 Pro | crashes | crashes |
| Web | Not Support | x |

Note that the iPhone 15 Pro and Pixel 6 Pro crash when running the RSA circuit due to the large memory requirements. The circuit needs
around 5GB of memory to run, while the iPhone 15 Pro and Pixel 6 Pro usually limits the application memory usage to 3GB.
Note that the RSA circuit is built on top of former version of Halo2. When it generates a proof on web, it crashes inside Halo2 module.

Noir[​](#noir "Direct link to Noir")
------------------------------------

We target [`nargo v1.0.0-beta.3`](https://noir-lang.org/docs/getting_started/quick_start#nargo) and [`bb v0.82.2`](https://noir-lang.org/docs/getting_started/quick_start#proving-backend-1), ensuring all related dependencies are aligned (e.g., [@noir-lang/[email protected]](https://www.npmjs.com/package/@noir-lang/noir_js/v/1.0.0-beta.3), [@aztec/[email protected]](https://www.npmjs.com/package/@aztec/bb.js/v/0.82.2)). The benchmarks below were run on a MacBook Air M3 (2024) and a Pixel 6 (2021). We report the fastest result from multiple runs to reduce cold-start effects.

On iOS, the benchmark runs using the "My Mac (Designed for iPad)" target. On Android, it runs on a Pixel 6 in release mode. On bb CLI, we measure wall-clock time with the `time bb prove` command. On the website, the benchmark runs in a browser-based (WASM) environment.

| Circuit | iOS | Android | bb CLI (Laptop) | Web (Laptop) |
| --- | --- | --- | --- | --- |
| [Keccak256](https://github.com/moven0831/mopro-example-app-keccak256) | 349 ms (~11.8×) | 1303 ms (~3.2×) | 345 ms (~12.0×) | 4122 ms |
| [RSA](https://github.com/moven0831/mopro-example-app-rsa) | 312 ms (~6.6×) | 1091 ms (~1.9×) | 221 ms (~9.4×) | 2068 ms |
| [zkemail](https://github.com/Mach-34/zkemail.nr_header_demo) | 1309 ms (~5.0×) | 4757 ms (~1.4×) | 804 ms (~8.2×) | 6590 ms |
| [anon aadhaar](https://github.com/moven0831/mopro-example-app-anon-aadhaar-noir) | 2225 ms (~7.7×) | 8179 ms (~2.1×) | 1366 ms (~12.5×) | 17030 ms |
| [semaphore](https://github.com/moven0831/mopro-example-app-semaphore-noir) | 828 ms (~5.6×) | 3990 ms (~1.2×) | 436 ms (~10.6×) | 4638 ms |

**Benchmark circuit details:**

* Keccak256: From [Noir Stdlib](https://noir-lang.org/docs/noir/standard_library/cryptographic_primitives/hashes#keccak256).
* RSA: pkcs1v15 signature verification (SHA-256). (see details in [zkpassport/noir\_rsa](https://github.com/zkpassport/noir_rsa/blob/acb50389b79dbf38b1828f4fa82c28d742b140fc/src/rsa.nr#L286-L309))
* zkEmail: Header extraction circuit ([`Mach-34/zkemail.nr_header_demo`](https://github.com/Mach-34/zkemail.nr_header_demo)).
* Anon Aadhaar: Updated circuit (nargo v1.0.0-beta.3) in [`anon-aadhaar/anon-aadhaar-noir#6`](https://github.com/anon-aadhaar/anon-aadhaar-noir/pull/6).
* Semaphore: From [`hashcloak/semaphore-noir`](https://github.com/hashcloak/semaphore-noir).

info

**Details:** [zkmopro/mopro#414](https://github.com/zkmopro/mopro/issues/414)

[Edit this page](https://github.com/zkmopro/mopro/tree/main/docs/docs/performance.md)

[Previous

noir-rs](/docs/crates/noir-rs)[Next

Community and Talks](/docs/community)

* [Circom](#circom)
  + [iOS](#ios)
  + [Android](#android)
  + [macOS](#macos)
  + [Web](#web)
* [Halo2](#halo2)
* [Noir](#noir)

Resources

* [Github](https://github.com/zkmopro/mopro)
  ![](/img/link_arrow.svg)
* [Documentation](/docs/intro)

Community

* [Telegram](https://t.me/zkmopro)
  ![](/img/link_arrow.svg)
* [Twitter](https://twitter.com/zkmopro)
  ![](/img/link_arrow.svg)

![Mopro Logo](/img/logo_lg.svg)![Mopro Logo](/img/logo_lg.svg)

Copyright © 2026 Mopro