# Source/6 selected-definition image result

Local executable experiment, 2026-09-30. Developers can parse a complete
Source/6 S0 document and produce or compare its purpose-1 selected-definition
image using the local hash-image-codec Proxy repair-01 API. Both
`TransferLiteralFee` and `RepayAccrualFirst` are exercised through the actual
parser. This supplies content projection only. No commit or merge was made.

## Callable boundary and field origin

`adapter.mjs` exports `projectSource6Definition(source)`,
`encodeSource6Definition(source)` and
`compareSource6DefinitionContent(source, claimedDigest)`. Each accepts primitive
source text and calls the real `parseSource6` before privately projecting its
owned AST. There is no public arbitrary-AST input, version override, coerced
object input, caller-owned nested field or claim-fill operation. Parser errors
propagate with their existing code and byte offset. The projected definition
and nominal identity wrappers are frozen. Image byte buffers are ordinary
owned mutable output buffers; callers must retain their exact bytes for use.

| Closed image field | Origin |
| --- | --- |
| `sourceVersion=6` | Explicit `SOURCE_IMAGE_SUITE` metadata; exact Source/6 profile is checked by the parser |
| `profile` | `ast.profile` |
| `wireProfile=1` | Explicit local codec-suite metadata; **absent from Source6Ast and grammar** |
| `agreementInstanceId:agreement` | `ast.programId`, the identifier after `agreement`; this is not the selected builtin |
| `domain:domain` | `ast.domain` |
| `asset:asset` | `ast.settlement.asset` |
| `scale` | `ast.settlement.scale`, canonical decimal text |
| `selectedActionId:action` | `ast.selected.actionId` |
| `operationKind` | `ast.intent.signedAction.kind`, not the submitted action |

Nominal wrappers assign the destination record's explicit identifier roles;
they do not authenticate those identities. The parser enforces the supported
selector/kind pairs. No required purpose-1 content field is missing from this
parser. The version and wire-profile distinction above is the exact API gap:
neither is a separate AST member, and wire profile is not declared in source.
The adapter does not pretend either constant was such a parsed member.

## Executed observations

Node `v24.21.0`: **50 tests passed, 0 failed, 0 skipped** with
`node --test --test-reporter=tap adapter.test.mjs`. Complete output is retained
in `test-results.tap`. The initial pre-implementation run retains 0/50 passing
in `initial-test-results.tap`. The first implementation run retained 49/50:
one test expected `NON_CANONICAL_INTEGER` where the existing parser actually
returns `INVALID_INTEGER`. `expected-code-failure.tap` preserves that run; only
the test expectation changed. No parser error code or formation rule changed.

`reference-vectors.py` specifies the expected fields directly and uses Python
`struct`, integer bytes and `hashlib`, without importing the JS parser, adapter
or codec. `vectors.json` freezes complete expected payload, envelope and digest
for one transfer and one repayment. This is a separate serialization
implementation by the same author, not an independent person/provider audit.
Both actual parsed fixtures match every expected byte and the existing codec.

| Parsed fixture | Payload bytes | Purpose-1 SHA256 |
| --- | --- | --- |
| Transfer | 81 | `01f54eeb2a1ff0dd9808e901c03716022b99f07b6ae360b7ec5a0fcec83df20a` |
| Repay | 80 | `c71215c155d569f53c0fbb309eb0e1ac5cc829fa2d83bacdf395a104ee82c45d` |

Envelope overhead is 27 bytes. `run-experiment.mjs` exited 0, verified all six
frozen read-only inputs before importing executable modules, and retained
complete projected records, bytes and content comparisons in `result-data.json`.
It also saved the actual parsed texts as `transfer.source.mori` and
`repay.source.mori`, with their SHA256 values in that result. `run-results.json`
retains the command summary. `protected-inputs.json` pins the codec/API,
parser, Core module, Source preparation wrapper and grammar. The wrapper is
used only for test boundary controls; the adapter itself never calls it.

Positive controls include formed and locally prepared transfer, repayment,
zero-fee transfer and full repayment. Changing agreement identity, domain,
asset or scale changes the image for both kinds; changing between the two
valid selector/kind pairs changes it. The projected record has exactly the
nine codec fields, with distinct agreement/domain/asset/action nominal sorts.

Presentation-invariance controls cover whitespace, CRLF, leading/trailing and
block comments, a decoded claim-string escape, source and policy claim changes,
intent key/nonce changes, authenticated round/work changes and submitted-effect
amount changes. All compared sources successfully form. Neither digest claim
is filled, authenticated, validated as a hash or copied into the image.

Hostile controls reject old or escaped profiles, unsupported/mismatched
selectors, duplicate/extra fields, a source wire-profile field, noncanonical
integers, out-of-range scale, non-ASCII identifiers, mismatched authenticated
cells, inconsistent repayment totals, trailing source, unterminated comments,
source-size excess and lone surrogates. Caller AST objects, boxed strings,
Proxy objects and coercion objects reject without property reads. Formation
rejection precedes malformed content comparison claims.

## Limits and reproduction

Purpose 1 intentionally excludes intent, state and submitted effects. A source
with an incorrect submitted debit can successfully form and receive the same
definition image while `prepareSource6S0Unqualified` returns `CoreRejected`;
the test suite exercises that boundary. Parsed `authenticated` syntax supplies
claims and is not evidence of external authentication. Image equality is not
complete source/policy equality or a statement about valid execution.

Content comparisons retain `scope:"content-only"` and `authenticated:false`.
The experiment does not implement or close B01/B05, selected-suite/provider
authentication, policy/Core image derivation, a full consumer, signatures,
compilation correspondence, native proof, PCD or ledger acceptance. No public
transaction was submitted. Source/Core/codec/H proposal documents and modules
were not edited. Independent current executable reviews remain pending; the
codec's existing repair-01 review status is not advanced here. The declaration
file has not received a separate TypeScript compile check.

From this directory:

```bash
node --test --test-reporter=tap adapter.test.mjs
node run-experiment.mjs
```

`python3 reference-vectors.py` regenerates the standalone expected vectors
from its literal expectations. Preserve `artifact-manifest.json` before
regeneration or edits. That manifest hashes all delivered files except itself
and records the measured test scope; input drift makes the runner fail.
All writes and retained failures are confined to `source-image-adapter/`.
