## Verdict

**ACTUAL K5 SETUP PREREQUISITE CLOSED FOR K6 SOURCE PREPARATION ONLY — no execution authority, no proof acceptance.** The completed exact K5 run independently cross-checks against its raw public logs, not merely root PASS claims. This fresh muse-wonder-1.0 HIGH reviewer did not author the candidate, operator, gates, setup or result, executed nothing, and casts no operator votes (`positions` empty).

## Actual cross-checks (raw logs first)

- **Identity/counters/exits:** All ten actual `.exit` files opened read `0` (P1,P0,Q5,K5a,K5b,K5M,K5g,H5,Z51 in logs/ plus Z52.exit; hash `9a271f2a…`), all ten journals are terminal start/end records, and in-view `Exit status: 0` appears in K5a/K5b/K5M logs. This is the step result, not the wrapper scripts that always exit 0 (`K5-RAW-OBSERVATION.json:19`). One native setup invocation, zero retries.
- **P1/P0:** `P1.log:1` records preconditions PASS before mkdir against authorization `ab376625…`, tally `1378425…` and preconditions `56ad3904…`; root P1 stdout is empty. `P0.log:21-31` shows the guarded copy verified as `d446ab0c0dececae719fe6583532f6c2a4285f67fbc89893d61251adff190902`, 3941352 bytes mode 555, with the four inputs mode 444 — no root direct protected-tree access is claimed.
- **Probe:** `Q5.log` shows CapInh/Prm/Eff/Bnd/Amb all zero, NoNewPrivs 1, distinct user/pid/net/ipc/uts namespaces, loopback only, `/task/bin` and `/task/in` read-only, only `/task/out` and `/task/keys` writable, tmpfs `/tmp`, and no host/toolchain paths — probed before any compile.
- **Compile-only descriptors:** K5a descriptor is byte-identical to stage-C K4b (`7bae4fb1ded96b8c767acfabc135bb54aafab0c8a13a648686da32d38696e959`) and K5b to K4c (`8cf25094747b774bfda66d91c76999c6c3c6e845313cea6363209fe0ed67192a`), with empty out/keys at that stage. K5a gives R1CS `e458153e583e0a817fbab08f24f4ba16465e459969c1f8cd4e13b801e82694c0`, 308790 constraints, 3 instance variables, 2 public inputs, predicate `0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd`.
- **K5M setup:** One `mori-k2 setup` for invoice/partial only (source `main.rs:167` single `OsRng`, `setup.rs:87` one Groth16 generation, generator/OsRng excerpts pinned). GNU time in the full raw log and H5 report: user 23.69 s, sys 0.62 s, wall 24.35 s, peak RSS 814200 KiB — well within the reviewed 8 GiB AS / CPU 3600 + kill 60 caps, behind the MemAvailable/disk gates. No retry or cap change.
- **K5g gate:** PASS with zero identity, custody and prediction problems. Proving key is metadata-only in public evidence: 98715456 bytes, mode 0600, nlink 1, keys dir 0700, SHA-256 `cadcf470…`; setup record 3926 bytes, SHA-256 `86b588fe…`. Public metadata pins only — no key, VK or setup-record body is public in this mirror and none was followed to `/w`. R1CS and predicate match K5a/stage-C above; VK BLAKE2b-256 `d19d99a4…` is recomputed in-gate.
- **Z51/Z52:** Z51 repeats the same hashes/pins, shows exactly one keys entry and one out entry, unchanged protected-tree evidence and custody only by guarded hashes, with keys retained unread and no deletion. Z52 freezes 32 logs/receipts rows in `stage-K5-final.json`; the root public manifest is 45 rows at `bceacff30d81b722464b13796aa6912bcdeb3172288ea40c9f94b79dd65e77ef`, and all 45 rows were read in this review.

## Findings on known residuals

F-K5M-01, F-K5M-02 and F-K5M-03 (detailed in `findings`) do not block this actual predicate: the actual authorization/pre-launch records contain only their stated keys and were hash-verified by P1/P0 before creation; the actual K5g exact-keys result is exactly one key with no extra entries for Z51 to classify; and the actual freeze path set is logs/receipts only with matching hash pins, without claiming symlink hygiene stronger than the mirror can show. The operator helper does not runtime-hash all top-level docs — root rehashed all 26 before P1 — and the old false Gemini R2 review remains unadopted; actual voices are the seven exact clauses as tallied.

## Scope, trust and first blocked boundary

Root source bindings are provenance only: `consumerCanCheckVkToMatrixLink: false`. No PK deserialization, embedded-VK extraction, curve subgroup checks, PK–VK matching, Groth16 proving, independent pairing verification or ledger work has occurred. Under `single-party-test-setup/1` the setup controller could forge invoice/partial proofs, so this is test-only evidence, not a ceremony or production setup. Actual review scope is the first genuine setup only, not a whole-language or arbitrary-witness security theorem and not the full financial frame.

**First blocked boundary:** K6 must newly and exactly review an operator and crypto authorization for PK deserialization/subgroup/PK–VK matching and a real prove plus independent pairing verify. The existing K5 authorization never allows K6.

## Custody disposition (explicit, DV-K2-1-R)

Because the actual setup is ready and custody is clean, this review **names the retaining followup `current-cardano-k2-k6-proof-roundtrip-20261016`** (future SOURCE/plan only, no authorization or run yet) and retains this run's `invoice-partial.pk` and setup record **in place, unchanged**, for that followup under DV-K2-1-R, instead of deletion at review close. No root direct read, copy, deletion, publication or proving is permitted until that followup's new exact reviewed operator and crypto authorization. This review deletes nothing and grants no future execution.

All 20 UTXO families, ACTUS/DeFi/general obligations, recursion, private handoff, bounded multi-parent PCD, universal certificates, authenticated ledger and real Preprod remain open, with no percentage of full compatibility claimed.
