# Reviewed financial expression authoring and CLI

The explicit `moriarty-financial-expression-source/1` profile connects real
`.mori` source to the 48-constructor local expression runtime. The CLI checks and
formats this profile and the original `moriarty-expression-source/1` profile.
Terra completed the existing candidate with a shipped vault-quote subprocess
regression and copyable command documentation.

The accepted source candidate is `07e19d4daa014c320aa483c8b5435bc111a60f71`.
[The manifest](candidate.json) binds the complete inherited implementation and
Terra completion. Parent verification passed the build, all 679 tests and ten
independently derived public API probes. [Astra medium](astra-review.json)
verified the hashes and ran separate conditional, typing, work, rollback,
ownership and CLI checks. [Grok 4.6 high](grok-review.json) reviewed the complete
source and CLI independently.

Grok's original CLI-BOM-R1 finding reversed the decoder option's behavior.
[The parent reproduction](bom-reproduction.json) and
[Astra's independent reproduction](astra-bom-disposition.json) establish that
`ignoreBOM:true` retains U+FEFF on Node24.18.1 and that both unchanged CLI profiles
match their APIs. [Grok's narrow follow-up](bom-followup-03/grok-review.json)
retracted that sole finding and approved the CLI. The original review remains
intact; no decoder change was made. Both reviews now give source and CLI
`PASS_SCOPED`. [Acceptance record](accepted-result.json).

The complete source review finished in630.960seconds; the narrow follow-up in
136.255seconds, both within their recorded900-second allowances and with
terminal `end_turn`. Returned identity was `grok-4.6`; usage identified
`grok-4.6-build`. Grok's reviews were static; the retained executed checks were
performed by Terra, the parent and Astra as their records specify.

This accepts the single-action, trusted-schema expression component and CLI.
Full source-defined schemas and financial authority, multiple actions, complete
financial transitions, K/proof correspondence, mandatory PCD, ACTUS/DeFi and
Preview acceptance remain open. It does not close full SP02.
