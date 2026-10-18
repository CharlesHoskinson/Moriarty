# Independent Core/2 partial-delivery oracle

Status: specified-only; all actual consumers UNRUN. This packet freezes one
finite local staged lifecycle predicate: the same delivery duty is reduced by
two explicit partial discharges across approval and receipt awaits, while a
false receipt leaves the pending continuation unchanged. It is not proof,
authentication, ledger settlement, a U2 walkthrough or a parent exit.

Use the source/scenario/events as inputs. Compare the complete Program, filled
Plan and four report JSON values; compare public stdout against the four
`.stdout.expected.json` byte files, exit0 and the exact zero-byte stderr file.
Pass each ACTUAL pending continuation to the next event; no expected report,
boundary or continuation may become runtime state. Invocation order is start,
approval-true, receipt-false, receipt-true. Both start and resume caps are absent;
continuations report explicit cap:null.

All values are derived from the actual input source and the pinned adopted
public observation/M01/M03 rules in `basis/`. `DERIVATION.md` records the fixed
literal arithmetic and representation. `PROVENANCE.json` explicitly discloses
incidental code/doc exposure and unrelated earlier audits; this is not a
blinded derivation. No selected producer, test candidate or existing selected
expected output supplied values. Standard-library JSON and SHA256 packaged
fixed literals; no product parser/evaluator/compiler or consumer was executed.

The proposed shared destination is
`crates/fixtures/core2-partial-delivery-complete`. Keep names and frozen bytes
unchanged when copying. `CASES.json` specifies the comparison/sequence contract.
The report files and raw stdout files intentionally have identical pretty
ASCII bytes; their semantic-value and process-byte roles remain distinct.
Each stdout has exactly one final LF and lexical object keys; arrays retain
prescribed order. Program/Plan `.preimage.json` files instead use compact
canonical UTF-8 JSON WITHOUT a trailing LF, as required by owner-adopted M03.
Those preimages are local observational identity inputs, not source hashes.

`FINANCIAL-AND-WORK.json` retains all seven complete snapshot literals, all six
ordered effect deltas, custody/reserve amounts, gross debits, zero fee accounting,
liability presence/evolution and every local work charge. These derived tables
are not extra wire report members. `GAPS.json` keeps all fourteen historical
gaps OPEN and names the conditional-admission limit. `STATIC-CHECKS.json` is
expected-data consistency only, not an actual consumer result.

`MANIFEST.json` identifies every payload except itself and the freeze.
`FREEZE.sha256` binds every payload including the manifest, excluding itself.
No source/test execution or acceptance is implied by matching these hashes.
