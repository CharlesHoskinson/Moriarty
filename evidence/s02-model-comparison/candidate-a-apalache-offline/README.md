# Candidate A direct offline Apalache attempt

Experiment observation, 2026-09-05. This supersedes the earlier file-format
preflight limitation, not the absence of a verification result.

Root inspected installed Quint `cliCommands.js:489–510`: JSON target writes the
single flattened main module to stdout. `--out` separately serializes the whole
procedure stage and is not the same artifact. Root checked Apalache 0.56.1's
[file-source dispatch](https://github.com/apalache-mc/apalache/blob/v0.56.1/mod-infra/src/main/scala/at/forsyte/apalache/infra/passes/options.scala#L308):
Quint JSON uses the compound `.qnt.json` suffix. Plain `.qnt` is refused before
parsing; plain `.json` selects a different IR parser. The investigating native
agent initially repeated the wrong `.qnt` recommendation; root challenged it
against the retained refusal, and the source-based correction preceded execution.

## Generated input and reproduction

```bash
quint compile specs/quint/s02/candidate_a_harness.qnt --main=candidate_a_harness --target=json --invariant=coreTraceSafety --verbosity=0 > /tmp/moriarty-quint-verify.J18Azu/candidate-a-flat.qnt.json
/home/charl/.quint/apalache-dist-0.56.1/apalache/bin/apalache-mc --out-dir=/tmp/moriarty-quint-verify.J18Azu/apalache-out check --init=q::init --next=q::step --inv=q::inv --length=4 --no-deadlock /tmp/moriarty-quint-verify.J18Azu/candidate-a-flat.qnt.json
```

The generated file is retained locally, not committed: 93,413,860 bytes,
SHA-256 `8c23899f5a4c43da7d5d5c6e4a68d12279cc0b4397e33c6979221928457389c0`,
one module named candidate_a_harness. Compile exited 0. Paths and tool stages
inside generated JSON can affect byte reproducibility; reproduce semantics from
the pinned source closure and record newly generated artifact bytes explicitly.

Candidate A source closure is unchanged from the semantic-unit handoff's
`2f53817de7dad8553b5ecb43746b676006a2d28e`: candidate_a_harness, types, programs,
core, projection, effects and observations. Current unrelated adapter files are
not imported by this check. This command uses the direct Apalache CLI, not a
successful `quint verify` receipt, and uses neither TLC nor a listening server.

## Four-GiB result

Default maximum Java heap was 4,294,967,296 bytes. Apalache parsed the Quint
input, completed Snowcat typechecking and reached InlinePass. It then reported
Java heap exhaustion and exit 255 after 275.18 seconds. Exact tool command log
and detailed log are retained beside this README. The JVM also emitted a
sun.misc.Unsafe deprecation warning on stderr; that warning was not a semantic
failure and is disclosed separately from the preserved Apalache log.

No invariant check result or counterexample was produced. A resource failure
before SMT checking does not establish an architecture stop or semantic defect.
The depth requested was four; deadlock checking was disabled for the genuinely
terminating trace harness. Neither setting turns an unfinished check into proof.

An identical-input retry with an 8-GiB heap was launched separately. Its terminal
result must be recorded separately when observed; this receipt makes no claim
about its eventual result or process liveness. Future checking still needs to
establish the requested S02 bounds and full candidate integration properties.
