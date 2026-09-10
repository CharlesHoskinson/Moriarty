# Source12 rejected before execution

GPT-6 independently reproduced a blocking producer-class mismatch: queryContractState returns protocol/compact-runtime ContractState, while source12 required transaction ledger-v8 ContractState. The same8150-byte state has correct balances in both runtimes, but their JavaScript constructors are distinct. Source12 would fail CONTRACT_BALANCE_STATE before financial comparison.

Grok's supplied-source review returned PASS_SCOPED after289.364seconds; it did not execute the actual SDK boundary. GPT-6 returned CHANGES_REQUESTED with the actual reproduction. Preserve both responses. Source12 was not admitted or dispatched. Its448passing tests constructed the wrong runtime class for provider states and did not establish the production boundary.

source-snapshot-12/ preserves the rejected implementation bytes. Source-candidate-12 live paths describe that version, not successor code. The source13 correction binds the actual provider class and tests the real SDK method with controlled transport. No failed source verdict is rewritten.
