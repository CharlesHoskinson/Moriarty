# K5 operator: literal step list

PROPOSED SOURCE. Nothing here has run and nothing here authorizes a run. Where a script and a clause disagree, the clause governs and the disagreement is a defect for the fresh non-author reviewer to report. This R2 candidate was authored by Meta Muse Wonder 1.0 as a narrow repair of the frozen R1 candidate; it is unreviewed and is not its own reviewer or executor.

**Invocation of every step, exactly once and in the order below:**

```
env -i PATH=/usr/bin:/bin HOME=/nonexistent LC_ALL=C TZ=UTC /usr/bin/bash --noprofile --norc <OWN>/steps/<ID>.sh < /dev/null
```

Names fixed by these scripts:

| Name | Path |
|---|---|
| `BASE` | `/home/charl/moriarty-eutxo-validation-2026-10-12` |
| `M5` (future root materialization of this packet) | `BASE/current-cardano-k2-k5-setup-operator-materialized-20261016` |
| `OWN` | `M5/operator/request` |
| `RR5` (future root records, absent before root creates them) | `BASE/current-cardano-k2-k5-root-records-20261016` |
| `M` (K4H materialization) | `BASE/current-cardano-k2-k4h-hostile-materialized-20261016` |
| `TK` (K4H run directory) | `BASE/current-cardano-k2-k4h-hostile-local-execution-20261016` |
| `C` | `BASE/current-cardano-k2-g1-materialized-candidate-20261015` |
| `TOLD` (stage L/B/C run, read only) | `BASE/current-cardano-k2-ordinary-local-execution-20261015` |
| `T5` (fresh, absent before P1) | `BASE/current-cardano-k2-k5-single-party-test-setup-20261016`, `W5 = T5/w` |
| Source origin of this candidate | `/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k5-setup-operator-muse-source-r2-20261016/candidate` (root control record only; never an operator path) |

`M5`, `RR5` and `T5` must be absent before root placement and T5 creation. Every script exits 0. The step result is `T5/logs/<ID>.exit`, or `T5/Z52.exit` for Z52. A missing exit file means the step failed. A step refuses to run twice. A step with an unmet prerequisite writes `<ID>.skipped`. H5 is non-gating; Z51 and Z52 run whenever K5 started, even after an abnormal stop, subject to their own once-only guards.

## Root before P1

See `ROOT-PLACEMENT-INSTRUCTIONS.json` in the packet root. In outline: materialize this packet byte for byte at `M5`; create `RR5` with `K5-OPERATOR-PINS.sha256`, `K5-MATERIALIZATION-PIN.sha256`, `K5-PRECONDITIONS.json`, `K5-AUTHORIZATION.json` and `K5-PRE-LAUNCH.json` in the exact schema that file defines; obtain a fresh non-author Gemini source and effect review of the materialized bytes with no blocking finding; record two substantive agreeing positions on each of K5-1, K5-2, K5-4M, K5-5, K5-7, K5-8 and K5-8A; confirm the K4H actual-result review closed (KH3 10/10 ok, KH6 PASS, binaries unchanged, no relation-level security finding) and record its report path and hash in `K5-PRECONDITIONS.json`; confirm `T5` absent; only then record the exact crypto authorization naming DV-K2-1-R. A missing, malformed or placeholder value in any root record refuses in P1 and P0 before any effect. This source produces no authorization.

## Steps

| # | ID | Where | Prerequisites | CPU/wall s | Gate / effect | Failure class |
|---|---|---|---|---|---|---|
| 1 | P1 | host, unwrapped | RR5 complete; T5 absent | - | root preconditions via `k5_preconditions.py p1`; then one atomic `mkdir T5` tree, `w/keys` mode 0700 | BLOCKED(prerequisite) on refusal; creates nothing |
| 2 | P0 | host | P1 | 300/600 | operator and public-input pins; `k5_preconditions.py p0` into `receipts/P0-root-pins.sha256`; host copy of the K4H release `mori-k2` verified against `KH-binaries-pre.sha256`, mode 0555; two sources and two selectors into `w/in` mode 0444; `out`/`keys` empty; disk and memory gates | BLOCKED(identity), BLOCKED(hygiene) or BLOCKED(resource) |
| 3 | Q5 | host + `sbx_run` | P1 P0 | 120/300 | full probe of the K5 run view: namespaces differ, loopback only, no host task root, read-only `bin`/`in`, writable `out`/`keys` only, empty `out`/`keys`, synthetic `/dev`, stdin `/dev/null`, no toolchain mount | BLOCKED(capability) |
| 4 | K5a | `sbx_run` + in-view time | P1 P0 Q5 | 600/900 | `mori-k2 compile` invoice/partial; stdout byte-identical to stage-C `K4b-descriptor.json` | K5-5 |
| 5 | K5b | `sbx_run` + in-view time | P1 P0 Q5 | 600/900 | `mori-k2 compile` reserve/allocate; stdout byte-identical to stage-C `K4c-descriptor.json` | K5-5 |
| 6 | K5M | `sbx_run` + in-view time | P1 P0 Q5 K5a K5b | 3600/3600 (+60 kill) | ONE `mori-k2 setup` of invoice/partial only, OsRng inside the driver, AS 8 GiB, core 0, nice 10; writes `/task/out/invoice-partial.setup.json` and the 0600 `/task/keys/invoice-partial.pk`; stdout summary to `receipts/K5M-summary.json`; no retry, no cap change | K5-5; resource and abnormal classes preserved |
| 7 | K5g | host `python3 -I` | K5M | 300/600 | strict gate `data/k5g_gate.py`: record under 1 MiB, exact format, trust profile, notice and driver, relation field-for-field equal to the stage-C descriptor, 488-byte VK with `0300000000000000` at bytes 336..344 and recomputed BLAKE2b-256, observed R1CS digest `e458153e...`, key regular non-link 0600 with size and SHA-256 equal to the record, exactly one file in `keys`. Records metadata only, never key bytes | STOP-CUSTODY, STOP-IDENTITY, FAIL-PREDICTION or PASS per K5-5 |
| 8 | H5 | host | a Q5, K5a, K5b or K5M log exists | 60/120 | in-view time and abnormal-line extracts; K5M summary and K5g receipt copied as records | non-gating |
| 9 | Z51 | host | K5 started | 300/600 | final hygiene: operator pins re-verified, binary and inputs re-verified, `out`/`keys` listed behind link guards, key path, mode, size and SHA-256 recorded only when the custody guard permits; no deletion | BLOCKED(hygiene) |
| 10 | Z52 | host, unwrapped | K5 started | - | freeze of `logs` and `receipts` only to `receipts/stage-K5-final.json`; never `w`, `bin`, `keys` or key material | BLOCKED(hygiene) on failure |

**Every wrapped step runs under:** `nice -n 10 timeout --kill-after=60 <wall> prlimit --as=8589934592 --cpu=<cpu> --core=0`. K5M is the single experiment attempt: 3600 s CPU and wall plus the 60 s kill grace, exactly as clause K5-4M states.

**After Z52:** a fresh independent actual-result review of the full public logs and receipts. `T5/w`, and in particular `T5/w/keys`, stays unread and unchanged until that review closes, apart from the guarded metadata hashing in K5g and Z51. No key is deleted in this run. Deletion is a later, separately reviewed exact custody step at review close unless that review names a follow-up that retains the key.

**Never, in any step:**
- `mori-k2 prove`, `verify`, `inspect`, any pairing, subgroup check or K6 work;
- any RNG other than the driver's `OsRng`, a seed, a deterministic fallback, or reuse of an existing key;
- publishing, copying or printing proving-key, VK-hex-derived custody, SRS, wallet, cache or private material (the public setup record itself is written only inside `w/out` and is hashed, never copied into receipts);
- network, wallet, ledger, Cardano, Preprod, Midnight, signing or funds actions;
- a retry, a second setup, a cap change, or an alternate isolation mechanism;
- writes to `C`, `TOLD`, `TK`, `M`, `M5` or any root record.
