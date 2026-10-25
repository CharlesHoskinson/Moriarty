# FSYN first P0/S0 slice

This SDK-free Python implementation validates source identity and reconstructs the
frozen PUBLIC_SYNTHETIC FRTF v3 corpus. It implements useful parts of PR396's
approved design; P0/G0 and fixture release remain open.

## Generate and replay

From the repository root, with the pinned Fenrir checkout available:

For relocated sources, set `FSYN_SOURCE_MAP` to a JSON object mapping original
cut paths to absolute local paths. Aliases affect only file lookup: every byte
count and SHA256, the original cut digest, duplicate-source rejection and corpus
authority binding remain checked. Aliases are not included in fixture identity.
`scripts/qa/check-b1.sh` prepares this map from `FENRIR_ROOT` (default
`~/repos/projectfenrir-main`) and the original revision-20 Git snapshot, caching
matching historical bytes under `target/fsyn-sources/`. If an original public
source was never imported into Git, `FSYN_ARCHIVE_ROOT` may name an archive
mirroring its original home-relative path (`research/...`). Missing or stale pins
remain hard failures; preparation never refreshes the cut.

## B1 recovered-source baseline (2026-10-24)

The owner authorized a new B1 baseline while Sherlock's design is unfrozen. It
uses the 123 recovered sources with their original size and SHA256 pins. The new
cut digest is `884f0f1162ea3589f42a39820b8a7a3dc35051ea8e3c0f1bd924c321d3187bfd`.
The old 125-source digest was
`345c4f139c724dbf261e7aed343f4537b8b6c636464e0aa1e4af7246bcb98e02`.
Two historical sources were not recoverable on this host or in Git history:

| Original research path | Bytes | SHA256 |
|---|---:|---|
| `fenrir-rollup-design-2026-10-19/scripts/build-rev20.py` | 15535 | `faea2379c597cafc82d403b35cb11d7237bd3d79d32fd2e90af4749b83fead8a` |
| `fenrir-rollup-design-2026-10-19/audits/ROLLUP-ARCHITECTURE.rev19.md` | 524974 | `6c7d5831e0dd2ead7b8a66bf48d9eae0d7a612b98cc9f7864f9a732a5f610f62` |

Their original pins and the old digest remain in the cut's `baseline` record.
This baseline does not claim to run the missing architecture builder. It keeps
all 659 public corpus members and independently rebuilds 13 positive envelopes.
With seed `0x5354524157`, the fixture has 662 files and identity
`44ca50713872b4ce7a27566ae3f271f7f163176d97458216be575c429a1024fa`.
`baseline.json` pins its manifest SHA256
`a35a6b037484962b8108cc48432f0e016ddee0f113dbf0323c3d9a9d75376cc9`
and content digest
`19f51b924638947ef698aa9a6b7eae39203fdac5e8312ef7fd1006b67c7a90d1`.
The B1 check generates twice, compares every member byte-for-byte, checks these
fixed digests, and retains stale-source and tamper rejection tests. Full FSYN
release gates remain open.

```sh
python3 models/sherlock/rollup-synth/fsyn.py cut check --cut openspec/changes/sherlock-synthetic-rollup-data/design-cut.json
python3 models/sherlock/rollup-synth/fsyn.py generate --cut openspec/changes/sherlock-synthetic-rollup-data/design-cut.json --golden-root "$HOME/projectfenrir/recursion/rollup-test-format/v3" --out NEW_OUTPUT_DIRECTORY --seed 0x5354524157 --scale S0 --profile both --mix strawman
python3 models/sherlock/rollup-synth/fsyn.py verify --cut openspec/changes/sherlock-synthetic-rollup-data/design-cut.json --golden-root "$HOME/projectfenrir/recursion/rollup-test-format/v3" --fixture NEW_OUTPUT_DIRECTORY
```

`generate` refuses an existing destination and unsupported seed, profile, scale
or mix. Verification returns exit1 when committed bytes or file membership differ;
stale or malformed cut inputs and invalid directory inputs retain exit2. This first slice covers the frozen seed, both profiles, minimal and all
six full batches, including the b1/b2/b4/b6 workload descriptions. It copies the
complete 659-member public corpus and its FRTF manifest. All 13 positive binary
envelopes are independently encoded from their JSON views and must match the
original bytes. Negative cases and I1 retain their original bytes. No referenced
VK, proof or wallet file is opened. Every source pin is checked before output
creation. An IO failure leaves partial output visible; remove or retain that
failed destination explicitly before selecting a new output path.

The root `MANIFEST.json` covers every payload file with size and SHA256 and hashes
the canonical sorted row array. It excludes itself. `FIXTURE.json` records the
seed, mix, profile, scale, generator version and canonical source-cut digest.
Raw cut-file receipt hashes belong in external execution provenance, outside the
deterministic fixture. Whitespace and retrieval-only cut metadata do not change
identity or payload bytes. Generation adds no clock, hostname or absolute output path. The
regeneration recipe is labelled as a recipe; actual argv belongs in an execution
receipt. `verify` regenerates independently from the pinned public sources and
compares complete membership and bytes, including both manifests and metadata.

`cut refresh --cut OLD --out NEW` writes a new cut and reports changed paths;
it refuses an existing output. Matching or refreshed hashes approve no source
semantics. The command does not run `build-rev20.py --check` or extract constant,
layout or rule changes: that part of SSRD-001/P0 and the revision-20 coupling
remain pending. Its response explicitly reports that check as `not-run`.

## Encoding and scope

`frtf.py` implements the actual Rust envelope layout, canonical field encodings,
bounded counts, step links/order, complete header-close coverage, Profile-B slot
and covenant-reference association, and the tagged envelope digest. Its SHA256
is only FRTF's `TH`: u64 little-endian UTF-8 tag length, tag, then ordered parts.
It does not implement Poseidon, authenticate history, check signatures or verify
proofs. JSON root/commitment values remain the pinned public values.

The random stream is exactly `TH("moriarty/fsyn/rng/v1", u64LE(seed),
UTF8(stream_name), u64LE(counter))`, starting at zero, with a persistent byte
buffer so chunk boundaries do not change output. The planned stream names are
schedule, mix, amounts, committee, celestia, forced, escape and mutation. S0 uses
the already fixed public corpus; it consumes no stream choices. Fixture identity
uses `TH("moriarty/fsyn/fixture/v1", raw32(cut_digest), UTF8(version),
u64LE(seed), UTF8(profile), UTF8(scale), UTF8(mix))`. The finite arguments in this
slice are `fsyn/2.0`, `both`, `S0`, `strawman`; any broader profile needs an explicit
encoding/version contract. These exact byte conventions have a literal unit test
for the stream and are exposed here for a future Rust port.

Every fixture is marked **unreleased**. The Rust `rtf generate` comparison,
`rtf verify`, Poseidon relation-digest computation, Quint forward/reverse replay,
two-host comparison and G0/G2 release are unobserved. Python tests establish
byte replay and hash-free consistency only. No P3/SRPOC-002, S3 performance,
ledger acceptance or complete Sherlock requirement is claimed.

## Existing benchmark consumers

The existing consumer accepts `sherlock-benchmark-workloads/2`, with seven fixed
cases, and `sherlock-benchmark-sample/1` samples. This slice changes neither schema
nor its financial, proof, stagenet, scalar-oracle or sample acceptance rules.
A future catalog/3 bridge must add FSYN provenance while preserving all /2 minima;
this fixture metadata is not a benchmark sample or performance result.

## Next implementation

P0 still needs source constant/layout/rule extraction, the planned standalone
Rust package with exact dependency pins, native-reference generation/verification,
revision-20 check coupling, and catalog/3 integration. P1 needs genuine S1/S3
production-shape event/state generation and independent Quint/crypto checks. P2
needs adversary families, mutation killing, two-host reproduction and the explicit
`fsyn-fixtures/1` release. Nothing in this first slice substitutes SHA256 for a
required Poseidon computation or changes a Fenrir rule.

Run new SDK-free tests from this directory with `python3 -B -m unittest discover
-s tests -v`; the retained-artifact test requires an explicit fresh
`FSYN_TEST_ARTIFACT` directory. Test execution requires the existing ordinary
resource class used in the source handoff. Test methods written by the source
authors are independent source expectations, not a separate non-author oracle.
