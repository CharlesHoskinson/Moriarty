# W-D2 postreview disposition

**Candidate:** `w-d2-postreview-candidate-packet.md`, SHA-256 `8c56d42983da09f128ba37567a52774db890f04e4509b1cfe4e2f1d1f6e56d31`.

Independent GPT-6.1 Sol high and Grok 4.7 xhigh (`grok-4.7-build`) reviewed the same frozen packet. Both accept the codec as a useful provisional `/3` S0 byte experiment. Both reject normative W-D2 closure. The GPT review checked all 11 embedded file hashes, 13 receipt artifacts, reran 114 Node tests and independently ran the Python vector builder. Grok made a static embedded-byte review and did not claim execution or hash verification. Its raw output is `w-d2-postreview-grok-4.7-xhigh-raw.json`; the GPT findings are preserved in the session review record.

The earlier high/medium JavaScript API findings are repaired: the encoder uses descriptor snapshots and the decoder applies intrinsic view metadata and a length bound before copying. Both vector digests remain unchanged. Grok identified residual low-severity proxy, trap and mutable-global boundaries. These do not establish a wallet, signature, proof or ledger consumer.

W-D2 remains **open**. The next decision must reconcile nominal `2^127−1` and scale 38 in the wire prototype with the Source/6 scale 18 boundary and the UInt128 recommendation. It must define the canonical effect encoding and a consumer that compares its commitment to signed field 26. Replay and key-rotation policy, cap feasibility, `/4` migration and native/ledger binding remain open. The pinned ledger host-signature experiment is evidence for one primitive only, not W-D1 closure.
