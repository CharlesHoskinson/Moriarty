# Bounded-parser authoring exception

On2026-10-18UTC the user answered **“Sol writes; Opus/Gemini audits”** to the pending question allowing GPT-6.1 Sol to write only Sherlock’s parser budget checks after two empty Gemini attempts. Fresh Opus/Gemini security review is required before executing the candidate.

The exception covers the bounded parser and its necessary early token-budget hook, with the existing grammar, legacy APIs and hard lexer limits preserved. It does not transfer other security authoring, native guards, policy judgments or grants to Sol. Current nonsecurity authoring remains GPT-6.1SolHIGH; the standing actual-outage reviewer fallback remains applicable.

R181 contains the Sol source candidate. R184 is its designated security review. Broader checker, copy, reporting, destruction/cleanup and general H1 admission remain their own predicates; no native operation is granted here.
