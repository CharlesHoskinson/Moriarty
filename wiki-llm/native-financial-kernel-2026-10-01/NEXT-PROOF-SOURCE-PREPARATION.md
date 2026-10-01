# Next native financial proof: source preparation only

2026-10-01. Repository observation: four original/corrected draft files are preserved in [the immutable draft archive](evidence/native-financial-v8-draft-history/archive-manifest.json). Python AST parsing passed. Neither draft ran; both refuse allocation while actual v7 result pins are absent. This working note is not canonical claim acceptance or a resource vote.

## Design choice and source correction

Continue the same production signed Beta/Core/native financial pipeline with one real finalized proof, then a separate native verifier process. The proposed proof budget is 600 seconds wall/1,200 CPU/4 GiB; separate verification is 120 seconds wall/240 CPU/2 GiB. These are proposals requiring fresh source/resource votes after actual successful build, exact SRS body and key generation plus current result reviews. No new proving authority exists.

Root source inspection found a concrete format error in the first draft: the unchanged native constructor calls `IrSource::load`, whose pinned official implementation reads JSON. Passing generated tagged IR would fail there. The corrected second draft pins the original JSON `pay.zkir` (SHA c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae); the provider serializes the loaded IR for its tagged resolver. Generated tagged IR remains a key-generation identity artifact. The original erroneous draft remains intact.

The second draft also pins the retained producer configuration, rechecks the child configuration hash and checks the complete output file set after independent verification. Actual outcome schema, receipt success conditions and complete input closure still need integration once v7 results exist.

## What the later result would establish

A successful run would establish conditional local ledger9 financial acceptance from the declared public development genesis: real nonzero finalized binding, registered/resolver key identity, proof verification, native default well-formedness, full application Success, exact financial storage and UTXO conservation, protocol DUST fees and replay refusal. It would not establish authenticated custody, asset/account registry, deployment/funding/time/history, generic lowering, formal correspondence, recursive PCD or Preview settlement.

The existing verifier implements cryptographic/public-input, decoder, missing ownership-signature and replay controls. Several broader native financial cases remain specified only: registration/funding/dust/time failures and competing candidates. Earlier adapter/kernel checks must not be relabeled native ledger refusals.
