# A102 registered source and independent normal expectation

This package binds the exact ten-file registered A102 source graph to separate source and oracle-data reviews. `SOURCE-FREEZE.json` lists all ten file digests, the five paths changed from base `8b32c0a491c7adfaa658cc32eb34c7c7f93986d6`, and original integration manifest `2ccd8027e250a5b6a57049c5da39570ad96518fe4745fbc683ad490c130afd63`. It is publication preparation, not an execution or publication grant.

The public source path is `mori_backend::sherlock_machine::{literal_schema, emit_literal_step}`. The five A102 wiring tests are registered under `cfg(test)` alongside eleven existing machine tests. All sixteen tests, compilation, actual instruction emission and SDK checks remain **UNRUN**. No A102 runtime/CLI consumer, witness packer or public-input binding was added.

## Separate source and data decisions

- [Opus full adapter](opus-source.json.gz) and [exact public text](opus-source.md.gz): canonical `claude-opus-5-5`, requested high effort, returned `is_error=false`; source `APPROVED_SOURCE_WITH_LIMITS`, separate data `ORACLE_DATA_APPROVED_WITH_LIMITS`.
- [Muse full adapter](muse-source.json.gz) and [exact public text](muse-source.md.gz): returned `muse-wonder-1.0`, requested high effort, terminal completed; source `APPROVED_SOURCE_WITH_LIMITS`, separate data `APPROVED_NORMAL_EXPECTATION_SOURCE`. Its exact public text is a structured JSON string, preserved without normalization despite the `.md.gz` capture name.

These reviewers inspected supplied source text; neither compiled or executed it. The receipts preserve their complete reading limitations. They do not approve the entire compiler closure, a whole circuit, SDK behavior, resource admission, proofs or publication. No verdict is inferred from the older standalone oracle response that lacked its required outer marker.

## Owner conditions remain open

The unchanged pre-existing `emit_public_fetch` and `pack_witness` become public through the module declaration. Opus requires that this wrapper remain unused as a bound statement until a binding review exists (or that a separate change narrow its visibility). This package keeps it unused and adds no consumer; its output has no approved statement binding.

A composed circuit must derive an exactly-one enable from constrained pre-state dispatch, rather than use a free enable witness. Consumers of plain `selected_expression` operands must gate them with that same enable. Full constant typing and `uiValid`, a bound canonical ProgramBlock, full StepInv/O1–O4 and active-reference/occurrence/depth checks, F4 and least-equal projection, the nonfuel canonical L4 byte/Boolean bridge, and global 39-rule dispatch/free publication remain unwaived. So do the other events/P1/P2–P6 work, program/statement/trace/public-private binding, concrete resource admission, B/general ADTs, financial/PCD/ledger/stagenet obligations and native correctness. These are the owner's conditions, not newly selected policy.

## Independent source expectation

[Raw expectation](independent-raw-a102-expectation.json), [derivation](INDEPENDENT-DERIVATION.md) and [original provenance](independent-provenance.json) are exact copies. [The complete packet](independent-normal-source-packet.json.gz) retains every original manifest-listed source/provenance text plus the original manifest: 21 members, each with its original SHA256 and bytes. Its original manifest hash is `deaaa61f5d1157ea99444c2cb9d5d729659740be02ba3f356e8ab46c7ae5d600`; expectation hash is `ea3e5f732c370223f4848f2893c236731d713ca29ed663f5bc9ba84514ff3dd9`.

It supplies one hand-derived positive arbitrary Step case with Constant aliasing, full preservation and fuel 256 to 255. Both candidate and oracle were authored by Astra; independence is from reading/invoking the prospective fragment or its outputs during derivation, not a blinded separate-author oracle. It is not consumed by these tests. No startup reachability, canonical boundary, byte stream/native carrier, hostile or disabled-case coverage, candidate comparison or execution result is claimed. Owner-listed future hostile cases remain in the complete receipt; none was implemented or run here.

## Historical context

[Atlas iteration 004](../../../../wiki-llm/sherlock-circuit-atlas/ITERATION-004-FULL-IMAGE-LITERAL-AND-BORROW.md) is a separate creative note about original unregistered manifest `662bc7748f2a98e2186372970951c08ebed0328b04c574f6f7e4f5eab42e7f85`. Its three-file snapshot and original replies retain that historical scope. Source/data approvals do not approve that note or implement a website.

Native authority remains zero. The older compile worktree, terminal caps/counters, P3 FAIL and P10 BLOCKED are unchanged. Root owns the final publication review, seal and Git operations.
