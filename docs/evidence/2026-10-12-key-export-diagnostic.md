# Bounded key export diagnostic repair

Repository observation: the original Signed installed keygen attempt exited 1
after 17.05380298900127 seconds with
`mori::ledger::native-proving: generated-key-size-or-encode` and no keyset.
Its immutable capture remains under
`.moriarty-dev/runtime/installed-call-20261012/signed-installed-source-20261012/native-signed-keygen-execution-01`.
The independent expectations record describes that historical negative; the old
`d4db1429` producer and outputs are preserved. The generic failure establishes neither a
cap exceedance nor a serializer defect. It does not identify the exact action,
degree, key kind, accepted count or total encoded PK size, or establish that
selected PK serialization was reached. This repair adds future diagnostics and
does not qualify or repair that native result.

Repository observation: the existing error layer, code and Display remain
compatible. `Error::key_export()` now exposes typed metadata for bounded export
failures: action, Prover/Verifier kind, already discovered degree, effective
byte cap, accepted byte count, reason and optional failed-write count/attempted
minimum. Counts include tagged headers. The attempted minimum is only the
accepted prefix plus the first rejected chunk, never the complete encoded key
size. Checked count overflow and serializer-only failure have no numeric
attempted minimum. Raw serializer messages and accepted bytes are not attached
to the error.

The bounded writer refuses before append and records its first failure. Further
writes cannot append or replace that evidence. The single existing tagged
serialization call rejects even a wrapped/replaced error or an ignored writer
refusal. An independent serializer error remains Serialization even if its text
imitates the former writer message. Ordinary counting fixtures panic on size
probes and check one payload serialization call; no new pass was added.

The public VK, selected PK and selected full VK calls forward their actual
existing action, kind, degree and effective caps. The public VK caller passes
the degree it already discovered. Bounds, discovery/native invocation counts,
SDK decoding, all-action derivation, private derived owners and create-new,
marker-last publication remain under their existing contracts. Changed source
closure bytes invalidate old exports for this new consumer; old outputs retain
their original producer identity.

CLI observation: ledger conversion renders the exact action once as an
ASCII-safe valid JSON string, including surrogate escapes for supplementary
Unicode. The constructed error survives Quiet/ColorNever shell reporting on
actual stderr. Trusted shell box drawing and wrapping remain present. Existing
degree diagnostics preserve their previous renderer. These ordinary tests
construct diagnostics and serializer fixtures, not native derived owners,
keys, Params or a successful keygen command.

Experiment observations are captured under the sibling runtime directory
`key-export-diagnostic-01`. `red-ignored-writer.log` records the old encoder
accepting a truncated prefix after an ignored refusal. `red-cli-context.log`
records three missing diagnostic renderer controls before its repair.
`green-ledger-focused.log` records seven focused passing controls;
`green-cli-focused-02.log` records three passing CLI controls. The intermediate
CLI capture is retained; two assertions were corrected to account for a source
backslash adjacent to newline and trusted renderer wrapping/box drawing.
Final inspection added ASCII DEL to the hostile-action control. The old ASCII
pass left that JSON-permitted character literal; `red-cli-del.log` records the
failure, and the final pass escapes ASCII controls as well as Unicode.

Final ordinary verification used Rust 1.99.0, `--locked --offline`, the existing
shared target and `CARGO_INCREMENTAL=0`, without dependency downloads:

| Check | Terminal result | Capture |
| --- | --- | --- |
| mori-ledger standard suite | 118 passed, including 12 compile-fail doctests; no failures or ignored tests | ledger-core-final.log |
| mori-core | 34 passed; no failures | ledger-core-final.log |
| mori binary units | 17 passed; no failures, including the DEL regression | cli-final-02.log |
| installed_call / signed_installed_call | 4 / 4 passed; early refusal/help only | cli-final-02.log |
| mori ordinary build | exit 0 | build-final-02.log |
| workspace fmt / diff whitespace | exit 0 | freeze verification record |

The independent Quint slice models the bounded writer's first failure and
finish precedence, not the SDK, cryptographic keys or native feasibility. See
`experiments/moriarty-language/formal/quint/language/key_export_diagnostic.md`
for its bounded checks and explicit correspondence limits.
Its recorded focused `core_io.rs` inspection predates the final DEL control/
escape repair. Writer, reason/count and forwarding source pins remain unchanged;
final Rust CLI tests and the required fresh full-source audits cover the renderer.

No native model selection, synthesis, MockProver, keygen/prove, valid SRS access
or initialization, network/application action or resource amendment was run.
The standard ordinary negative parameter fixtures contain only newly created
invalid small files and reject before native parameter initialization. Any
later native experiment still requires root-owned exact resource review,
allocation and independent result review.
Both fresh full-source audits and separate historical-negative/ordinary-
diagnostic result audits remain root-owned requirements before publication.
