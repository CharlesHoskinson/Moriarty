# ENC-1 to ENC-5 and FP-3 facts 1 to 3: results

Loop items ENC and FP3 (`research/sherlock-e-path-20261112/LOOP.md`). Plan: `PLAN.md`. Abbreviations: `FD/` is `research/sherlock-final-design-20261111/`, `PR` is `research/sherlock-processor-final-20261111/PROPOSAL.md`, `SBW` is the stark-backend worktree `~/.cache/m7-wrap/src/stark-backend-enc-fp3` (branch `enc-fp3` of the clone `~/.cache/m7-wrap/src/openvm-org_stark-backend`, from `e2-lfr` 5f698b5). Labels: MEASURED (computed by the code named), DERIVED (by a stated rule), LEAD (a pick of this item, binding only after the owner confirms it).

## 0. Summary

| Gate | Status after this item | Evidence |
|---|---|---|
| ENC-1 Fr tag values | written by a rule (LEAD pick P-1, P-2); Rust reference vectors MEASURED; AIR and wrap vectors not run | sections 1, 6 |
| ENC-2 `enc(e)` | written word by word; the `DIGEST` conflict reconciled to OD-17 (2 values, `param` = tag index); Lean amendment listed; Rust vectors MEASURED | sections 2, 6 |
| ENC-3 V+ parameters, `layout_d` | written (LEAD pick P-4: one byte per word); Rust vector MEASURED | sections 3, 6 |
| ENC-4 `StackStep` and stack cells | step and cell framing written; the record lane types wait on the CORE state schema (not frozen) | section 4 |
| ENC-5 `plan_d` word lists, `DOM_PLAN` | code 41 or 33, a pick (LEAD P-5, 41 recommended); the five word lists need `sherlock plan` data structures that do not exist | section 4 |
| `P2` absorption mode | not fixed by any text; LEAD pick P-3 (overwrite, as the stock BabyBear leaf hash) | section 5 |
| G3 transcript vectors `L` (u = 3) and `L_Fr` (u = 7) | MEASURED against a hand model of each duplex | section 6 |
| FP-3 fact 1 | STOPPED: needs a design choice; options and a recommendation | section 7.1 |
| FP-3 fact 2 | evidence only (the `acceptRun` lanes of the kit sketch); not closed (N-11 and fact 3) | section 7.2 |
| FP-3 fact 3 | STOPPED: needs a design choice; options and a recommendation | section 7.3 |

No 1.12 row reaches FROZEN: section 1.12 closes ENC-1 to ENC-3 only with vectors on the Rust reference, the AIR and the wrap, and this item gives the Rust reference only; every encoding here also carries a LEAD pick. So `FD/ARCHITECTURE.md` 1.12 is not edited: no gate closes. The rows I-19, I-22, I-29, I-31, I-33, I-34 and I-36 (ENC) and I-21, I-23, I-24, I-25 (FP-3) stay FREEZE-PENDING; this file is their evidence for the next step.

## 1. ENC-1: the Fr values of the public digest tags

**Rule (LEAD pick P-1).** The Fr value of a public digest tag is its ASCII name, at most 16 bytes, zero padded to 16 bytes, read as a little-endian `u128`: `tagFr(name) = sum_i name[i] 256^i`. This is the landed rule `dom()` of `crates/mori-recursion/src/parent.rs:115` (the halo2 prototype's `SHK_RUN`, `SHK_SD` and `SHK_PROG` domain words), so no new rule enters the code base. Each value is below `2^128 < r`, the four names are distinct, so the four values are distinct; the use code `u = 4` in the capacity `cap(4, 2) = 2 + 4 * 2^66` separates them from every other Fr hash of `PR` 3.6 and from the `L_Fr` codes 5 to 7. Values (DERIVED by the rule, printed by the test):

| Index (`DIGEST` `param`) | Tag | Fr value |
|---:|---|---|
| 0 | `SHK_PROGC` | `0x000000000000000000000000000000000000000000000043474f52505f4b4853` |
| 1 | `SHK_LOCK` | `0x0000000000000000000000000000000000000000000000004b434f4c5f4b4853` |
| 2 | `SHK_INPUT` | `0x00000000000000000000000000000000000000000000005455504e495f4b4853` |
| 3 | `SHK_OUTCOME` | `0x000000000000000000000000000000000000000000454d4f4354554f5f4b4853` |

**Tag index (LEAD pick P-2).** The `param` of a `DIGEST` entry is the index of its tag in the frozen list of `PR` 3.6, in the order written there: 0 `SHK_PROGC`, 1 `SHK_LOCK`, 2 `SHK_INPUT`, 3 `SHK_OUTCOME`. Bound `0 <= param <= 3`; a new public tag takes the next index. The wrap maps `param` to `tagFr` by a constant table of its key. The anchor rule of `PR` 7.3 reads the tag of an anchor entry from this `param` (0 for ORD and the end chunks, 1 for a lock).

**Forms** (FP-4, written out with the types of FP-1; all `P2` per section 5):

```text
C_P         = PoseidonFr_4(tagFr(SHK_PROGC),   pack(P2(6,  r_P[8], prog_d[8])))                        P2 preimage 17 words
A_j         = PoseidonFr_4(tagFr(SHK_LOCK),    pack(P2(7,  r_P[8], prog_d[8], run_tag[8], j: u32[2]))) P2 preimage 27 words
input_pub   = PoseidonFr_4(tagFr(SHK_INPUT),   pack(input_d))
outcome_pub = PoseidonFr_4(tagFr(SHK_OUTCOME), pack(P2(31, input_d[8], outcome_d[8])))                 P2 preimage 17 words
PoseidonFr_4(t, x): state (t, x, 2 + 4 * 2^66), one Midnight permutation, output r_0
pack(d) = sum_{i<8} d_i 2^(31 i), the stark-backend `pack_f_to_sf` (`FrLog.lean` `pack`)
```

`j: u32` is two 16-bit limbs (FP-1), not one word: `A_j` binds `j` in `1 <= j < k` with `k` unbounded by any key constant.

## 2. ENC-2: the word sequence `enc(e)` of a log entry

**Conflict.** `FrLog.lean:774-814` (at `bdb92973a`; `git diff bdb92973a 6661325d4 -- FrLog.lean` on `bridge-lean` changes 9 lines, none in `Op.code`, `Op.arity` or the `DIGEST` case) gives `DIGEST` the code `[8, 0]`, 9 values (the 8 BabyBear words of `d2` as raw words, then the output) and one tag `digestTag = 0x5348`. `ARCHITECTURE` 1.8 and `PR` 3.2 give `DIGEST` `param` = tag index, 2 values (`pack(d2)`, output), 1 permutation slot, and the owner confirmed that table with OD-17 (the reservation `R` of 4 entries, 8 values and 4 permutations holds only at 2 values per `DIGEST`). So ENC-2 follows OD-17, and the Lean changes.

**Encoding.** For an entry `e = (op, param, v_0 .. v_(k-1))` with `k = vals(op, param)` of the 1.8 table:

```text
enc(e) = [op, param] ++ limbs(v_0) ++ ... ++ limbs(v_(k-1))          2 + 9k BabyBear words
limbs(x) = [x_0 .. x_8], x = sum_i x_i 2^(30 i), 0 <= x_i < 2^30, x < r (Canon)
```

| `op` | `[op, param]` | values in order | words |
|---|---|---|---:|
| `NOP` | `[0, 0]` | none | 2 |
| `PERM` | `[1, 0]` | `a, b, c` (input state), `x, y, z` (output state) | 56 |
| `H2` | `[2, 0]` | `a, b, out` | 29 |
| `SPONGE(n)` | `[3, n]` | `in_1 .. in_n, out` | `11 + 9n` |
| `PATH(d)` | `[4, d]` | `leaf, idx, root` (siblings wrap-private) | 29 |
| `ECMUL` | `[5, 0]` | `k, px, py, ox, oy` | 47 |
| `ECMULG` | `[6, 0]` | `k, ox, oy` | 29 |
| `PUB(i)` | `[7, i]` | `v` | 11 |
| `DIGEST` | `[8, t]`, `t` the tag index of section 1 | `pack(d2)`, `out = PoseidonFr_4(tagFr(t), pack(d2))` | 20 |

The onion takes `enc(e)` as one `P2` preimage: `d_(i+1) = P2(tagStep, d_i, P2(tagEnt, enc(e_i)))`. `pv` splits the same words into its two tables: row `i` of the entry table is `[op, param]`, and the value table holds the `limbs` of the values in entry order from row `off_i`. So `pv` and the onion carry the same words, and the running sums `off_i` are not words. `H2`, `SPONGE(n)` and `PERM` are the native `u = 0` definitions; the test checks `H2` and `SPONGE(3)` against midnight-circuits `PoseidonChip::hash`. The leaf's log writer computes `pack(d2)` from the 8 `d2` words it holds and range-decomposes it into 9 limbs; `pack` is injective on 8 words below `2^31` (`pack_injective`, proved), so the logged value fixes `d2`.

**Lean amendment (listed, not made; `bridge-lean` owns `FrLog.lean`).** `Op.digest` takes the tag index (`digest (t : Nat)`), `Op.code (digest t) = [8, t]`, `Op.arity digest = 2`, and `ValidOn .digest [p, o] = (t <= 3 ∧ o = frSponge4 (tagFr t) p)` with `p = pack d2` for some `d2` of 8 words below `pBB` (the leaf obligation), replacing `digestTag = 0x5348` and the 9-value form. `Prims.frSponge` has no use code; the amendment adds the `u = 4` sponge as its own primitive, so `frSponge [tag, x]` (u = 0) and the public digest stay distinct.

## 3. ENC-3: the typed V+ parameters and `layout_d`

Tape words of `bind_label` (FP-7), with the FP-1 types:

| Parameter | Type | Words | Check |
|---|---|---:|---|
| `dom_len = |domain|` | `u16` (P-4) | 1 | RANGE 16; `dom_len <= M_B(e)` |
| `domain[0 .. dom_len)` | `tag(8)` each, one byte per word (LEAD pick P-4) | `dom_len` | RANGE 8 each; equal to `m[0 .. dom_len)` |
| `offset` | `u32` (P-4) | 2 | RANGE 16 per limb; `offset = dom_len + 32` |
| `len_m` | `u32` | 2 | `len_m <= M_B(e)`; selects the block `ceil((len_m + 9) / 64)` |

```text
layout_d = P2(DOM_LAYOUT = 11, dom_len: u16, domain bytes (dom_len words), offset: u32)      3 + dom_len words after the tag
```

At the measured ledger layout (`"midnight:binding-input[v1]"`, 26 bytes, offset 58, `PR` 7.2) the preimage is 30 words with the tag, 4 permutations. `len_m` is not in `layout_d`: it is a per-call length, while `layout_d` is a key constant. The variable length of `domain` is safe: the total length `1 + l` sits in the capacity word of `P2`, and `dom_len` is a word.

P-4 is three picks, the three things ENC-3 names: `dom_len: u16` (the domain is 26 bytes and `M_B` is at most 2,039, so `u16` holds every value), `offset: u32` (the same type as `len_m: u32` of FP-7; `u16` would also hold it), and the byte packing. Options for the byte packing: (a) one byte per word, `tag(8)`, recommended: the `sha2` chip and the tape array `m` are byte cells, so the window check is one equality per byte, with no packing gadget; cost 26 words, 4 permutations, once per call. (b) Two bytes per `u16` word: 13 words, 3 permutations, and a byte split at each window cell. (c) Three bytes per word (below `2^24 < p`): 9 words, 2 permutations, the same split. The saving of (b) or (c) is 1 or 2 permutations per call, so (a).

## 4. ENC-4 and ENC-5: written as far as the rulings fix them

**ENC-4 (`StackStep` and the stack cells).** The framing follows from kit STACKS and FP-1, with two picks (LEAD P-6): `count: u64` and the `op` codes 0 none, 1 push, 2 pop:

```text
StackStep(stack) = [rw: bit, rw_top: Rec, op: tag(2) (0 none, 1 push, 2 pop), new_top: Rec]       2 + 2 |Rec| words
StackCells(stack) = [count: u64, below: digest, top: Rec]                                          12 + |Rec| words
```

`count` is `u64` because CORE compares `frame_count <= stack_limit` and `stack_limit` is `u64` in `input_d` (FP-4). `Rec` is the frame record (13 lanes) or the task record (7 lanes). The type of each record lane is not fixed: in the reference the lanes are a tagged union per record kind (`encode_frame`, `crates/mori-core/src/sherlock/helper_machine/c6_phys.rs:670`; `encode_task_record`, `c6_phys_helper.rs:988`), so a lane holds an image index in one kind and a heap reference or an operator tag in another, and the CORE state schema that types them (`core/state_schema.rs`, `FD/ARCHITECTURE.md` kit CORE: "the lane count is not known until the schema is frozen") does not exist. Proposed rule, for the schema work: each lane takes the widest type of the fields that any kind maps to it, with a heap reference `addr` (5 words at TOP = 63, AD-1), an image index `u32`, an operator or kind `tag(w)` and a flag `bit`; lane 0 is the kind, `tag(4)` for frames (10 kinds) and `tag(2)` for tasks (3 kinds). ENC-4 stays open on the CORE state schema.

**ENC-5 (`plan_d`).** The code of `DOM_PLAN` is a pick (LEAD P-5). Section 1.6 says it "takes the next free BabyBear tag code", which today is 33. Read with the FP-4 rule "the memory, stack and ROM domains of the HASH registry (HSH-002) and every later Sherlock digest take the next free codes, in registry order": the HASH kit registry is `MemRec, MemPad, MemEmpty, MemNode, FrameChain, TaskChain, StateBody, RomLeaf, RomNode`; `StateBody` already has 30 (`DOM_STATE_BODY`), so the other eight take 33 to 40 and `DOM_PLAN` = 41. No text breaks the tie; 41 is recommended because it keeps the registry codes contiguous and in registry order. Either value holds only while the registry list stays as written (`chip/hash_domains.rs` is not built); a change of that list moves the code. The preimage framing is fixed by 1.6: `plan_d = P2(41, 1: u16, itab_d, menu_d, policy_d, layout_rule_d, place_d)`, 42 words with the tag. The word lists of the five parts are not written: `sherlock plan` and its data structures (instruction table rows, shape menu, cut rule, padding list, placement rule) do not exist, and each list needs choices (for example whether `itab_d` is the stark-backend preprocessed-trace commitment, which is a digest of the stock Merkle hash and not a `P2`). ENC-5 stays open on the `sherlock plan` build.

## 5. `P2`: the one choice the FP-4 text leaves open (LEAD pick P-3)

FP-4 fixes the tag word first, the total length `1 + l` in the first capacity word, zero padding of the last rate block and one `digest` output, but not how a block enters the state. No Rust `P2` exists: rg for `SHK_PROGC`, `SHK_LOCK`, `SHK_INPUT`, `DOM_LAYOUT`, `tagEnt`, `D_EMPTY`, `digestTag` over the Moriarty worktrees `final-design`, `e2-lfr`, `e5-wrap`, `m7-wrap`, `stark` and over `~/.cache/m7-wrap/src` (`openvm-org_openvm`, `openvm-e4`, `openvm-e5`, the stark-backend clone) finds no implementation (the only hits are prototype test names and `CONTROL_TAG_ENTER`), so no lane of E3 or E5 holds a conflicting tag value or `P2` yet. Pick: overwrite, as the stock BabyBear leaf hash (`PaddingFreeSponge`, `SB/crates/stark-sdk/src/config/baby_bear_poseidon2.rs:29`) and the stock BabyBear duplex:

```text
state = [0; 16]; state[8] = 1 + l
for each block of 8 words of [tag, x_1 .. x_l], the last padded with zeros:
    state[0..8] = block; state = Poseidon2_16(state)
P2 = state[0..8]
```

This equals a chain of the stock `poseidon2_compress_with_capacity(block, cap)` with `cap_0 = [1 + l, 0, ...]`, which the test uses as the second path. The alternative, additive absorption (Midnight's Fr sponge), differs from the first block on, so every `P2` vector of section 6 holds only under this pick. Both are injective under premise A5.

## 6. Test vectors (Rust reference, MEASURED)

Test `SBW/crates/stark-sdk/tests/enc_vectors.rs` (commits 829a966 and 92d77ef on branch `enc-fp3` of the clone). Run: `cargo test --release --locked --offline -p openvm-stark-sdk --features bls12381-fr-poseidon --test enc_vectors`, log `~/.cache/enc-fp3/vec9.log` (pass, 1 test, 55 pinned vectors; `cargo clippy -D warnings` on the test clean, same log; formatted with stable rustfmt, nightly fmt not run). A copy of the test source is `enc_vectors.rs.txt` here. Every digest is computed on two paths and the test asserts equality:

- `P2`: the overwrite sponge on `poseidon2_perm()` and the chain of `poseidon2_compress_with_capacity`.
- `pack`: `pack_words` and a `BigUint` sum.
- `PoseidonFr_4`: `fixed_len_hash(perm, 4, ..)` and one hand permutation of `(t, x, 2 + 4 * 2^66)`; `u = 0` (`H2`, `SPONGE(3)`) against midnight-circuits 7.3.0 `PoseidonChip::hash`.
- G3 `L` (u = 3): the stark-sdk `baby_bear_bls12381_poseidon::Transcript` (stock `MultiFieldTranscript`) against a hand model of `PR` 3.6 (overwrite duplex, 8-word observe buffer packed base `2^31`, flush before squeeze, commitment observe, 5 base-`p` digits least significant first, digits dropped by an observe). Cases: 6 samples from the fresh state; a partial buffer of 1 to 8 words then a sample; 16 words (block-completing absorb) then 2 samples; 3 words, a commitment, 2 samples; sample, observe, sample.
- G3 `L_Fr` (u = 7): the stark-sdk `bls12381_fr_poseidon::Transcript` at `U_LFR_TRANSCRIPT` against a hand model of the Fr overwrite duplex from `(0, 0, 2^64 + 7 * 2^66)`; cases: 3 fresh samples, 1 observe then a sample, a full block then 3 samples. The first squeezes of u = 3 and u = 7 differ.
- Inputs: `r_P = [101 .. 108]`, `prog_d = [201 .. 208]`, `run_tag = [301 .. 308]`, `j = 1`; `input_d` is the FP-4 hand-computable case (salt `[1 .. 8]`, `VAL_NONE`, fuel `2^64`, stack limit `2^16`, limbs checked as FP-4 states); `outcome_d` is Rejected with code 7; the log is `DIGEST(0)` of `C_P`, `H2(3, 5)`, `SPONGE(3)(3, 5, 7)`, `PERM(3, 5, 7)`, `PUB(1)(0x1234)`, `DIGEST(2)` of `input_pub`.

T-FS coverage (`PR` section 13): covered on the Rust reference: the key pre-hash (a fixed commitment observe standing in for the pre-hash) then one sample; 1 to 8 words then a sample; 16 words then a sample; 9 words then a commitment then a sample; 11 samples in a row; sample, observe, sample; the same final sponge state and absorb index as the hand model after every case; `u` = 1, 2 and 4 against a hand Midnight sponge with the Sherlock capacity (`u` = 0 against midnight-circuits). Not covered: the non-final WHIR round of `L`, the full transcript of a D10 proof, and the GPU prover, stark-backend verifier and wrap sides; so G3 is not closed. `enc2.pv.p2_fingerprint` is a test checksum of the 2,816 `pv` words (a `P2` under `tagEnt`), not a protocol digest.

The values are pinned in the test (`PIN`) and copied to `VECTORS.txt` here. Selected values:

| Vector | Value |
|---|---|
| `enc1.C_P` | `0x65f6874b0be5aaa5a42b2e3b6b94ee9adf10dad60d7eff1211c65e3331ff2994` |
| `enc1.A_1` | `0x37adc0f165135af5f670e5ba16a4d473c4a6ec42847cb53833acaecdb08e82a2` |
| `enc1.input_d` | `[1844765863, 1946263212, 480254560, 1610161683, 900750315, 731375867, 384171174, 1479089448]` |
| `enc1.input_pub` | `0x675781f2fdaf30f1e06631db92a4806ee474b86159f82f31d650f648fb6fc555` |
| `enc1.outcome_d` | `[1050408218, 1118957329, 591472421, 1582239, 1404349968, 1660012201, 1063109601, 1001026748]` |
| `enc1.outcome_pub` | `0x5eeb4e2b1dd1044fcbb1391358218509b3b3623b96682a5ec4ca8e71183ee5fe` |
| `enc1.claim_d` | `[305136256, 440959392, 205775117, 793323288, 1408360988, 1841990927, 316644702, 1120322470]` |
| `enc2.D_EMPTY` | `[1601720553, 1813189633, 533296967, 1863848096, 1186913909, 1231646054, 526551043, 780997282]` |
| `enc2.onion6` | `[365509232, 1748783776, 666338835, 1517722680, 393187787, 100716010, 257089906, 1188286804]` |
| `enc3.layout_d` | `[1120272538, 1014741024, 1563488800, 1022622642, 354000869, 288819430, 75200499, 498114873]` |
| `g3.L.s2` | `0x00000000000000000000000000000000000000000000000d0000000000000000` |
| `g3.Lfr.s2` | `0x00000000000000000000000000000000000000000000001d0000000000000000` |
| `g3.L.fresh6` | `[1363105379, 892007186, 735613566, 1339319426, 1207660350, 392318480]` |
| `g3.L.obs16_sample2` | `[1428490671, 1363299102]` |
| `g3.Lfr.obs2_sample3` | `["0x08a4bccbdcfc1aeee6e31a3902b75e7ec0c744ee6a42ca2aa7dbab0c93caece8", "0x6e2e334d21a05391c7600b357ada7f1cd77bc56250914df86355863048101c19", "0x0669914356acda504ea5af22e1ed3c3905e69dffcf6305f7406f071bd05e60fa"]` |

Not covered: the AIR and wrap sides of each vector (no Sherlock leaf AIR, `L` log AIR or wrap replay gadget exists yet), the GPU, and `claim_pub` (Compact `transientHash`, section 7.2).

## 7. FP-3

A timeboxed search of the later sources (`FD/DECISIONS.md`, `research/sherlock-s10x-20261112/RESULTS.md`, `CO/FRONTEND.md` 1.4 and 1.9, the kits L AND WRAP, LEDGER CONTRACT and GP-2 hook) found no text that fixes fact 1 or fact 3. Both need a design choice that no ruling covers, so this item stops on them with options; no closure test is written against an invented form.

### 7.1 Fact 1: carrier of the effect region and `comm` (STOPPED)

Precise statement. For a run of two or more shards, name the words that carry the effect region `E` (`L_e` Fr words) and `comm` (one Fr word) from the terminal state into the call-vector table of `L` (words 2,930 onward), and the checks that make `fp3_effect_forged` fail and `pos_fp3_effect_multishard` pass.

What exists. (i) This item's reading, not design text: for a general run the statement lanes are outputs of the mandatory `DIGEST` entries of `R`, bound by the lane map (`L` word 106); the log onion `(d, n)` already crosses every shard and node. The missing check there is that `L` equates the `pack(d2)` operands of the `C_P`, `input_pub` and `outcome_pub` entries with `P2(DOM_PROGC, r_P, prog_d)`, `input_d` and `P2(DOM_OUTHIDE, input_d, outcome_d)` of the claim preimage that it opens (with `r_P` a witness of `L`); that is one check inside `L`, not a carrier. (ii) For a Compact call, `E` and `comm` are written by `R_SL_EFF` and the `SPONGE` entry of `comm`; `CO/FRONTEND.md` 1.4 rules out `PUB(i)` entries for `E` (no overflow path), and the FP-2 envelope has no effect word.

Options:

- (A) Effect onion. A second onion `(e_d, e_n)` of `(lane, value)` pairs in the boundary state, beside `(d, n)`, merged by the FP-2 log rule; `L` recomputes it from the call-vector table. Shard-local and incremental, keeps stop and resume. Cost: 9 words more in the leaf public values (I-12) and in the node envelope (I-13), and the merge in every node mode.
- (B) Header word. `eff_d = P2(DOM_EFFECT, comm, E)` as a seventh header word of `state_d` (FP-5, 42 to 50 words); the SEAL node opens it and a SET head form carries it to `L`. Cost: changes FP-5 and the SET forms (I-14, I-15).
- (C) Outcome carrier. A Compact call halts with the outcome value `(comm, E)`, so `outcome_d`, and through it `claim_d`, binds them; `L` already opens `claim_d` from `(prog_d, input_d, outcome_d)`, and adds the opening of `outcome_d` to the call-vector words. No envelope, header or SET form changes; it does change the settled `vd` forms of I-14. Cost: the `vd` forms have no Fr value (the largest integer form is `UInt n: u128`), so `sherlock-value/2` gains one form (`Fr`, 9 limbs), and `L` hashes up to `L_TX` values (about 1,200 `P2` permutations at `L_TX` = 1,024, ESTIMATE). It does not apply to a general run, whose effect words are computed from `outcome_d` itself; that path keeps (i).

Recommendation: (C), with check (i) for general runs. It reuses the one binding that already crosses every node and seal (`claim_d`), and of the three it changes the fewest settled forms (one `vd` form against the I-12 and I-13 layouts of (A) or the FP-5 header and SET forms of (B)); (A) is the fallback if the `vd` extension is refused. Owner item.

### 7.2 Fact 2: call-vector lanes (evidence, not closed)

Precise statement. Read from the compiled `abi.json` (or zkir) of `acceptRun` and `acceptCollection` the lane index of every word of the run statement and of the collection statement in the `2 + L_TX` call vector, so that the lane map of `L` names them, and refuse `fp3_cross_entry_replay`.


The kit sketch of `acceptRun` (`fp3-fact2/sherlock-ledger-sketch.compact`: `runs: Map<Field, Field>`, `claim_pub = transientHash([c_p, input_pub, outcome_pub])`, refuse if a member, insert `claim_pub -> outcome_pub`) compiles with `compactc` 0.36.0 (`--skip-zk`, log `~/.cache/enc-fp3/fact2-compactc.log`; zkir copied to `fp3-fact2/acceptRun.zkir`). Its public inputs, one lane per impact word as in the captured template `crates/mori-recursion/tests/fixtures/run-template.json` (DERIVED from the compiled zkir; the lane positions before the anchor prefix of `add-sherlock-anchor`, which shifts them by `A`):

| Lanes | Words | Op (`L/onchain-vm/src/ops.rs:520-578`) |
|---|---|---|
| 0, 1 | `binding_input`, `comm` | |
| 2 | `0x30` | `Dup { n: 0 }` |
| 3 to 6 | `0x50`, `1 1 0` | `Idx` (not cached, no push path, one path entry: field 0, `runs`) |
| 7 to 11 | `0x10`, `1 1 -2`, `claim_pub` | `Push { storage: false }` of the key |
| 12 | `0x18` | `Member` |
| 13 to 16 | `0x0d`, `1 1`, `member` | `Popeq { cached: true }` of the result |
| 17 to 20 | `0x70`, `1 1 0` | `Idx` with push path (field 0) |
| 21 to 25 | `0x10`, `1 1 -2`, `claim_pub` | `Push { storage: false }` of the key |
| 26 to 30 | `0x11`, `1 1 -2`, `outcome_pub` | `Push { storage: true }` of the value |
| 31 | `0x91` | `Ins { cached: false, n: 1 }` |
| 32 | `0xa1` | `Ins { cached: true, n: 1 }` |

The words inside each value (`1 1 -2`, `1 1 0`) are its `field_repr` as the zkir writes it; their split into lanes follows `run-template.json` and is not checked against a native proof here.

Finding: under the kit's own sketch, the call vector carries `claim_pub` (lanes 11 and 25) and `outcome_pub` (lane 30), not `C_P` and `input_pub`: `transientHash` runs inside the contract circuit, and its inputs are not public. So the FP-3 sentence "the effect words carry the three values" holds only if the contract discloses them (for example by a ledger write of each), or the lane map binds `claim_pub` to the output of a logged `SPONGE(3)` entry over the three `DIGEST` outputs. The second needs `transientHash` to equal the `u = 0` Midnight sponge of 3 inputs, which is not checked here (M1 XC-3b parity). Fact 2 stays open: the contract surface is owner item N-11, `acceptCollection` waits on fact 3, and the negative `fp3_cross_entry_replay` needs both entries.

### 7.3 Fact 3: the public and contract-side form of a collection (STOPPED)

Precise statement. Fix the form in which a claim inside an accepted collection settles on chain so that (1) its replay key is `claim_pub`, the key of `acceptRun`; (2) no collection entry publishes `n_steps`, plain `claim_d`, or a word from which either is computed without the salt (`fp3_collection_private`); (3) a Compact contract can check it, although the SET grammar is `P2` over BabyBear.

Options:

- (a) Opening by proof. `openClaim` becomes a proved entry: an `L` and wrap proof that opens a path from a `set_root` in `verified_roots` to a `LEAF` or `DEP` head and outputs `(set_root, C_P, input_pub, outcome_pub)` of that claim through mandatory `DIGEST` entries; the contract checks `set_root` and inserts `claim_pub = transientHash([C_P, input_pub, outcome_pub])` into `runs`. One key, nothing private on chain, no `P2` in Compact. Cost: one wrap proof per settled member (proving time per opening, no new key if it shares the universal wrap with a new statement kind).
- (b) Fr shadow tree. `L` computes, beside `set_root`, a public root over Fr whose leaves are the `claim_pub` of the members (a `transientHash` per member and a Midnight `H2` per node, as log entries); `acceptCollection` stores that root, and `descend` and `openClaim` open it with `transientHash` in the contract. One key and no `P2` in Compact, but the log capacity `R` grows with the collection, against OD-17 (`R` static).
- (c) Bulk settlement. `acceptCollection` inserts every member's `claim_pub`. Breaks the kit invariant "no per-claim write at `acceptCollection`" and the 256-operation cap for large collections.

Recommendation: (a). It meets the three conditions with the existing `acceptRun` binding, keeps `R` static, and keeps `descend` only as a contract-side root check. Owner item; it also amends the LEDGER CONTRACT kit (`openClaim`, `descend`) and the Quint model `ledgerContract.qnt`.

## 8. Owner items raised

| Id | Item | Recommendation |
|---|---|---|
| P-1 | Fr tag value rule | ASCII little-endian `u128` (landed `dom()`) |
| P-2 | `DIGEST` tag index | 0 to 3 in the `PR` 3.6 order |
| P-3 | `P2` absorption | overwrite (stock leaf hash) |
| P-4 | V+ types: `dom_len`, `offset`, domain byte packing | `u16`, `u32`, one byte per word |
| P-5 | `DOM_PLAN` code | 41 (registry first), against 33 (next free today) |
| P-6 | `StackStep` `op` codes and `count` type | 0 none, 1 push, 2 pop; `u64` |
| FP3-1 | effect carrier | (C) outcome carrier, with check (i) |
| FP3-3 | collection form | (a) opening by proof |
| FP3-2 note | the `acceptRun` sketch does not publish `C_P` and `input_pub` | disclose them, or bind `claim_pub` to a logged `SPONGE(3)` after `transientHash` parity |

## 9. Commits

- Clone `~/.cache/m7-wrap/src/openvm-org_stark-backend`, branch `enc-fp3` (worktree `~/.cache/m7-wrap/src/stark-backend-enc-fp3`): 829a966 and 92d77ef, the test `crates/stark-sdk/tests/enc_vectors.rs`. Not pushed.
- Moriarty, branch `enc-fp3`: this folder (`PLAN.md`, `RESULTS.md`, `VECTORS.txt`, `enc_vectors.rs.txt`, `fp3-fact2/`). Not pushed.

Next: the owner confirms P-1 to P-4 and chooses FP3-1 and FP3-3; then the AIR and wrap sides of the vectors (the `L` log AIR, the wrap replay of `DIGEST`) close ENC-1 to ENC-3, ENC-4 waits on the CORE state schema, ENC-5 on `sherlock plan`, and fact 2 on N-11 with the chosen fact 3 form.
