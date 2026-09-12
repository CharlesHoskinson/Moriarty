Actions can now read outstanding debt, principal, accrued debt, balances and allowances directly from a fully validated local financial projection. The new agreement-source `/3` profile and Core `/2` keep these reads separate from ordinary snapshots, check every action, preserve runtime short-circuiting and carry exact expression-plus-kernel work.

The source fixture pays 30, then 20, then computes the remaining 50 itself. It settles the debt with balances 0/100, allowance 0/100, retained identifiers and 115 of 256 work remaining. Existing `/1` and `/2` behavior remains compatible.

The root and language READMEs now describe the implemented syntax and semantics: complete current EBNF, typed financial-read rules, immutable financial pre-state, funded composition and executable CLI/demo commands. They distinguish the TypeScript runtime from the bounded K subset and ledger acceptance.

Validation: 794 package tests, typecheck, 52 independent API/CLI probes, 37 Core probes, 174 old-profile comparisons and four actual README commands. Grok4.6 high authored the change (returned `grok-4.6-build`); fresh Astra medium approved the full exact candidate with no substantive findings (127 independent probes and 112 legacy comparisons). Audit evidence and the full candidate manifest are retained in `deliverables/financial-state-reads-2026-09-12/`.

Stacked draft PR on #3 (`feat/multiple-named-actions`). This extends local language execution; it does not add ledger authentication, financial writes beyond Transfer/Repay, or K/proof/Preview acceptance.
