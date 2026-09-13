# The developer loop for the Moriarty surface language

Design proposal, 2026-09-13. Companion to
`moriarty-surface-language-2026-09-13.md`, whose syntax (`party`, `pays`,
unit literals, frame inference) is assumed here. This document owns the
tooling: layout, build, inner loop, errors, tests, evidence, the path to
chain, and toolchain pinning.

The question every section answers is the one the brief asks: which timeless
practice does this serve? Each part ends with one line naming it. Anything
that served only novelty was cut.

## 0. Where today's loop actually stands

Measured from the repository, not from memory:

- Two packages, two runners (`node --test` plain and `--test-reporter=tap`),
  ten `npm` scripts that are tests, and one that is a build.
- The gated ledger suites read `MORIARTY_SWAP_BUILD_RECEIPT`,
  `MORIARTY_LOAN_BUILD_RECEIPT` and `MORIARTY_CUSTODY_ARTIFACTS` from the
  environment and fail closed without them. The paths point outside the
  repository into `~/.local/state/moriarty/`.
- `src/diagnostics.ts:7` builds every error as
  `{code, message: code, primarySpan, ...}`. The span is a byte range that is
  already correct; the message is the code repeated. Nothing renders a line,
  a caret, or a fix.
- `formal/k/toolchain.lock.json` pins K by version, Nix store path, NAR hash
  and per-executable SHA-256. That is the right shape for a lockfile and it
  exists for exactly one of the three toolchains.
- `compiled-lifecycle/binding-01.json` pins Compact 0.31.1 (language 0.23.0,
  runtime 0.16.0) and records that 0.34.0 is available and deliberately not
  used, because the pin is bound into retained artifacts.
- `test-evidence.json` is a hand-assembled record of argv, cwd, exit code,
  test counts and source hashes. It is the right record; it should be
  produced by the tool, not by a person.
- Every stage of `language-to-ledger-lifecycle/tasks.md` ends with an
  independent audit whose verdict is a JSON file under `deliverables/`.

The design below keeps every one of those properties and makes each one
cheap.

## 1. Project layout and the one command

`moriarty new loan` produces:

```
loan/
  moriarty.json          manifest: what this project is and what it pins
  moriarty.lock.json     lockfile: executable hashes of the pinned toolchains
  src/loan.mori          the contract
  tests/loan.test.mori   scenarios: state in, action, effects and work out
  golden/                elaborated Core for each source, hash-pinned
  evidence/              receipts written by the tool; committed, never edited
  .gitignore             target/ only
```

**Manifest format: JSON, closed schema.** Rust developers expect TOML and
TypeScript developers expect JSON; the tie is broken by the repository, which
canonically encodes and hashes JSON everywhere and rejects unknown fields in
every closed schema it owns. A manifest that the tool hashes into receipts
must be canonicalisable, and the codec for that exists (`canonicalEncode`).
Unknown fields fail, as in `moriarty-financial-record/1`.

```json
{
  "schema": "moriarty.manifest/1",
  "name": "loan",
  "profile": "moriarty-financial-agreement-source/6",
  "toolchain": { "moriarty": "0.1.0", "compact": "0.31.1", "k": "7.1.337" },
  "networks": { "preview": { "faucet": true } }
}
```

The manifest pins versions. The lockfile pins bytes: for each toolchain, the
executable paths and SHA-256 digests, in the shape `toolchain.lock.json`
already uses for K. `k` is optional; the other two are not.

**The one command is `moriarty test`.** It type-checks, elaborates every
source to Core, compares each elaboration against its golden, runs every
scenario, and writes an evidence record. It needs no network, no Docker and
no retained artifact. `moriarty check` is the subset an editor runs on save.
`moriarty build` is `test` plus writing the elaborated Core and its hash to
`target/` with a build receipt.

Practice served: one command builds and tests it, from a fresh clone, and the
manifest says what it will produce before it runs.

## 2. The inner loop

Three commands, each under a second on the examples in the repository, each
refusing to touch a network:

```
moriarty check                       parse, resolve, type, elaborate, golden-compare
moriarty run originate --state s.json  evaluate one action against a state file
moriarty sim tests/loan.test.mori    run a scenario file, print the trace
```

`run` and `sim` use the evaluator that `cli.ts simulate` already wraps. The
addition is the output format: every action line reports the work it
consumed against the lifetime budget, because the work meter is observable in
results (`workRemaining` in the expression result; `WORK_EXHAUSTED` is a real
rejection) and a developer who first sees it at admission has been failed by
the tool.

```
originate   ok   work 9/64   phase 0→1   Lender -100 Cash  Borrower +100 Cash
accrue      ok   work 4/64   Loan1.accrued +10 Cash
repay       FAIL work 6/64   GUARD_FAILED at src/loan.mori:41  "payer must be Borrower"
```

The work figures are measured by evaluation, not predicted. The repository
has exact-work tests per action ("exact work for originate") and no static
work bound; the tool prints what it measured and does not invent a bound.

Nothing in the inner loop assumes a circuit backend. Moriarty's core has no
bit-level or circuit definition anywhere in the repository (the ZKIR that
appears in `binding-01.json` is Compact's output for one originate circuit,
downstream of Moriarty, not a semantics for Moriarty's Core). So there is no
`moriarty prove` and no "constraint count" column. When a lower definition
exists, the work column gains a neighbour; until then, printing one would be
a fabricated number.

Practice served: the fast inner loop needs no network, and the cost you will
be charged is visible while you are still typing.

## 3. Errors

Keep the existing codes. They are stable identifiers that tests, audits and
the K corpus match on; renumbering them Rust-style (`E0412`) would break
every pinned expectation for nothing. Give each code what it lacks: a
sentence, a rendered span, a note, and, when the fix is mechanical, the fix.
The record stays the same `Diagnostic` shape with `message` no longer equal
to `code`, plus a `help` field; `--json` emits it raw for editors.

Line and column are derived from the byte span at render time. Byte offsets
remain the identity in the record, because the hash-bound artifacts already
carry `spans: [{startByte, endByte}]` and an auditor comparing an error
against a receipt must not depend on line-ending normalisation.

Three real examples, in the surface syntax of the companion document.

**Unit mismatch.** Today `checker.ts` reports this as `TYPE_MISMATCH`. The
surface compiler splits out `UNIT_MISMATCH`, because the fix differs.

```
error[UNIT_MISMATCH]: cannot add Cash to Shares
  --> src/loan.mori:27:24
   |
27 |   let total = principal + vault.shares(Borrower);
   |               ---------   ^^^^^^^^^^^^^^^^^^^^^^ Shares
   |               |
   |               Cash
   = note: Amount arithmetic requires both sides to carry the same unit vector
   = help: convert with an explicit price: `vault.shares(Borrower) * price`
```

**Missing payer.** The grammar makes the payer mandatory, so this is a parse
error, and the parser knows exactly what is absent and who the candidates
are.

```
error[FLOW_PAYER_REQUIRED]: a flow must name who pays
  --> src/loan.mori:33:3
   |
33 |   pays 100 Cash to Borrower;
   |   ^^^^ no payer before `pays`
   = note: a transfer with no payer draws on the submitting transaction's
           offer, which is the defect the 5.1a audit found; it is not writable
   = help: declared parties in scope: Lender, Borrower
           `Lender pays 100 Cash to Borrower;`
```

**Partial projection not handled.** `ProjectSome` and `ProjectVariant` fail at
runtime (`OPTION_NONE`, `VARIANT_CASE`). The surface language has no unwrap
spelling; a projection out of an Option or variant must sit under a `match`
or after a `require` that establishes the case. The checker reports the
uncovered case at the projection site.

```
error[PARTIAL_PROJECTION]: `grace.value` is not defined when `grace` is None
  --> src/loan.mori:52:19
   |
52 |   let cutoff = at + grace.value;
   |                     ^^^^^^^^^^^ projection out of Option<Time>
   |
   = note: no enclosing `match grace` or `require grace is Some`
   = help: either
             require grace is Some, "grace period must be set";
           or
             match grace { Some(g) => at + g, None => at }
```

Practice served: errors point at the source, say what is wrong in a
sentence, and hand you the fix when there is only one.

## 4. Testing and evidence

**A test is a scenario in Moriarty.** Expected values are written by the
author as literals, which keeps the README's rule that expected amounts come
from independent arithmetic and never from the evaluator under test.

```
scenario first_period for Loan {
  given Lender holds 20_000 Cash, Borrower holds 0 Cash;
  originate();                expect Borrower holds 100 Cash, work 9;
  accrue(at: t0 + 60s);       expect Loan1.accrued == 10 Cash;
  repay(30 Cash) by Lender;   expect fail GUARD_FAILED;
  repay(30 Cash) by Borrower; expect Loan1.outstanding == 80 Cash;
}
```

`expect fail CODE` is a first-class expectation because half the suite
today is negative cases and the fault matrix. `work N` is an expectation
because exact-work tests already exist and should not need JavaScript.

**A golden pins a desugaring.** For every `.mori` under `src/`,
`golden/<name>.core.json` holds the elaborated Core. `moriarty test` fails if
the elaboration differs, showing the diff. `moriarty golden --accept` rewrites
it and prints the diff it accepted. Since the elaborated form, not the surface
form, is the artifact of record for hashing and admission, the golden diff is
the unit of review for any change to sugar. This is Cairo's
`plugin_test_data` practice, which the survey recorded as the one to steal.

**A passing run produces evidence without being asked.** Every `moriarty
test` writes `evidence/test-<utc>.json`:

```json
{
  "schema": "moriarty.test-evidence/1",
  "argv": ["moriarty", "test"], "cwd": "/home/dev/loan", "exitCode": 0,
  "toolchain": { "moriarty": {"version": "0.1.0", "sha256": "…"} },
  "sources": { "src/loan.mori": "sha256:…", "golden/loan.core.json": "sha256:…" },
  "scenarios": { "passed": 4, "failed": 0, "skipped": 0, "rows": [ … ] },
  "work": { "originate": "9", "accrue": "4", "repay": "6" }
}
```

Those are the fields `test-evidence.json` carries today by hand. `moriarty
evidence verify <file>` re-hashes the named sources and toolchain and
reports whether the record still describes the tree, which is the check the
README makes in prose ("do not treat an older hash as a run of later
source").

**Audits are a tool artifact, not a folder convention.** `moriarty audit
bundle` writes a self-contained directory: sources, goldens, the evidence
record, the manifest and lockfile. An auditor, on any vendor, runs `moriarty
evidence verify` inside it and writes `audit-<name>.json` with a verdict and
the evidence hash it judged. `moriarty audit verify` checks that the verdict
names the hash of the evidence that is actually present. Cross-vendor
independence is the project's character; the tool's job is to make the
bundle and the check one command each, so that skipping the audit is more
work than doing it.

Practice served: the rigorous thing is the default output of the command
you were already running.

## 5. The path to chain

Four stages. Each writes a receipt into `evidence/`, and each refuses to run
without the previous receipt matching the current sources.

```
moriarty build                  → evidence/build.json     (Core hash, profile, toolchain)
moriarty compile                → evidence/compile.json   (Compact 0.31.1 output, skipZk flag)
moriarty run --docker           → evidence/docker.json    (local lifecycle, native readback)
moriarty deploy --network preview → evidence/preview.json (contract id, tx ids, effects)
```

Refusals are specific:

```
$ moriarty compile
error[STAGE_RECEIPT_STALE]: evidence/build.json describes src/loan.mori at
  sha256:9f3c…, but the file now hashes to sha256:41ab…
  = help: run `moriarty build` first
```

`compile` records `skipZk` exactly as the custody build does today, because
a skip-zk compile is the honest fast path and the receipt must say so.
`run --docker` needs the Docker engine and is the first stage that is not
sub-second; it is also the first that can reach a wallet, so it is the last
stage that runs without a network flag.

**Preview is cheap on purpose.** Its tokens are worthless, so `deploy
--network preview` requires only the Docker receipt, funds from the faucet
the manifest names, and asks nothing else. The caution belongs at mainnet:
`deploy --network mainnet` additionally requires a verified `audit-*.json`
naming the Docker receipt's hash and a Preview receipt for the same Core
hash. No mainnet path exists in the repository today; this document
specifies the gate, not the deployment.

**Retained artifacts stop being environment variables.** A test that needs a
prebuilt artifact declares it:

```
scenario finalized_state needs receipt "swap-build" { … }
```

The lockfile maps the name to a content hash; `~/.local/state/moriarty/` is
a content-addressed store the tool resolves. When the artifact is absent the
scenario is reported `skipped: missing receipt swap-build (sha256:3789…)`,
never as a failure that looks like a real one. Under `--strict`, which CI and
audit bundles use, a skip is a failure. This preserves fail-closed where it
matters and removes the false failure from the loop where it does not.

The sandbox-permissions false failure has the same treatment: the tool
distinguishes `EACCES`/`EPERM` on its own paths from an assertion failure and
reports the first as `ENVIRONMENT`, not as a failed test.

Practice served: each stage refuses to run on stale input and says which
input; the cost of a mistake rises only where a mistake costs money.

## 6. Toolchain pinning and upgrade

The manifest pins versions; the lockfile pins executable digests; every
receipt records both. `moriarty toolchain install` populates the store from
the lockfile and fails if a digest does not match. `moriarty check` fails
with `TOOLCHAIN_LOCK_MISMATCH` if manifest and lockfile disagree, so a
hand-edited version cannot slip through.

An upgrade is one command that does a lot of honest bookkeeping:

```
$ moriarty toolchain upgrade compact 0.34.0
updated moriarty.json      compact 0.31.1 → 0.34.0
updated moriarty.lock.json compactc sha256 a1c0… → 7e29…
stale   evidence/compile.json  (compiled under 0.31.1)
stale   evidence/docker.json   (depends on compile.json)
stale   evidence/preview.json  (depends on docker.json)
kept    evidence/build.json    (Core is toolchain-independent of compact)
next    moriarty compile; moriarty run --docker; moriarty audit bundle
```

Nothing is deleted; stale receipts stay in `evidence/` as history, marked
stale in an index, exactly as `binding-01.json` today keeps the note that
0.34.0 was available and not used. The upgrade is therefore a reviewable
commit that changes two files and invalidates named receipts, and the
required re-audit is listed rather than remembered.

Practice served: the build is reproducible from the lockfile, and an upgrade
is a deliberate, diffable act with a printed list of what it invalidates.

## 7. Where this disagrees with the brief

The brief lists `moriarty new` first and asks what the single most-run
command is. The most-run command is not `new` and not `build`; it is
`test`, and the design puts the evidence writer there, because a receipt
produced only by a rarely-run command is a receipt that will be missing when
the audit needs it.

The brief also implies the gated suites' fail-closed behaviour is a problem.
It is half a problem. Failing closed under `--strict` is correct and stays;
failing with an error indistinguishable from a real defect in the inner loop
is the actual defect, and that is what section 5 fixes.

## 8. A complete session

Outputs below are the designed format; no `moriarty` binary exists yet, so
none of these were captured.

```
$ moriarty new loan && cd loan
created loan/ (profile moriarty-financial-agreement-source/6)
  moriarty.json  moriarty.lock.json  src/loan.mori  tests/loan.test.mori  golden/  evidence/
toolchain: moriarty 0.1.0 (installed), compact 0.31.1 (installed), k 7.1.337 (optional, absent)

$ cat > src/loan.mori <<'M'
agreement Loan {
  party Lender; party Borrower; asset Cash;
  obligation Loan1 : Loan;

  action originate() {
    pays 100 Cash to Borrower;
    Loan1 = originate 100 Cash from Lender to Borrower
             at 10% per 60s from t0 cap 110 Cash;
  }
  action accrue(at: Time) { Loan1.accrue(at); }
  action repay(amount: Cash) by Borrower {
    Borrower pays amount to Lender;
    Loan1.repay(amount);
  }
}
M

$ moriarty test
error[FLOW_PAYER_REQUIRED]: a flow must name who pays
  --> src/loan.mori:6:5
   |
 6 |     pays 100 Cash to Borrower;
   |     ^^^^ no payer before `pays`
   = help: declared parties in scope: Lender, Borrower
           `Lender pays 100 Cash to Borrower;`
1 error; no evidence written

$ sed -i '6s/pays/Lender pays/' src/loan.mori && moriarty test
elaborated  src/loan.mori → golden/loan.core.json   (new golden, 1 file accepted on first run)
scenario first_period
  originate            ok   work 9/64
  accrue(at: t0+60s)   ok   work 4/64
  repay(30) by Lender  ok   expected fail GUARD_FAILED
  repay(30)            ok   work 6/64   Loan1.outstanding 80 Cash
4 passed, 0 failed, 0 skipped
evidence/test-20260913T141203Z.json  sha256:c04e…

$ moriarty build
core     sha256:9f3c…   profile 6   work budget 64
evidence/build.json

$ moriarty compile
compactc 0.31.1 (language 0.23.0, runtime 0.16.0)  skipZk: true
artifacts: target/compact/contract-info.json, target/compact/originate.zkir
evidence/compile.json  bound to build sha256:9f3c…

$ moriarty run --docker
midnight local (docker) ... up
originate  accepted   native readback: Borrower 100 Cash
accrue     accepted
repay 30   accepted   native readback: Lender 30 Cash, Loan1 outstanding 80
evidence/docker.json  bound to compile sha256:e88d…

$ moriarty deploy --network preview
wallet: faucet-funded (preview tokens)
deployed contract 0x5b1f…   originate tx 0x7a…  accrue tx 0x2c…  repay tx 0x91…
effects match evidence/docker.json
evidence/preview.json

$ moriarty audit bundle
wrote audit/loan-20260913T1419Z/  (sources, goldens, 6 receipts, manifest, lock)
an auditor runs: moriarty evidence verify audit/loan-20260913T1419Z
```

Twelve commands, one error, one fix, and every stage left a receipt that
`moriarty evidence verify` can check on a machine that has never seen this
repository.
