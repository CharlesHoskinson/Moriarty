# V7 actual build, parameter acquisition and key generation

2026-10-01. Experiment observation: all three separately reserved stages completed with exit 0, no resource stop and preserved source/binary identities. [Immutable evidence archive](evidence/native-keygen-v7-build-srs-keygen-history/archive-manifest.json). Both independent actual-result audits remain required before publication; prior resource votes are not result approval.

| Stage | Observed result | Qualification |
| --- | --- | --- |
| Offline build | New ELF SHA `9e1b153a17cd55969dd800fd47bc728923963093267cc8d23765f0a6ea484559`; 696,706,768 bytes; 24.63 s; sampled peak RSS 3,265,310,720 bytes | Exact locked/offline shared-target command; no new dependency graph |
| New public SRS request | Exactly 25,166,212 bytes; SHA `4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74`; 7.51 s | New full-body request, exact hash and length; original failed v6 request preserved; ceremony independently unaudited |
| Native keygen | 183.93 s; sampled peak RSS 2,415,271,936 bytes; CPU 206.45 s; actual tagged PK/VK/IR round trips and EOF checks passed | k17 fixed JSON IR3.1; no financial proof, well-formedness or application |

Actual artifacts: PK 234,911,946 bytes / SHA `28fbadfa9db2cc227f49347140f9f7f062ced44f4f77aa2b1579e3810c6ea6da`; VK 2,745 bytes / SHA `6152e754000515f07b63b8296d6072bc4b8f98504653075ae15b7ab5f1e4326c`; tagged IR 25,252 bytes / SHA `a5a6f501847521e9744b66965b4a9917ae2a72ae2f1963271e3a60a8f1f19214`. Large public PK/SRS bodies and ELF remain externally preserved with exact identities; they are not copied into Git. Small tagged VK/IR and all stage source/metadata/receipts are archived.

The result freeze has 44 file entries plus the actual produced ELF identity; SHA `44c63a77861faca25737320db910fa404ae1081653918fceab1593e03b3e3389`. Target apparent storage is 8,503,734,623 bytes, cache unchanged; final keygen receipt observed 20,623,282,176 bytes free. Monitoring was sampled and can overshoot. Rayon limits its pool rather than every native thread; parent hashing is outside child budgets.

The actual result leaves `authority_valid:null`, proof/well-formed/application/ledger flags false, resolver/registration agreement NotChecked. Original caller, failed download and consumed attempts are preserved. Source descriptions saying uncompiled/unexecuted are immutable historical preparation labels; current execution claims come from the new receipts.

Next: two fresh current source/actual-result audits, combined where fully specified with review of the proposed next proof/independent-verification allocation. No proof runs until both required current scopes approve the exact new source and limits. Authenticated genesis/funding/account/asset/time/history, generic language/formal/PCD correspondence and conditional Preview compatibility remain open.
