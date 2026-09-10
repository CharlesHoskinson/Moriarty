# Reviewed finalized-state reader

Both independent reviews pass the corrected candidate: [GPT-6 Astra](result-review-gpt6-02.json) and [Grok 4.6](result-review-grok-02.json). The [result receipt](reviewed-result.json) binds their exact source hashes.

The reader captures full native contract bytes, balances and decoded financial state at an explicit finalized block. The local RPC adapter accepts the address and block explicitly, retains its response-size limit, and checks the deadline after receiving a response. The reader rejects late results before further reads and checks again after native cleanup.

All 27 reader/RPC tests and 88 integration regressions passed, with no skips. The original event-loop counterexample now returns `OBSERVATION_TIMEOUT_UNKNOWN`. Grok completed in 351.786 seconds without a caller timeout.

The original GPT-6 blocking audit, Grok scoped approval, source snapshots and failing logs remain intact. Both corrected audits were required; the earlier approval was not reused.

These checks use controlled RPC responses with actual retained native state and generated contract decoders. Live node method support, failed-transaction financial nonmutation, Preview settlement and PCD remain unverified. The caller retains responsibility for contract/build identity and transport cancellation; the node remains a trusted observation source.
