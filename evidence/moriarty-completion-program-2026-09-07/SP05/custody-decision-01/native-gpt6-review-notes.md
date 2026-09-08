# Native independent proposal review

Substantive analysis completed for candidate `cand_0403B2CCC37604BBF9FDFBDBC8`. This is a native advisory decision review under `native-review-admission.json`. Model identity belongs to the accepted spawn metadata; this note makes no self-attested identity or CLI-token equivalence claim. The parent must observe the native terminal response before counting the vote.

The exact compiled blinded prompt was read and hashed. Its candidate, ACE text, bundle and response schema match `contract-03.json` and `response-schema.json`. All four required artifacts were inspected; UTF-8 lengths and SHA256 values matched. The embedded source-extracts bytes also match `sources.json`. The underlying three source-file hashes match the extract manifest. No other reviewer verdicts or reasoning were accessed.

Before and after observations gave the same hashes below. Required evidence order was custody-contract, source-extracts, response-schema, review-diff.

| Artifact | Required bytes where specified | Before SHA256 | After SHA256 |
| --- | ---: | --- | --- |
| review-prompt-02.json | | bb6e488922368d30d411b074564d11e67b9dc99f2f7b7b45fdd52ded00972c48 | bb6e488922368d30d411b074564d11e67b9dc99f2f7b7b45fdd52ded00972c48 |
| contract-03.json | | 24d1cf9d51176ee4809bfe0471d1241879d96ed10b4acebd69835556dc33630f | 24d1cf9d51176ee4809bfe0471d1241879d96ed10b4acebd69835556dc33630f |
| custody-contract, embedded | 5079 | 5fe9cf1e8531c0599ffb6286d2d3e1558464792038311208aa7db2807f026038 | 5fe9cf1e8531c0599ffb6286d2d3e1558464792038311208aa7db2807f026038 |
| source-extracts / sources.json | 21718 | f0bbbbe358831cc836fcf515b0ff06c4cffee81caf22a2dfe9d26e7f70ab3e83 | f0bbbbe358831cc836fcf515b0ff06c4cffee81caf22a2dfe9d26e7f70ab3e83 |
| response-schema.json | 577 | 59af589b1640a4c230a36dd0ebd2686c40a7fdd195053a81ef03ec173abb8cd3 | 59af589b1640a4c230a36dd0ebd2686c40a7fdd195053a81ef03ec173abb8cd3 |
| review-diff / review.diff | 0 | e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 |
| raw/midnight-docs-2026-09-07/markdown/tokens/unshielded-token.md | | 60005ede6545ce13c213a2ab08705486a2c79d341308e15baf4bf35fc70bedf6 | 60005ede6545ce13c213a2ab08705486a2c79d341308e15baf4bf35fc70bedf6 |
| raw/midnight-docs-2026-09-07/markdown/compact/reference/compact-reference.md | | 61a828679c922fce519352c5f184098c8f2f9d0cd02e609a64e8d876fed469ce | 61a828679c922fce519352c5f184098c8f2f9d0cd02e609a64e8d876fed469ce |
| repos/LFDT-Minokawa/compact/compiler/standard-library.compact | | 166487c61232e4058f0a5cb1a2a154d5b926a574ba5bc24f5def905c2a938e52 | 166487c61232e4058f0a5cb1a2a154d5b926a574ba5bc24f5def905c2a938e52 |

The frozen bundle identifies base=head=`1a9721e5634e9196aadbe7c28a51081e820a9247`. That object exists as a commit, and its base-to-head diff matches the required empty diff. The live repository HEAD observed during review was `45c42cf95d0e500f8307e99214c8f23ebc701ce6`. This review approves only the immutable proposal; it does not approve the live working tree or claim the live HEAD equals the bundle.

The proposal's acceptance criterion at custody-contract line 19 was evaluated with the constraints at lines 5-17 and exclusions at lines 3 and 21. It requires the later packet to specify exact source and encodings, disclosure, custody/recovery, initialization and revision guards, balance semantics, financial comparisons and all rejection controls. It separately gates proof-enabled public tests, network finality and transaction reporting. Those are specified obligations, not reproduced experiments. Exact fixed-design financial values and a working payer-attribution verifier are not established by this evidence; their later admission remains required. The signature-helper extracts establish no complete wallet-compatible signing route.

No source implementation, builds, proofs, network calls, wallet operations or key inspection were performed. Only the authorized review JSON and this note were written.
