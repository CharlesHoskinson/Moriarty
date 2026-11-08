# FSYN first P0/S0 slice

This SDK-free Python implementation checks the frozen in-repository S0 golden and
reconstructs the PUBLIC_SYNTHETIC FRTF v3 corpus from it. Generation and tests read
only tracked files of this repository (SSRD-001, SSRD-005, SSRD-030); the T0 guard
`scripts/qa/check-no-fenrir.sh` (F17) keeps it that way. P0/G0 and fixture release
remain open.

## The S0 golden

`golden/s0/` holds the 659-member S0 corpus with its FRTF `MANIFEST.json`.
`golden/s0.json` pins the manifest bytes and SHA256, the corpus authorities
(`params-v3.json`, `workloads.json`) and the member count. Its `golden_digest`
covers the whole pin and is the fixture authority. Before any output is created,
FSYN checks the pin digest, the manifest, every member's SHA256, and that the tree
holds no unlisted member or symlink. A mismatch refuses with exit 2.

The golden was frozen on 2026-11-08 (task 1.9) from the public corpus bytes that
the retired B1 baseline used; `golden/s0.json` `provenance` records the pins it was
checked against. That source is history: no generation, test or check reads it.
The Moriarty-owned parameter set (`params/`, FSYN task 0.0) is not written yet.

## Generate and replay

```sh
python3 models/sherlock/rollup-synth/fsyn.py golden check
python3 models/sherlock/rollup-synth/fsyn.py generate --out NEW_OUTPUT_DIRECTORY --seed 0x5354524157 --scale S0 --profile both --mix strawman
python3 models/sherlock/rollup-synth/fsyn.py verify --fixture NEW_OUTPUT_DIRECTORY
```

The CLI takes no source cut and no golden root. `generate` refuses an existing
destination and unsupported seed, profile, scale or mix. Verification returns exit
1 when committed bytes or file membership differ; a stale or malformed golden and
invalid directory inputs return exit 2. This slice covers the frozen seed, both
profiles, minimal and all six full batches, including the b1/b2/b4/b6 workload
descriptions. It copies the complete 659-member corpus and its FRTF manifest. All
13 positive binary envelopes are independently encoded from their JSON views and
must match the golden bytes. Negative cases and I1 retain their golden bytes. No
VK, proof or wallet file is opened. An IO failure leaves partial output visible;
remove or retain that failed destination explicitly before selecting a new path.

The root `MANIFEST.json` covers every payload file with size and SHA256 and hashes
the canonical sorted row array. It excludes itself. `FIXTURE.json` records the
seed, mix, profile, scale, generator version and golden digest. Generation adds no
clock, hostname or absolute output path. The regeneration recipe is labelled as a
recipe; actual argv belongs in an execution receipt. `verify` regenerates from the
golden and compares complete membership and bytes, including both manifests and
metadata.

## B1 baseline

With seed `0x5354524157`, the fixture has 662 files and identity
`fecb674104c27eac2a1167c027818335ed861163c2ac88abf06ded7876f109d2`.
`baseline.json` pins its manifest SHA256
`aa863b04b796276612b30da06661cf7ac9e0eeb0354ed6691325536d32803277`
and content digest
`09c9ae0f4cb0ac381c8c23047fd1834f15a3b0ca47e930489494fbb085b270e8`.
The corpus member bytes are those of the retired baseline (fixture `44ca5071...`);
only the identity binding moved from the historical design cut to the golden, so
the generator version is `fsyn/2.1`. `baseline.json` `retired_baseline` keeps the old
digests. `scripts/qa/check-b1.sh` runs `golden check` and this directory's tests:
generation twice with byte comparison, the fixed digests, and stale-golden and
tamper rejection.

## B2 state replay

`scripts/qa/check-b2.sh` builds `crates/mori-recursion/examples/i1_replay.rs` and
runs `scripts/qa/verify-fsyn-replay.py`. It generates the exact B1 fixture, checks
its manifest against `baseline.json`, and replays the I1 trace with the
in-repository oracle (`crates/mori-recursion/src/rollup/i1`): 300 headers and 96
transactions from genesis, with Poseidon state digests, Merkle transitions and
signatures recomputed and each result compared to the frozen expected state. It
then binds the six-batch chain of `i1/batches.json` to the replayed states and
record hashes. The check changes a frozen expected state and a transaction
signature and requires both to fail inside the oracle. Run directories under
`target/sherlock-b2/` keep the logs and `receipt.json`.

The oracle is Moriarty code that the Sherlock witness builder also uses; it is not
a separate implementation, and the receipt says so. The in-repository Quint oracle
models of SSRD-015 (FSYN task 0.0a) are not written yet. Envelope encoding is
checked by B1. No recursive-proof or ledger acceptance follows from B2.

## Encoding and scope

`frtf.py` implements the actual Rust envelope layout, canonical field encodings,
bounded counts, step links/order, complete header-close coverage, Profile-B slot
and covenant-reference association, and the tagged envelope digest. Its SHA256
is only FRTF's `TH`: u64 little-endian UTF-8 tag length, tag, then ordered parts.
It does not implement Poseidon, authenticate history, check signatures or verify
proofs. JSON root/commitment values remain the golden values.

The random stream is exactly `TH("moriarty/fsyn/rng/v1", u64LE(seed),
UTF8(stream_name), u64LE(counter))`, starting at zero, with a persistent byte
buffer so chunk boundaries do not change output. The planned stream names are
schedule, mix, amounts, committee, celestia, forced, escape and mutation. S0 uses
the already fixed golden corpus; it consumes no stream choices. Fixture identity
uses `TH("moriarty/fsyn/fixture/v1", raw32(golden_digest), UTF8(version),
u64LE(seed), UTF8(profile), UTF8(scale), UTF8(mix))`. The finite arguments in this
slice are `fsyn/2.1`, `both`, `S0`, `strawman`; any broader profile needs an explicit
encoding/version contract. These exact byte conventions have a literal unit test
for the stream and are exposed here for a future Rust port.

Every fixture is marked **unreleased**. Generator metadata describes the Python
generation checks; separate B2 receipts record the oracle replay. Quint
forward/reverse replay, two-host comparison and G0/G2 release remain unobserved.
Python tests establish byte replay and hash-free consistency only. No
P3/SRPOC-002, S3 performance, ledger acceptance or complete Sherlock requirement
is claimed.

## Existing benchmark consumers

The existing consumer accepts `sherlock-benchmark-workloads/2`, with seven fixed
cases, and `sherlock-benchmark-sample/1` samples. This slice changes neither schema
nor its financial, proof, stagenet, scalar-oracle or sample acceptance rules.
A future catalog/3 bridge must add FSYN provenance while preserving all /2 minima;
this fixture metadata is not a benchmark sample or performance result.

## Next implementation

P0 still needs the Moriarty-owned parameter set (task 0.0), the planned standalone
Rust package with exact dependency pins, native-reference generation/verification
and catalog/3 integration. P1 needs genuine S1/S3 production-shape event/state
generation and the in-repository Quint oracle models. P2 needs adversary families,
mutation killing, two-host reproduction and the explicit `fsyn-fixtures/1` release.
Nothing in this slice substitutes SHA256 for a required Poseidon computation.

Run the SDK-free tests from this directory with `python3 -B -m unittest discover
-s tests -v`; the retained-artifact test requires an explicit fresh
`FSYN_TEST_ARTIFACT` directory.
