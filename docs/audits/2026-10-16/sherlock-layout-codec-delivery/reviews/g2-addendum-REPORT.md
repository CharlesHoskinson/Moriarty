# Opus 5.5 security-owner addendum: correction to prefix preflight G-2

## Verdict: **ADDENDUM_APPROVED_WITH_LIMITS**

Reviewer: Claude Opus 5.5 (`claude-opus-5-5`). I used no tools and executed nothing. I recomputed no hashes; every digest is taken as supplied. I cross-checked the digests by reading the baseline `CANDIDATE.json`, the prefix `CANDIDATE.json`, both `CONFIG.json` files, the baseline `receipt.json` and `MEMBERSHIP-FACTS.json` against one another. Prefix Rust is **UNRUN**.

## 1. What was wrong in the original G-2

The original G-2 says 23 compile-closure members are shared with the baseline, listing 6 sherlock sources and 17 `layout_a` files. That premise was impossible as written:

- **Baseline closure is 24 files.** Per baseline `CONFIG.json` and the receipt's `compile_inputs`, it is the harness, the shim, 4 sherlock sources (`reference_types`, `reference`, `reference_display`, `compare`) and 18 `layout_a` files.
- **The two input modules are prefix-only.** `normal_oracle_inputs.rs` and `extra_oracle_programs.rs` are in neither the baseline closure, the baseline snapshot nor the baseline manifest. G-2 wrongly counted them as baseline-shared.
- **The prefix closure count was right.** The prefix closure is those 24 paths plus the 2 input modules plus the leaf, which is 27. My §2 count ("6 sherlock sources … 27") was correct for the prefix closure. Only G-2's baseline-sharing claim was wrong.

The original G-2 is preserved unchanged as historical text. It is **superseded for gating** by the corrected G-2 below. It is not to be evaluated as written.

## 2. Facts checked by reading

| Class | Count | Finding |
|---|---|---|
| Same path and byte-identical to the passing baseline | 21 | 4 sherlock sources and the 17 `layout_a` files other than `mod.rs`. Each digest in `MEMBERSHIP-FACTS` matches the baseline manifest `a164cdae…d57d2f`, the receipt's `sha256_before` and `checkout_sha256_after`, and the prefix manifest. |
| Same path, changed | 3 | `layout_a/mod.rs` `949bc3bb…df45` → `1431971b…4805` (covered by G-3). `layout_harness.rs` `75786195…6bf0` → `852fa59d…97fa`. `layout_harness_shim.rs` `b0cc8b51…2f94` → `d648bfcd…80a1`. The prefix digests of all three equal the rows reviewed in the original preflight. |
| Prefix-only | 3 | `normal_oracle_inputs.rs` `d02cf40e…d2b8`, `extra_oracle_programs.rs` `e0c6819d…f152`, and the leaf `layout_observations.rs` `18c906e3…3f3f`. Each equals its prefix-manifest row. Root's receipt reports each equal to the working tree. |

**Security reading of the two input modules (full source supplied):**
- **Contents.** Both are pure typed-AST constructors: `Arc::new` over `Expr`, `Program`, `Function`, `Parameter`, `Arm` and `Value`. There is no `unsafe`, I/O, environment, argv, FFI, `extern crate`, third-party `use`, `include!`, `include_bytes!` or build-time input.
- **Path attribute.** The single `#[path = "extra_oracle_programs.rs"]` resolves inside `crates/mori-core/src/sherlock/`. That is consistent with the 27-member closure, and the run's dep-info check enforces it.
- **What the producer uses.** The producer reaches only `normal_program("N01")` (`7`) and `normal_program("N02")` (`2 + 3`).
  - N03–N08 are compiled but never invoked.
  - The `panic!` for an unknown id is unreachable with the fixed `INPUTS`.
- **H1.** These are trusted, fixed normal inputs. That matches H1 and adds no input endpoint.

**Conclusion:** the baseline-identity predicate actually holds for 21 members. The two input modules cannot satisfy a baseline-identity predicate. They do satisfy a reviewed-and-pinned predicate: they were in the reviewed 50-row prefix manifest at these exact digests, and their full source is now read. No real predicate is waived. The premise is corrected to name the evidence that actually exists.

## 3. Corrected G-2 (binding; replaces the original G-2 for gating)

> **G-2 (corrected). Compile-closure provenance.** The 27-member prefix compile closure must partition exactly as follows, all bound through the prefix manifest `6975756204ac…df36`. Root records the full digest under G-4.
>
> - **G-2a. 21 baseline-identical members.** These must be byte-identical to the corresponding members of passing baseline manifest `a164cdae…d57d2f`, as recorded in baseline receipt `441060b6…7b22`. The members are `reference_types.rs`, `reference.rs`, `reference_display.rs` and `compare.rs`, plus the 17 `layout_a/*` files other than `mod.rs`, at the digests listed in `MEMBERSHIP-FACTS.json` `byte_identical_shared`.
> - **G-2b. 3 same-path reviewed changes.** These are not baseline-tested bytes.
>   - `layout_a/mod.rs` at `1431971b…4805`, subject to G-3 unchanged.
>   - `layout_harness.rs` at `852fa59d…97fa`.
>   - `layout_harness_shim.rs` at `d648bfcd…80a1`.
> - **G-2c. 3 prefix-only reviewed members.** These are not baseline-tested bytes.
>   - `normal_oracle_inputs.rs` at `d02cf40e…d2b8`.
>   - `extra_oracle_programs.rs` at `e0c6819d…f152`.
>   - `layout_observations.rs` at `18c906e3…3f3f`.
>
> No other file may appear in the closure. C-4.3's dep-info equality with these 27 members stays required.

## 4. Scope and limits of this addendum

- **AL-1. G-2 only.** This changes only G-2. The original G-1 and G-3 to G-5, C-1 to C-6, L-1 to L-6 and S-1 to S-6 remain exactly as written. This includes one attempt (`attempt-01`), no retry, no change to dimensions, capacities or defaults, and native authority 0.
- **AL-2. Baseline evidence does not cover new bytes.** The 6 G-2b/G-2c members carry review evidence only, with no execution evidence. The baseline pass gives no assurance about them.
- **AL-3. Baseline evidence does not transfer to the new overlay.** The baseline pass covered the 21 G-2a members under the *baseline* harness and shim overlay. Byte identity does not carry that execution result over to the prefix overlay. The prefix compile and run are their own, still-unobserved evidence.
- **AL-4. Working-tree match is provenance only.** The run binds to the prefix manifest digests. If the working tree later diverges, that does not affect this run. It also confers nothing on the S-5 integration review, which stays separate.
- **AL-5. No other claim.** This addendum makes no security claim beyond H1 trusted fixed normal inputs.

On G-1, observed only and not re-adjudicated here: the supplied receipt and stdout are consistent with G-1. They show manifest `a164cdae…d57d2f`, `terminal_state: success`, all six steps `ok`, and `35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`.

**ADDENDUM_APPROVED_WITH_LIMITS.** The corrected G-2 above, subject to AL-1 to AL-5, together with all other original gates, conditions and limits unchanged.
