
If C1–C7 all hold, CUR03 is satisfied for membership selection by this unchanged test at the frozen tree. Specifically, the declared `source_closure` `include_str!` list is all of the following:
- the frozen 61-file baseline followed by the 8 Core stage files, 69 in total, distinct and existing;
- covering every `.rs` file under `mori-core/src`;
- covering every `.rs` file under `mori-lexer/src` and `mori-ledger/src`, except three exclusions that are reasoned, `#[cfg(test)]`-declared and named nowhere else.

It also confirms the C1 cleanup at the file-membership level: any leftover Cardano or export `.rs` file under `mori-core/src` would fail tests 3 and 4.

It does **not** establish any of the following:
- The closure hash value, or that it matches any `PublicDerivedCallKey` snapshot. The disclosed root `Cargo.lock` addition changes the hashed closure. Any previously derived snapshot would therefore refuse with `derived-source-identity`, and build identity stays unqualified.
- That the actual Rust build used these inputs. The closure is declared identity, not build authentication.
- Coverage outside the three walked roots. This includes:
  - new files under `crates/mori/src` (only listed CLI files are pinned)
  - `build.rs` at crate roots
  - non-`.rs` files pulled in by `include_*`
  - non-test `#[path]` modules outside `src`
  - feature or `cfg` selection
  - git and SDK dependency sources
- Any semantic or native qualification: SRS provenance or setup, SDK keygen correspondence, the source–constraint theorem, proofs, verification, Cardano export semantics, settlement.

The author's source report counted an SDK effect based on the assumed Cargo path. For this direct path only, that effect finding is superseded. Independent source-comparison stages do not waive any other required native qualification.