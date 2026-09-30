# W-D2H local image-codec experiment result

Status: **local executable experiment, 2026-09-30**. The proposed suite is
not adopted. B05–B07, W-D2/W-D3 and Sprint 1 remain open. No authenticated
consumer, signature, native proof or ledger acceptance result is supplied.
No commit or merge was performed.

## Frozen proposal and scope

The codec is pinned to **hash-images repair-02 SPEC**, SHA256
`52ffc9b4af619a00176157d6f6b1d43f8876b5069c99c1711e0ae7ff949c89bc`.
Its exact bytes are embedded in the immutable repair-02 candidate packet,
SHA256 `daa09426d9e7c64362c0c715b04fff961448d8942441f007f948afb750f551e9`.
The runner verifies the packet and embedded SPEC bytes, rather than silently
selecting a newer live proposal.

The parent reported that repair-03 is being prepared for two medium static
consumer-order findings at tag35 debtor/creditor and tag16/17 B05/B07 fact
precedence. **Comparison to repair-03 is required before any design claim.**
Those consumer anchors are outside this experiment. Prior design review does
not approve these executable bytes.

All authored files are under this `hash-image-codec/` directory. The existing
hash-images documents, audit packets and source modules were read only.
[Protected input hashes](protected-inputs.json) record six matching frozen
inputs at the measurement point: three repair-02 documents and three modules.
Another agent's subsequent design revision must not be mistaken for a codec
write or adoption.

## Delivered behavior

[codec.mjs](codec.mjs) produces canonical payload, envelope and SHA256 for
purposes 1 (Source definition), 3 (explicit raw implementation package) and
4 (policy). [codec.d.mts](codec.d.mts) declares the closed records and distinct
nominal identifier roles. Runtime admission rejects extra fields, accessors,
symbols, unsupported constants/selector pairs, malformed/collapsed ID roles,
noncanonical decimal strings and out-of-range amounts/rounds/scales.

Text is exact decoded Unicode scalar content, without repair or normalization.
Policy keyRef enforces nonempty text, simultaneous 1024-byte/1024-code-unit
bounds and the Core opaque control exclusions. Hashes present as lowercase
64-character hex and encode as exactly 32 raw bytes. Every closed empty/none
constant is explicit in the policy payload.

Package files are three caller-supplied role-ordered byte arrays, each capped
at 1MiB, with the full payload capped at 4MiB. Paths and filesystem metadata
are absent. Raw bytes, line endings, BOM and invalid UTF-8 bytes are preserved.
Intrinsic typed-array slots enforce byte caps and exact subview contents, so
shadowed JavaScript length/buffer properties cannot bypass bounds. Source and
policy payloads enforce the specified 4096-byte caps before final allocation.

`produceImages` computes Source and Core first, constructs the policy with those
computed links, and requires coherent selected Source/package identities.
`compareContent` reports matching content with `authenticated:false` and
`scope:"content-only"`; it issues no B05/B06/B07 or tag8/11/12 result.
See [issues and boundaries](ISSUES.md) for policy-body admission/context
separation and the unimplemented Source formation/artifact transport obligations.

## Executed observations

Node `v24.21.0`: **72 tests passed, 0 failed, 0 skipped** with
`node --test --test-reporter=tap codec.test.mjs`. The complete retained output
is [test-results.tap](test-results.tap). The initial API assertion failed before
the codec existed. Subsequent negative controls reproduced a shadowed-byte-cap
bypass and a shadowed-buffer getter before the intrinsic-slot repair; both now
pass. Node syntax checks exited 0 for codec and runner. The declaration file
has not received a separate TypeScript compile check.

[reference-vectors.py](reference-vectors.py) generated expected bytes before
the Node producer was implemented. [vectors.json](vectors.json) freezes nine
complete payload/envelope/digest vectors (transfer, repayment, Unicode policy,
each with purposes 1/3/4), plus two actual-module package digest/length vectors.
Python `struct`/`to_bytes` and Node `Buffer` are separate serializers by the
same author, **not an independent person/provider audit**.

Tests cover nominal/round boundaries, reserved/common IDs, version/constructor
selection, decoded controls/surrogates, exact text caps, Unicode distinctions,
hash shape, retention constants, package role/count/size/raw bytes, acyclic
links, identity content differences, zero-fee recipient commitment, explicit
purpose rejection and content comparison. The actual modules total 41303
raw bytes and agree with both frozen Python package digests.

[run-experiment.mjs](run-experiment.mjs) exited 0 and retained
[result-data.json](result-data.json). These are projected typed input examples,
not newly parsed Source/full-consumer positives.

| Typed example | Source digest | Actual package digest | Linked policy digest |
| --- | --- | --- | --- |
| Transfer | `07ef99c28fad13e9ba80f83fdfd36de87fdb92f86e66bee4a7b887f99ac4a8e3` | `21a5a5c01858be797ee49894c02dceb777a1ad31d70d196cddc93d423804e585` | `6aa3c9cfbe90002a12aa1e6b6a43c8c10faad78b89ec2e554d5ce7363919102e` |
| Repayment | `dca58b38c41070228a5f6625bd0ec2dd4aa8f183e2b4809aa56b81a3e595ae78` | `6e153b337f696bcb9f7ec0bcd84e3dcf5a3bac70b604cd417836e65178e2b425` | `09af673fac67829bde03788c074c8fe4e48cf0e47f8ac43ed5d1cecc4123c3cc` |

For both examples exact policy content matches; increasing the action amount
by one fails content equality. Transfer Source/Core/policy payload sizes are
87/41446/274 bytes; repayment sizes are 86/41445/254 bytes. Purpose-separated
envelopes add exactly 27 bytes. These are local observations, not empirical
wallet/proof/ledger limits or an injectivity theorem for SHA256.

## Reproduction and handoff

From this directory:

```bash
python3 reference-vectors.py
node --test --test-reporter=tap codec.test.mjs
node run-experiment.mjs
```

Regenerating vectors reads the current three module inputs and changes their
freeze if those files change; retain the recorded manifest before doing so.
The runner refuses changed packet or module inputs. Test measurements become
stale after changes to codec/tests/vectors or the actual package files.
[artifact-manifest.json](artifact-manifest.json) pins the local handoff bytes.
The task's directory restriction takes precedence over the checkpoint skill's
repository `.foreman/session.db` destination; local machine-readable results,
commands and input/output hash scopes provide the bounded handoff record.

Remaining work includes repair-03 comparison, fresh independent executable
reviews, successful Source formation/full D sweep, an artifact decoder and
authenticated suite selection, provider provenance/selection, actual export/
closure/loaded-artifact/toolchain/lowering correspondence, a real B16 handoff,
signature verification, native proof and ledger evidence. None is discharged
by this experiment's content digest agreement.
