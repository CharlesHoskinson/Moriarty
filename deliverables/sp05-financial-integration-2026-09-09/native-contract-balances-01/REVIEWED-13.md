# Reviewed native reserve and swap mint correction

GPT-6 Astra and Grok 4.6 independently passed candidate13, whose digest is recorded in [the reviewed result](reviewed-source-13.json). The correction reads native reserves from the SDK provider’s exact state class and admits the retained fallible initialize mint under the existing strict wallet checks. The separate initialized-swap helpers are prepared but not connected to execution.

The retained public initialize state contains 1,000,000 A and 2,000,000 B despite an empty indexed balance projection. Source12 used the wrong runtime class and was rejected; its source snapshot, both disagreeing reviews and actual reproduction remain. Source13 passes 450 core tests and 15 supplemental tests; GPT-6 independently checked all 128 candidate files and passed 102 focused tests. Grok reviewed supplied source only.

The first source13 Grok call ended after 26.541 seconds with error_max_turns during startup skill reads. It was not a timeout and supplied no verdict. A corrected bounded invocation used the actual read_file tool identifier, a dispatched review role and up to four turns with the same 900-second allowance. It made no tool calls and returned PASS_SCOPED in 164.112 seconds. Both attempts and reported costs remain recorded.

No new chain operation occurred. The eight cumulative reservations remain charged. A separately reviewed continuation must preserve the original swap contract and store, re-establish canonical deploy/initialize observations and complete only swap and close. Full SP05, Preview, independently funded counterparties and mandatory PCD remain open.
