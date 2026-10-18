### Fresh Independent Non-Author Ordinary Source Review — Four Literal `include_str!` Path Replacements (`witness-reference-projection-portable-source-20261018`)

- **Reviewer Identity:** Fresh independent non-author Gemini 4 Argon (High) ordinary source reviewer (`CID: 21399dd6-faf2-479b-823a-9bfec7feeba7`), distinct context from prior reviewer `fe7486d5-c674-4f04-9f52-ef41e1eff966` and from the Sol path-patch author.
- **Execution Discipline:** Zero tools, commands, searches, file reads/writes, provider calls, or subject actions executed. Review conducted strictly over the inline-supplied artifacts under Root's `ordinarySource` exception for four literal `include_str!` path replacements. Prior `REPORT.md` (`bytes=36174`, `sha256=c5d693f9fbab3ddeadd50a43ae0813b82544131a0d8bb2ca92cb264db74fbd42`) is treated as historical semantic context only, not a new vote.

---

### 1. Verdicts

1. **`PORTABILITY_SOURCE` Verdict:** **`APPROVE`** (Strictly scoped to the four literal `include_str!` path replacements in `afterimages/crates/mori-core/tests/support/sherlock_witness_reference_prefix_positive.rs` (`bytes=18254`, `sha256=7391f8f8302e5441a52742962836bdb291b46f832a35099f38caf8aa6c6551cc`) and the four staged byte-identical opaque fixture copies under `afterimages/crates/mori-core/tests/fixtures/witness-reference-prefix/`).
2. **`OPTIONAL_NEW_EXTERNAL_CONSUMER_COMPILE` Classification (`01-compilePortableWitnessReferenceConsumer` in `PORTABLE-COMPILE.proposed.json`):** **`APPROVE / SAFE FOR ROOT-ONLY SINGLE EXECUTION`** (Strictly bounded single-file external `--test` compilation linking the retained `libmori_core.rlib` `sha256=4afb66098943d7a3da24bcfd4e2930b30944f81d1b06268cf369ace9454fe873` under the existing 30s wall / 25s CPU / 2 GiB AS / 64 MiB FSIZE / 0 core / 131,072B stream / exact-env envelope, with no Core rebuild and no 3-case test execution selected).

---

### 2. Material Findings & Coverage Verification

#### A. Exact Four-Line Literal `include_str!` Substitution & Reverse Byte-Identity Proof
- Comparing `prior/sherlock_witness_reference_prefix_positive.rs` (`bytes=18646`, `sha256=abf69577b7538c9ddd04558dba3b5ce68dc50971d5bbdef252dcad343f3b7980`) against `afterimages/crates/mori-core/tests/support/sherlock_witness_reference_prefix_positive.rs` (`bytes=18254`, `sha256=7391f8f8302e5441a52742962836bdb291b46f832a35099f38caf8aa6c6551cc`) and `PORTABLE-ONLY.diff` (`bytes=1585`, `sha256=32a7a222e46bedc8491eac3471afa4cdf00db3bd8a5e7a001fbd67910ef496c7`):
  - **Only** the four string literal arguments to `include_str!(...)` on lines 11–14 (`FACT`, `LIST`, `TREE`, `PENDING`) changed.
  - Each line replaces the 135-byte absolute runtime directory prefix `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/UIREQ035-independent-reference-prefix-oracle-20261018/expectations/` with the 37-byte source-relative prefix `../fixtures/witness-reference-prefix/`, keeping the exact same four filenames (`98` bytes shorter per line; `4 * 98 = 392` bytes total reduction; `18646 - 392 = 18254` bytes).
  - Lines 1–10 and lines 15–208 are character-for-character and byte-for-byte identical. Replacing those four path literals back restores every original byte, assertion, helper function, API call, and data-handling path of the accepted consumer (`sha256=abf69577b7538c9ddd04558dba3b5ce68dc50971d5bbdef252dcad343f3b7980`).

#### B. Source-Relative Path Resolution & Byte-Identical Opaque Fixture Pins
- In Rust (`--edition=2024`), `include_str!` resolves relative paths anchored at the directory of the invoking source file (`.../crates/mori-core/tests/support/`).
- Resolving `../fixtures/witness-reference-prefix/<filename>` from `.../crates/mori-core/tests/support/sherlock_witness_reference_prefix_positive.rs` targets `.../crates/mori-core/tests/fixtures/witness-reference-prefix/<filename>` (matching the canonical repository layout `crates/mori-core/tests/fixtures/witness-reference-prefix/<filename>`).
- Per `PATH-ONLY-READBACK.json` (`bytes=6432`, `sha256=b42c5dbf3d66d7ed878e275ec9d9b2c880669a234377764b172fb889f1898cf7`), all four staged fixtures are byte-for-byte identical to the original frozen expectation files (and match the actual consumer dep-info inputs, with `semantic_fixture_cells_read: false`):
  1. `factorial3-complete-A39-prefix.json`: `464,178` bytes, `sha256=af15b0e50e6f58b456f9c4b63773eccd802771d8ea64d0a2730ae14c8a6af1d5`
  2. `list2-5-complete-A39-prefix.json`: `301,342` bytes, `sha256=6feb5bb6bec696425bec29ab2abc656de52ccc07a7e8b97b424175666a7494db`
  3. `tree4-1-3-complete-A39-prefix.json`: `716,490` bytes, `sha256=c9238deef8bb1193f2b67c39fca9c9e0a56aac950f3fa2df405745b4aa9df4c6`
  4. `factorial-pending-caller3.json`: `14,726` bytes, `sha256=48cb2d9de73c2af3441bac2acd4f95a79ad4cce2e68239e7bba94f8d10dafb00`

#### C. Unchanged Production Copies & Read-Only Proposal Discipline
- Both production files (`reference_projection.rs` and `helper_machine/mod.rs`) remain unchanged byte-exact copies (`production2copies_byteexact: true`).
- No new expected data, source guard, schema, runtime/admission/budget rule, or provider is introduced.
- All source and fixture artifacts are staged read-only proposals (`no_W_Git_provider_subject_actions: true`), with no worktree writes or git mutations performed.

---

### 3. Separate Classification of Optional Single Consumer Compile (`PORTABLE-COMPILE.proposed.json`)

- **Descriptor Audited:** `PORTABLE-COMPILE.proposed.json` (`bytes=3960`, `sha256=a15d90f8caad332ca71515cc9917b44838f085d1ec23ad8fc023e47060f5d91e`), operation `01-compilePortableWitnessReferenceConsumer`.
- **Toolchain & Link Binding:**
  - Invokes the explicit toolchain binary `/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc` with `--sysroot /home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu` (authoritative over the `stable-...` paraphrase in `prior/REPORT.md`).
  - Compiles only `afterimages/crates/mori-core/tests/support/sherlock_witness_reference_prefix_positive.rs` (`--edition=2024 --crate-name witness_reference_prefix_positive --test -C debuginfo=0 -C opt-level=0 -C debug-assertions=yes -C linker=/usr/bin/x86_64-linux-gnu-gcc-15 -C link-arg=-fuse-ld=bfd`).
  - Links against the already-built retained `libmori_core.rlib` (`bytes=43779842`, `sha256=4afb66098943d7a3da24bcfd4e2930b30944f81d1b06268cf369ace9454fe873`) and `libserde_json-76dc87088753b61e.rlib`.
  - Emits only `build/witness-reference-prefix-portable` and `build/witness-reference-prefix-portable.d` under `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/witness-reference-projection-portable-source-20261018`.
- **Resource & Isolation Envelope:**
  - Enforces `/usr/bin/timeout --signal=KILL 30s` and `/usr/bin/prlimit --cpu=25:25 --as=2147483648:2147483648 --fsize=67108864:67108864 --core=0:0`.
  - Enforces `keptBytesPerStream: 131072`, `serialFailStopNoRetry: true`, and exact isolated environment (`PATH=/usr/bin:/bin`, `LANG=C.UTF-8`, `TMPDIR=.../witness-reference-projection-portable-source-20261018/tmp`).
  - Performs no `mori_core` rebuild, no Cargo/dev-deps/SDK/native/model invocation, and no 3-case test replay. Root alone executes after review.

#### Precise Permissible Claim After Optional Compile
- **If `01-compilePortableWitnessReferenceConsumer` is executed by Root and succeeds (`exit == 0` with pre/post input pin invariance and dep-info verification):**
  - It establishes **only** **source-relative `include_str!` resolution and portable consumer test compilation** against the retained `libmori_core.rlib` (`sha256=4afb6609...`) and the four staged byte-identical fixtures in `afterimages/crates/mori-core/tests/fixtures/witness-reference-prefix/`.
  - It does **NOT** establish a fresh three-case functional runtime `PASS` of the newly compiled `witness-reference-prefix-portable` binary unless those three test cases are actually executed and observed on that binary.
  - The prior five-phase `PASS` (`actual-five-01/RESULT.json`, `bytes=66419`, `sha256=0d974b2b53517b15678cc1498f4ff74736fa319c457829e8404aa7052adc4980`) remains historical evidence for the accepted absolute-include consumer bytes (`sha256=abf69577...`) and is not silently relabeled as a runtime execution receipt for the new portable binary.

---

### 4. Preserved Open Scope Boundaries

- Prior B absolute clock/counter bound cases (`work_ChargedTotal`), H1 physical performance/memory claims, canonical `ProjectRel`/`Link` projection, native/proof/target gates, and whole `UIREQ-035` / `SCON013` / `SCON024` acceptance remain **OPEN**.
- No file, hash, or test-count check constitutes semantic auto-approval.

