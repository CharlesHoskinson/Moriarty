# Beta code repair result — 2026-09-30

Repository/experiment observation; local experimental beta only. Implements the focused M1/M2/M3 repair from `audits/code-astra-v1.md`. No commit was made. Fresh independent review of the frozen full candidate remains required.

M1: scenario and candidate-effect strings now check nonempty scalar transport representability and the existing 1024-byte bound. JSON-escaped controls pass unchanged through Source/6 to the actual Core/5 judgment. No authoring restriction was substituted for the agreed comparison domain. Tests compare complete original Core results for nonce, pre-head, authenticated head, post-head, predecessor and explicit replay-effect strings, including competing stale-head, work and validity failures. Source/6 and Core/5 files are byte-identical to HEAD.

M2: `inspect` returns bounded `intents: [{name, terms}]`. All actual source-fixed signed terms appear: named operation arguments, nominal quantity atoms, entity/economic claims, signer/key, nonce/head, validity, gross/fee/net bounds, program/policy claims and the closed failure/retention fields. Entity claims are shallow; declaration DAGs are not expanded. Authentication remains explicitly false/open. Scenarios are not consulted to invent source terms. CLI, MCP and packed CLI use this same API.

M3: the existing parser now records immutable field use spans as additive analysis metadata. Origin nodes memoize source ranges, reference uses and declaration definitions; reference use nodes depend on their defining declaration and resolved value expression. Every emitted source field uses its concrete expression span. Operation, cap/floor and automatic-effect roots reach original arithmetic literals and nominal economic declarations through a shared finite DAG. The 180-level doubled-reference chain remains linear; a larger use vector exceeds the origin budget atomically.

Validation: the three supplied discriminators were copied into `tests/repair.test.mjs` with relative imports and observed RED (0/3 passing) before production changes. Expanded focused probes also exposed the missing behavior. Final beta package suite: **63/63 passing**, including eight repair regressions, actual CLI inspection for transfer and repayment, real MCP inspection and existing protocol paths, packed installation, declaration imports, bridge differential checks and frontend/editor tests. `npm run typecheck` passed. Raw final receipts: `/home/charl/research/moriarty-beta-2026-09-30/repair-beta-tests.txt` and `repair-beta-typecheck.txt`; initial discriminator receipt: `repair-red.txt` in that directory. The supplied unchanged experimental regression receipt remains 938/938; that separate suite was not rerun in this repair.

Bounds preserved: 2048 origins; 65536 generated Source/6 bytes; 524288 serialized expansion/inspection bytes. Inspection projection also has an 8192-visit bound before serialization, so shared nested values cannot expand exponentially. Budget failures return diagnostics and no partial origins, field map, generated Source/6 or signed-scope artifact. Explicit malformed effects remain formation failures. The repayment overpayment placeholder is unchanged.

These checks establish local authoring, inspection and preparation behavior only. They do not close authentication, native proof, financial correspondence, atomic ledger acceptance, editor activation or independent result-review gates. The required Grok review had cancelled startup attempts in the preceding candidate cycle and supplied no approval.

## Frozen repair file hashes

- `packages/moriarty-beta/src/bridge.ts` — `56a71b4a709bb4de7fc4723867c9c735a95d1d2dd1cd8d0289ef6b952d299bb7`
- `packages/moriarty-beta/src/frontend.ts` — `aaf285b6b4a7f197934d28c52b967e44f9ca541760bbfd52a13c2612e167ee3f`
- `packages/moriarty-beta/src/index.ts` — `c8884fbfa8e39de3a9d9bc11ca434ccdf7deb485c6f11971f593f6412439a827`
- `packages/moriarty-beta/tests/repair.test.mjs` — `db39d9cad945b4205b3190808f920d22acd059774e0baad12fc526e8d220321a`
- `packages/moriarty-beta/tests/cli.test.mjs` — `1740e0b974e748c2980c9dc0bc9ba80baa06032a3233d8df231bfc9393358349`
- `packages/moriarty-beta/tests/protocol.test.mjs` — `e0cb368215c647f706bc41c4fc2cde00fd35d14eec59cc7a241c5f64c1777d6f`
- `packages/moriarty-beta/tests/distribution.test.mjs` — `892eb1fc3b0845faf940a3631b2de23cdea7e882d0c5a03d4649dd3fa8f25e77`
