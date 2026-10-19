# F01 fixture copy manifest (custodian staging, mechanical identical copies only)

Do not regenerate, reformat or re-derive any body. Verify byte length and SHA-256 after each copy; stop on any mismatch. Legacy absolute validation paths must not be read at test runtime.

| Source body (author-input ID) | Repo-relative destination | Bytes | SHA-256 |
|---|---|---:|---|
| F26 escrow-custody-v5.mori | experiments/moriarty-eutxo/tests/fixtures/f01/designers/02-escrow-time/escrow-custody-v5.mori | 2907 | 00c9893ee5d2cbf41adb619bbe2a4b37e4e3f4de64889aa4567bbfe0174ec957 |
| F28 financial-seed-custody-v5.json | experiments/moriarty-eutxo/tests/fixtures/f01/designers/02-escrow-time/financial-seed-custody-v5.json | 587 | e31a3a7a808c1cecf36768be52cd93d279617d80de7d6f075c3dbb8dd0d471a6 |
| F27 financial-residual-custody-v5.json | experiments/moriarty-eutxo/tests/fixtures/f01/designers/02-escrow-time/financial-residual-custody-v5.json | 1208 | 5ab8453e9d22651777600c37984e60553499acc65680fc0ec909c03ce0b8ebd6 |
| F25 cases.json | experiments/moriarty-eutxo/tests/fixtures/f01/designers/02-escrow-time/cases.json | 53983 | b3e74aba4d06402679421f42fb4bb87ea2bf7ed813390ac0faeab3ceeee76cf3 |
| F24 E13-release-real-custody.expected.json | experiments/moriarty-eutxo/tests/fixtures/f01/designers/02-escrow-time/E13-release-real-custody.expected.json | 1173 | 02efee11c65eb6f684523479ef6e8a39d0498b2be79bc5611426e39cee4404e8 |
| F23 E11-refund-real-custody.expected.json | experiments/moriarty-eutxo/tests/fixtures/f01/designers/02-escrow-time/E11-refund-real-custody.expected.json | 1172 | 6218ba8da1bb67fb660cb486b8376bb774688217a6549c2e5acaf6d8dc159149 |
| F29 RESULTS.json | experiments/moriarty-eutxo/tests/fixtures/f01/t1-independent-oracles-19/RESULTS.json | 166136 | 3ebe82c927e5693b5714a7133b7a1bcb79df76d6bcd5f84529c1a1db5c9521d8 |
| F05 genuine OLD producer | experiments/moriarty-eutxo/tests/fixtures/f01/compile-custody.original-b5c1.mjs.txt | 20622 | 4e5b5a393ae9e11edc43ff68d47486f191f67317866553058bb65d650ff759e3 |

Total staged fixture bodies: 227166 bytes (7 fixtures) plus 20622-byte OLD producer. Current producer and publication are read from experiments/moriarty-eutxo/src/ and are not copied here. Project aiken.toml/aiken.lock texts embedded in custody-fixture-world.mjs are the supplied 364B 32a26e5b5244da16bfe352ecd23cffef17db6267fdddec2a81a3e47f60905272 and 268B 88ea28f7520d83d207bae89c30e2f6a2d27b337e36eb19b59a5ef01edefb820b bodies.
