# Supervisor input contract

Prove config fields: ir, pk, vk, preimage, srs. Each is {"path":"absolute reviewed artifact path","sha256":"supervisor frozen64lowercasehex"}. Verify config fields: vk, srs, proof, statement in same form. Unknown fields rejected. Supervisor supplies config SHA256 as separate CLI argument; consumer output receipt is never verifier authority. No production wallet/address/key config. Prove retains local binding0 and exact adapter key_location.

Future commands, specified only:
`beta-native-proof-consumer prove /absolute/supervisor-prove.json <supervisor_config_SHA> /absolute/new-empty-output-directory`
`beta-native-proof-consumer verify /absolute/supervisor-verify.json <independently_frozen_config_SHA>`

Run modes in distinct processes with network namespace, reviewed resource caps and no retry. Supervisor must independently freeze rawproof/statement/VK/parameter identities after successful prove; copying hashes from untrusted receipt alone is not validation. Rawproof has no fixed golden length. File read hard ceiling2GiB supports bounded PK read but is not permission for2GiB process memory. Prover failure leaves reserved empty directory; partial I/O output after native success must be retained as failure, never rerun proof automatically.
