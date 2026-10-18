# Public payload inventory

This inventory lists exactly **179 staged declared-public payloads, 879,614 bytes**. Each named file was read as bytes and checked against its recorded byte length and SHA-256; all matched. No directory walk or original `sourcePublicPath` access was used. Input `STAGING-MANIFEST.json` SHA-256: `9483dac5b970d05b197411ef43c7c495fca1c084a72ac87c644728c63a9d93ae`.

Links resolve relative to the eventual publication root `evidence/utxo/current-rust-groth16-first-proof-2026-10-16/`, with payload paths preserved. [README.md](README.md) explains acceptance and limits. This inventory excludes the two new documentation files, their output allowlist and the staging manifest itself from the 179-payload total. Hash verification is byte-integrity evidence, not semantic or security review of every file.

## Native run: stdout and public layout log

Captured positive outputs, negative/usage stdout (including empty captures), and public layout report. Verification acceptance is scoped to the runtime reports. 20 files; 16,354 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [run/logs/A6.stdout](run/logs/A6.stdout) | 2038 | `7bae4fb1ded96b8c767acfabc135bb54aafab0c8a13a648686da32d38696e959` |
| [run/logs/I6.stdout](run/logs/I6.stdout) | 5745 | `e41b2cf6549017aa511156c4d92abd1d26cc237d59728f8b8b2005dca7092023` |
| [run/logs/N1.stdout](run/logs/N1.stdout) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/N10.stdout](run/logs/N10.stdout) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/N11.stdout](run/logs/N11.stdout) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/N2.stdout](run/logs/N2.stdout) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/N3.stdout](run/logs/N3.stdout) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/N4.stdout](run/logs/N4.stdout) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/N5.stdout](run/logs/N5.stdout) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/N6.stdout](run/logs/N6.stdout) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/N7.stdout](run/logs/N7.stdout) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/N8.stdout](run/logs/N8.stdout) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/N9.stdout](run/logs/N9.stdout) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/P6.stdout](run/logs/P6.stdout) | 293 | `c41f694595ed095616206f1d2a747c2a655ed7919c6ee5659306467418c7d81d` |
| [run/logs/Q6N.stdout](run/logs/Q6N.stdout) | 2045 | `68d51741a67fb3a3a533f0a03f93e18ea7461fe6da13803eb4cb0be5262dfdc7` |
| [run/logs/Q6P.stdout](run/logs/Q6P.stdout) | 2200 | `310155441dcf5973979812cc28928d74fbdcd717dd2bf1b327c5495c25055bba` |
| [run/logs/V6-export.stdout](run/logs/V6-export.stdout) | 1933 | `9acd1ec9b7bb389b7b0d7fe585b158d826b78e1cd1ff51f918d368c1bed473e7` |
| [run/logs/V6-source.stdout](run/logs/V6-source.stdout) | 2032 | `782664e56999a2ca508ebae5d0f6f14a3d70a4d0713194cd4adb33c331fe4456` |
| [run/logs/custody-layout.log](run/logs/custody-layout.log) | 68 | `1a3c09670ed4da961d9dc54b657147fd8e7dbb37c7d425319cb2dc5c13728ada` |
| [run/logs/usage.stdout](run/logs/usage.stdout) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

## Native run: public stderr captures

Declared-public diagnostic captures for this exact run. These are the staged public copies; no original protected stderr was accessed. 19 files; 2,307 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [run/logs/A6.stderr](run/logs/A6.stderr) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/I6.stderr](run/logs/I6.stderr) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/N1.stderr](run/logs/N1.stderr) | 87 | `ec9d38583e39ca82475e3e2c41cc66fabc0a9d96d084394fb842d2315a4cacc9` |
| [run/logs/N10.stderr](run/logs/N10.stderr) | 915 | `cd8c122284d04a0f366c9cff3a4b5427ce17b9d8077bb67c1dbbfcb722639db9` |
| [run/logs/N11.stderr](run/logs/N11.stderr) | 80 | `a2f5e075ac1cfd634ecce787654f29719db396e2084852321ad5f6068b86b968` |
| [run/logs/N2.stderr](run/logs/N2.stderr) | 87 | `ec9d38583e39ca82475e3e2c41cc66fabc0a9d96d084394fb842d2315a4cacc9` |
| [run/logs/N3.stderr](run/logs/N3.stderr) | 87 | `ec9d38583e39ca82475e3e2c41cc66fabc0a9d96d084394fb842d2315a4cacc9` |
| [run/logs/N4.stderr](run/logs/N4.stderr) | 126 | `63908ebb4522df09ff37ec2c1a0bd661375f9eba70e5bb9d66cec9e21504e65d` |
| [run/logs/N5.stderr](run/logs/N5.stderr) | 201 | `b194d23fcff050945119596d2713b0b1bc7169aea3e24810273d83ac0e1ba2b5` |
| [run/logs/N6.stderr](run/logs/N6.stderr) | 63 | `403c4ecba5ea4f31dec60fa3fe445b65152db7ba93919b2fab9f9a3b3d3e6172` |
| [run/logs/N7.stderr](run/logs/N7.stderr) | 41 | `32b158e5a6cd3e9936ce90ba8a1296d0ac522e53e8013d50b6cb3f710b06f088` |
| [run/logs/N8.stderr](run/logs/N8.stderr) | 194 | `568beac4723fa00a08dc18733d593871f49ee0b5fc96d4182af55d8cc697666a` |
| [run/logs/N9.stderr](run/logs/N9.stderr) | 43 | `673ba28403699502c292d18a41a3a5a11bec1e3e0885058827b6dc2700addcee` |
| [run/logs/P6.stderr](run/logs/P6.stderr) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/Q6N.stderr](run/logs/Q6N.stderr) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/Q6P.stderr](run/logs/Q6P.stderr) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/V6-export.stderr](run/logs/V6-export.stderr) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/V6-source.stderr](run/logs/V6-source.stderr) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/logs/usage.stderr](run/logs/usage.stderr) | 383 | `5b920f1771a27cf3d072c48e794cd74cbeed8cee1d0d660dd55cd20962302971` |

## Native run: exit records

Per-command bounded-run exit, stream and nominal-charge records. Resource fields do not measure transitive native CPU. 19 files; 58,881 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [run/logs/A6.exit.json](run/logs/A6.exit.json) | 1594 | `cc9a03c1b7083980f22f701ad8094267707e5275f2072f0bc378baee38d2a0d5` |
| [run/logs/I6.exit.json](run/logs/I6.exit.json) | 1675 | `884d264e7b970b9804f3f73a3db3f7fbbaa4069e7e36fc9bd88caf659a31e0cf` |
| [run/logs/N1.exit.json](run/logs/N1.exit.json) | 1602 | `f092b16e278a4f7e3d6e2f4f6e4bffa204ce87dd97d56b956fff93cc25b17c49` |
| [run/logs/N10.exit.json](run/logs/N10.exit.json) | 1769 | `ab45c1cc02369b100c93031162120003ed92296bb091a8805ac5e047cbb996b9` |
| [run/logs/N11.exit.json](run/logs/N11.exit.json) | 1906 | `490780f42f0ebd7819fab3f744e0edd0b49b222f20b59eaf48a19229f8abb05c` |
| [run/logs/N2.exit.json](run/logs/N2.exit.json) | 1605 | `d79c51b8d42e5468d4e0a02bfdad490de33f368331e1ac5765e018e92cf895bd` |
| [run/logs/N3.exit.json](run/logs/N3.exit.json) | 1605 | `df9b098479f777ecc36cd03575f5546ccea1d01ac621aff4e3baa3e9a6fddbf5` |
| [run/logs/N4.exit.json](run/logs/N4.exit.json) | 1598 | `d4bb83b928fd88d0cbd48a527580332c1bc327878c1b044f40890090ec38e535` |
| [run/logs/N5.exit.json](run/logs/N5.exit.json) | 1611 | `0a13dd4ea32102fccd09adf589f5a9fcaabaa0bd6311db50f606fb8a56dec5e6` |
| [run/logs/N6.exit.json](run/logs/N6.exit.json) | 1604 | `0fcaacaa84783e6e57e1a5e09c0f147840d81ed3c75cc1102f57b8553181a5b8` |
| [run/logs/N7.exit.json](run/logs/N7.exit.json) | 1603 | `bcae5070d17bf79fb481bd04197992cc21a69003699387f55a8d163b9e4ee279` |
| [run/logs/N8.exit.json](run/logs/N8.exit.json) | 1606 | `91efbb2ed1bdf19d27dc7aef21992e27f962893641b77beb2a3b76171404a528` |
| [run/logs/N9.exit.json](run/logs/N9.exit.json) | 1648 | `79ed7b1718d8dc02f01c74cb8fff282d6e827f32086a4db6e7574f562c6c31c3` |
| [run/logs/P6.exit.json](run/logs/P6.exit.json) | 1903 | `1b0620c101d928d921fdc5ee7652d9c8af8c7b594a323856a5dbd35277730ddf` |
| [run/logs/Q6N.exit.json](run/logs/Q6N.exit.json) | 15299 | `951fb9e5ce7eeba939b282bc9d99b25157cfffd354bdaeda3fa1b533ad60a779` |
| [run/logs/Q6P.exit.json](run/logs/Q6P.exit.json) | 15443 | `0f6d2cae03056da0dabf919235edcb2e7fa549a1a70f38cf15dee986f35fba4d` |
| [run/logs/V6-export.exit.json](run/logs/V6-export.exit.json) | 1605 | `37df2d2a1f21c210908ea3983849df079c1cd12d9ac6425ab8162f7b391cb51f` |
| [run/logs/V6-source.exit.json](run/logs/V6-source.exit.json) | 1654 | `440c880dff788b396c4d36d59ca78aba0dd4fe0a4e8eb2ff48795b316b00660c` |
| [run/logs/usage.exit.json](run/logs/usage.exit.json) | 1551 | `78f44c36d706d7545ec7cfd65c5a50020d28fc0301db7918d7b47a7b4eefdd7a` |

## Native run: journals

Ordered-run evidence duplicated from the corresponding exit records; duplicate copies are not independent observations. 19 files; 58,881 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [run/journal/A6.json](run/journal/A6.json) | 1594 | `cc9a03c1b7083980f22f701ad8094267707e5275f2072f0bc378baee38d2a0d5` |
| [run/journal/I6.json](run/journal/I6.json) | 1675 | `884d264e7b970b9804f3f73a3db3f7fbbaa4069e7e36fc9bd88caf659a31e0cf` |
| [run/journal/N1.json](run/journal/N1.json) | 1602 | `f092b16e278a4f7e3d6e2f4f6e4bffa204ce87dd97d56b956fff93cc25b17c49` |
| [run/journal/N10.json](run/journal/N10.json) | 1769 | `ab45c1cc02369b100c93031162120003ed92296bb091a8805ac5e047cbb996b9` |
| [run/journal/N11.json](run/journal/N11.json) | 1906 | `490780f42f0ebd7819fab3f744e0edd0b49b222f20b59eaf48a19229f8abb05c` |
| [run/journal/N2.json](run/journal/N2.json) | 1605 | `d79c51b8d42e5468d4e0a02bfdad490de33f368331e1ac5765e018e92cf895bd` |
| [run/journal/N3.json](run/journal/N3.json) | 1605 | `df9b098479f777ecc36cd03575f5546ccea1d01ac621aff4e3baa3e9a6fddbf5` |
| [run/journal/N4.json](run/journal/N4.json) | 1598 | `d4bb83b928fd88d0cbd48a527580332c1bc327878c1b044f40890090ec38e535` |
| [run/journal/N5.json](run/journal/N5.json) | 1611 | `0a13dd4ea32102fccd09adf589f5a9fcaabaa0bd6311db50f606fb8a56dec5e6` |
| [run/journal/N6.json](run/journal/N6.json) | 1604 | `0fcaacaa84783e6e57e1a5e09c0f147840d81ed3c75cc1102f57b8553181a5b8` |
| [run/journal/N7.json](run/journal/N7.json) | 1603 | `bcae5070d17bf79fb481bd04197992cc21a69003699387f55a8d163b9e4ee279` |
| [run/journal/N8.json](run/journal/N8.json) | 1606 | `91efbb2ed1bdf19d27dc7aef21992e27f962893641b77beb2a3b76171404a528` |
| [run/journal/N9.json](run/journal/N9.json) | 1648 | `79ed7b1718d8dc02f01c74cb8fff282d6e827f32086a4db6e7574f562c6c31c3` |
| [run/journal/P6.json](run/journal/P6.json) | 1903 | `1b0620c101d928d921fdc5ee7652d9c8af8c7b594a323856a5dbd35277730ddf` |
| [run/journal/Q6N.json](run/journal/Q6N.json) | 15299 | `951fb9e5ce7eeba939b282bc9d99b25157cfffd354bdaeda3fa1b533ad60a779` |
| [run/journal/Q6P.json](run/journal/Q6P.json) | 15443 | `0f6d2cae03056da0dabf919235edcb2e7fa549a1a70f38cf15dee986f35fba4d` |
| [run/journal/V6-export.json](run/journal/V6-export.json) | 1605 | `37df2d2a1f21c210908ea3983849df079c1cd12d9ac6425ab8162f7b391cb51f` |
| [run/journal/V6-source.json](run/journal/V6-source.json) | 1654 | `440c880dff788b396c4d36d59ca78aba0dd4fe0a4e8eb2ff48795b316b00660c` |
| [run/journal/usage.json](run/journal/usage.json) | 1551 | `78f44c36d706d7545ec7cfd65c5a50020d28fc0301db7918d7b47a7b4eefdd7a` |

## Native run: proof, gates, metadata and freeze

The positive proof export, G6 financial/native-outcome gate, H6 nominal accounting/counts, metadata-only Z61 custody check, negative metadata, and the 90-path FROZEN manifest. No VK bytes or setup record are included. 6 files; 89,077 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [run/receipts/FREEZE-MANIFEST.json](run/receipts/FREEZE-MANIFEST.json) | 16745 | `b0dc2925a83a1a29df0442d55ab2b0105b2d5de37292ff7a4a25c8fe8d1cf854` |
| [run/receipts/g6.json](run/receipts/g6.json) | 4602 | `81b1fe0d6aebafba1545ea75fbacd00c09e14107609ed521bd78edd1d3a26c08` |
| [run/receipts/h6.json](run/receipts/h6.json) | 59575 | `d40246f3aa45cd2930e0bf9ff348159066eaf74e8daa99a64a9a79a81509902d` |
| [run/receipts/invoice-partial.proof.json](run/receipts/invoice-partial.proof.json) | 5800 | `930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b` |
| [run/receipts/negatives-metadata.json](run/receipts/negatives-metadata.json) | 2014 | `71318cf88e38e7b7914103c1f2211c8ab5f87ec11d33f19e34557ddeb52fd2d2` |
| [run/receipts/z61.json](run/receipts/z61.json) | 341 | `602d87af4a1a139e584d8e3d1d9962931536eabebe1899fe3d43dae16a70c211` |

## Native run: published negative variants

Seven tampered proof exports (N1–N7) and the N10 finish-selector request. N8/N11 protected variant bodies are excluded. 8 files; 43,608 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [run/receipts/negatives/n1-body-rederived.proof.json](run/receipts/negatives/n1-body-rederived.proof.json) | 5801 | `1ee1ba3379b28ddb0f4fcb4a18d5304574067869ba58f95e1a7dae2337d99b76` |
| [run/receipts/negatives/n10-finish-selector.request.json](run/receipts/negatives/n10-finish-selector.request.json) | 3007 | `920499d184bfc38cffce306585f0166b21feba1cc0f1d5c74317c4d22793ab8f` |
| [run/receipts/negatives/n2-context-rederived.proof.json](run/receipts/negatives/n2-context-rederived.proof.json) | 5802 | `b91459a7e7847757454a7248db5ad21dcf8e214b24762d382844934359b217b0` |
| [run/receipts/negatives/n3-proof-ac-swapped.proof.json](run/receipts/negatives/n3-proof-ac-swapped.proof.json) | 5800 | `e3b11bf5d45780598057d34cbec424ee8c42a41ecbd96d2dbb9a916fd39dd752` |
| [run/receipts/negatives/n4-x-swapped.proof.json](run/receipts/negatives/n4-x-swapped.proof.json) | 5800 | `020fa9826957683815cf5d7ce2de471619977abac0ff4dccb2d981fe1aa74eda` |
| [run/receipts/negatives/n5-statement-stale-digest.proof.json](run/receipts/negatives/n5-statement-stale-digest.proof.json) | 5800 | `19154042f827d19b793ad2e90a364878e1fb2501d56c7f592cdf515ccdf03e7f` |
| [run/receipts/negatives/n6-proof-truncated.proof.json](run/receipts/negatives/n6-proof-truncated.proof.json) | 5798 | `c3f04e5fed2e791cfc4d007640661c7070720c528d37236807b6a76992382e9f` |
| [run/receipts/negatives/n7-proof-identity.proof.json](run/receipts/negatives/n7-proof-identity.proof.json) | 5800 | `f10f59c763e4bbcc339ff1e43f24c530ea2370504ab5519de459d08f9a949e50` |

## Root authority, source adoption and actual observations

Spent exact-run authorization, launch evidence, public snapshot manifest, historical source/ordinary adoptions and final narrow K6 actual adoption. Earlier false adoption flags retain their original time and scope. 14 files; 121,255 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [ROOT-OBSERVATION.json](ROOT-OBSERVATION.json) | 964 | `befdfad9a13a00b0c2ea07ad12e57e29674f0eaeea5f31ff23c1fcc5502d287b` |
| [root/AUTH-LAUNCH-PROPOSAL-ADOPTION-R3.json](root/AUTH-LAUNCH-PROPOSAL-ADOPTION-R3.json) | 22054 | `3c5d9a617b42f537f5bf45544ff1addd6c75aa700ebe9ebc53d67e426eef181b` |
| [root/AUTH.json](root/AUTH.json) | 9066 | `4e20c8bc04cc2f7407d1348a2085782fff563335a05de9d46af24adcd9eced80` |
| [root/AUTHORIZATION-RECORD.json](root/AUTHORIZATION-RECORD.json) | 2183 | `0ba2cec6ea7559b182030f66b61035dce69cc9c8ede5eeb12b00e48f7efe2d98` |
| [root/K6-ACTUAL-ADOPTION.json](root/K6-ACTUAL-ADOPTION.json) | 4005 | `83a6e4fd1624774ee050ce88d3671187548e65c294ad147e5910a42685a6cb4d` |
| [root/LAUNCH-RECORD.json](root/LAUNCH-RECORD.json) | 1227 | `2e2d3087bf1a8bf3251d44fa5b52da520b9331010baaf7ef9873f3aa2f32afca` |
| [root/LAUNCH.stderr](root/LAUNCH.stderr) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [root/LAUNCH.stdout](root/LAUNCH.stdout) | 303 | `3dfd9f578cd1610e8f28452e500b4982b719bc85bd2bbdd6ec6830c326f071af` |
| [root/LAUNCHER-DRAIN-ACTUAL-ADOPTION.json](root/LAUNCHER-DRAIN-ACTUAL-ADOPTION.json) | 867 | `5f28f30583f08821a787f1ce1a5d113b2926a215053756f7af1fd3de13911573` |
| [root/M6-SOURCE-ATTESTATION.json](root/M6-SOURCE-ATTESTATION.json) | 2335 | `c6057fb97fd31ab3823eac69648548c8f403507b5bcaaa651833d41602de6c8a` |
| [root/ORDINARY-R2-ACTUAL-ADOPTION.json](root/ORDINARY-R2-ACTUAL-ADOPTION.json) | 1640 | `f36f85ecb8733b0b67964bbcb568cb193c9e5b0ed1bddf46122f2f438550e04e` |
| [root/PUBLIC-RESULT-MANIFEST.json](root/PUBLIC-RESULT-MANIFEST.json) | 33307 | `d300f2c685e66278517ebc3d2acedd9ee0ae21d81765b65f62b19e91fbd21b57` |
| [root/SOURCE-ADOPTION-R3.json](root/SOURCE-ADOPTION-R3.json) | 25791 | `bfc7854a8c4d5b3957c5fdf1febc8a7e695ff74d92ff6588dd56e3b02b21fd0f` |
| [root/launcher.py](root/launcher.py) | 17513 | `b07f7e27c9e143f1616021c326f87a9d223667b9bd578e2343ece60ad189a65e` |

## Archived operator source and proposal

Historical public experiment operator, claims/custody helpers, probe and ordinary harness, with proposal, positions and source pins. Hardcoded paths are provenance, not portable retry instructions. 9 files; 189,547 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [operator-source/DESIGN-POSITIONS.json](operator-source/DESIGN-POSITIONS.json) | 14709 | `f986ea080cfcbdbdc21e5c5465246741a794f5d633f2a968977a0218b0caf401` |
| [operator-source/EXECUTION-PROPOSAL.json](operator-source/EXECUTION-PROPOSAL.json) | 10270 | `558b4b966035b7ed31e65d5998029804a0094e548fde282a7cc438df65f98ef6` |
| [operator-source/README.md](operator-source/README.md) | 7942 | `851230745e685ca8d453ff64e8adeec0c1cb39dc5d9847cebb0531e393cc00c4` |
| [operator-source/ROOT-SOURCE-MANIFEST.json](operator-source/ROOT-SOURCE-MANIFEST.json) | 1983 | `758f6653f0a82206229d99095f9b439f76a32a29819cb42c35d3e9f385abc8c1` |
| [operator-source/claims.py](operator-source/claims.py) | 49656 | `97a500e639fd6ddf80960504e2b4bfebdfd47fd784aa36805b2a434cb4b3e544` |
| [operator-source/custody.py](operator-source/custody.py) | 25208 | `a2362a13fe95cc7647db06bac1285d2ce8887c96ef266c51b01cbb56ee692687` |
| [operator-source/operator.py](operator-source/operator.py) | 50480 | `90c313d1d255f3daedad232b8e8ef7c011278f5847a1976e3e62055139f65ca9` |
| [operator-source/probe-k6.sh](operator-source/probe-k6.sh) | 12741 | `19638b9cd245e6963305df7d304639ddc2196c8644ed7c4fc93114cd3c0a2503` |
| [operator-source/selftest.py](operator-source/selftest.py) | 16558 | `286f9fb859e2ea9c6977e97cd9477e8f11bfd176bee2457b6ee57c9e3ef3508d` |

## Archived launcher source and proposal

Historical launcher, ordinary capture harness, candidate AUTH, proposal, positions and source pins. Candidate authority is distinguished from the spent root AUTH. 7 files; 60,445 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [launcher-source/AUTH-CANDIDATE.json](launcher-source/AUTH-CANDIDATE.json) | 9066 | `4e20c8bc04cc2f7407d1348a2085782fff563335a05de9d46af24adcd9eced80` |
| [launcher-source/DESIGN-POSITIONS.json](launcher-source/DESIGN-POSITIONS.json) | 6461 | `1faba5d8746005d1b8efb5a829f910837d21fc008368474444c27a2afd8458ef` |
| [launcher-source/LAUNCH-PROPOSAL.json](launcher-source/LAUNCH-PROPOSAL.json) | 12115 | `ff13e6cbdd99431d25ad46a780d9852fed9a3c54349c0ee36cee3d662034e181` |
| [launcher-source/README.md](launcher-source/README.md) | 7954 | `b0669ab8c996b235d6b53ac0972941d600714fb278eddf5c56e7915f3ecb3f34` |
| [launcher-source/ROOT-SOURCE-DATA-MANIFEST.json](launcher-source/ROOT-SOURCE-DATA-MANIFEST.json) | 1417 | `be8a79cdc609699223c29a7251427a49a4d9da3253f6f0a02c90600576828829` |
| [launcher-source/launcher.py](launcher-source/launcher.py) | 17513 | `b07f7e27c9e143f1616021c326f87a9d223667b9bd578e2343ece60ad189a65e` |
| [launcher-source/ordinary_selftest.py](launcher-source/ordinary_selftest.py) | 5919 | `6567dbaee1565ad90998e00cfe65c2134202b0fa145df587456a38e556eccddc` |

## Reviews: operative-source

Six preserved reviewer artifacts: REPORT, FINDINGS, ADOPTION-RECOMMENDATION, DESIGN-POSITIONS, READ-COVERAGE and PROCESS. Scope: current operative-source readiness. Original internal links/absolute references remain archival; this inventory supplies the published paths. 6 files; 30,356 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [reviews/operative-source/ADOPTION-RECOMMENDATION.json](reviews/operative-source/ADOPTION-RECOMMENDATION.json) | 2590 | `48e51eef549035dd18777ef7df3abdb918cf0c0b0f5101c148091b87c9dcedb7` |
| [reviews/operative-source/DESIGN-POSITIONS.json](reviews/operative-source/DESIGN-POSITIONS.json) | 13501 | `ffbf5cd95e6e966291507d065fa02a06703abbe5c58ab0355593b4127d03c513` |
| [reviews/operative-source/FINDINGS.json](reviews/operative-source/FINDINGS.json) | 4245 | `2b051260069fb0928b362fe9c4d70daa8a5303cae149e433d2413ff70abdd5ae` |
| [reviews/operative-source/PROCESS.json](reviews/operative-source/PROCESS.json) | 1118 | `9cb825b33098b476d362ec48acdda5749de87b1320dc214eb512aa3c162923b0` |
| [reviews/operative-source/READ-COVERAGE.json](reviews/operative-source/READ-COVERAGE.json) | 2905 | `d03522b481569ced04151fb081d17a1fe436d068b87c3e103e4cdd6f8dd9e756` |
| [reviews/operative-source/REPORT.md](reviews/operative-source/REPORT.md) | 5997 | `cad525464531f217c018cbf20b844fbecaf455b5d431a7ce0f65263452925b07` |

## Reviews: isolated-harness-source

Six preserved reviewer artifacts: REPORT, FINDINGS, ADOPTION-RECOMMENDATION, DESIGN-POSITIONS, READ-COVERAGE and PROCESS. Scope: isolated ordinary-harness source readiness. Original internal links/absolute references remain archival; this inventory supplies the published paths. 6 files; 36,722 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [reviews/isolated-harness-source/ADOPTION-RECOMMENDATION.json](reviews/isolated-harness-source/ADOPTION-RECOMMENDATION.json) | 3841 | `604430262289d5a277324ec0f685f1922672e3e9a6af0dd3577ab621bd33f9ca` |
| [reviews/isolated-harness-source/DESIGN-POSITIONS.json](reviews/isolated-harness-source/DESIGN-POSITIONS.json) | 13526 | `a2dfdb080904f43d859604a37ea67ce90c922566597624c74149b9f2e36efdf9` |
| [reviews/isolated-harness-source/FINDINGS.json](reviews/isolated-harness-source/FINDINGS.json) | 7694 | `301ac5e1ad912139a220f90043efd7d66623789dd997dff62991778dcde3f35a` |
| [reviews/isolated-harness-source/PROCESS.json](reviews/isolated-harness-source/PROCESS.json) | 1121 | `c4cff20fa295f9e94fc4163ee5602b72ad60179f4a5b35dc71cd47c3ce981823` |
| [reviews/isolated-harness-source/READ-COVERAGE.json](reviews/isolated-harness-source/READ-COVERAGE.json) | 4553 | `47998a33c725de1eb4b2d74b161ad1ebd7e14d564b11b58ef2c3c815a7789db6` |
| [reviews/isolated-harness-source/REPORT.md](reviews/isolated-harness-source/REPORT.md) | 5987 | `8575f83526a496a1e767567df080986d9212ca5f4ace294f8ee1993c4f2df435` |

## Reviews: launcher-source

Six preserved reviewer artifacts: REPORT, FINDINGS, ADOPTION-RECOMMENDATION, DESIGN-POSITIONS, READ-COVERAGE and PROCESS. Scope: launcher and AUTH-proposal source readiness. Original internal links/absolute references remain archival; this inventory supplies the published paths. 6 files; 28,094 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [reviews/launcher-source/ADOPTION-RECOMMENDATION.json](reviews/launcher-source/ADOPTION-RECOMMENDATION.json) | 1571 | `f18008406c090ec8c628bd30792000e9562e6ab4bc9b6ab7a6561dd7c09f3e08` |
| [reviews/launcher-source/DESIGN-POSITIONS.json](reviews/launcher-source/DESIGN-POSITIONS.json) | 8706 | `426838248ec7a3fdc1a6336f7a37c8c852a8eaa69746ab2ea9d88b6e14505e70` |
| [reviews/launcher-source/FINDINGS.json](reviews/launcher-source/FINDINGS.json) | 4020 | `04c4ef16d17c866bae4032f546e5b25362314db2855d032f462d5dd9d14e44a4` |
| [reviews/launcher-source/PROCESS.json](reviews/launcher-source/PROCESS.json) | 1117 | `1964b04c7d20c1ed34c65b5e60ee4f8156602195bc9db3938e83e557ec16e714` |
| [reviews/launcher-source/READ-COVERAGE.json](reviews/launcher-source/READ-COVERAGE.json) | 7754 | `dcfe0cd9494417485ed8d52174151ad89c80ffa2edf99070689c7358194b6255` |
| [reviews/launcher-source/REPORT.md](reviews/launcher-source/REPORT.md) | 4926 | `915d8fca7b58311509a50cb906680c44e313bfb30ddeebddd4d18f47ccea0302` |

## Reviews: ordinary18-actual

Six preserved reviewer artifacts: REPORT, FINDINGS, ADOPTION-RECOMMENDATION, DESIGN-POSITIONS, READ-COVERAGE and PROCESS. Scope: the 18-check ordinary mechanics result. Original internal links/absolute references remain archival; this inventory supplies the published paths. 6 files; 18,260 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [reviews/ordinary18-actual/ADOPTION-RECOMMENDATION.json](reviews/ordinary18-actual/ADOPTION-RECOMMENDATION.json) | 2683 | `b58805102b404db4037a26b4fdfe6db28ebed075963a322abdaf871403caea2a` |
| [reviews/ordinary18-actual/DESIGN-POSITIONS.json](reviews/ordinary18-actual/DESIGN-POSITIONS.json) | 3 | `37517e5f3dc66819f61f5a7bb8ace1921282415f10551d2defa5c3eb0985b570` |
| [reviews/ordinary18-actual/FINDINGS.json](reviews/ordinary18-actual/FINDINGS.json) | 3918 | `e2b6c3a850344592928a48b3a2af31b8cfa4ef3f58ccfc046251c73f47458659` |
| [reviews/ordinary18-actual/PROCESS.json](reviews/ordinary18-actual/PROCESS.json) | 1065 | `e8cb4d87f42a53052f8d0650f408d9a997d243f2232f304aeeeeb1ae78057880` |
| [reviews/ordinary18-actual/READ-COVERAGE.json](reviews/ordinary18-actual/READ-COVERAGE.json) | 4481 | `e10b85a34bca040b84412097af6d0448d2753843009ec76e90be23233ea51409` |
| [reviews/ordinary18-actual/REPORT.md](reviews/ordinary18-actual/REPORT.md) | 6110 | `0e65ccb148869b6a7058128cb938531ae8516012de52202764288c2f59b30f91` |

## Reviews: ordinary20-actual

Six preserved reviewer artifacts: REPORT, FINDINGS, ADOPTION-RECOMMENDATION, DESIGN-POSITIONS, READ-COVERAGE and PROCESS. Scope: the 20-check finite capture/drain result. Original internal links/absolute references remain archival; this inventory supplies the published paths. 6 files; 23,680 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [reviews/ordinary20-actual/ADOPTION-RECOMMENDATION.json](reviews/ordinary20-actual/ADOPTION-RECOMMENDATION.json) | 1654 | `22e10a82ffc6011cfc6ef1961aed0c10649a17c41250b3464526fb628c4c88c1` |
| [reviews/ordinary20-actual/DESIGN-POSITIONS.json](reviews/ordinary20-actual/DESIGN-POSITIONS.json) | 7664 | `45b3f729a1ddd847b68d4d9901172862a8e3ea28282c1b0c197f9f4763860641` |
| [reviews/ordinary20-actual/FINDINGS.json](reviews/ordinary20-actual/FINDINGS.json) | 3760 | `983340d60be76bedb27afc64b02ee02307ffb237c6360fa85a96e03c05d506c4` |
| [reviews/ordinary20-actual/PROCESS.json](reviews/ordinary20-actual/PROCESS.json) | 1057 | `a9686e5a2f426eeba4274832bb48eb620838768b5ca1237bcc200b348bdd3169` |
| [reviews/ordinary20-actual/READ-COVERAGE.json](reviews/ordinary20-actual/READ-COVERAGE.json) | 4450 | `750b70b98f3afdab406b403b7c301ec90af873ef84c94bb39a168359bf873042` |
| [reviews/ordinary20-actual/REPORT.md](reviews/ordinary20-actual/REPORT.md) | 5095 | `c8358dfd35f9580764ce856fcf8c44851ccc275212106d565de106e2b0ca2800` |

## History: ordinary-first-15-of-17

Preserved first ordinary run: 15/17 checks, exit 1. Failure evidence is not replaced by the later isolated run. 5 files; 4,306 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [history/ordinary-first-15-of-17/PRE-RUN.json](history/ordinary-first-15-of-17/PRE-RUN.json) | 1214 | `435324da8fcf9e6c08519eafde9f4831661e9e680d556c70beaba7867f97fdfd` |
| [history/ordinary-first-15-of-17/RESULT-MANIFEST.json](history/ordinary-first-15-of-17/RESULT-MANIFEST.json) | 677 | `01bd839a993f606fdd3561abd52f9c8e9242af35c3ec9aee2bfb38593f180d2c` |
| [history/ordinary-first-15-of-17/RESULT.json](history/ordinary-first-15-of-17/RESULT.json) | 749 | `fd3f707c0597effdb6896037bf97c4e830f9558f7ef0ef996aedde161dbdbba9` |
| [history/ordinary-first-15-of-17/stderr.log](history/ordinary-first-15-of-17/stderr.log) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [history/ordinary-first-15-of-17/stdout.log](history/ordinary-first-15-of-17/stdout.log) | 1666 | `fb2da8ba418c1235f6d46599b5292290aefc5c44f42054c0ba58ce01c006d2de` |

## Ordinary supervision: 18-check captured result

Later isolated ordinary run, including input pins, captured result, stream hashes and logs. It establishes finite supervision mechanics only. 5 files; 3,153 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [ordinary18/PRE-RUN.json](ordinary18/PRE-RUN.json) | 1210 | `1f781a761fc6d42c2f6fa9c9b6a811e7bee9944af6173be25dc8bd9add5d304c` |
| [ordinary18/RESULT-MANIFEST.json](ordinary18/RESULT-MANIFEST.json) | 676 | `74fceef1088981a354b2c3fea78a6c5d8645afe7debfc79b037d1758398b8165` |
| [ordinary18/RESULT.json](ordinary18/RESULT.json) | 666 | `06be3c24437ca3ac6d8b8d465c483746ef653b252d8c62b3b631402ac960958a` |
| [ordinary18/stderr.log](ordinary18/stderr.log) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [ordinary18/stdout.log](ordinary18/stdout.log) | 601 | `c1ba369466888b4aae642f527e48f1f286f6c3945b4b9914de1e35180bb5b9bb` |

## Ordinary capture/drain: 20-check captured result

Finite launcher capture/drain checks with stub limitations, preserved separately from the 18-check supervision evidence. 5 files; 4,282 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [ordinary20/PRE-RUN.json](ordinary20/PRE-RUN.json) | 1152 | `2edefa9a5c59eaf7d5c983a8fd91e8cede2aeef2f1b423b114517ecb41344525` |
| [ordinary20/RESULT-MANIFEST.json](ordinary20/RESULT-MANIFEST.json) | 654 | `b9dca796f238281a7a0056e29df46156f07b7854872e1e649ba7535c239579bc` |
| [ordinary20/RESULT.json](ordinary20/RESULT.json) | 1610 | `3d0626a785e502657624a802719bd784f35d10f9b38eb7b96eb3e4fa2f9327da` |
| [ordinary20/stderr.log](ordinary20/stderr.log) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [ordinary20/stdout.log](ordinary20/stdout.log) | 866 | `e6e4a0b5e0d8885ebfa77f0464abae71d4b69e50a8c0b5d741c431d56a41c9b0` |

## History: auth-first-invalid

Preserved invalid first AUTH proposal crosscheck/delivery evidence, not the final adopted runtime authorization. 2 files; 2,446 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [history/auth-first-invalid/AUTH-GUARD-DATA-CROSSCHECK.json](history/auth-first-invalid/AUTH-GUARD-DATA-CROSSCHECK.json) | 867 | `70ead6f0219b291d0a9ac536c0ea40cdb8f0890504f9ab00e7ad84f79e3f0e6d` |
| [history/auth-first-invalid/DELIVERY-VALIDATION.json](history/auth-first-invalid/DELIVERY-VALIDATION.json) | 1579 | `e762cc0190c2a06bb0442790e3b4e543db4d7e4000e1d988e91ef6bafcd9480c` |

## Public source and request fixtures

Current-Rust invoice source/request/selector used for the positive relation and conditional-reserve source used for the N9 compile refusal. 4 files; 6,172 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [fixtures/conditional-reserve.mori](fixtures/conditional-reserve.mori) | 1415 | `395e647534160fde45325232e5f1650a8bd48ce1455a68b28af203bf8bdbe357` |
| [fixtures/installment-invoice.mori](fixtures/installment-invoice.mori) | 1497 | `105137af8ac6a1f060ec7b634e3ec4aa913808e94bd1b7cffb736f8105d2cfc9` |
| [fixtures/invoice-partial.request.json](fixtures/invoice-partial.request.json) | 3008 | `aa1703e7b3aa2e0dd6288668cc871996fc1ee49ecc90861a3c4b40f02107f3cd` |
| [fixtures/invoice-partial.selector.json](fixtures/invoice-partial.selector.json) | 252 | `1f011dec179e394591a69859d06c999337ba45133d63627140a89c30fa591ee4` |

## Independently frozen financial oracle

Financial oracle used by G6; its synthetic VK label must not be substituted for the actual runtime VK digest. 1 files; 39,353 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [gold/K4H-GOLD.json](gold/K4H-GOLD.json) | 39353 | `65c90d3d13912b762b4977e08ea86212b2b83a16bb2b3ddcaa7a21c905075e46` |

## Reviews: native-actual

Six preserved reviewer artifacts: REPORT, FINDINGS, ADOPTION-RECOMMENDATION, DESIGN-POSITIONS, READ-COVERAGE and PROCESS. Scope: the qualified native roundtrip result. Original internal links/absolute references remain archival; this inventory supplies the published paths. 6 files; 42,435 bytes.

| Payload | Bytes | SHA-256 |
|---|---:|---|
| [reviews/native-actual/ADOPTION-RECOMMENDATION.json](reviews/native-actual/ADOPTION-RECOMMENDATION.json) | 2001 | `b51f936ca0239fae28c95d609862c4f26dea6dc8941333b083149a85c54daac2` |
| [reviews/native-actual/DESIGN-POSITIONS.json](reviews/native-actual/DESIGN-POSITIONS.json) | 16180 | `1d3a737d4e717f23d37dddd13d1ca8290ee2d258cdf32e7c9bf940645a93ed74` |
| [reviews/native-actual/FINDINGS.json](reviews/native-actual/FINDINGS.json) | 8984 | `cea9d38942b7632c93f1d2dff26d88627e3c1ef624c5e84347d36ec72a3b88e8` |
| [reviews/native-actual/PROCESS.json](reviews/native-actual/PROCESS.json) | 1073 | `9c0cc8c417ebf5bdea52086143d287c0252e22dedd87dc73f3d8d197e41412ea` |
| [reviews/native-actual/READ-COVERAGE.json](reviews/native-actual/READ-COVERAGE.json) | 4073 | `32ab99a475b34e16aa23aa18a1fa0e3b0eea125b9745601762c95357ff754fd9` |
| [reviews/native-actual/REPORT.md](reviews/native-actual/REPORT.md) | 10124 | `d0926597ef0e3f869fd1036f56f90ec068c089f8a6b4c3ad172c9e4bec5c877e` |
