# Candidate A Apalache preflight: no verification result

Repository observation on 2026-09-05: the root attempted bounded verification
of the actual swap harness with Quint 0.32.0 and Apalache 0.56.1, maximum four
steps and coreTraceSafety. No TLC backend was invoked.

The automatic gRPC server opened a wildcard listener on port 8822. The tool
also printed a protobuf dependency security warning. Root stopped only its
owned Quint process 1066126 and child server 1066622 with SIGTERM; terminal
exit was 143. A subsequent socket check showed no listener on 8822. This is
an operator-interrupted attempt, not a counterexample or completed check.
The server had reached its inlining pass after parsing/typechecking; those
passes are not invariant verification. Exact returned command chunks and
original server logs are preserved here.

A subsequent offline format probe used the installed CLI against generated
Quint JSON IR saved with a .qnt suffix. It refused the file format with exit
255 before checking a property. That is a tool-input refusal, not evidence
against Candidate A. The no-deadlock option would have disabled the tool's
separate deadlock check, because the harness deliberately has terminal states
with no action; coreTraceSafety still checks terminal versus enabledness.

Generated artifact: `.superpowers/sdd/candidate-a-swap-apalache-input.qnt`,
87,175,855 bytes, SHA-256 eba51eb7aa37880531a24ae5ac63b96fc7d927d48b2d9cf28542f0c2dcb770e5.
It is retained locally, not copied into this evidence bundle. It came from
`quint compile specs/quint/s02/candidate_a_harness.qnt --target=json --invariant=coreTraceSafety --out=.superpowers/sdd/candidate-a-swap-apalache-input.qnt`.
The compile exited zero but its large stdout was truncated; no complete stdout
receipt is claimed. The output file contains seven modules, not the separate
flattened-main stdout representation. A future offline route must resolve that
distinction and the accepted input format before claiming a valid model check.

Source milestone at both attempts: 02a4e94b604760e54b04f94c0dc089ede0de938b.
The swap dependency closure has the source bytes used by the installment
milestone c002a84 (including its program addition), not the older pre-installment
swap receipt. No source was changed by either attempt.

Installed Quint entrypoint SHA-256:
ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501.
Installed Apalache JAR SHA-256:
4753c0ebb2cbb266e2c6ac19ab5ca3827d726cc80fd1fc5d7c1eeb64736cd60b.
Temporary top-level Apalache output is ignored; the original logs above remain
archived. No dependency repair, network-service configuration change, or
Foreman development follows. Exhaustive model checking remains open.
