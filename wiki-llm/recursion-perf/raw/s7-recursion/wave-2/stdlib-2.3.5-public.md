# Pinned Midnight stdlib 2.3.5 source
> Source: https://docs.rs/midnight-zk-stdlib/2.3.5/src/midnight_zk_stdlib/lib.rs.html
> Collected: 2026-10-26
> Published: Unknown

lib.rs - source

[Docs.rs](/)

* [midnight-zk-stdlib-2.3.5](# "Standard library of circuits and utilities for Midnight zero-knowledge proofs")

  + midnight-zk-stdlib 2.3.5
  + [Docs.rs crate page](/crate/midnight-zk-stdlib/2.3.5 "See midnight-zk-stdlib in docs.rs")
  + 11 October 2026

  + Links
  + [crates.io](https://crates.io/crates/midnight-zk-stdlib "See midnight-zk-stdlib in crates.io")
  + [Source](/crate/midnight-zk-stdlib/2.3.5/source/ "Browse source of midnight-zk-stdlib-2.3.5")

  + Owners
  + [iquerejeta](https://crates.io/users/iquerejeta)

  + Dependencies
  + - [base64 ^0.13.1
      *normal*](/base64/^0.13.1/)
    - [bincode ^2.0
      *normal*](/bincode/^2.0/)
    - [blake2b\_halo2 ^0.2.0
      *normal*](/blake2b_halo2/^0.2.0/)
    - [blake2b\_simd ^1
      *normal*](/blake2b_simd/^1/)
    - [ff ^0.13
      *normal*](/ff/^0.13/)
    - [group ^0.13
      *normal*](/group/^0.13/)
    - [hex-literal ^1.1
      *normal*](/hex-literal/^1.1/)
    - [sha3-circuit ^0.2.0
      *normal*](/sha3-circuit/^0.2.0/)
    - [midnight-circuits ^7.2.4
      *normal*](/midnight-circuits/^7.2.4/)
    - [midnight-curves ^0.3.1
      *normal*](/midnight-curves/^0.3.1/)
    - [midnight-proofs ^0.8.2
      *normal*](/midnight-proofs/^0.8.2/)
    - [num-bigint ^0.4
      *normal*](/num-bigint/^0.4/)
    - [num-traits ^0.2
      *normal*](/num-traits/^0.2/)
    - [rand ^0.8
      *normal*](/rand/^0.8/)
    - [rayon ^1.10
      *normal*](/rayon/^1.10/)
    - [serde ^1.0
      *normal*
      *optional*](/serde/^1.0/)
    - [sha2 ^0.10.9
      *normal*](/sha2/^0.10.9/)
    - [bellman ^0.14.0
      *dev*](/bellman/^0.14.0/)
    - [blake2 ^0.10.6
      *dev*](/blake2/^0.10.6/)
    - [criterion ^0.7
      *dev*](/criterion/^0.7/)
    - [dhat ^0.3
      *dev*](/dhat/^0.3/)
    - [goldenfile =1.8.0
      *dev*](/goldenfile/=1.8.0/)
    - [itertools ^0.14
      *dev*](/itertools/^0.14/)
    - [num-bigint ^0.4
      *dev*](/num-bigint/^0.4/)
    - [rand\_chacha ^0.3
      *dev*](/rand_chacha/^0.3/)
    - [serde\_json ^1
      *dev*](/serde_json/^1/)
    - [serial\_test ^3.2.0
      *dev*](/serial_test/^3.2.0/)
    - [sha3 ^0.11.0
      *dev*](/sha3/^0.11.0/)
    - [midnight-curves ^0.3.0
      *normal*](/midnight-curves/^0.3.0/)

  + Versions

  + [**100%**
    of the crate is documented](/crate/midnight-zk-stdlib/2.3.5)
* [Platform](#)
  + [x86\_64-unknown-linux-gnu](/crate/midnight-zk-stdlib/2.3.5/target-redirect/src/midnight_zk_stdlib/lib.rs.html)
* [Feature flags](/crate/midnight-zk-stdlib/2.3.5/features "Browse available feature flags of midnight-zk-stdlib-2.3.5")

* [docs.rs](#)
  + [About docs.rs](/about)
  + [Badges](/about/badges)
  + [Builds](/about/builds)
  + [Metadata](/about/metadata)
  + [Shorthand URLs](/about/redirections)
  + [Download](/about/download)
  + [Rustdoc JSON](/about/rustdoc-json)
  + [Build queue](/releases/queue)
  + [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

* [Rust](#)
  + [Rust website](https://www.rust-lang.org/)
  + [The Book](https://doc.rust-lang.org/book/)
  + [Standard Library API Reference](https://doc.rust-lang.org/std/)
  + [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  + [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
  + [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[Skip to main content](#main-content)

Files
-----

midnight\_zk\_stdlib/ lib.rs
============================

```
1// This file is part of MIDNIGHT-ZK.
2// Copyright (C) Midnight Foundation
3// SPDX-License-Identifier: Apache-2.0
4// Licensed under the Apache License, Version 2.0 (the "License");
5// You may not use this file except in compliance with the License.
6// You may obtain a copy of the License at
7// http://www.apache.org/licenses/LICENSE-2.0
8// Unless required by applicable law or agreed to in writing, software
9// distributed under the License is distributed on an "AS IS" BASIS,
10// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
11// See the License for the specific language governing permissions and
12// limitations under the License.
13
14#![doc = include_str!("../README.md")]
15//!
16//! ## Implementation Details
17//!
18//! This library uses a fixed configuration, meaning that regardless of what one
19//! uses, it will always consist of the same columns, lookups, permutation
20//! enabled columns, or gates. The motivation for this is twofold:
21//!
22//! * It facilitates recursion (we always aggregate circuits that have the same
23//!   verification logic).
24//!
25//! * We could optimise the verifier, who can store part of the circuit
26//!   description in memory and does not need to reproduce it everytime it
27//!   receives a new proof.
28
29mod external;
30pub mod utils;
31
32use std::{cell::RefCell, cmp::max, convert::TryInto, fmt::Debug, io, rc::Rc};
33
34use bincode::{config::standard, Decode, Encode};
35use blake2b::blake2b::{
36    blake2b_chip::{Blake2bChip, Blake2bConfig},
37    NB_BLAKE2B_ADVICE_COLS,
38};
39use ff::{Field, PrimeField};
40use group::{prime::PrimeCurveAffine, Group};
41use keccak_sha3::packed_chip::{PackedChip, PackedConfig, PACKED_ADVICE_COLS, PACKED_FIXED_COLS};
42use midnight_circuits::{
43    biguint::biguint_gadget::BigUintGadget,
44    ecc::{
45        foreign::{
46            edwards_chip::{
47                nb_foreign_edwards_chip_columns, ForeignEdwardsEccChip, ForeignEdwardsEccConfig,
48            },
49            weierstrass_chip::{
50                nb_foreign_ecc_chip_columns, ForeignWeierstrassEccChip, ForeignWeierstrassEccConfig,
51            },
52        },
53        hash_to_curve::HashToCurveGadget,
54        native::{EccChip, EccConfig, NB_EDWARDS_COLS},
55    },
56    field::{
57        decomposition::{
58            chip::{P2RDecompositionChip, P2RDecompositionConfig},
59            pow2range::Pow2RangeChip,
60        },
61        foreign::{
62            nb_field_chip_columns, params::MultiEmulationParams as MEP, FieldChip, FieldChipConfig,
63        },
64        native::{NB_ARITH_COLS, NB_ARITH_FIXED_COLS},
65        NativeChip, NativeConfig, NativeGadget,
66    },
67    hash::{
68        poseidon::{
69            constants::RATE, PoseidonChip, PoseidonConfig, VarLenPoseidonGadget,
70            NB_POSEIDON_ADVICE_COLS, NB_POSEIDON_FIXED_COLS,
71        },
72        sha256::{
73            Sha256Chip, Sha256Config, VarLenSha256Gadget, NB_SHA256_ADVICE_COLS,
74            NB_SHA256_FIXED_COLS,
75        },
76        sha512::{Sha512Chip, Sha512Config, NB_SHA512_ADVICE_COLS, NB_SHA512_FIXED_COLS},
77    },
78    instructions::{
79        hash::VarHashInstructions, public_input::CommittedInstanceInstructions,
80        vector::VectorBounds, *,
81    },
82    map::map_gadget::MapGadget,
83    parsing::{
84        self,
85        scanner::{ScannerChip, ScannerConfig, NB_SCANNER_ADVICE_COLS, NB_SCANNER_FIXED_COLS},
86        Base64Chip, Base64Config, ParserGadget, NB_BASE64_ADVICE_COLS,
87    },
88    types::{
89        AssignedBit, AssignedByte, AssignedNative, AssignedNativePoint, ComposableChip, InnerValue,
90        Instantiable,
91    },
92    vec::{vector_gadget::VectorGadget, AssignedVector, Vectorizable},
93    verifier::{BlstrsEmulation, VerifierGadget},
94};
95use midnight_curves::{
96    curve25519::{self as curve25519_mod, Curve25519},
97    k256::{self as k256_mod, K256},
98    p256::{self as p256_mod, P256},
99    Fq, G1Affine, G1Projective,
100};
101use midnight_proofs::{
102    circuit::{Layouter, SimpleFloorPlanner, Value},
103    dev::cost_model::{circuit_model, CircuitModel},
104    plonk::{
105        self, keygen_vk_with_k, prepare, Circuit, ConstraintSystem, Error, ProvingKey, VerifyingKey,
106    },
107    poly::{
108        commitment::{Guard, Params},
109        kzg::{
110            params::{ParamsKZG, ParamsVerifierKZG},
111            KZGCommitmentScheme,
112        },
113    },
114    transcript::{CircuitTranscript, Hashable, Sampleable, Transcript, TranscriptHash},
115    utils::SerdeFormat,
116};
117use num_bigint::BigUint;
118use rand::{CryptoRng, RngCore};
119
120use crate::{
121    external::{blake2b::Blake2bWrapper, keccak_sha3::KeccakSha3Wrapper},
122    utils::plonk_api::BlstPLONK,
123};
124
125type C = midnight_curves::JubjubExtended;
126type F = midnight_curves::Fq;
127
128// Type aliases, for readability.
129type NG = NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>;
130type Secp256k1BaseChip = FieldChip<F, k256_mod::Fp, MEP, NG>;
131type Secp256k1ScalarChip = FieldChip<F, k256_mod::Fq, MEP, NG>;
132type Secp256k1Chip = ForeignWeierstrassEccChip<F, K256, MEP, Secp256k1ScalarChip, NG>;
133type P256BaseChip = FieldChip<F, p256_mod::Fp, MEP, NG>;
134type P256ScalarChip = FieldChip<F, p256_mod::Fq, MEP, NG>;
135type P256Chip = ForeignWeierstrassEccChip<F, P256, MEP, P256ScalarChip, NG>;
136type Bls12381BaseChip = FieldChip<F, midnight_curves::Fp, MEP, NG>;
137type Bls12381Chip = ForeignWeierstrassEccChip<
138    F,
139    midnight_curves::G1Projective,
140    midnight_curves::G1Projective,
141    NG,
142    NG,
143>;
144type Curve25519BaseChip = FieldChip<F, curve25519_mod::Fp, MEP, NG>;
145type Curve25519ScalarChip = FieldChip<F, curve25519_mod::Scalar, MEP, NG>;
146type Curve25519Chip = ForeignEdwardsEccChip<F, Curve25519, MEP, Curve25519ScalarChip, NG>;
147
148const ZKSTD_VERSION: u32 = 2;
149
150/// Byte size of a serialized BLS12-381 G1 commitment (compressed).
151const COMMITMENT_BYTE_SIZE: usize = 48;
152
153/// Byte size of a serialized BLS12-381 scalar.
154const SCALAR_BYTE_SIZE: usize = 32;
155
156/// Architecture of the standard library. Specifies what chips need to be
157/// configured.
158#[derive(Clone, Copy, Debug, Eq, PartialEq, Encode, Decode)]
159pub struct ZkStdLibArch {
160    /// Enable the Jubjub chip?
161    pub jubjub: bool,
162
163    /// Enable the Poseidon chip?
164    pub poseidon: bool,
165
166    /// Enable the SHA256 chip?
167    pub sha2_256: bool,
168
169    /// Enable the SHA512 chip?
170    pub sha2_512: bool,
171
172    /// Enable the Keccak chip? (third-party implementation)
173    ///
174    /// Note: is configured using the same columns and tables as sha3_256,
175    /// meaning enabling either of the two, or both, requires the same
176    /// configuration resources.
177    pub keccak_256: bool,
178
179    /// Enable the Sha3 chip? (third-party implementation)
180    ///
181    /// Note: is configured using the same columns and tables as keccak_256,
182    /// meaning enabling either of the two, or both, requires the same
183    /// configuration resources.
184    pub sha3_256: bool,
185
186    /// Enable the Blake2b chip? (third-party implementation)
187    pub blake2b: bool,
188
189    /// Enable the Secp256k1 chip?
190    pub secp256k1: bool,
191
192    /// Enable the P256 chip?
193    pub p256: bool,
194
195    /// Enable BLS12-381 chip?
196    pub bls12_381: bool,
197
198    /// Enable Curve25519 chip?
199    pub curve25519: bool,
200
201    /// Enable base64 chip?
202    pub base64: bool,
203
204    /// Enable scanner chip (automaton-based parsing and substring checks)?
205    pub automaton: bool,
206
207    /// Number of parallel lookups for range checks.
208    pub nr_pow2range_cols: u8,
209}
210
211impl Default for ZkStdLibArch {
212    fn default() -> Self {
213        ZkStdLibArch {
214            jubjub: false,
215            poseidon: false,
216            sha2_256: false,
217            sha2_512: false,
218            sha3_256: false,
219            keccak_256: false,
220            blake2b: false,
221            secp256k1: false,
222            p256: false,
223            bls12_381: false,
224            curve25519: false,
225            base64: false,
226            automaton: false,
227            nr_pow2range_cols: 1,
228        }
229    }
230}
231
232/// Serialized layout of [`ZkStdLibArch`] as it existed in
233/// `midnight-zk-stdlib` v1.0.0 (ZKSTD_VERSION 1). Used only for migration
234/// when reading old serialized verifying keys.
235#[derive(Decode)]
236struct ZkStdLibArchV1 {
237    jubjub: bool,
238    poseidon: bool,
239    sha2_256: bool,
240    sha2_512: bool,
241    keccak_256: bool,
242    sha3_256: bool,
243    blake2b: bool,
244    secp256k1: bool,
245    bls12_381: bool,
246    base64: bool,
247    automaton: bool,
248    nr_pow2range_cols: u8,
249}
250
251impl From<ZkStdLibArchV1> for ZkStdLibArch {
252    fn from(v1: ZkStdLibArchV1) -> Self {
253        ZkStdLibArch {
254            jubjub: v1.jubjub,
255            poseidon: v1.poseidon,
256            sha2_256: v1.sha2_256,
257            sha2_512: v1.sha2_512,
258            keccak_256: v1.keccak_256,
259            sha3_256: v1.sha3_256,
260            blake2b: v1.blake2b,
261            secp256k1: v1.secp256k1,
262            p256: false,
263            bls12_381: v1.bls12_381,
264            curve25519: false,
265            base64: v1.base64,
266            automaton: v1.automaton,
267            nr_pow2range_cols: v1.nr_pow2range_cols,
268        }
269    }
270}
271
272impl ZkStdLibArch {
273    /// Writes the ZKStd architecture to a buffer.
274    pub fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
275        writer.write_all(&ZKSTD_VERSION.to_le_bytes())?;
276        bincode::encode_into_std_write(self, writer, standard())
277            .map(|_| ())
278            .map_err(io::Error::other)
279    }
280
281    /// Reads the ZkStd architecture from a buffer.
282    pub fn read<R: io::Read>(reader: &mut R) -> io::Result<Self> {
283        let mut version = [0u8; 4];
284        reader.read_exact(&mut version)?;
285        let version = u32::from_le_bytes(version);
286        match version {
287            1 => bincode::decode_from_std_read::<ZkStdLibArchV1, _, _>(reader, standard())
288                .map(ZkStdLibArch::from)
289                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
290            2 => bincode::decode_from_std_read(reader, standard())
291                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
292            _ => Err(io::Error::new(
293                io::ErrorKind::InvalidData,
294                format!("Unsupported ZKStd version: {}", version),
295            )),
296        }
297    }
298
299    /// Reads the ZkStdArchitecture from a buffer where a MidnightVK was
300    /// serialized. This enables the reader to know the architecture without
301    /// the need of deserializing the full verifying key.
302    pub fn read_from_serialized_vk<R: io::Read>(reader: &mut R) -> io::Result<Self> {
303        // The current serialization of the verifying key places the architecture at
304        // the beginning.
305        Self::read(reader)
306    }
307}
308
309#[derive(Debug, Clone)]
310/// Configured chips for [ZkStdLib].
311pub struct ZkStdLibConfig {
312    native_config: NativeConfig,
313    core_decomposition_config: P2RDecompositionConfig,
314    jubjub_config: Option<EccConfig>,
315    sha2_256_config: Option<Sha256Config>,
316    sha2_512_config: Option<Sha512Config>,
317    poseidon_config: Option<PoseidonConfig<midnight_curves::Fq>>,
318    secp256k1_scalar_config: Option<FieldChipConfig>,
319    secp256k1_config: Option<ForeignWeierstrassEccConfig<K256>>,
320    p256_scalar_config: Option<FieldChipConfig>,
321    p256_config: Option<ForeignWeierstrassEccConfig<P256>>,
322    bls12_381_config: Option<ForeignWeierstrassEccConfig<midnight_curves::G1Projective>>,
323    curve25519_scalar_config: Option<FieldChipConfig>,
324    curve25519_config: Option<ForeignEdwardsEccConfig<Curve25519>>,
325    base64_config: Option<Base64Config>,
326    scanner_config: Option<ScannerConfig>,
327
328    // Configuration of external libraries.
329    keccak_sha3_config: Option<PackedConfig>,
330    blake2b_config: Option<Blake2bConfig>,
331}
332
333/// The `ZkStdLib` exposes all tools that are used in circuit generation.
334#[derive(Clone, Debug)]
335#[allow(clippy::type_complexity)]
336pub struct ZkStdLib {
337    // Internal chips and gadgets.
338    native_gadget: NG,
339    core_decomposition_chip: P2RDecompositionChip<F>,
340    jubjub_chip: Option<EccChip<C>>,
341    sha2_256_chip: Option<Sha256Chip<F>>,
342    varlen_sha2_256_gadget: Option<VarLenSha256Gadget<F>>,
343    sha2_512_chip: Option<Sha512Chip<F>>,
344    poseidon_gadget: Option<PoseidonChip<F>>,
345    varlen_poseidon_gadget: Option<VarLenPoseidonGadget<F>>,
346    htc_gadget: Option<HashToCurveGadget<F, C, AssignedNative<F>, PoseidonChip<F>, EccChip<C>>>,
347    map_gadget: Option<MapGadget<F, NG, PoseidonChip<F>>>,
348    biguint_gadget: BigUintGadget<F, NG>,
349    secp256k1_chip: Option<Secp256k1Chip>,
350    p256_chip: Option<P256Chip>,
351    bls12_381_chip: Option<Bls12381Chip>,
352    curve25519_chip: Option<Curve25519Chip>,
353    base64_chip: Option<Base64Chip<F>>,
354    parser_gadget: ParserGadget<F, NG>,
355    scanner_chip: Option<ScannerChip<F>>,
356    vector_gadget: VectorGadget<F>,
357    verifier_gadget: Option<VerifierGadget<BlstrsEmulation>>,
358
359    // Third-party chips.
360    keccak_sha3_chip: Option<KeccakSha3Wrapper<F>>,
361    blake2b_chip: Option<Blake2bWrapper<F>>,
362
363    // Flags that indicate if certain chips have been used. This way we can load the tables only
364    // when necessary (thus reducing the min_k in some cases).
365    // Such a usage flag has to be added and updated correctly for each new chip using tables.
366    used_sha2_256: Rc<RefCell<bool>>,
367    used_sha2_512: Rc<RefCell<bool>>,
368    used_secp256k1: Rc<RefCell<bool>>,
369    used_p256: Rc<RefCell<bool>>,
370    used_bls12_381: Rc<RefCell<bool>>,
371    used_curve25519: Rc<RefCell<bool>>,
372    used_base64: Rc<RefCell<bool>>,
373    used_scanner: Rc<RefCell<bool>>,
374    used_keccak_or_sha3: Rc<RefCell<bool>>,
375    used_blake2b: Rc<RefCell<bool>>,
376}
377
378impl ZkStdLib {
379    /// Creates a new [ZkStdLib] given its config.
380    pub fn new(config: &ZkStdLibConfig, max_bit_len: usize) -> Self {
381        let native_chip = NativeChip::new(&config.native_config, &());
382        let core_decomposition_chip =
383            P2RDecompositionChip::new(&config.core_decomposition_config, &max_bit_len);
384        let native_gadget = NativeGadget::new(core_decomposition_chip.clone(), native_chip.clone());
385        let jubjub_chip = (config.jubjub_config.as_ref())
386            .map(|jubjub_config| EccChip::new(jubjub_config, &native_gadget));
387        let sha2_256_chip = (config.sha2_256_config.as_ref())
388            .map(|sha256_config| Sha256Chip::new(sha256_config, &native_gadget));
389        let varlen_sha2_256_gadget = sha2_256_chip.as_ref().map(VarLenSha256Gadget::new);
390        let sha2_512_chip = (config.sha2_512_config.as_ref())
391            .map(|sha512_config| Sha512Chip::new(sha512_config, &native_gadget));
392        let poseidon_gadget = (config.poseidon_config.as_ref())
393            .map(|poseidon_config| PoseidonChip::new(poseidon_config, &native_chip));
394        let varlen_poseidon_gadget = (poseidon_gadget.as_ref())
395            .map(|poseidon| VarLenPoseidonGadget::new(poseidon, &native_gadget));
396        let htc_gadget = (jubjub_chip.as_ref())
397            .zip(poseidon_gadget.as_ref())
398            .map(|(ecc_chip, poseidon_gadget)| HashToCurveGadget::new(poseidon_gadget, ecc_chip));
399        let biguint_gadget = BigUintGadget::new(&native_gadget);
400        let map_gadget = poseidon_gadget
401            .as_ref()
402            .map(|poseidon_gadget| MapGadget::new(&native_gadget, poseidon_gadget));
403        let secp256k1_scalar_chip = (config.secp256k1_scalar_config.as_ref())
404            .map(|scalar_config| FieldChip::new(scalar_config, &native_gadget));
405        let secp256k1_chip = (config.secp256k1_config.as_ref())
406            .zip(secp256k1_scalar_chip.as_ref())
407            .map(|(curve_config, scalar_chip)| {
408                ForeignWeierstrassEccChip::new(curve_config, &native_gadget, scalar_chip)
409            });
410        let p256_scalar_chip = (config.p256_scalar_config.as_ref())
411            .map(|scalar_config| FieldChip::new(scalar_config, &native_gadget));
412        let p256_chip = (config.p256_config.as_ref()).zip(p256_scalar_chip.as_ref()).map(
413            |(curve_config, scalar_chip)| {
414                ForeignWeierstrassEccChip::new(curve_config, &native_gadget, scalar_chip)
415            },
416        );
417        let bls12_381_chip = (config.bls12_381_config.as_ref()).map(|curve_config| {
418            ForeignWeierstrassEccChip::new(curve_config, &native_gadget, &native_gadget)
419        });
420        let curve25519_scalar_chip = (config.curve25519_scalar_config.as_ref())
421            .map(|scalar_config| FieldChip::new(scalar_config, &native_gadget));
422        let curve25519_chip = (config.curve25519_config.as_ref())
423            .zip(curve25519_scalar_chip.as_ref())
424            .map(|(curve_config, scalar_chip)| {
425                ForeignEdwardsEccChip::new(curve_config, &native_gadget, scalar_chip)
426            });
427
428        let base64_chip = (config.base64_config.as_ref())
429            .map(|base64_config| Base64Chip::new(base64_config, &native_gadget));
430
431        let parser_gadget = ParserGadget::new(&native_gadget);
432        let scanner_chip =
433            config.scanner_config.as_ref().map(|c| ScannerChip::new(c, &native_gadget));
434        let vector_gadget = VectorGadget::new(&native_gadget);
435
436        let verifier_gadget = bls12_381_chip.as_ref().zip(poseidon_gadget.as_ref()).map(
437            |(curve_chip, sponge_chip)| {
438                VerifierGadget::<BlstrsEmulation>::new(curve_chip, &native_gadget, sponge_chip)
439            },
440        );
441
442        let keccak_sha3_chip = config
443            .keccak_sha3_config
444            .as_ref()
445            .map(|sha3_config| KeccakSha3Wrapper::new(sha3_config, &native_gadget));
446        let blake2b_chip = config
447            .blake2b_config
448            .as_ref()
449            .map(|blake2b_config| Blake2bWrapper::new(blake2b_config, &native_gadget));
450
451        Self {
452            native_gadget,
453            core_decomposition_chip,
454            jubjub_chip,
455            sha2_256_chip,
456            varlen_sha2_256_gadget,
457            sha2_512_chip,
458            poseidon_gadget,
459            varlen_poseidon_gadget,
460            map_gadget,
461            htc_gadget,
462            biguint_gadget,
463            secp256k1_chip,
464            p256_chip,
465            bls12_381_chip,
466            curve25519_chip,
467            base64_chip,
468            parser_gadget,
469            scanner_chip,
470            vector_gadget,
471            verifier_gadget,
472            keccak_sha3_chip,
473            blake2b_chip,
474            used_sha2_256: Rc::new(RefCell::new(false)),
475            used_sha2_512: Rc::new(RefCell::new(false)),
476            used_secp256k1: Rc::new(RefCell::new(false)),
477            used_p256: Rc::new(RefCell::new(false)),
478            used_bls12_381: Rc::new(RefCell::new(false)),
479            used_curve25519: Rc::new(RefCell::new(false)),
480            used_base64: Rc::new(RefCell::new(false)),
481            used_scanner: Rc::new(RefCell::new(false)),
482            used_keccak_or_sha3: Rc::new(RefCell::new(false)),
483            used_blake2b: Rc::new(RefCell::new(false)),
484        }
485    }
486
487    /// Configure [ZkStdLib] from scratch.
488    pub fn configure(
489        meta: &mut ConstraintSystem<F>,
490        (arch, max_bit_len): (ZkStdLibArch, u8),
491    ) -> ZkStdLibConfig {
492        let nb_advice_cols = [
493            NB_ARITH_COLS,
494            arch.nr_pow2range_cols as usize,
495            arch.jubjub as usize * NB_EDWARDS_COLS,
496            arch.poseidon as usize * NB_POSEIDON_ADVICE_COLS,
497            arch.sha2_256 as usize * NB_SHA256_ADVICE_COLS,
498            arch.sha2_512 as usize * NB_SHA512_ADVICE_COLS,
499            arch.secp256k1 as usize
500                * max(
501                    nb_field_chip_columns::<F, k256_mod::Fq, MEP>(),
502                    nb_foreign_ecc_chip_columns::<F, K256, MEP, k256_mod::Fq>(),
503                ),
504            arch.p256 as usize
505                * max(
506                    nb_field_chip_columns::<F, p256_mod::Fq, MEP>(),
507                    nb_foreign_ecc_chip_columns::<F, P256, MEP, p256_mod::Fq>(),
508                ),
509            arch.bls12_381 as usize
510                * max(
511                    nb_field_chip_columns::<F, midnight_curves::Fp, MEP>(),
512                    nb_foreign_ecc_chip_columns::<
513                        F,
514                        midnight_curves::G1Projective,
515                        MEP,
516                        midnight_curves::Fp,
517                    >(),
518                ),
519            arch.curve25519 as usize
520                * max(
521                    nb_field_chip_columns::<F, curve25519_mod::Scalar, MEP>(),
522                    nb_foreign_edwards_chip_columns::<F, Curve25519, MEP>(),
523                ),
524            arch.base64 as usize * NB_BASE64_ADVICE_COLS,
525            arch.automaton as usize * NB_SCANNER_ADVICE_COLS,
526            (arch.keccak_256 || arch.sha3_256) as usize * PACKED_ADVICE_COLS,
527            arch.blake2b as usize * NB_BLAKE2B_ADVICE_COLS,
528        ]
529        .into_iter()
530        .max()
531        .unwrap_or(0);
532
533        let nb_fixed_cols = [
534            NB_ARITH_FIXED_COLS,
535            arch.poseidon as usize * NB_POSEIDON_FIXED_COLS,
536            arch.sha2_256 as usize * NB_SHA256_FIXED_COLS,
537            arch.sha2_512 as usize * NB_SHA512_FIXED_COLS,
538            (arch.keccak_256 || arch.sha3_256) as usize * PACKED_FIXED_COLS,
539            arch.automaton as usize * NB_SCANNER_FIXED_COLS,
540        ]
541        .into_iter()
542        .max()
543        .unwrap_or(0);
544
545        let advice_columns = (0..nb_advice_cols).map(|_| meta.advice_column()).collect::<Vec<_>>();
546        let fixed_columns = (0..nb_fixed_cols).map(|_| meta.fixed_column()).collect::<Vec<_>>();
547        let committed_instance_column = meta.instance_column();
548        let instance_column = meta.instance_column();
549
550        let native_config = NativeChip::configure(
551            meta,
552            &(
553                advice_columns[..NB_ARITH_COLS].try_into().unwrap(),
554                fixed_columns[..NB_ARITH_FIXED_COLS].try_into().unwrap(),
555                [committed_instance_column, instance_column],
556            ),
557        );
558
559        let nb_parallel_range_checks = arch.nr_pow2range_cols as usize;
560        let max_bit_len = max_bit_len as u32;
561
562        let pow2range_config =
563            Pow2RangeChip::configure(meta, &advice_columns[1..=arch.nr_pow2range_cols as usize]);
564
565        let core_decomposition_config =
566            P2RDecompositionChip::configure(meta, &(native_config.clone(), pow2range_config));
567
568        let jubjub_config = arch.jubjub.then(|| {
569            EccChip::<C>::configure(meta, &advice_columns[..NB_EDWARDS_COLS].try_into().unwrap())
570        });
571
572        let sha2_256_config = arch.sha2_256.then(|| {
573            Sha256Chip::configure(
574                meta,
575                &(
576                    advice_columns[..NB_SHA256_ADVICE_COLS].try_into().unwrap(),
577                    fixed_columns[..NB_SHA256_FIXED_COLS].try_into().unwrap(),
578                ),
579            )
580        });
581
582        let sha2_512_config = arch.sha2_512.then(|| {
583            Sha512Chip::configure(
584                meta,
585                &(
586                    advice_columns[..NB_SHA512_ADVICE_COLS].try_into().unwrap(),
587                    fixed_columns[..NB_SHA512_FIXED_COLS].try_into().unwrap(),
588                ),
589            )
590        });
591
592        let poseidon_config = arch.poseidon.then(|| {
593            PoseidonChip::configure(
594                meta,
595                &(
596                    advice_columns[..NB_POSEIDON_ADVICE_COLS].try_into().unwrap(),
597                    fixed_columns[..NB_POSEIDON_FIXED_COLS].try_into().unwrap(),
598                ),
599            )
600        });
601
602        let secp256k1_scalar_config = arch.secp256k1.then(|| {
603            Secp256k1ScalarChip::configure(
604                meta,
605                &advice_columns,
606                nb_parallel_range_checks,
607                max_bit_len,
608            )
609        });
610
611        let secp256k1_config = arch.secp256k1.then(|| {
612            let base_config = Secp256k1BaseChip::configure(
613                meta,
614                &advice_columns,
615                nb_parallel_range_checks,
616                max_bit_len,
617            );
618            Secp256k1Chip::configure(
619                meta,
620                &base_config,
621                &advice_columns,
622                nb_parallel_range_checks,
623                max_bit_len,
624            )
625        });
626
627        let p256_scalar_config = arch.p256.then(|| {
628            P256ScalarChip::configure(meta, &advice_columns, nb_parallel_range_checks, max_bit_len)
629        });
630
631        let p256_config = arch.p256.then(|| {
632            let base_config = P256BaseChip::configure(
633                meta,
634                &advice_columns,
635                nb_parallel_range_checks,
636                max_bit_len,
637            );
638            P256Chip::configure(
639                meta,
640                &base_config,
641                &advice_columns,
642                nb_parallel_range_checks,
643                max_bit_len,
644            )
645        });
646
647        let bls12_381_config = arch.bls12_381.then(|| {
648            let base_config = Bls12381BaseChip::configure(
649                meta,
650                &advice_columns,
651                nb_parallel_range_checks,
652                max_bit_len,
653            );
654            Bls12381Chip::configure(
655                meta,
656                &base_config,
657                &advice_columns,
658                nb_parallel_range_checks,
659                max_bit_len,
660            )
661        });
662
663        let curve25519_scalar_config = arch.curve25519.then(|| {
664            Curve25519ScalarChip::configure(
665                meta,
666                &advice_columns,
667                nb_parallel_range_checks,
668                max_bit_len,
669            )
670        });
671
672        let curve25519_config = arch.curve25519.then(|| {
673            let base_config = Curve25519BaseChip::configure(
674                meta,
675                &advice_columns,
676                nb_parallel_range_checks,
677                max_bit_len,
678            );
679            Curve25519Chip::configure(
680                meta,
681                &base_config,
682                &advice_columns,
683                &fixed_columns,
684                nb_parallel_range_checks,
685                max_bit_len,
686            )
687        });
688
689        let base64_config = arch.base64.then(|| {
690            Base64Chip::configure(
691                meta,
692                advice_columns[..NB_BASE64_ADVICE_COLS].try_into().unwrap(),
693            )
694        });
695
696        let scanner_config = arch.automaton.then(|| {
697            ScannerChip::configure(
698                meta,
699                &(
700                    advice_columns[..NB_SCANNER_ADVICE_COLS].try_into().unwrap(),
701                    fixed_columns[0],
702                    parsing::spec_library(),
703                ),
704            )
705        });
706
707        let constant_column =
708            (arch.keccak_256 || arch.sha3_256 || arch.blake2b).then(|| meta.fixed_column());
709
710        let keccak_sha3_config = (arch.keccak_256 || arch.sha3_256).then(|| {
711            PackedChip::configure(
712                meta,
713                constant_column.unwrap(),
714                advice_columns[..PACKED_ADVICE_COLS].try_into().unwrap(),
715                fixed_columns[..PACKED_FIXED_COLS].try_into().unwrap(),
716            )
717        });
718
719        let blake2b_config = arch.blake2b.then(|| {
720            Blake2bChip::configure(
721                meta,
722                constant_column.unwrap(),
723                advice_columns[0],
724                advice_columns[1..NB_BLAKE2B_ADVICE_COLS].try_into().unwrap(),
725            )
726        });
727
728        ZkStdLibConfig {
729            native_config,
730            core_decomposition_config,
731            jubjub_config,
732            sha2_256_config,
733            sha2_512_config,
734            poseidon_config,
735            secp256k1_scalar_config,
736            secp256k1_config,
737            p256_scalar_config,
738            p256_config,
739            bls12_381_config,
740            curve25519_scalar_config,
741            curve25519_config,
742            base64_config,
743            scanner_config,
744            keccak_sha3_config,
745            blake2b_config,
746        }
747    }
748}
749
750impl ZkStdLib {
751    /// Native EccChip.
752    pub fn jubjub(&self) -> &EccChip<C> {
753        self.jubjub_chip.as_ref().expect("ZkStdLibArch must enable jubjub")
754    }
755
756    /// Gadget for performing in-circuit big-unsigned integer operations.
757    pub fn biguint(&self) -> &BigUintGadget<F, NG> {
758        &self.biguint_gadget
759    }
760
761    /// Gadget for performing map and non-map checks
762    pub fn map_gadget(&self) -> &MapGadget<F, NG, PoseidonChip<F>> {
763        self.map_gadget
764            .as_ref()
765            .unwrap_or_else(|| panic!("ZkStdLibArch must enable poseidon"))
766    }
767
768    /// Chip for performing in-circuit operations over the Secp256k1 curve, its
769    /// scalar field or its base field.
770    pub fn secp256k1(&self) -> &Secp256k1Chip {
771        *self.used_secp256k1.borrow_mut() = true;
772        self.secp256k1_chip
773            .as_ref()
774            .unwrap_or_else(|| panic!("ZkStdLibArch must enable secp256k1"))
775    }
776
777    /// Chip for performing in-circuit operations over the P256 curve, its
778    /// scalar field or its base field.
779    pub fn p256(&self) -> &P256Chip {
780        *self.used_p256.borrow_mut() = true;
781        self.p256_chip
782            .as_ref()
783            .unwrap_or_else(|| panic!("ZkStdLibArch must enable p256"))
784    }
785
786    /// Chip for performing in-circuit operations over the BLS12-381 curve, its
787    /// scalar field or its base field.
788    pub fn bls12_381(&self) -> &Bls12381Chip {
789        *self.used_bls12_381.borrow_mut() = true;
790        self.bls12_381_chip
791            .as_ref()
792            .unwrap_or_else(|| panic!("ZkStdLibArch must enable bls12_381"))
793    }
794
795    /// Chip for performing in-circuit operations over Curve25519.
796    pub fn curve25519(&self) -> &Curve25519Chip {
797        *self.used_curve25519.borrow_mut() = true;
798        self.curve25519_chip
799            .as_ref()
800            .unwrap_or_else(|| panic!("ZkStdLibArch must enable curve25519"))
801    }
802
803    /// Chip for performing in-circuit base64 decoding.
804    pub fn base64(&self) -> &Base64Chip<F> {
805        *self.used_base64.borrow_mut() = true;
806        self.base64_chip
807            .as_ref()
808            .unwrap_or_else(|| panic!("ZkStdLibArch must enable base64"))
809    }
810
811    /// Gadget for column-free parsing helpers (fetch_bytes, date_to_int, etc.).
812    /// Always available (no arch flag needed).
813    pub fn parser(&self) -> &ParserGadget<F, NG> {
814        &self.parser_gadget
815    }
816
817    /// Chip for various scanning functions based on lookups. This includes
818    /// automaton-based parsing ([`ScannerChip::parse`]) and substring checks
819    /// ([`ScannerChip::check_subsequence`], [`ScannerChip::check_bytes`]).
820    ///
821    /// Returns the scanner chip for automaton-based parsing and substring
822    /// checks. The static automaton table is loaded automatically when
823    /// `parse` is called with a `Static(..)` variant.
824    pub fn scanner(&self) -> &ScannerChip<F> {
825        *self.used_scanner.borrow_mut() = true;
826        (self.scanner_chip.as_ref()).unwrap_or_else(|| panic!("ZkStdLibArch must enable automaton"))
827    }
828
829    /// Chip for performing in-circuit verification of proofs
830    /// (generated with Poseidon as the Fiat-Shamir transcript hash).
831    pub fn verifier(&self) -> &VerifierGadget<BlstrsEmulation> {
832        *self.used_bls12_381.borrow_mut() = true;
833        self.verifier_gadget
834            .as_ref()
835            .unwrap_or_else(|| panic!("ZkStdLibArch must enable bls12_381 & poseidon"))
836    }
837
838    /// Assert that a given assigned bit is true.
839    ///
840    /// ```
841    /// # midnight_zk_stdlib::run_test_stdlib!(chip, layouter, 13, {
842    /// let input: AssignedBit<F> = chip.assign_fixed(layouter, true)?;
843    /// chip.assert_true(layouter, &input)?;
844    /// # });
845    /// ```
846    pub fn assert_true(
847        &self,
848        layouter: &mut impl Layouter<F>,
849        input: &AssignedBit<F>,
850    ) -> Result<(), Error> {
851        self.native_gadget.assert_equal_to_fixed(layouter, input, true)
852    }
853
854    /// Assert that a given assigned bit is false
855    pub fn assert_false(
856        &self,
857        layouter: &mut impl Layouter<F>,
858        input: &AssignedBit<F>,
859    ) -> Result<(), Error> {
860        self.native_gadget.assert_equal_to_fixed(layouter, input, false)
861    }
862
863    /// Returns `1` iff `x < y`.
864    ///
865    /// ```
866    /// # midnight_zk_stdlib::run_test_stdlib!(chip, layouter, 13, {
867    /// let x: AssignedNative<F> = chip.assign_fixed(layouter, F::from(127))?;
868    /// let y: AssignedNative<F> = chip.assign_fixed(layouter, F::from(212))?;
869    /// let condition = chip.lower_than(layouter, &x, &y, 8)?;
870    ///
871    /// chip.assert_true(layouter, &condition)?;
872    /// # });
873    /// ```
874    ///
875    /// # Unsatisfiable Circuit
876    ///
877    /// If `x` or `y` are not in the range `[0, 2^n)`.
878    ///
879    /// ```should_panic
880    /// # midnight_zk_stdlib::run_test_stdlib!(chip, layouter, 13, {
881    /// let x: AssignedNative<F> = chip.assign_fixed(layouter, F::from(127))?;
882    /// let y: AssignedNative<F> = chip.assign_fixed(layouter, F::from(212))?;
883    /// let _condition = chip.lower_than(layouter, &x, &y, 7)?;
884    /// # });
885    /// ```
886    ///
887    /// Setting `n > (F::NUM_BITS - 1) / 2` will result in a compile-time
888    /// error.
889    pub fn lower_than(
890        &self,
891        layouter: &mut impl Layouter<F>,
892        x: &AssignedNative<F>,
893        y: &AssignedNative<F>,
894        n: u32,
895    ) -> Result<AssignedBit<F>, Error> {
896        let bounded_x = self.native_gadget.bounded_of_element(layouter, n as usize, x)?;
897        let bounded_y = self.native_gadget.bounded_of_element(layouter, n as usize, y)?;
898        self.native_gadget.lower_than(layouter, &bounded_x, &bounded_y)
899    }
900
901    /// Poseidon hash from a slice of native values into a native value.
902    ///
903    /// ```
904    /// # midnight_zk_stdlib::run_test_stdlib!(chip, layouter, 13, {
905    /// let x: AssignedNative<F> = chip.assign_fixed(layouter, F::from(127))?;
906    /// let y: AssignedNative<F> = chip.assign_fixed(layouter, F::from(212))?;
907    ///
908    /// let _hash = chip.poseidon(layouter, &[x, y])?;
909    /// # });
910    /// ```
911    pub fn poseidon(
912        &self,
913        layouter: &mut impl Layouter<F>,
914        input: &[AssignedNative<F>],
915    ) -> Result<AssignedNative<F>, Error> {
916        self.poseidon_gadget
917            .as_ref()
918            .unwrap_or_else(|| panic!("ZkStdLibArch must enable poseidon"))
919            .hash(layouter, input)
920    }
921
922    /// Poseidon hash over a payload whose element count can vary between
923    /// proofs.
924    ///
925    /// Unlike [`poseidon`](Self::poseidon), which takes a plain slice whose
926    /// length is structurally fixed in the circuit (the same for every proof),
927    /// this variant takes an [`AssignedVector`]: a specialized structure for
928    /// carrying data of varying length, up to a maximum capacity of `M`
929    /// elements.
930    ///
931    /// `M` must be a multiple of 2 (the Poseidon absorption rate).
932    pub fn poseidon_varlen<const M: usize>(
933        &self,
934        layouter: &mut impl Layouter<F>,
935        input: &AssignedVector<F, AssignedNative<F>, M, RATE>,
936    ) -> Result<AssignedNative<F>, Error> {
937        assert!(
938            M.is_multiple_of(RATE),
939            "poseidon_varlen only supports assigned vector whose maxlen M is a multiple of {RATE} (here M = {M})"
940        );
941        self.varlen_poseidon_gadget
942            .as_ref()
943            .unwrap_or_else(|| panic!("ZkStdLibArch must enable poseidon"))
944            .poseidon_varlen(layouter, input)
945    }
946
947    /// Hashes a slice of assigned values into `(x, y)` coordinates which are
948    /// guaranteed to be in the curve `C`. For usage, see [HashToCurveGadget].
949    pub fn hash_to_curve(
950        &self,
951        layouter: &mut impl Layouter<F>,
952        inputs: &[AssignedNative<F>],
953    ) -> Result<AssignedNativePoint<C>, Error> {
954        self.htc_gadget
955            .as_ref()
956            .unwrap_or_else(|| panic!("ZkStdLibArch must enable poseidon and jubjub"))
957            .hash_to_curve(layouter, inputs)
958    }
959
960    /// SHA2-256.
961    /// Takes as input a slice of assigned bytes and returns the assigned
962    /// input/output in bytes.
963    /// We assume the field uses little endian encoding.
964    /// ```
965    /// # midnight_zk_stdlib::run_test_stdlib!(chip, layouter, 13, {
966    /// let input = chip.assign_many(
967    ///     layouter,
968    ///     &[
969    ///         Value::known(13),
970    ///         Value::known(226),
971    ///         Value::known(119),
972    ///         Value::known(5),
973    ///     ],
974    /// )?;
975    ///
976    /// let _hash = chip.sha2_256(layouter, &input)?;
977    /// # });
978    /// ```
979    pub fn sha2_256(
980        &self,
981        layouter: &mut impl Layouter<F>,
982        input: &[AssignedByte<F>], // F -> decompose_bytes -> hash
983    ) -> Result<[AssignedByte<F>; 32], Error> {
984        *self.used_sha2_256.borrow_mut() = true;
985        self.sha2_256_chip
986            .as_ref()
987            .expect("ZkStdLibArch must enable sha256")
988            .hash(layouter, input)
989    }
990
991    /// SHA-256 hash over a payload whose byte count can vary between proofs.
992    ///
993    /// Unlike [`sha2_256`](Self::sha2_256), which takes a plain slice whose
994    /// length is structurally fixed in the circuit (the same for every proof),
995    /// this variant takes an [`AssignedVector`]: a specialized structure for
996    /// carrying data of varying length, up to a maximum capacity of `M` bytes.
997    ///
998    /// `M` must be a multiple of 64 (the SHA-256 block size).
999    pub fn sha2_256_varlen<const M: usize>(
1000        &self,
1001        layouter: &mut impl Layouter<F>,
1002        input: &AssignedVector<F, AssignedByte<F>, M, 64>,
1003    ) -> Result<[AssignedByte<F>; 32], Error> {
1004        *self.used_sha2_256.borrow_mut() = true;
1005        assert!(
1006            M.is_multiple_of(64),
1007            "sha2_256_varlen only supports assigned vector whose maxlen M is a multiple of 64 (here M = {M})"
1008        );
1009        self.varlen_sha2_256_gadget
1010            .as_ref()
1011            .expect("ZkStdLibArch must enable sha256")
1012            .varhash(layouter, input)
1013    }
1014
1015    /// SHA2-512 hash.
1016    pub fn sha2_512(
1017        &self,
1018        layouter: &mut impl Layouter<F>,
1019        input: &[AssignedByte<F>], // F -> decompose_bytes -> hash
1020    ) -> Result<[AssignedByte<F>; 64], Error> {
1021        *self.used_sha2_512.borrow_mut() = true;
1022        self.sha2_512_chip
1023            .as_ref()
1024            .expect("ZkStdLibArch must enable sha512")
1025            .hash(layouter, input)
1026    }
1027
1028    /// SHA3-256 hash (third-party implementation).
1029    pub fn sha3_256(
1030        &self,
1031        layouter: &mut impl Layouter<F>,
1032        input: &[AssignedByte<F>],
1033    ) -> Result<[AssignedByte<Fq>; 32], Error> {
1034        *self.used_keccak_or_sha3.borrow_mut() = true;
1035        let chip = self
1036            .keccak_sha3_chip
1037            .as_ref()
1038            .expect("ZkStdLibArch must enable sha3 (or keccak)");
1039        chip.sha3_256_digest(layouter, input)
1040    }
1041
1042    /// Keccak-256 hash (third-party implementation).
1043    pub fn keccak_256(
1044        &self,
1045        layouter: &mut impl Layouter<F>,
1046        input: &[AssignedByte<F>],
1047    ) -> Result<[AssignedByte<Fq>; 32], Error> {
1048        *self.used_keccak_or_sha3.borrow_mut() = true;
1049        let chip = self
1050            .keccak_sha3_chip
1051            .as_ref()
1052            .expect("ZkStdLibArch must enable keccak (or sha3)");
1053        chip.keccak_256_digest(layouter, input)
1054    }
1055
1056    /// Blake2b hash with a 256-bit output, unkeyed (third-party
1057    /// implementation).
1058    pub fn blake2b_256(
1059        &self,
1060        layouter: &mut impl Layouter<F>,
1061        input: &[AssignedByte<F>],
1062    ) -> Result<[AssignedByte<F>; 32], Error> {
1063        *self.used_blake2b.borrow_mut() = true;
1064        let chip = self.blake2b_chip.as_ref().expect("ZkStdLibArch must enable blake2b");
1065        chip.blake2b_256_digest(layouter, input)
1066    }
1067
1068    /// Blake2b hash with a 512-bit output, unkeyed (third-party
1069    /// implementation).
1070    pub fn blake2b_512(
1071        &self,
1072        layouter: &mut impl Layouter<F>,
1073        input: &[AssignedByte<F>],
1074    ) -> Result<[AssignedByte<F>; 64], Error> {
1075        *self.used_blake2b.borrow_mut() = true;
1076        let chip = self.blake2b_chip.as_ref().expect("ZkStdLibArch must enable blake2b");
1077        chip.blake2b_512_digest(layouter, input)
1078    }
1079}
1080
1081impl<T> AssignmentInstructions<F, T> for ZkStdLib
1082where
1083    T: InnerValue,
1084    T::Element: Clone,
1085    NG: AssignmentInstructions<F, T>,
1086{
1087    fn assign(
1088        &self,
1089        layouter: &mut impl Layouter<F>,
1090        value: Value<T::Element>,
1091    ) -> Result<T, Error> {
1092        self.native_gadget.assign(layouter, value)
1093    }
1094
1095    fn assign_fixed(
1096        &self,
1097        layouter: &mut impl Layouter<F>,
1098        constant: T::Element,
1099    ) -> Result<T, Error> {
1100        self.native_gadget.assign_fixed(layouter, constant)
1101    }
1102
1103    fn assign_many(
1104        &self,
1105        layouter: &mut impl Layouter<F>,
1106        values: &[Value<T::Element>],
1107    ) -> Result<Vec<T>, Error> {
1108        self.native_gadget.assign_many(layouter, values)
1109    }
1110}
1111
1112impl<T> PublicInputInstructions<F, T> for ZkStdLib
1113where
1114    T: Instantiable<F>,
1115    T::Element: Clone,
1116    NG: PublicInputInstructions<F, T>,
1117{
1118    fn as_public_input(
1119        &self,
1120        layouter: &mut impl Layouter<F>,
1121        assigned: &T,
1122    ) -> Result<Vec<AssignedNative<F>>, Error> {
1123        self.native_gadget.as_public_input(layouter, assigned)
1124    }
1125
1126    fn constrain_as_public_input(
1127        &self,
1128        layouter: &mut impl Layouter<F>,
1129        assigned: &T,
1130    ) -> Result<(), Error> {
1131        self.native_gadget.constrain_as_public_input(layouter, assigned)
1132    }
1133
1134    fn assign_as_public_input(
1135        &self,
1136        layouter: &mut impl Layouter<F>,
1137        value: Value<<T>::Element>,
1138    ) -> Result<T, Error> {
1139        self.native_gadget.assign_as_public_input(layouter, value)
1140    }
1141}
1142
1143impl<T> CommittedInstanceInstructions<F, T> for ZkStdLib
1144where
1145    F: PrimeField,
1146    T: Instantiable<F>,
1147    NG: CommittedInstanceInstructions<F, T>,
1148{
1149    fn constrain_as_committed_public_input(
1150        &self,
1151        layouter: &mut impl Layouter<F>,
1152        assigned: &T,
1153    ) -> Result<(), Error> {
1154        self.native_gadget.constrain_as_committed_public_input(layouter, assigned)
1155    }
1156}
1157
1158impl<T> AssertionInstructions<F, T> for ZkStdLib
1159where
1160    T: InnerValue,
1161    NG: AssertionInstructions<F, T>,
1162{
1163    fn assert_equal(&self, layouter: &mut impl Layouter<F>, x: &T, y: &T) -> Result<(), Error> {
1164        self.native_gadget.assert_equal(layouter, x, y)
1165    }
1166
1167    fn assert_not_equal(&self, layouter: &mut impl Layouter<F>, x: &T, y: &T) -> Result<(), Error> {
1168        self.native_gadget.assert_not_equal(layouter, x, y)
1169    }
1170
1171    fn assert_equal_to_fixed(
1172        &self,
1173        layouter: &mut impl Layouter<F>,
1174        x: &T,
1175        constant: T::Element,
1176    ) -> Result<(), Error> {
1177        self.native_gadget.assert_equal_to_fixed(layouter, x, constant)
1178    }
1179
1180    fn assert_not_equal_to_fixed(
1181        &self,
1182        layouter: &mut impl Layouter<F>,
1183        x: &T,
1184        constant: T::Element,
1185    ) -> Result<(), Error> {
1186        self.native_gadget.assert_not_equal_to_fixed(layouter, x, constant)
1187    }
1188}
1189
1190impl<T> EqualityInstructions<F, T> for ZkStdLib
1191where
1192    T: InnerValue,
1193    NG: EqualityInstructions<F, T>,
1194{
1195    fn is_equal(
1196        &self,
1197        layouter: &mut impl Layouter<F>,
1198        x: &T,
1199        y: &T,
1200    ) -> Result<AssignedBit<F>, Error> {
1201        self.native_gadget.is_equal(layouter, x, y)
1202    }
1203
1204    fn is_not_equal(
1205        &self,
1206        layouter: &mut impl Layouter<F>,
1207        x: &T,
1208        y: &T,
1209    ) -> Result<AssignedBit<F>, Error> {
1210        self.native_gadget.is_not_equal(layouter, x, y)
1211    }
1212
1213    fn is_equal_to_fixed(
1214        &self,
1215        layouter: &mut impl Layouter<F>,
1216        x: &T,
1217        constant: T::Element,
1218    ) -> Result<AssignedBit<F>, Error> {
1219        self.native_gadget.is_equal_to_fixed(layouter, x, constant)
1220    }
1221
1222    fn is_not_equal_to_fixed(
1223        &self,
1224        layouter: &mut impl Layouter<F>,
1225        x: &T,
1226        constant: T::Element,
1227    ) -> Result<AssignedBit<F>, Error> {
1228        self.native_gadget.is_not_equal_to_fixed(layouter, x, constant)
1229    }
1230}
1231
1232impl<T1, T2> ConversionInstructions<F, T1, T2> for ZkStdLib
1233where
1234    T1: InnerValue,
1235    T2: InnerValue,
1236    NG: ConversionInstructions<F, T1, T2>,
1237{
1238    fn convert_value(&self, x: &T1::Element) -> Option<T2::Element> {
1239        ConversionInstructions::<_, T1, T2>::convert_value(&self.native_gadget, x)
1240    }
1241
1242    fn convert(&self, layouter: &mut impl Layouter<F>, x: &T1) -> Result<T2, Error> {
1243        self.native_gadget.convert(layouter, x)
1244    }
1245}
1246
1247impl CanonicityInstructions<F, AssignedNative<F>> for ZkStdLib {
1248    fn le_bits_lower_than(
1249        &self,
1250        layouter: &mut impl Layouter<F>,
1251        bits: &[AssignedBit<F>],
1252        bound: BigUint,
1253    ) -> Result<AssignedBit<F>, Error> {
1254        self.native_gadget.le_bits_lower_than(layouter, bits, bound)
1255    }
1256
1257    fn le_bits_geq_than(
1258        &self,
1259        layouter: &mut impl Layouter<F>,
1260        bits: &[AssignedBit<F>],
1261        bound: BigUint,
1262    ) -> Result<AssignedBit<F>, Error> {
1263        self.native_gadget.le_bits_geq_than(layouter, bits, bound)
1264    }
1265}
1266
1267impl DecompositionInstructions<F, AssignedNative<F>> for ZkStdLib {
1268    fn assigned_to_le_bits(
1269        &self,
1270        layouter: &mut impl Layouter<F>,
1271        x: &AssignedNative<F>,
1272        nb_bits: Option<usize>,
1273        enforce_canonical: bool,
1274    ) -> Result<Vec<AssignedBit<F>>, Error> {
1275        self.native_gadget.assigned_to_le_bits(layouter, x, nb_bits, enforce_canonical)
1276    }
1277
1278    fn assigned_to_le_bytes(
1279        &self,
1280        layouter: &mut impl Layouter<F>,
1281        x: &AssignedNative<F>,
1282        nb_bytes: Option<usize>,
1283    ) -> Result<Vec<AssignedByte<F>>, Error> {
1284        self.native_gadget.assigned_to_le_bytes(layouter, x, nb_bytes)
1285    }
1286
1287    fn assigned_to_le_chunks(
1288        &self,
1289        layouter: &mut impl Layouter<F>,
1290        x: &AssignedNative<F>,
1291        nb_bits_per_chunk: usize,
1292        nb_chunks: Option<usize>,
1293    ) -> Result<Vec<AssignedNative<F>>, Error> {
1294        self.native_gadget
1295            .assigned_to_le_chunks(layouter, x, nb_bits_per_chunk, nb_chunks)
1296    }
1297
1298    fn sgn0(
1299        &self,
1300        layouter: &mut impl Layouter<F>,
1301        x: &AssignedNative<F>,
1302    ) -> Result<AssignedBit<F>, Error> {
1303        self.native_gadget.sgn0(layouter, x)
1304    }
1305}
1306
1307impl ArithInstructions<F, AssignedNative<F>> for ZkStdLib {
1308    fn linear_combination(
1309        &self,
1310        layouter: &mut impl Layouter<F>,
1311        terms: &[(F, AssignedNative<F>)],
1312        constant: F,
1313    ) -> Result<AssignedNative<F>, Error> {
1314        self.native_gadget.linear_combination(layouter, terms, constant)
1315    }
1316
1317    fn mul(
1318        &self,
1319        layouter: &mut impl Layouter<F>,
1320        x: &AssignedNative<F>,
1321        y: &AssignedNative<F>,
1322        multiplying_constant: Option<F>,
1323    ) -> Result<AssignedNative<F>, Error> {
1324        self.native_gadget.mul(layouter, x, y, multiplying_constant)
1325    }
1326
1327    fn div(
1328        &self,
1329        layouter: &mut impl Layouter<F>,
1330        x: &AssignedNative<F>,
1331        y: &AssignedNative<F>,
1332    ) -> Result<AssignedNative<F>, Error> {
1333        self.native_gadget.div(layouter, x, y)
1334    }
1335
1336    fn inv(
1337        &self,
1338        layouter: &mut impl Layouter<F>,
1339        x: &AssignedNative<F>,
1340    ) -> Result<AssignedNative<F>, Error> {
1341        self.native_gadget.inv(layouter, x)
1342    }
1343
1344    fn inv0(
1345        &self,
1346        layouter: &mut impl Layouter<F>,
1347        x: &AssignedNative<F>,
1348    ) -> Result<AssignedNative<F>, Error> {
1349        self.native_gadget.inv0(layouter, x)
1350    }
1351}
1352
1353impl ZeroInstructions<F, AssignedNative<F>> for ZkStdLib {}
1354
1355impl<Assigned> ControlFlowInstructions<F, Assigned> for ZkStdLib
1356where
1357    Assigned: InnerValue,
1358    NG: ControlFlowInstructions<F, Assigned>,
1359{
1360    fn select(
1361        &self,
1362        layouter: &mut impl Layouter<F>,
1363        cond: &AssignedBit<F>,
1364        x: &Assigned,
1365        y: &Assigned,
1366    ) -> Result<Assigned, Error> {
1367        self.native_gadget.select(layouter, cond, x, y)
1368    }
1369
1370    fn cond_swap(
1371        &self,
1372        layouter: &mut impl Layouter<F>,
1373        cond: &AssignedBit<F>,
1374        x: &Assigned,
1375        y: &Assigned,
1376    ) -> Result<(Assigned, Assigned), Error> {
1377        self.native_gadget.cond_swap(layouter, cond, x, y)
1378    }
1379}
1380
1381impl FieldInstructions<F, AssignedNative<F>> for ZkStdLib {
1382    fn order(&self) -> BigUint {
1383        self.native_gadget.order()
1384    }
1385}
1386
1387impl RangeCheckInstructions<F, AssignedNative<F>> for ZkStdLib {
1388    fn assign_lower_than_fixed(
1389        &self,
1390        layouter: &mut impl Layouter<F>,
1391        value: Value<F>,
1392        bound: &BigUint,
1393    ) -> Result<AssignedNative<F>, Error> {
1394        self.native_gadget.assign_lower_than_fixed(layouter, value, bound)
1395    }
1396
1397    fn assert_lower_than_fixed(
1398        &self,
1399        layouter: &mut impl Layouter<F>,
1400        x: &AssignedNative<F>,
1401        bound: &BigUint,
1402    ) -> Result<(), Error> {
1403        self.native_gadget.assert_lower_than_fixed(layouter, x, bound)
1404    }
1405}
1406
1407impl DivisionInstructions<F, AssignedNative<F>> for ZkStdLib {}
1408
1409impl BinaryInstructions<F> for ZkStdLib {
1410    fn and(
1411        &self,
1412        layouter: &mut impl Layouter<F>,
1413        bits: &[AssignedBit<F>],
1414    ) -> Result<AssignedBit<F>, Error> {
1415        self.native_gadget.and(layouter, bits)
1416    }
1417
1418    fn or(
1419        &self,
1420        layouter: &mut impl Layouter<F>,
1421        bits: &[AssignedBit<F>],
1422    ) -> Result<AssignedBit<F>, Error> {
1423        self.native_gadget.or(layouter, bits)
1424    }
1425
1426    fn xor(
1427        &self,
1428        layouter: &mut impl Layouter<F>,
1429        bits: &[AssignedBit<F>],
1430    ) -> Result<AssignedBit<F>, Error> {
1431        self.native_gadget.xor(layouter, bits)
1432    }
1433
1434    fn not(
1435        &self,
1436        layouter: &mut impl Layouter<F>,
1437        bit: &AssignedBit<F>,
1438    ) -> Result<AssignedBit<F>, Error> {
1439        self.native_gadget.not(layouter, bit)
1440    }
1441}
1442
1443impl BitwiseInstructions<F, AssignedNative<F>> for ZkStdLib {}
1444
1445impl<const M: usize, const A: usize, T> VectorInstructions<F, T, M, A> for ZkStdLib
1446where
1447    T: Vectorizable,
1448    T::Element: Copy,
1449    NG: AssignmentInstructions<F, T> + ControlFlowInstructions<F, T>,
1450{
1451    fn trim_beginning(
1452        &self,
1453        layouter: &mut impl Layouter<F>,
1454        input: &AssignedVector<F, T, M, A>,
1455        n_elems: usize,
1456    ) -> Result<AssignedVector<F, T, M, A>, Error> {
1457        self.vector_gadget.trim_beginning(layouter, input, n_elems)
1458    }
1459
1460    fn padding_flag(
1461        &self,
1462        layouter: &mut impl Layouter<F>,
1463        input: &AssignedVector<F, T, M, A>,
1464    ) -> Result<(Box<[AssignedBit<F>; M]>, VectorBounds<F>), Error> {
1465        self.vector_gadget.padding_flag(layouter, input)
1466    }
1467
1468    fn get_limits(
1469        &self,
1470        layouter: &mut impl Layouter<F>,
1471        input: &AssignedVector<F, T, M, A>,
1472    ) -> Result<VectorBounds<F>, Error> {
1473        self.vector_gadget.get_limits(layouter, input)
1474    }
1475
1476    fn resize<const L: usize>(
1477        &self,
1478        layouter: &mut impl Layouter<F>,
1479        input: AssignedVector<F, T, M, A>,
1480    ) -> Result<AssignedVector<F, T, L, A>, Error> {
1481        self.vector_gadget.resize(layouter, input)
1482    }
1483
1484    fn assign_with_filler(
1485        &self,
1486        layouter: &mut impl Layouter<F>,
1487        value: Value<Vec<<T>::Element>>,
1488        filler: Option<<T>::Element>,
1489    ) -> Result<AssignedVector<F, T, M, A>, Error> {
1490        self.vector_gadget.assign_with_filler(layouter, value, filler)
1491    }
1492}
1493
1494/// Circuit structure which is used to create any circuit that can be compiled
1495/// into keys using the ZK standard library.
1496#[derive(Clone, Debug)]
1497pub struct MidnightCircuit<'a, R: Relation> {
1498    relation: &'a R,
1499    k: u32,
1500    instance: Value<R::Instance>,
1501    witness: Value<R::Witness>,
1502    nb_public_inputs: Rc<RefCell<Option<usize>>>,
1503    circuit_error: Rc<RefCell<Option<R::Error>>>,
1504}
1505
1506impl<'a, R: Relation> MidnightCircuit<'a, R> {
1507    /// A MidnightCircuit with unknown instance-witness for the given relation.
1508    /// `k` is the log2 of the circuit size (i.e. the circuit has `2^k` rows).
1509    /// If `k` is `None`, the optimal value is computed automatically.
1510    pub fn from_relation(relation: &'a R, k: Option<u32>) -> Self {
1511        MidnightCircuit::new(relation, Value::unknown(), Value::unknown(), k)
1512    }
1513
1514    /// Creates a new MidnightCircuit for the given relation.
1515    /// `k` is the log2 of the circuit size (i.e. the circuit has `2^k` rows).
1516    /// If `k` is `None`, the optimal value is computed automatically.
1517    pub fn new(
1518        relation: &'a R,
1519        instance: Value<R::Instance>,
1520        witness: Value<R::Witness>,
1521        k: Option<u32>,
1522    ) -> Self {
1523        let k = k.unwrap_or_else(|| optimal_k(relation));
1524        MidnightCircuit {
1525            relation,
1526            k,
1527            instance,
1528            witness,
1529            nb_public_inputs: Rc::new(RefCell::new(None)),
1530            circuit_error: Rc::new(RefCell::new(None)),
1531        }
1532    }
1533
1534    /// Returns the log2 of the circuit size.
1535    pub fn k(&self) -> u32 {
1536        self.k
1537    }
1538
1539    /// Takes the circuit error stashed during synthesis, if any.
1540    pub fn take_error(&self) -> Option<R::Error> {
1541        self.circuit_error.take()
1542    }
1543}
1544
1545/// A verifier key of a Midnight circuit.
1546#[derive(Clone, Debug)]
1547pub struct MidnightVK {
1548    architecture: ZkStdLibArch,
1549    k: u8,
1550    nb_public_inputs: usize,
1551    vk: VerifyingKey<midnight_curves::Fq, KZGCommitmentScheme<midnight_curves::Bls12>>,
1552}
1553
1554impl MidnightVK {
1555    /// Writes a verifying key to a buffer.
1556    ///
1557    /// Depending on the `format`:
1558    /// - `Processed`: Takes less space, but more time to read.
1559    /// - `RawBytes`: Takes more space, but faster to read.
1560    ///
1561    /// Using `RawBytesUnchecked` will have the same effect as `RawBytes`,
1562    /// but it is not recommended.
1563    pub fn write<W: io::Write>(&self, writer: &mut W, format: SerdeFormat) -> io::Result<()> {
1564        self.architecture.write(writer)?;
1565
1566        writer.write_all(&[self.k])?;
1567
1568        writer.write_all(&(self.nb_public_inputs as u32).to_le_bytes())?;
1569
1570        self.vk.write(writer, format)
1571    }
1572
1573    /// Reads a verification key from a buffer.
1574    ///
1575    /// The `format` must match the one that was used when writing the key.
1576    /// If the key was written with `RawBytes`, it can be read with `RawBytes`
1577    /// or `RawBytesUnchecked` (which is faster).
1578    ///
1579    /// # WARNING
1580    /// Use `RawBytesUnchecked` only if you trust the party who wrote the key.
1581    pub fn read<R: io::Read>(reader: &mut R, format: SerdeFormat) -> io::Result<Self> {
1582        let architecture = ZkStdLibArch::read(reader)?;
1583
1584        let mut byte = [0u8; 1];
1585        reader.read_exact(&mut byte)?;
1586        let k = byte[0];
1587
1588        let mut bytes = [0u8; 4];
1589        reader.read_exact(&mut bytes)?;
1590        let nb_public_inputs = u32::from_le_bytes(bytes) as usize;
1591
1592        let mut cs = ConstraintSystem::default();
1593        let _config = ZkStdLib::configure(&mut cs, (architecture, k - 1));
1594
1595        let vk = VerifyingKey::read_from_cs::<R>(reader, format, cs)?;
1596
1597        Ok(MidnightVK {
1598            architecture,
1599            k,
1600            nb_public_inputs,
1601            vk,
1602        })
1603    }
1604
1605    /// The size of the domain associated to this verifying key.
1606    pub fn k(&self) -> u8 {
1607        self.k
1608    }
1609
1610    /// The underlying midnight-proofs verifying key.
1611    pub fn vk(
1612        &self,
1613    ) -> &VerifyingKey<midnight_curves::Fq, KZGCommitmentScheme<midnight_curves::Bls12>> {
1614        &self.vk
1615    }
1616}
1617
1618/// A proving key of a Midnight circuit.
1619#[derive(Clone, Debug)]
1620pub struct MidnightPK<R: Relation> {
1621    k: u8,
1622    relation: R,
1623    pk: ProvingKey<midnight_curves::Fq, KZGCommitmentScheme<midnight_curves::Bls12>>,
1624}
1625
1626impl<Rel: Relation> MidnightPK<Rel> {
1627    /// Writes a proving key to a buffer.
1628    ///
1629    /// Depending on the `format`:
1630    /// - `Processed`: Takes less space, but more time to read.
1631    /// - `RawBytes`: Takes more space, but faster to read.
1632    ///
1633    /// Using `RawBytesUnchecked` will have the same effect as `RawBytes`,
1634    /// but it is not recommended.
1635    pub fn write<W: io::Write>(&self, writer: &mut W, format: SerdeFormat) -> io::Result<()> {
1636        writer.write_all(&[self.k])?;
1637
1638        Rel::write_relation(&self.relation, writer)?;
1639
1640        self.pk.write(writer, format)
1641    }
1642
1643    /// Reads a proving key from a buffer.
1644    ///
1645    /// The `format` must match the one that was used when writing the key.
1646    /// If the key was written with `RawBytes`, it can be read with `RawBytes`
1647    /// or `RawBytesUnchecked` (which is faster).
1648    ///
1649    /// # WARNING
1650    /// Use `RawBytesUnchecked` only if you trust the party who wrote the key.
1651    pub fn read<R: io::Read>(reader: &mut R, format: SerdeFormat) -> io::Result<Self> {
1652        let mut byte = [0u8; 1];
1653
1654        reader.read_exact(&mut byte)?;
1655        let k = byte[0];
1656
1657        let relation = Rel::read_relation(reader)?;
1658
1659        let pk = ProvingKey::read::<R, MidnightCircuit<Rel>>(
1660            reader,
1661            format,
1662            MidnightCircuit::new(
1663                &relation,
1664                Value::unknown(),
1665                Value::unknown(),
1666                Some(k as u32),
1667            )
1668            .params(),
1669        )?;
1670
1671        Ok(MidnightPK { k, relation, pk })
1672    }
1673
1674    /// The size of the domain associated to this proving key.
1675    pub fn k(&self) -> u8 {
1676        self.k
1677    }
1678
1679    /// The underlying midnight-proofs proving key.
1680    pub fn pk(
1681        &self,
1682    ) -> &ProvingKey<midnight_curves::Fq, KZGCommitmentScheme<midnight_curves::Bls12>> {
1683        &self.pk
1684    }
1685}
1686
1687/// Helper trait, used to abstract the circuit developer from Halo2's
1688/// boilerplate.
1689///
1690/// `Relation` has a default implementation for loading only the tables
1691/// needed for the requested chips. The developer needs to implement the
1692/// function [Relation::circuit], which essentially contains the
1693/// statement of the proof we are creating.
1694///
1695/// # Important note
1696///
1697/// The API provided here guarantees that the number of public inputs
1698/// used during verification matches the number of public inputs (as raw
1699/// scalars) declared in [Relation::circuit] through the
1700/// [PublicInputInstructions] interface. Proof verification will fail if
1701/// this requirement is not met.
1702///
1703/// # Example
1704///
1705/// ```
1706/// # use midnight_circuits::{
1707/// #     instructions::{AssignmentInstructions, PublicInputInstructions},
1708/// #     types::{AssignedByte, Instantiable},
1709/// # };
1710/// # use midnight_zk_stdlib::{utils::plonk_api::filecoin_srs, Relation, ZkStdLib, ZkStdLibArch};
1711/// # use midnight_proofs::{
1712/// #     circuit::{Layouter, Value},
1713/// #     plonk::Error,
1714/// # };
1715/// # use rand::{rngs::OsRng, Rng, SeedableRng};
1716/// # use rand_chacha::ChaCha8Rng;
1717/// # use sha2::Digest;
1718/// #
1719/// type F = midnight_curves::Fq;
1720///
1721/// #[derive(Clone, Default)]
1722/// struct ShaPreImageCircuit;
1723///
1724/// impl Relation for ShaPreImageCircuit {
1725///     // When defining a circuit, one must clearly state the instance and the witness
1726///     // of the underlying NP-relation.
1727///     type Instance = [u8; 32];
1728///     type Witness = [u8; 24]; // 192 = 24 * 8
1729///     type Error = Error;
1730///
1731///     // We must specify how the instance is converted into raw field elements to
1732///     // be process by the prover/verifier. The order here must be consistent with
1733///     // the order in which public inputs are constrained/assigned in [circuit].
1734///     fn format_instance(instance: &Self::Instance) -> Result<Vec<F>, Error> {
1735///         Ok(instance.iter().flat_map(AssignedByte::<F>::as_public_input).collect())
1736///     }
1737///
1738///     // Define the logic of the NP-relation being proved.
1739///     fn circuit(
1740///         &self,
1741///         std_lib: &ZkStdLib,
1742///         layouter: &mut impl Layouter<F>,
1743///         _instance: Value<Self::Instance>,
1744///         witness: Value<Self::Witness>,
1745///     ) -> Result<(), Error> {
1746///         let assigned_input = std_lib.assign_many(layouter, &witness.transpose_array())?;
1747///         let output = std_lib.sha2_256(layouter, &assigned_input)?;
1748///         output.iter().try_for_each(|b| std_lib.constrain_as_public_input(layouter, b))
1749///     }
1750///
1751///     fn used_chips(&self) -> ZkStdLibArch {
1752///         ZkStdLibArch {
1753///             sha2_256: true,
1754///             ..ZkStdLibArch::default()
1755///         }
1756///     }
1757///
1758///     fn write_relation<W: std::io::Write>(&self, _writer: &mut W) -> std::io::Result<()> {
1759///         Ok(())
1760///     }
1761///
1762///     fn read_relation<R: std::io::Read>(_reader: &mut R) -> std::io::Result<Self> {
1763///         Ok(ShaPreImageCircuit)
1764///     }
1765/// }
1766///
1767/// const K: u32 = 13;
1768/// let mut srs = filecoin_srs(K);
1769///
1770/// let relation = ShaPreImageCircuit;
1771///
1772/// let vk = midnight_zk_stdlib::setup_vk(&srs, &relation);
1773/// let pk = midnight_zk_stdlib::setup_pk(&relation, &vk);
1774///
1775/// let mut rng = ChaCha8Rng::from_entropy();
1776/// let witness: [u8; 24] = core::array::from_fn(|_| rng.gen());
1777/// let instance = sha2::Sha256::digest(witness).into();
1778///
1779/// let proof = midnight_zk_stdlib::prove::<ShaPreImageCircuit, blake2b_simd::State>(
1780///     &srs, &pk, &relation, &instance, witness, OsRng,
1781/// )
1782/// .expect("Proof generation should not fail");
1783///
1784/// assert!(
1785///     midnight_zk_stdlib::verify::<ShaPreImageCircuit, blake2b_simd::State>(
1786///         &srs.verifier_params(),
1787///         &vk,
1788///         &instance,
1789///         None,
1790///         &proof
1791///     )
1792///     .is_ok()
1793/// )
1794/// ```
1795pub trait Relation: Clone {
1796    /// The instance of the NP-relation described by this circuit.
1797    type Instance: Clone;
1798
1799    /// The witness of the NP-relation described by this circuit.
1800    type Witness: Clone;
1801
1802    /// The error type returned by [Self::circuit] and [Self::format_instance].
1803    type Error: From<plonk::Error> + Debug;
1804
1805    /// Produces a vector of field elements in PLONK format representing the
1806    /// given [Self::Instance].
1807    fn format_instance(instance: &Self::Instance) -> Result<Vec<F>, Self::Error>;
1808
1809    /// Produces a vector of field elements in PLONK format representing the
1810    /// data inside the committed instance.
1811    fn format_committed_instances(_witness: &Self::Witness) -> Vec<F> {
1812        vec![]
1813    }
1814
1815    /// Defines the circuit's logic.
1816    fn circuit(
1817        &self,
1818        std_lib: &ZkStdLib,
1819        layouter: &mut impl Layouter<F>,
1820        instance: Value<Self::Instance>,
1821        witness: Value<Self::Witness>,
1822    ) -> Result<(), Self::Error>;
1823
1824    /// Specifies what chips are enabled in the standard library. A chip needs
1825    /// to be enabled if it is used in [Self::circuit], but it can also be
1826    /// enabled even if it is not used (possibly to share the same architecture
1827    /// with other circuits).
1828    ///
1829    /// The blanket implementation enables none of them.
1830    fn used_chips(&self) -> ZkStdLibArch {
1831        ZkStdLibArch::default()
1832    }
1833
1834    /// Writes a relation to a buffer.
1835    fn write_relation<W: io::Write>(&self, writer: &mut W) -> io::Result<()>;
1836
1837    /// Reads a relation from a buffer.
1838    fn read_relation<R: io::Read>(reader: &mut R) -> io::Result<Self>;
1839}
1840
1841impl<R: Relation> Circuit<F> for MidnightCircuit<'_, R> {
1842    type Config = ZkStdLibConfig;
1843
1844    // FIXME: this could be parametrised by MidnightCircuit.
1845    type FloorPlanner = SimpleFloorPlanner;
1846
1847    type Params = (ZkStdLibArch, u8);
1848
1849    fn without_witnesses(&self) -> Self {
1850        unreachable!()
1851    }
1852
1853    fn params(&self) -> Self::Params {
1854        (self.relation.used_chips(), (self.k - 1) as u8)
1855    }
1856
1857    fn configure_with_params(
1858        meta: &mut ConstraintSystem<F>,
1859        params: (ZkStdLibArch, u8),
1860    ) -> Self::Config {
1861        ZkStdLib::configure(meta, params)
1862    }
1863
1864    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
1865        ZkStdLib::configure(meta, (ZkStdLibArch::default(), 8))
1866    }
1867
1868    fn synthesize(
1869        &self,
1870        config: Self::Config,
1871        mut layouter: impl Layouter<F>,
1872    ) -> Result<(), Error> {
1873        let max_bit_len = (self.k - 1) as usize;
1874        let zk_std_lib = ZkStdLib::new(&config, max_bit_len);
1875
1876        self.relation
1877            .circuit(
1878                &zk_std_lib,
1879                &mut layouter.namespace(|| "Running logic circuit"),
1880                self.instance.clone(),
1881                self.witness.clone(),
1882            )
1883            .map_err(|e| {
1884                let error_msg = format!("Relation::circuit error: {e:?}");
1885                *self.circuit_error.borrow_mut() = Some(e);
1886                Error::Synthesis(error_msg)
1887            })?;
1888
1889        // After the circuit function has been called, we can update the expected
1890        // number of raw public inputs in [Self] (via a RefCell). This number will
1891        // be stored in the MidnightVK so that we can make sure it matches the number of
1892        // public inputs provided during verification.
1893        *self.nb_public_inputs.borrow_mut() =
1894            Some(zk_std_lib.native_gadget.native_chip.nb_public_inputs());
1895
1896        // We load the tables at the end, once we have figured out what chips/gadgets
1897        // were actually used.
1898        zk_std_lib.core_decomposition_chip.load(&mut layouter)?;
1899
1900        if let Some(sha256_chip) = zk_std_lib.sha2_256_chip {
1901            if *zk_std_lib.used_sha2_256.borrow() {
1902                sha256_chip.load(&mut layouter)?;
1903            }
1904        }
1905
1906        if let Some(sha512_chip) = zk_std_lib.sha2_512_chip {
1907            if *zk_std_lib.used_sha2_512.borrow() {
1908                sha512_chip.load(&mut layouter)?;
1909            }
1910        }
1911
1912        if let Some(b64_chip) = zk_std_lib.base64_chip {
1913            if *zk_std_lib.used_base64.borrow() {
1914                b64_chip.load(&mut layouter)?;
1915            }
1916        }
1917
1918        if let Some(scanner_chip) = zk_std_lib.scanner_chip {
1919            if *zk_std_lib.used_scanner.borrow() {
1920                scanner_chip.load(&mut layouter)?;
1921            }
1922        }
1923
1924        if let Some(keccak_sha3_chip) = zk_std_lib.keccak_sha3_chip {
1925            if *zk_std_lib.used_keccak_or_sha3.borrow() {
1926                keccak_sha3_chip.load(&mut layouter)?;
1927            }
1928        }
1929
1930        if let Some(blake2b_chip) = zk_std_lib.blake2b_chip {
1931            if *zk_std_lib.used_blake2b.borrow() {
1932                blake2b_chip.load(&mut layouter)?;
1933            }
1934        }
1935
1936        Ok(())
1937    }
1938}
1939
1940/// Generates a verifying key for a `MidnightCircuit<R>` circuit.
1941///
1942/// The log2 of the circuit size (`k`) is derived from the SRS parameters.
1943/// For optimal performance, downsize the SRS to the circuit's optimal `k`
1944/// beforehand (see [optimal_k]). Otherwise, the circuit will use the full
1945/// size of the SRS, which may be unnecessarily large.
1946pub fn setup_vk<R: Relation>(
1947    params: &ParamsKZG<midnight_curves::Bls12>,
1948    relation: &R,
1949) -> MidnightVK {
1950    let k = params.max_k();
1951    let circuit = MidnightCircuit::from_relation(relation, Some(k));
1952    let vk = keygen_vk_with_k(params, &circuit, k).expect("keygen_vk should not fail");
1953
1954    // During the call to [setup_vk] the circuit RefCell on public inputs has been
1955    // mutated with the correct value. The following [unwrap] is safe here.
1956    let nb_public_inputs = circuit.nb_public_inputs.clone().borrow().unwrap();
1957
1958    MidnightVK {
1959        architecture: relation.used_chips(),
1960        k: circuit.k as u8,
1961        nb_public_inputs,
1962        vk,
1963    }
1964}
1965
1966/// Generates a proving key for a `MidnightCircuit<R>` circuit.
1967pub fn setup_pk<R: Relation>(relation: &R, vk: &MidnightVK) -> MidnightPK<R> {
1968    let circuit = MidnightCircuit::new(
1969        relation,
1970        Value::unknown(),
1971        Value::unknown(),
1972        Some(vk.k() as u32),
1973    );
1974    let pk = BlstPLONK::<MidnightCircuit<R>>::setup_pk(&circuit, &vk.vk);
1975    MidnightPK {
1976        k: vk.k(),
1977        relation: relation.clone(),
1978        pk,
1979    }
1980}
1981
1982/// Produces a proof of relation `R` for the given instance (using the given
1983/// proving key and witness).
1984pub fn prove<R: Relation, H: TranscriptHash>(
1985    params: &ParamsKZG<midnight_curves::Bls12>,
1986    pk: &MidnightPK<R>,
1987    relation: &R,
1988    instance: &R::Instance,
1989    witness: R::Witness,
1990    rng: impl RngCore + CryptoRng,
1991) -> Result<Vec<u8>, R::Error>
1992where
1993    G1Projective: Hashable<H>,
1994    F: Hashable<H> + Sampleable<H>,
1995{
1996    let pi = R::format_instance(instance)?;
1997    let com_inst = R::format_committed_instances(&witness);
1998    let circuit = MidnightCircuit::new(
1999        relation,
2000        Value::known(instance.clone()),
2001        Value::known(witness),
2002        Some(pk.k as u32),
2003    );
2004    BlstPLONK::<MidnightCircuit<R>>::prove::<H>(
2005        params,
2006        &pk.pk,
2007        &circuit,
2008        1,
2009        &[com_inst.as_slice(), &pi],
2010        rng,
2011    )
2012    .map_err(|e| circuit.take_error().unwrap_or_else(|| e.into()))
2013}
2014
2015/// Verifies the given proof of relation `R` with respect to the given instance.
2016/// Returns `Ok(())` if the proof is valid.
2017pub fn verify<R: Relation, H: TranscriptHash>(
2018    params_verifier: &ParamsVerifierKZG<midnight_curves::Bls12>,
2019    vk: &MidnightVK,
2020    instance: &R::Instance,
2021    committed_instance: Option<G1Affine>,
2022    proof: &[u8],
2023) -> Result<(), R::Error>
2024where
2025    G1Projective: Hashable<H>,
2026    F: Hashable<H> + Sampleable<H>,
2027{
2028    let pi = R::format_instance(instance)?;
2029    let committed_pi = committed_instance.unwrap_or(G1Affine::identity());
2030    if pi.len() != vk.nb_public_inputs {
2031        return Err(Error::InvalidInstances.into());
2032    }
2033    Ok(BlstPLONK::<MidnightCircuit<R>>::verify::<H>(
2034        params_verifier,
2035        &vk.vk,
2036        &[committed_pi],
2037        &[&pi],
2038        proof,
2039    )?)
2040}
2041
2042/// Verifies a batch of proofs with respect to their corresponding vk.
2043/// This method does not need to know the `Relation` the proofs are associated
2044/// to and, indeed, it can verify proofs from different `Relation`s.
2045/// For that, this function does not take `instance`s, but public inputs
2046/// in raw format (`Vec<F>`).
2047///
2048/// Returns `Ok(())` if all proofs are valid.
2049pub fn batch_verify<H: TranscriptHash + Send + Sync>(
2050    params_verifier: &ParamsVerifierKZG<midnight_curves::Bls12>,
2051    vks: &[MidnightVK],
2052    pis: &[Vec<F>],
2053    proofs: &[Vec<u8>],
2054) -> Result<(), Error>
2055where
2056    G1Projective: Hashable<H>,
2057    F: Hashable<H> + Sampleable<H>,
2058{
2059    use rayon::prelude::*;
2060
2061    // TODO: For the moment, committed instances are not supported.
2062    let n = vks.len();
2063    if pis.len() != n || proofs.len() != n {
2064        // TODO: have richer types in halo2
2065        return Err(Error::InvalidInstances);
2066    }
2067
2068    let prepared: Vec<(_, F)> = vks
2069        .par_iter()
2070        .zip(pis.par_iter())
2071        .zip(proofs.par_iter())
2072        .map(|((vk, pi), proof)| {
2073            if pi.len() != vk.nb_public_inputs {
2074                return Err(Error::InvalidInstances);
2075            }
2076
2077            let mut transcript = CircuitTranscript::init_from_bytes(proof);
2078            let dual_msm = prepare::<
2079                midnight_curves::Fq,
2080                KZGCommitmentScheme<midnight_curves::Bls12>,
2081                CircuitTranscript<H>,
2082            >(
2083                &vk.vk,
2084                &[&[midnight_curves::G1Projective::identity()]],
2085                // TODO: We could batch here proofs with the same vk.
2086                &[&[pi]],
2087                &mut transcript,
2088            )?;
2089            let summary: F = transcript.squeeze_challenge();
2090            transcript.assert_empty().map_err(|_| Error::Opening)?;
2091            Ok((dual_msm, summary))
2092        })
2093        .collect::<Result<Vec<_>, Error>>()?;
2094
2095    let mut r_transcript = CircuitTranscript::init();
2096    let mut guards = Vec::with_capacity(n);
2097    for (guard, summary) in prepared {
2098        r_transcript.common(&summary)?;
2099        guards.push(guard);
2100    }
2101    let r: F = r_transcript.squeeze_challenge();
2102
2103    let n_guards = guards.len();
2104    let powers: Vec<F> =
2105        std::iter::successors(Some(F::ONE), |p| Some(*p * r)).take(n_guards).collect();
2106    guards.par_iter_mut().enumerate().for_each(|(i, guard)| guard.scale(powers[i]));
2107
2108    // Phase 4: add scaled guards sequentially.
2109    let Some(mut acc_guard) = guards.pop() else {
2110        return Ok(());
2111    };
2112    for guard in guards {
2113        acc_guard.add_msm(guard);
2114    }
2115    // TODO: Have richer error types
2116    acc_guard.verify(params_verifier).map_err(|_| Error::Opening)
2117}
2118
2119/// Cost model of the given relation for the given `k`.
2120/// `k` is the log2 of the circuit size. If `None`, the optimal value is
2121/// computed automatically.
2122pub fn cost_model<R: Relation>(relation: &R, k: Option<u32>) -> CircuitModel {
2123    let circuit = MidnightCircuit::from_relation(relation, k);
2124    circuit_model::<_, COMMITMENT_BYTE_SIZE, SCALAR_BYTE_SIZE>(&circuit)
2125}
2126
2127/// Finds the optimal `k` (log2 of the circuit size) for the given relation.
2128/// Tries different values of `k` (9..=25) and picks the smallest one where
2129/// the circuit fits. The pow2range table uses `max_bit_len = k - 1`.
2130pub fn optimal_k<R: Relation>(relation: &R) -> u32 {
2131    let mut best_k = u32::MAX;
2132
2133    for k in 9..=25 {
2134        let model = cost_model(relation, Some(k));
2135
2136        if model.k < best_k {
2137            best_k = model.k;
2138        }
2139
2140        // Stop when the pow2range table (2^k rows) becomes the bottleneck.
2141        if model.rows < (1 << k) {
2142            break;
2143        }
2144    }
2145
2146    best_k
2147}
```