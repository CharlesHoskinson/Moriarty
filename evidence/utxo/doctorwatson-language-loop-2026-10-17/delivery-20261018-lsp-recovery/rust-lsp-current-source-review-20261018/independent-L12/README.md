Independent normal LSP diagnostic expectations, frozen from source and LSP3.17 before consumer execution.

The two unknown-name inputs each produce one semantic diagnostic: `I cannot find a declaration named `missing`.` (code `mori::check::unknown_name`, Error severity 1). UTF16 half-open ranges are (3,16)-(3,23) for the normal beta input and (2,45)-(2,52) for the Unicode+CRLF input. The latter token begins 48 UTF8 bytes but 45 UTF16 units into its line. Both valid beta variants and the supported Core1/Core2 examples require empty diagnostic lists.

INDEPENDENT-EXPECTATIONS.json freezes complete source hashes and exact semantic messages/ranges. NORMAL-EVENT-EXPECTATIONS.json freezes normal change and single-file close clearing. A valid change must replace any prior error with an empty list. Close clearing is scoped to single-file/open-document diagnostics; LSP does not require the same behavior for project-wide diagnostics. Transport IDs, optional properties and timing remain unasserted.

PARSER-CHECKER-BASIS.json contains source excerpts at 74a909d69639df8823a527a4d608a7516ccd69a2; LSP-NORMATIVE-EXCERPTS.json and WEB-SOURCE.json identify the public official LSP3.17 basis acquired with Scrapling. No LSP candidate or its test/expected-session fixture was read. No protocol/server/compiler/SDK was run.

Prior DevEx exposure is disclosed in CUSTODY.json: the custodian previously saw unrelated init.rs financial expected literals, so makes no independent financial claim. These LSP inputs and expected diagnostics were freshly derived.
