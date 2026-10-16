# Selected public capsule inventory

This inventory describes the 104 payload files selected by [SOURCE-PUBLICATION-SELECTION.json](SOURCE-PUBLICATION-SELECTION.json), totaling **401,254 bytes**. Paths below are relative to the capsule root. Byte counts and SHA-256 values are exact pins from that selection; the documentation author checked all 104 staged public payloads against them.

Control record SHA-256: `890e5088da5560a3bd89c1e81f213521a5376275dd55badcff8c540799b2f8ce`. The selection record, README, this inventory and documentation allowlist are outside the 104 payload rows.

The dated [actual review](reviews/actual-muse/REPORT.md) and [adoption](root/K5-ACTUAL-ADOPTION.json) close only the first invoice/partial single-party test-setup prerequisite for K6 source preparation. They do not authorize K6 execution or establish PK/VK cryptographic validation, proof generation, keyed pairing verification or ledger acceptance. The gate reports `consumerCanCheckVkToMatrixLink: false`; it records setup provenance only. The review states that the setup controller could forge proofs, so this is test-only evidence.

Frozen author-time “unreviewed” and “never run” prose and the raw observation’s pre-review status remain historical snapshots. The later review and adoption supply the narrow status update without modifying those bytes. No binary, proving key, verifying-key bytes, setup-record body or protected work-tree contents are bundled.

Absolute `source` and other host paths in the selected records are archival identifiers. Their named files are unbundled unless a row below explicitly selects a capsule-relative copy. Hashes alone do not promise external availability. In particular, proving-key and setup-record paths in public metadata refer to retained, unbundled artifacts; they are not paths to resolve from this capsule. The custody disposition in the actual review retains them unchanged in place for `current-cardano-k2-k6-proof-roundtrip-20261016`, with no root direct read/copy/delete/publication or K6 execution before a new exact reviewed operator and cryptographic authorization.

The operator scripts hardcode spent targets and are historical source evidence, not retry instructions. Maintainer experiment authorization records never become public developer permission requirements. Fixture expectations use synthetic declared context and oracle-unit checks, not authenticated Cardano ledger facts. All 20 UTxO families, ACTUS/DeFi/general obligations, recursive PCD, private handoff, multi-parent composition, universal certificates and real Preprod remain open.

| Group | Files | Bytes |
|---|---:|---:|
| gold/ | 1 | 39,353 |
| operator-source/ | 27 | 176,610 |
| reviews/ | 19 | 82,652 |
| root/ | 13 | 51,227 |
| run/ | 44 | 51,412 |
| **Total** | **104** | **401,254** |

## Independent frozen expectations

One K4H gold record; fixture expectations do not authenticate ledger observations.

| Capsule-relative path | Bytes | SHA-256 |
|---|---:|---|
| [gold/K4H-GOLD.json](gold/K4H-GOLD.json) | 39353 | `65c90d3d13912b762b4977e08ea86212b2b83a16bb2b3ddcaa7a21c905075e46` |

## Reviewed operator source and manifest

The 27 files comprise 26 operator/author files and their root source manifest. These frozen scripts and records describe a spent maintainer experiment.

| Capsule-relative path | Bytes | SHA-256 |
|---|---:|---|
| [operator-source/DESIGN-POSITIONS.json](operator-source/DESIGN-POSITIONS.json) | 8428 | `536684d66f46621194aef1beaf01c6b87ecf393f3c76e8704957bdd18d7a8835` |
| [operator-source/EXECUTION-PROPOSAL.json](operator-source/EXECUTION-PROPOSAL.json) | 7426 | `ae115d5d14b995c1736c62eb2943eb6f580c28d549af35f977009e5187a72ea6` |
| [operator-source/EXPECTATIONS.json](operator-source/EXPECTATIONS.json) | 7582 | `4da98c1f48ad01ee35cbafdb849b9cbb16f30b52a3ecc34da59ba7d6ee209d6e` |
| [operator-source/OWN-OUTPUT-ALLOWLIST.json](operator-source/OWN-OUTPUT-ALLOWLIST.json) | 1424 | `8498c5f7889e389159fc5108b54381a7921ccbbd34175eb449735de338d195bb` |
| [operator-source/READ-COVERAGE.json](operator-source/READ-COVERAGE.json) | 5012 | `32db590ce360c23cf66de0aa8733e157cc4b9ef6ee98ecee110ac09d72bf5289` |
| [operator-source/REPORT.md](operator-source/REPORT.md) | 7718 | `acfa5a96eddb820d543b0f099a4b4508a29972c53234ec522804d97dd42c1db5` |
| [operator-source/ROOT-PLACEMENT-INSTRUCTIONS.json](operator-source/ROOT-PLACEMENT-INSTRUCTIONS.json) | 10121 | `cc8d51c04c216438ecfa32aa86130e82462377121189d0be76f805bf5e18a1bb` |
| [operator-source/ROOT-SOURCE-MANIFEST.json](operator-source/ROOT-SOURCE-MANIFEST.json) | 3902 | `0110969fb47287949fd22e738086a9c51b2240c6fc46f63403cf6f96fee2f062` |
| [operator-source/SOURCE-CHANGE-MAP.json](operator-source/SOURCE-CHANGE-MAP.json) | 8068 | `9b94edc067ff8bc45d06668a2cd16e338bdb23f91e231245789c6dbe58758532` |
| [operator-source/operator/request/OPERATOR-STEPS.md](operator-source/operator/request/OPERATOR-STEPS.md) | 7205 | `27e4f10b7a5755015b798e5c43b9d66732f446ef4bfba4300cee118e18fd494f` |
| [operator-source/operator/request/data/expected-r1cs.txt](operator-source/operator/request/data/expected-r1cs.txt) | 413 | `0b52edcc7b955f1c3702712fdd5b38387b53cf4eabd7798d4504ca4b685bcd1f` |
| [operator-source/operator/request/data/k5-abs.sha256](operator-source/operator/request/data/k5-abs.sha256) | 5189 | `2b861dbab6bcec1a3a5b780c11f759d0d20fe1b9b084ff913defdb46f906c696` |
| [operator-source/operator/request/data/k5-in-4.sha256](operator-source/operator/request/data/k5-in-4.sha256) | 375 | `ddef0ce9284f893c97c5407676af561141cbe94b0dea196ed17df5504d4dbacf` |
| [operator-source/operator/request/data/k5-operator-files.paths](operator-source/operator/request/data/k5-operator-files.paths) | 294 | `1790f6b6654e68d5b5d97300d69dda04b4c0c3682f0f7e042bb66e8109da2bca` |
| [operator-source/operator/request/data/k5_preconditions.py](operator-source/operator/request/data/k5_preconditions.py) | 18706 | `b92b33893bc50a0257433b86911065bc761c2fa5fce62e48e9ea96a589be0a9a` |
| [operator-source/operator/request/data/k5g_gate.py](operator-source/operator/request/data/k5g_gate.py) | 21441 | `1dd43e10c0469baf7315a0b207e5127afdd9750ec389419a5f2b3dd14f8f72a5` |
| [operator-source/operator/request/data/probe-k5.sh](operator-source/operator/request/data/probe-k5.sh) | 5901 | `46cf87ebdee8e201ab1d05bfe93c2f75f31d07ad95f3aba981093eab72a1af39` |
| [operator-source/operator/request/steps/H5.sh](operator-source/operator/request/steps/H5.sh) | 3653 | `d1822f3022431c5c191fb5aae260c3cb2801ead2d9ac6c943639b2c8eff9df9a` |
| [operator-source/operator/request/steps/K5M.sh](operator-source/operator/request/steps/K5M.sh) | 7395 | `649548a07de4d58d6129cfcd14cd468c030106b9428fa60fb31a6e0954688fe5` |
| [operator-source/operator/request/steps/K5a.sh](operator-source/operator/request/steps/K5a.sh) | 6997 | `0daa6a5ed1514e568a110e1c5c11a0d5e6c0487c9b913b42d0a7844e1ee12318` |
| [operator-source/operator/request/steps/K5b.sh](operator-source/operator/request/steps/K5b.sh) | 6755 | `7094ff6f1e8b6cc518d02c8cdfbe47ead670e8e77f85e2c0761952c0375441d4` |
| [operator-source/operator/request/steps/K5g.sh](operator-source/operator/request/steps/K5g.sh) | 6042 | `38481f787cf5d62a4f928bcaa0db569b45e21d41c6c19c4c009d94b5c87feb41` |
| [operator-source/operator/request/steps/P0.sh](operator-source/operator/request/steps/P0.sh) | 7874 | `8240a5a05be47c4ee51738c5a63f8858c2fd0ecb67a7169e2c87921cbe5164ac` |
| [operator-source/operator/request/steps/P1.sh](operator-source/operator/request/steps/P1.sh) | 3140 | `2024bb2f62210a3feaf8f506a6d92b4f3e936ff869865ff30893912428c74761` |
| [operator-source/operator/request/steps/Q5.sh](operator-source/operator/request/steps/Q5.sh) | 6826 | `0222ecf3e0fa22147b6a2ae8baa143264a3d0e8155d9789edf448a77f8a14032` |
| [operator-source/operator/request/steps/Z51.sh](operator-source/operator/request/steps/Z51.sh) | 6871 | `823d2f673c5c2bee7104880550286dd29af84b81418340908102cbf0cdf38cab` |
| [operator-source/operator/request/steps/Z52.sh](operator-source/operator/request/steps/Z52.sh) | 1852 | `cd44598556ea96ffe61cd7575942f637ec1825f8aef05b591afaf9e742aee39f` |

## Source and actual-result reviews

The 19 files retain the source Muse and Gemini reviews and the separate completed-setup Muse actual-result review. Source readiness and actual-result adoption are distinct.

| Capsule-relative path | Bytes | SHA-256 |
|---|---:|---|
| [reviews/actual-muse/ADOPTION-RECOMMENDATION.json](reviews/actual-muse/ADOPTION-RECOMMENDATION.json) | 1926 | `503d5bc4402de136bc011697917f66df781a86e964f71f8c8518c5763c7b6b95` |
| [reviews/actual-muse/DESIGN-POSITIONS.json](reviews/actual-muse/DESIGN-POSITIONS.json) | 3 | `37517e5f3dc66819f61f5a7bb8ace1921282415f10551d2defa5c3eb0985b570` |
| [reviews/actual-muse/FINDINGS.json](reviews/actual-muse/FINDINGS.json) | 5784 | `7e67504224011614800b44e16d73f153a8465d1b273d656a0b1c1729cfd212d7` |
| [reviews/actual-muse/PROCESS.json](reviews/actual-muse/PROCESS.json) | 1018 | `3579c9b216ab6a3f7d240aaa24f7080dfd1dc046ce4b4395a3ce72c6469e010e` |
| [reviews/actual-muse/READ-COVERAGE.json](reviews/actual-muse/READ-COVERAGE.json) | 3221 | `e2d6d916a669a1f63b162573ffcd05c33fb117597e43d0247613af48d877a634` |
| [reviews/actual-muse/REPORT.md](reviews/actual-muse/REPORT.md) | 5716 | `3d4f510140ebddc86d27608fe0f1773c7f39d6ccdf8fe5c33ccec454df5ad20b` |
| [reviews/source-gemini/ADOPTION-RECOMMENDATION.json](reviews/source-gemini/ADOPTION-RECOMMENDATION.json) | 1814 | `21d3d00be1265a0b28ec35110b5d5cf364e50c7690c4aaca9454e0dd7be88aea` |
| [reviews/source-gemini/DESIGN-POSITIONS.json](reviews/source-gemini/DESIGN-POSITIONS.json) | 12071 | `478c4b8e4e86a9e1a8df00af500f26d5def692c0dc4d727a9b43f5f1e1cdd178` |
| [reviews/source-gemini/FINDINGS.json](reviews/source-gemini/FINDINGS.json) | 6808 | `b464472bea9ff77378ab9a84ca16e982cc3b1287aeeb2e2955dde31a61f0d84d` |
| [reviews/source-gemini/PROCESS.json](reviews/source-gemini/PROCESS.json) | 953 | `f1bddec0400154ea5351c5f8f74543a4e4a4d4bf677ccb3439c96a355722ef06` |
| [reviews/source-gemini/READ-COVERAGE.json](reviews/source-gemini/READ-COVERAGE.json) | 10775 | `9382fae31a011cb84753bd94776a14ccf7881976792ac311e456e560cadf84fe` |
| [reviews/source-gemini/REPORT.md](reviews/source-gemini/REPORT.md) | 13145 | `02e41974a8fab08f48c0e94af0afca8e8f862002c38fa79683b5ed700393ff82` |
| [reviews/source-gemini/ROOT-TERMINAL-SELECTION.json](reviews/source-gemini/ROOT-TERMINAL-SELECTION.json) | 367 | `29bb7b0eba6c115ecb49aa802d136030cfa2189418cfd051757c0fbd21fba6c5` |
| [reviews/source-muse/ADOPTION-RECOMMENDATION.json](reviews/source-muse/ADOPTION-RECOMMENDATION.json) | 1606 | `d37b86603fe1fb22d31d916125fb187bc60207a6239dae1069bbb422a459dd9a` |
| [reviews/source-muse/DESIGN-POSITIONS.json](reviews/source-muse/DESIGN-POSITIONS.json) | 5954 | `523798ea9b8a621eeb259fb818abe9d1a7157ce7821a807a2bb2579009250af3` |
| [reviews/source-muse/FINDINGS.json](reviews/source-muse/FINDINGS.json) | 3397 | `99a75dbe2ce37e6f7f84214accefba4a20d3c79b70481a23c18bdfaa56044dfc` |
| [reviews/source-muse/PROCESS.json](reviews/source-muse/PROCESS.json) | 1005 | `fe67a9138c560dc3779de6f1921db084f7060e5b4fd46acfb9780b089dadc6e9` |
| [reviews/source-muse/READ-COVERAGE.json](reviews/source-muse/READ-COVERAGE.json) | 2486 | `9f674a9fa9b194b5a9014b29c2d482d57820761e7f27e76b6971b6b66e30383c` |
| [reviews/source-muse/REPORT.md](reviews/source-muse/REPORT.md) | 4603 | `74a897bc81de1174e91a0f8ceed6b8bd5ec30e908cdf754f5520d7f4851baa74` |

## Root records

The 13 files preserve authorization, exact scope, pins, raw observation, actual adoption and the result manifest. P1-root-stdout.txt is the empty root-side result row.

| Capsule-relative path | Bytes | SHA-256 |
|---|---:|---|
| [root/ACTUAL-PUBLIC-DEPENDENCY-PINS.json](root/ACTUAL-PUBLIC-DEPENDENCY-PINS.json) | 9869 | `97f3ad044466ec369b4cb69f8cbdf4ac5a108ccb9dbe519ef6c7c316fd9967af` |
| [root/EXACT-K5-R2-CLAUSES.json](root/EXACT-K5-R2-CLAUSES.json) | 6272 | `555e4abe36e87076ff38181e6f49c4331b92c7441571aabd0934739229716405` |
| [root/K5-ACTUAL-ADOPTION.json](root/K5-ACTUAL-ADOPTION.json) | 2339 | `6ce800db246abfed5044645b499783c5a07ec07268dcc350bc916bf10ffc7e44` |
| [root/K5-AUTHORIZATION.json](root/K5-AUTHORIZATION.json) | 950 | `ab3766258f713e294e59bf6242fa9049054554bdf5fd2b937e9cef74b2e91506` |
| [root/K5-EXACT-RUN-SCOPE.json](root/K5-EXACT-RUN-SCOPE.json) | 7862 | `3adaf14557393017dccb8ba837bb7f1a1769733de30fd7fd0d244cbf8cba6de5` |
| [root/K5-MATERIALIZATION-PIN.sha256](root/K5-MATERIALIZATION-PIN.sha256) | 200 | `34c4f9e8820bf93a874a8e41de7256f1bd9ced9b4d40e2aa758bb8ea4be54c9f` |
| [root/K5-OPERATOR-PINS.sha256](root/K5-OPERATOR-PINS.sha256) | 3732 | `ab6b3e1475eb38ee1d902d32a623cb2429418ff3df16847b11554dcf02bb762b` |
| [root/K5-PRE-LAUNCH.json](root/K5-PRE-LAUNCH.json) | 197 | `de6ee0bc3060ff6c124215e4f3fd3f43e446b42e275c5fd1e881ffc8c756914e` |
| [root/K5-PRECONDITIONS.json](root/K5-PRECONDITIONS.json) | 2154 | `56ad390457d8ef5309fed1c4172e662e4bf1d6d6ef183b553d360d11d4ac6fac` |
| [root/K5-RAW-OBSERVATION.json](root/K5-RAW-OBSERVATION.json) | 2225 | `079ecc363f76d63a7f6e32a86c43d50bcd539d504c5ea09b5cfb2e119ea04a41` |
| [root/K5-RESULT-MANIFEST.json](root/K5-RESULT-MANIFEST.json) | 11818 | `bceacff30d81b722464b13796aa6912bcdeb3172288ea40c9f94b79dd65e77ef` |
| [root/K5-VOTE-TALLY.json](root/K5-VOTE-TALLY.json) | 3609 | `1378425a94129962e1245c961dd7483bf99caafc43634012e3678624f77725c6` |
| [root/P1-root-stdout.txt](root/P1-root-stdout.txt) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

## Captured public run results

These 44 logs, receipts and journals, together with root/P1-root-stdout.txt, make the 45 actual-result rows in root/K5-RESULT-MANIFEST.json. No protected artifact body is included.

| Capsule-relative path | Bytes | SHA-256 |
|---|---:|---|
| [run/Z52.exit](run/Z52.exit) | 2 | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
| [run/journal/H5.txt](run/journal/H5.txt) | 108 | `acca3ec92662e0ff0b1223e18f467df43ac9f6a819268ae37c24b7762b62bcdc` |
| [run/journal/K5M.txt](run/journal/K5M.txt) | 108 | `22c1ec7fc5404ec9f8a71fd6caa723bac66cd4f4a9eee1777d9e3ae60454c302` |
| [run/journal/K5a.txt](run/journal/K5a.txt) | 108 | `b07ada31becb429c86d6a6437b41bbdf0670ba8fa2424398fba924bc0f541c69` |
| [run/journal/K5b.txt](run/journal/K5b.txt) | 108 | `2ae9d25ae08e248e935d94ba2488e0ae21cb112b6cc3d89a0b2a81fb1af0802e` |
| [run/journal/K5g.txt](run/journal/K5g.txt) | 108 | `0e3b010ff78b5e1e9a3b1818534afb2ff7e3c6f5dcbedb86c0cf79b1ed108920` |
| [run/journal/P0.txt](run/journal/P0.txt) | 108 | `0d6280866c63cfa6eb6adaa0e794a7ca939b208e05c7fac3ce737d20aa4bb98d` |
| [run/journal/P1.txt](run/journal/P1.txt) | 108 | `11137a069029be70aeb28aa372990c71d65fa439ade5fe07b79611dcb1c5f292` |
| [run/journal/Q5.txt](run/journal/Q5.txt) | 108 | `459e1fa9c01f37975402ff6556c705af63152ee089b0f19c9696746e57b077e3` |
| [run/journal/Z51.txt](run/journal/Z51.txt) | 108 | `186c4f34908be90de8aefa763796171844404fa7a67a6d7ba4da972a76363393` |
| [run/journal/Z52.txt](run/journal/Z52.txt) | 108 | `3f910c2d3fa27d356293d697a73f1349df368e23a26b88060e1e32b0bcb6517e` |
| [run/logs/H5.exit](run/logs/H5.exit) | 2 | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
| [run/logs/H5.log](run/logs/H5.log) | 5707 | `be2c1a40214b2be3dde25c9403b221b0086be168409cd784cd8dee34f0cc916c` |
| [run/logs/K5M.exit](run/logs/K5M.exit) | 2 | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
| [run/logs/K5M.log](run/logs/K5M.log) | 1888 | `8a36fe55445cdb615328ba11dcb76f72ce049b2acdc49923940b8b1bd3c33dc6` |
| [run/logs/K5a.exit](run/logs/K5a.exit) | 2 | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
| [run/logs/K5a.log](run/logs/K5a.log) | 3082 | `8c501820a0d474fae4776932b6b0ccb522273f734108d30b44e98efa160e41a5` |
| [run/logs/K5b.exit](run/logs/K5b.exit) | 2 | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
| [run/logs/K5b.log](run/logs/K5b.log) | 3323 | `0755bbef95b9383b58ff30c732b2f594aa044360568b162d7d49764568c43856` |
| [run/logs/K5g.exit](run/logs/K5g.exit) | 2 | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
| [run/logs/K5g.log](run/logs/K5g.log) | 3751 | `2c365717e1352fc20dbe1b8c674331bd6eb2a62fcf5855f8c21b2d37f14d8dd4` |
| [run/logs/P0.exit](run/logs/P0.exit) | 2 | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
| [run/logs/P0.log](run/logs/P0.log) | 4773 | `30ebb442a79ce1b146af1d680b96553539dfca1d228e340e72f8bbdd3c738a35` |
| [run/logs/P1.exit](run/logs/P1.exit) | 2 | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
| [run/logs/P1.log](run/logs/P1.log) | 3734 | `219c55481b0d9def26320fe1673d2223464419d99be72839cbb0314940e7c5eb` |
| [run/logs/Q5.exit](run/logs/Q5.exit) | 2 | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
| [run/logs/Q5.log](run/logs/Q5.log) | 2981 | `34f5d8528a0f9498daa9df508f09e5524f93551451b3c81dcc22422ee214c35b` |
| [run/logs/Z51.exit](run/logs/Z51.exit) | 2 | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
| [run/logs/Z51.log](run/logs/Z51.log) | 2528 | `51c0a3a79193ab019ea2b669e138f52b0032341fcb228e2849341dd62171843b` |
| [run/receipts/K5M-abnormal.txt](run/receipts/K5M-abnormal.txt) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/receipts/K5M-inview-time.txt](run/receipts/K5M-inview-time.txt) | 444 | `3f7dec81e5d23d2682cb598b445b0d93d0836dc0c20a1abb1c376e1c3eda7cd7` |
| [run/receipts/K5M-summary.json](run/receipts/K5M-summary.json) | 504 | `d4feb7f99a6e2734ff8bad471e0b15659d799906e0f40d9c46d1c3c1fa2a2a09` |
| [run/receipts/K5a-abnormal.txt](run/receipts/K5a-abnormal.txt) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/receipts/K5a-descriptor.json](run/receipts/K5a-descriptor.json) | 2038 | `7bae4fb1ded96b8c767acfabc135bb54aafab0c8a13a648686da32d38696e959` |
| [run/receipts/K5a-inview-time.txt](run/receipts/K5a-inview-time.txt) | 347 | `4dc575c44e0990be0e0b86cc721f66a76adbb32baa57619030543fa86e766c6f` |
| [run/receipts/K5b-abnormal.txt](run/receipts/K5b-abnormal.txt) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/receipts/K5b-descriptor.json](run/receipts/K5b-descriptor.json) | 2277 | `8cf25094747b774bfda66d91c76999c6c3c6e845313cea6363209fe0ed67192a` |
| [run/receipts/K5b-inview-time.txt](run/receipts/K5b-inview-time.txt) | 349 | `a5182f01154aa65812eaf4f1ca349cae86704e7f08dc681710b7fbbb34ce83af` |
| [run/receipts/K5g.json](run/receipts/K5g.json) | 3724 | `71b4ee8f2cbb57dc9a756ae712ae3334a077efe701acd9daf590f89671f35890` |
| [run/receipts/P0-bin.sha256](run/receipts/P0-bin.sha256) | 74 | `346163ce957d5a60b1c452db18b02f57b004dd4e19e6b9146d03d54cda6869db` |
| [run/receipts/P0-root-pins.sha256](run/receipts/P0-root-pins.sha256) | 3649 | `354528a0d7cec292ec47edea327f5d39b3e254f45af8b164c49a6ffaa5a0a8ca` |
| [run/receipts/Q5-abnormal.txt](run/receipts/Q5-abnormal.txt) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/receipts/Q5-inview-time.txt](run/receipts/Q5-inview-time.txt) | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [run/receipts/stage-K5-final.json](run/receipts/stage-K5-final.json) | 5139 | `3c8a168ecea15c1ebb4c7435fe05e5c769dbd4bea209c28f50c01ba2b9a80f28` |

## Separate provenance scopes

The separately published Rust package has 160 files at commit `515bd5c5`; its source inventory and README are not selected payloads in this capsule. Its frozen Rust Core/compiler/driver closure and ordinary observations do not establish a clean external installation/build: F03 remains unobserved. K5 reused the reviewed K4H binary rather than rebuilding that published source package. The graph publication at `dd449bc9` is another separate source/navigation scope.

The included [K5g receipt](run/receipts/K5g.json) pins the reused binary to SHA-256 `d446ab0c0dececae719fe6583532f6c2a4285f67fbc89893d61251adff190902`, the invoice R1CS to `e458153e583e0a817fbab08f24f4ba16465e459969c1f8cd4e13b801e82694c0` and the predicate to `0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd`. These identifiers do not bundle their referenced binary or protected artifacts and do not establish a consumer-verifiable VK-to-matrix link.

This documentation is a source proposal pending a fresh independent packaging audit before publication.
