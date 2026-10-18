# Independent security publication review: Cardano/Groth16 local research package

**Verdict: `APPROVE_SCOPED_RESEARCH_PUBLICATION`**. This covers the five exact files pinned below, unchanged, as historical scoped research. Publication requires the separate dated adoption addendum in §7. The five frozen files must not be edited.

This review grants no native, runtime, product, toolchain, transaction, setup, proving, key, SDK, wallet or ledger admission. It does not rerun or re-audit the proofs. The original scoped source and result reviews stand as written.

## 1. Approved files (exact pins)

| File | Bytes | SHA-256 |
|---|---|---|
| `README.md` | 5,796 | `863acb5c22044f61e76ad7610c333871f1b3b0673e4568f08f183584c0e8eb6f` |
| `DOC-MANIFEST.json` | 4,695 | `4db17582d2f23dc4a3c23ec00bf2a89daf20c543ec76420087b3aa1b2d372f16` |
| `PUBLICATION-MANIFEST.json` | 256,277 | `905daa86c36544d872c2c5f250290dcd39e3dafa28221e836e6082e29e98b662` |
| `ROOT-ROUNDTRIP-CHECK.json` | 363 | `f8b9ff8cc057cad94d5344d6365da9b4f7c185c42a8ccdaffb8a47576e71100b` |
| `cardano-groth16-local-research-20261015.tar.gz` | 709,326 | `257ff2a49630390d5a8c4d4e052f8563ffae2deeafefe5975843976f023dd7fc` |

I hashed each file myself; all five equal `ROOT-CANDIDATE-FREEZE.json`. Any other byte sequence is outside this approval.

Not approved for publication: `prepare_archive.py`, `ROOT-PREPARATION-NOTE.json`, and this directory's `PROMPT.md`, `run-review.py`, `process.stderr`, `provider-result.private.json` and `review-tools/`. The `review-tools/` scripts are my local stdlib verification helpers, not an entry point.

## 2. Reviewer, independence, method and limits

| Item | Statement |
|---|---|
| Reviewer | Claude Opus 5.5 (`claude-opus-5-5`). "High" effort is recorded as requested in `PROCESS.json`; I cannot observe it. |
| Independence | I did not author any source, experiment, result review, archive, README, manifest or preparation step. Independence is at the context level only: the authors, executors and earlier reviewers are the same model family. I consulted no other model or agent and launched no child processes. |
| Not read | The Muse documentation review directory (listed only, contents not opened), any draft, any `provider-result.private.json` or `provider-stream.private.jsonl`, any `process.stderr`, and any wallet material. My own `PROMPT.md` was not opened; the request reached me directly. |
| Method | Python stdlib only (`-I`), reading in memory: `tarfile`, `gzip`/`zlib`, `hashlib`, `json`, `re`. Nothing was extracted to disk, and no archive content was executed or imported. No project code, tests, SDK, compiler, Aiken, VM, curve/proof code, network, Git mutation or ledger action. |
| Startup | Loaded the develop skill, `references/execution-focus.md`, checkout `AGENTS.md`, `docs/MORIARTY-PRODUCT-CONTRACT.md` and `docs/audits/2026-10-14/SECURITY-ROUTING.md` from `utxo-pr54-currentness-20261015`. I relied on the root's refreshed guarded status and did not run the project CLI, because the request excludes project code. `SECURITY-REVIEW.md` was not read. |
| Harness denials | Four commands were denied automatically. Three were inline-script forms, which I redid as script files. The fourth was a read-only `git rev-parse`/`sha256sum` in the old `utxo-groth16-preprod-20261015` worktree; it was **not performed** (see L-6). |

## 3. Mechanical verification (performed by me)

**Archive container**
- The gzip stream ends with 0 trailing bytes. The header has no filename and mtime 0.
- The tar is 3,706,880 bytes: 476 members followed by 2,048 zero bytes. No member has nonzero padding slack.

**Members**
- **476/476** are regular files (`type 0`, mode `0644`, uid/gid 0, empty uname/gname, mtime 0).
- There are no links, devices, directories, absolute paths, `..` segments, backslashes, non-normalized paths or duplicates. PAX headers carry `path` only (429 long names).
- Every member sits under the single prefix `moriarty-cardano-local-research-20261015/`.

**Hash match**
- I recomputed SHA-256 and size for all 476 members. They equal the 476 `PUBLICATION-MANIFEST.json` rows: 0 mismatches, 0 missing either way, 2,895,618 bytes in total.
- All 476 `originalPath` files exist and match their size and SHA-256 (476 unique paths).
- Both `inputManifests` match their pinned SHA-256 (`00c62f14…`, `918104cd…`).
- The three `excluded` originals still exist and match their pins.

**Membership is exactly the reviewed union**
- The two accepted review input manifests list 475 unique paths, with no hash conflicts.
- Publication = union − 3 excluded + 4 review records:
  - the Aiken review `INPUT-MANIFEST.json`, `PROCESS.json` and `REPORT.md`;
  - the X7 review `INPUT-MANIFEST.json`.
- No union file is silently dropped, and no file outside the union and review records was added.

**Four external reference files**
- They are stored under `reference-checkout-utxo-groth16-preprod-20261015/`, which maps to `/home/charl/Moriarty/.worktrees/utxo-groth16-preprod-20261015/{AGENTS.md, crates/mori-core/src/ir.rs, crates/mori-core/src/runtime.rs, docs/MORIARTY-PRODUCT-CONTRACT.md}`.
- These are the same paths and digests pinned in the X7 `INPUT-MANIFEST.json` and `independent-opus-review/SOURCE-FREEZE.json`. The archived `source-freeze-check.txt` records OK for all four.

**Build-graph paths**
- `x7/src/main.rs` `#[path = "../../../../../groth16-preprod-experiments-20261015/g16/src/{gadgets,circuit}.rs"]` resolves inside the archive prefix.
- Both targets are present.

**Excluded by basename**
- The archive contains 0 files named `PROMPT.md`, `run-author.py`, `run-review.py`, `process.stderr`, `provider-result.private.json`, `provider-stream.private.jsonl` or `COMMAND.json`.
- The 21 `earlierUnexecutedContextOmissions` in the Aiken input manifest are also absent:
  - the revision-1 `PROMPT.md` and `run-author.py`;
  - 19 Aiken v1.1.24 compiler-reference files.

**Cross-checks of README and DOC-MANIFEST**
- `DOC-MANIFEST.json` pins the README and all four claim-source reports. Each equals its manifest row: archive path, original path, size and SHA. The package identities, including the unpublished `prepare_archive.py` and note, also match on disk.
- Headline counts recounted from raw archived logs:
  - S5 `9 passed; 0 failed`;
  - S6 21 `AS-EXPECTED`, 0 `UNEXPECTED`;
  - L1 8/8 and M1 14/14 `asExpected`;
  - S4–S12 exit files all `0`;
  - one S4 warning (`Liability` never constructed);
  - S7 `mismatches []`;
  - `gate-A6-76.json` 76/76 ok;
  - `observe-V3-89.json` 89/89 ok;
  - `plutus.json` `9ffc530d…`, validators `7f961dfa…`/`85592a6a…`;
  - `v1.1.21+42babe5` preamble.
- BLAKE2b-256 of the archived 582-byte `selected-body.bin` (SHA-256 `37bbdf92…`) is `9e6dfe33…8541`, the README's identifier.
- BLAKE2b-256 of each `vk.bin` is `7b014d8e…` (L1) and `8e1ae703…` (M1). Each equals its `claim.json` and `admission.json`. Both VKs have IC count 3 at offset 336.
- `aiken_r1_control.cpython-314.pyc` header: source size 25,681 bytes, equal to the `.py`; mtime `0x6ad09aa4`, consistent with Aiken review RR-3.

## 4. Privacy and credential-shape scan (476/476 members, bodies not printed)

**Zero hits** for:
- PEM/OpenSSH private keys;
- Cardano `SigningKey`/`.skey`/`_skey`;
- bech32 secret prefixes (`addr_xsk`, `ed25519_sk`, `xprv` and others);
- AWS, GitHub, OpenAI/Anthropic, Slack and Google token shapes;
- Blockfrost project IDs;
- mnemonic/seed-phrase keywords;
- email addresses;
- JSON `reasoning`/`thinking`/`encrypted_content`/`signature` keys;
- `<thinking`/`antml:` markers;
- Windows user paths;
- bidi, zero-width and BOM characters;
- ANSI escapes.

`selection.json` contains no URLs.

Non-text members: 8 in total.
- The `.pyc`.
- L1/M1 `proof.bin` (192 B), `statement.bin` and `vk.bin` (488 B).
- `selected-body.bin`, a transaction **body**. The witness-bearing `selected-tx.cbor` is not in the archive.

No proving key, trapdoor, seed, SRS or private witness file is present. The largest member is the 127,866-byte input manifest, and a Groth16 PK for these circuits would be far larger. The `seed` JSON fields are Aiken test-runner seeds (`2743329201`, `760865730`). The `privateMaterial` fields are prose statements, not material.

Every keyword hit was inspected with masked context. None is a secret:

| Hit | What it is |
|---|---|
| `env -i PATH=… HOME="$TASK/empty-home"` | Sandbox command text |
| `PRIVATE_NAMES`/`EXCLUDE` sets | Exclusion lists naming private files, not containing them |
| `"<redacted>"` | Redaction code in `x7_run_control.py` |
| `pk.bin`/`toxic waste` | B2 design prose and report prose |
| Bech32 testnet/script addresses | In `CANONICAL-MPC-CLAIM.md`/`SP2-PREVIEW.md`, copied from the upstream public repository `CharlesHoskinson/proof-zk-recovery` at `7f54c35f…`. Root's archived metadata receipt records it as public and Apache-2.0, and `LICENSE` is included. Addresses are public on-chain identifiers. |

Disclosed host identifiers:
- the host name `hoskinsonlaptop`, the WSL kernel string, user `charl` in absolute paths, swap and CPU sizes;
- agent `sessionId`s, token counts and list costs in seven `PROCESS.json` files.

None is a credential. The host name already appears in 9 files of the current checkout's working tree; I did not verify its GitHub state. The absolute paths are provenance locators.

Source-code crypto identifiers and comments, test bytes (`44…`, `55…`, `07…`), public VKs, proofs and statements are public research data. They are distinct from the never-saved PK, setup trapdoor and RNG seed.

## 5. Claim grounding (README against the four supporting reports)

Every current README claim is grounded in the X7 review (`adc6d924…`), the Aiken R1 review (`19316733…`) or the two execution reports (`a2779604…`, `0cda4d9f…`), and my recounts agree:

| README claim | Grounding |
|---|---|
| 9 tests, 21 ABI checks | X7 §2 |
| L1/M1 proofs under fresh single-party setups; mutations and forced proofs rejected; fixed hand-lowered relations; no `6c7` correspondence | X7 §9, R-7 |
| 76/76 then 89/89; X7's earlier GENERATED_ONLY label correct at its cutoff | Aiken §3; X7 header |
| `plutus.json` under v1.1.21; wrappers not evaluated; Aiken-model CPU is not ExUnits | Aiken §3.3, §5, §8 |
| VKs Class 0, host-trusted, orphan; no PK or transcript; swap active; no zeroization; never pin in a real genesis; VK-to-relation provenance open | X7 §6, §7; Aiken §7, §8, §10 |
| `refund_vkh`/expiry outside the statement, so re-wrap is possible; F-A is a deposit fix, not exclusivity or funds safety | X7 R-1; Aiken F-R1-1, §7 |
| Wrapper decoding, physical integration, Preprod deploy/genesis/issue/settle/refund unperformed and blocked | Aiken §10 |
| `44…44#1`, `55…55`, expiry 1000 are synthetic; the L1 body is provider-observed real Preprod bytes; byte or txid equality gives no inclusion, unspentness, history or finality; value unchanged | X7 §9; Aiken §8 |

**No relabeling, no new ABI**
- No source is relabeled Source/5.
- No new claimant/ABI v1 candidate appears in the archive. "ABI v1" occurs only as a future proposal (Aiken source review P2/P3) or a non-claim.
- No ledger transaction or genesis-key admission is included or claimed.

**Open items stated**
- Every one is stated as open: the 20 patterns, general obligations and liabilities, physical effects and value, recursion, private PCD and handoff, multi-party/multi-parent composition, and U/M/MC/SP/G exits.
- No Midnight ZKIRv3, Preview or Fenrir qualification is claimed.
- No public developer permission requirement is introduced, which is consistent with the product contract.

## 6. Findings

None blocks publication of the five frozen files.

| ID | Severity | Finding | Disposition |
|---|---|---|---|
| A-1 | **Required (process)** | All three manifests still record the pre-review state. README says the Opus and Muse reviews are "pending". `DOC-MANIFEST.status` reads "…reviews pending". `PUBLICATION-MANIFEST.status` is `PREPARED_CANDIDATE_NOT_PUBLISHED`, and its scope says "Full security publication review pending". After this terminal verdict these lines become historical. | Publish the dated adoption addendum (§7) as a separate file. Do not rewrite the five files. **Falsifier:** a published tree that contains the five files without that addendum, or any file whose hash differs from §1. |
| A-2 | Low (claim precision) | Some archived executor narratives were later qualified, but the README's "later observations qualify" sentence names only generated-only statements. Three statements are affected: **(1)** Rust `FINAL-RESPONSE.md` says "no setup or prove command was run" and "nothing was published or sent anywhere". **(2)** The first means no standalone `x7 setup`/`x7 prove` subcommand; `x7 run` itself performed in-process single-party setup and proving. **(3)** For the second, X7 §4 restricts the accurate scope to "no child step had IP egress". `out/{L1,M1}/results.json` "nothing persisted" is qualified by X7 §6 (not written by x7; swap residue possible; no zeroization). The README already covers the last point. | Addendum lines. The originals stay unchanged. |
| A-3 | Info | The README's `9e6dfe33…8541` is the transaction ID, i.e. BLAKE2b-256 of the 582-byte body `37bbdf92…`; I verified this above. It is not a digest of "bytes" in any other sense. | Addendum clarification |
| A-4 | Info (safety labelling) | The archive carries deployable `plutus.json` validator bytes (`c0` `7f961dfa…`, `settle0` `85592a6a…`) and two VKs. The preamble labels them Class 0 research. B-C is High, conditional on Preprod. | Addendum: name these hashes and digests as do-not-deploy, do-not-fund and do-not-pin |
| A-5 | Info | `DOC-MANIFEST.json` pins identities for `prepare_archive.py` and `ROOT-PREPARATION-NOTE.json`, which are intentionally unpublished. | Addendum note; harmless |
| A-6 | Info | `controller/__pycache__/aiken_r1_control.cpython-314.pyc` is opaque bytecode. It is retained because Aiken review RR-3 relies on its header. Extracted with mtime 0, it would be stale and is not auto-loaded in place of the source. | Keep as provenance |
| A-7 | Info | The README states open scope through "general obligations and liabilities" and "changes no U/M/MC/SP/G exit". It does not name "complete effects", "residual duties" or "Midnight native authority remains zero". | Addendum restatement |

## 7. Required adoption addendum: literal minimum contents

Root writes this after the terminal reviews, as a new file next to the five, for example `ADOPTION-20261015.md`. Fill in the bracketed values from actual receipts. If the Muse audit has not returned an accepting terminal verdict, record its actual status and do not describe the documentation audit as accepted.

```markdown
# Adoption receipt — 2026-10-15

This dated receipt supersedes the "pending" status lines in the frozen README.md, DOC-MANIFEST.json and PUBLICATION-MANIFEST.json (`PREPARED_CANDIDATE_NOT_PUBLISHED`). Those files are unchanged; their statements describe the state at freeze.

Published files (unchanged):
- README.md 5796 B 863acb5c22044f61e76ad7610c333871f1b3b0673e4568f08f183584c0e8eb6f
- DOC-MANIFEST.json 4695 B 4db17582d2f23dc4a3c23ec00bf2a89daf20c543ec76420087b3aa1b2d372f16
- PUBLICATION-MANIFEST.json 256277 B 905daa86c36544d872c2c5f250290dcd39e3dafa28221e836e6082e29e98b662
- ROOT-ROUNDTRIP-CHECK.json 363 B f8b9ff8cc057cad94d5344d6365da9b4f7c185c42a8ccdaffb8a47576e71100b
- cardano-groth16-local-research-20261015.tar.gz 709326 B 257ff2a49630390d5a8c4d4e052f8563ffae2deeafefe5975843976f023dd7fc

Publication reviews:
- Security content/privacy/provenance: Claude Opus 5.5 (claude-opus-5-5), requested effort high (not observable), terminal APPROVE_SCOPED_RESEARCH_PUBLICATION; report [path] [bytes] [sha256]; process [exit/status].
- Documentation: muse-wonder-1.0, requested effort high, terminal [actual verdict]; report [path] [bytes] [sha256].
Both reviewers are fresh non-authors; independence is by context. Neither review reran any experiment.

Scope (unchanged by adoption): historical Class 0 local research only. Not an installed backend, supported toolchain, approved transaction, or Midnight ZKIRv3/Preview/Fenrir result. Midnight native/SDK authority remains zero. Complete effects, residual duties, general obligations and liabilities, all twenty patterns, recursion, private PCD/handoff, multi-party/multi-parent composition and every U/M/MC/SP/G exit remain open.

Do not deploy, fund or pin: validators c0 7f961dfa78c89965e52f05dcc6ac34c8431a35f87faf70182deabd8d and settle0 85592a6ac1d67cffe628b93562f410e7e79f5a88be21a0e1edd69443 (wrapper decoding unevaluated; B-C open); VKs 7b014d8e…2198 (L1) and 8e1ae703…fc55 (M1) (Class 0, host-trusted, no transcript, no zeroization). `9e6dfe33…8541` is the Preprod transaction ID (BLAKE2b-256 of the archived 582-byte body); it authenticates no ledger state.

Archived executor narratives are qualified by the independent reviews: "no setup or prove command" means no standalone x7 setup/prove subcommand (x7 run performed in-process single-party setup and proving); "no network/nothing sent" means no child step had IP egress; "nothing persisted" means not written to the filesystem by x7, with swap residue possible.

Not a complete reproduction kit: see the omissions listed in the security report §8. `prepare_archive.py` and `ROOT-PREPARATION-NOTE.json`, pinned in DOC-MANIFEST.json, are intentionally unpublished preparation records. No private provider output, prompt or runner is published.
```

## 8. Precise limitations of the package (to state, not to fix)

- **L-1.** The package is not a complete original-filesystem restore or a byte-reproducible rerun kit.
- **L-2.** These are absent:
  - the stdlib `stdlib-source.tar.gz` (A0 hashed it; the extracted source, `SOURCE-MANIFEST.json` and `SOURCE-PIN.json` are present);
  - the 19 unexecuted Aiken v1.1.24 compiler-reference files;
  - the revision-1 and Rust-execution prompts and runners;
  - the X7-cited predecessor `FINAL-RESPONSE.md` `712bfd…` and `SOURCE-PIN.json` `60e1fd…`;
  - most of `groth16-preprod-experiments-20261015` beyond its 12 pinned inputs (for example `PLAN.md`, `REPORT.md`, `RESULTS.json`, `COMMANDS.json`, `gen_aiken.py`, `make_bundle.py`, `selected-tx.cbor`);
  - the full upstream `proof-zk-recovery` tree (10 selected files);
  - the toolchain binaries and the built x7 binary `d235ad33…`;
  - crates.io sources (pinned only by `Cargo.lock` checksums);
  - build and target directories.
- **L-3.** The pinned hashes of absent files (for example `promptSha256` in `PROCESS.json`, `reportSha256`, and the A0 tarball check) can be compared only against originals held outside the publication.
- **L-4.** `moriref.json` came from the older Core worktree. Only `ir.rs` and `runtime.rs` from that worktree are included, so the oracle cannot be regenerated from the package.
- **L-5.** The archive mtimes are normalized to 0. Original timestamps survive only inside receipts.
- **L-6.** The old worktree commit `af8cc9fb…` (tree `eb295f2d…`, Core "unchanged from `91fd`") is root-reported and unverified in the archive. My read-only Git check was not performed. The four reference files are pinned by content hash only.
- **L-7.** Upstream identity of the 10 `proof-zk-recovery` files rests on root's archived receipts. I did not fetch them live.

## 9. Not recommended or authorized

No further execution, key generation, setup, proving, Aiken run, wrapper evaluation, Preprod/ledger action or network fetch is recommended or authorized by this review. Publishing these records runs no experiment. This is maintainer-operated review only. It creates no public developer compile/prove/deploy requirement.

## 10. Read scope, denominators and terminal record

**Read in full**
- The five candidate files: README, DOC-MANIFEST, ROOT-ROUNDTRIP-CHECK, and PUBLICATION-MANIFEST (all 476 rows plus header, `inputManifests` and `excluded`, parsed and listed).
- The archive at the container and member level (476/476).
- Both adopted independent result reviews (X7 28,023 B; Aiken R1 23,927 B).
- Both execution reports (8,432 B; 11,766 B).
- Both executor `FINAL-RESPONSE.md` files.
- All seven archived `PROCESS.json` files and `ROOT-AUTHORIZATION.json`.
- L1 `results.json` and `admission.json`, x7 and g16 `Cargo.toml`, and the Rust `host-metadata`, `toolchain`, `variables` and `cargo-home` receipts.
- The two review `INPUT-MANIFEST.json` outlines.
- `proof-zk-recovery` `SOURCE-PIN.json` and its repo-metadata fields (originals outside the archive).

**Machine-scanned only:** all 476 members for credential shapes, privacy markers, Unicode and ANSI controls, and targeted strings, with masked context on every hit. The remaining members were not read line by line.

**Terminal record**
- Owned child processes: 0. Live processes owned at terminal: 0.
- Writes: only this directory (`REPORT.md`, `review-tools/*.py`).
- Candidate and original files untouched.

**Terminal verdict: `APPROVE_SCOPED_RESEARCH_PUBLICATION`** for the five files in §1, with the §7 addendum required at publication.
