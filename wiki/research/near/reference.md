---
title: "NEAR source and coverage reference"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, near-teardown, research]
---

# Source and coverage reference

- Repositories: 515 inventory entries; 513 commit-bearing snapshots; two verified empty repositories.
- Tracked source files: 146,868. Graphify detected62,191 supported corpus files including37,120 code files and9,928 documents. Detection is not full semantic reading.
- All515 AST graph outputs together contain485,028 nodes and1,125,507 edges. These are sums across repositories and include duplicated/forked code.112 parser-warning lines are retained; graph structure is not a semantic proof.
- Repository/doc/concept catalog:930 nodes,1,247 edges; per-repository file-ownership and AST graphs remain separate for scalability. The catalog is a navigation layer.
- Scrapling: all348 sitemap URLs attempted;331 HTTP200 and17 HTTP404 (Nomicon gaps). All checked-out repo docs remain local too. Current online docs can differ from source snapshots.
- PixelRAG: five selected official-paper PDFs rendered/chunked;31 repository PDFs rendered into776 pages. Rendering is not reading. The runtime study records exact Nightshade/Doomslug visual pages; other rendered pages are not represented as inspected.
- Three Fable5.1 medium and three GPT6Astra medium studies and exact same-candidate endorsements are retained. This is advisory research consensus.

## Selected primary evidence captures

Full corpus manifests and source files live in the research workspace; the vault preserves the primary sources used in the expert claims and supplied study packets, plus ledger records and paper attachments.

- [src-9d16fe02712e77204b1b](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/core/primitives/src/transaction.rs) — captured SHA256 `46c1fb51009f84d758ced32e5c6aea7f032929e53edc866fa9efff0e9d7bd5f4`.
- [src-786a41f41bdc2b3dfe2e](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/verifier.rs) — captured SHA256 `99faa55ed73c6e49b1f7cf86152d7e39fe1edd935517a8dbadb2713ff787d8d2`.
- [src-1707c72042372af4d428](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/lib.rs) — captured SHA256 `b8b606de69c0a1c139a1ba912a5fad857e16b337dc1f80cb7394b5ea60eee1c8`.
- [src-7e5b827d2f8c98b36cff](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/near-vm-runner/src/runner.rs) — captured SHA256 `a9a4995b0619a537834df91fa4fdba7b7f0c4c69a7414847af4288bc159a09c4`.
- [src-35061486a5c3f89152b4](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/function_call.rs) — captured SHA256 `67e82ca9cda9698e803ccba1f6ffdc71466bf25d86270c581605c7eefc73ec20`.
- [src-7b498ff9788d3516ac63](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/ext.rs) — captured SHA256 `56ff8fba7e3ba689b3b78e98e9711c9794929d9217506c7b8f2a16ea8fc982a8`.
- [src-13a544f7dc90054e9888](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/core/store/src/trie/update.rs) — captured SHA256 `d129759c072e259ab1f13502f2c92d5aba653a88c38c3c81dcff2d03b563d5c1`.
- [src-8e963c87612f340e13b1](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/near-vm-runner/src/logic/gas_counter.rs) — captured SHA256 `6112fc0cf5568af3fa8dcc5a8aec6097df95f30b64a0b225a7c1f639fb6632c6`.
- [src-e13327a35abbfb2b95b3](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/core/primitives-core/src/version.rs) — captured SHA256 `9a8192c6b2080e86209d868daee1348f4eb70418d024a9775caece899744bad0`.
- [src-a38dff5219e581f77645](https://github.com/near/docs/blob/c06865496870cdf8fa985424eca2b13ee2ac6ecf/protocol/accounts-contracts/access-keys.mdx) — captured SHA256 `5a7f6556ec752408046efe03e62f9ade0f96005f0f75abb4ed4a142b7322d8a3`.
- [src-cd425761556d2a448354](https://github.com/near/docs/blob/c06865496870cdf8fa985424eca2b13ee2ac6ecf/protocol/transactions/transaction-execution.mdx) — captured SHA256 `46b9755d9fdcdb9374aefbc815f61d02a2c119819025ebaed345a9265b1279a2`.
- [src-d538804da73ce57bdf82](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/chain/chain/src/chain.rs) — captured SHA256 `a1ce45d27faa57521aea3c3c3e4af0377385ceda730e38a3b3387aa593c2170b`.
- [src-3069d7959082b00b1c26](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/near-vm-runner/src/logic/context.rs) — captured SHA256 `accc83c5dc2324e8d31e93d7bf8ea63b464ec6da2c7f4a067415cb9f69a034cb`.
- [src-4e28082048c8e62f2d64](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/near-vm-runner/src/logic/host.rs) — captured SHA256 `51c09ee79eed0c5f9764982d84be1ca2c3bd5db48812e7c93426ab2a22c5de85`.
- [src-c11c1e3d19bb8211bae6](https://github.com/near/NEPs/blob/dcc0bcbd452f68c936696e5f9552e567c3b162e0/neps/nep-0509.md) — captured SHA256 `d6bf11c4a7ac65147cf0be8020ccefc0128a988eef271e9c57e5caf8fd62a291`.
- [src-8878b78b06a49f95f107](https://github.com/near/nearcore/blob/a47cf412bf55b020421e99e5fc5b15ed970a7a61/runtime/runtime/src/actions.rs) — captured SHA256 `b9167d9e8ea685bcfd97affe34ec98414a97953986f4d761bed6f626b64475f2`.
- [src-9ae1abee89534c020a79](https://github.com/near/docs/blob/c06865496870cdf8fa985424eca2b13ee2ac6ecf/smart-contracts/security/cross-contract-calls/callback-panics.mdx) — captured SHA256 `654ae91501a25bc802df2aa89656213c6e08a247ceb7d928ff2608758bae625d`.
- [src-3ad713c4a046472f4190](https://github.com/near/near-api-js/blob/2546c85354ab888dc1f559b0e54cfac51f03b484/src/accounts/account.ts) — captured SHA256 `f52c783b28d0c2f4b3e87d2e66bf1fe5a0057d319c4fe2727f6cef1a69b5353d`.
- [src-bd31f0152a4e81e49903](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/core/src/payload/mod.rs) — captured SHA256 `2c8821d9704f6fac3827070c68a1d7c5f891f90ef1eaa1c5ee4756ff9e7026b5`.
- [src-18e53e79d5fc7c2a13f4](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/core/src/payload/multi.rs) — captured SHA256 `1a84de51cc02eb69e3097a82bb670e95ad0254436bd535332b595ded601c6763`.
- [src-1fec20ef6c4b501072d1](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/core/src/payload/nep413.rs) — captured SHA256 `719427aceb87924417a903dcc37d955875822c8201924dc02124c524ce6c2ecb`.
- [src-e4541b9813fded5a9b5e](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/core/src/payload/erc191.rs) — captured SHA256 `0296011a7b913eaebb48a06f4e5eac2b6ae702be9263c9a0dbc65652e1d201cb`.
- [src-b49800fd38227848d682](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/crates/signatures/nep413/src/lib.rs) — captured SHA256 `24252a16c756edde94d6725ef09b10d62a3042621709f497b6c1185077fa9168`.
- [src-6c15dc471dde3813177e](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/crates/signatures/erc191/src/lib.rs) — captured SHA256 `edfe5a40ff14ac672f124c8d4c5adfb0c9df7520913346bbe6dfe6b6cf2d36a2`.
- [src-ee17983c578debfee822](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/core/src/engine/mod.rs) — captured SHA256 `d1518444bc637686df75a5ff4f537fd110b622aeeda063ddb24d4ee14375f402`.
- [src-3985083ac0187b7c021e](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/core/src/intents/token_diff.rs) — captured SHA256 `94b9fce9429fed24ad6619f45c5d386505326514a7773c28f8e8085d4c6a6591`.
- [src-12d9798c9e61f28c5c76](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/core/src/engine/state/deltas.rs) — captured SHA256 `7cdcdb6c3771f21721412e0418f5f28e4d09159b7f4d15f846fda6cb7d86c461`.
- [src-6934232c187ad5edf6e3](https://github.com/near/mpc/blob/3499a2a44207df89699c9791e21623613a365a16/crates/contract/src/api/sign.rs) — captured SHA256 `bc402efe429540b11644b3da5c926db9c24ac8e8be1b03742761f8ca97da62e7`.
- [src-6decb517c7f1314a34d2](https://github.com/near/mpc/blob/3499a2a44207df89699c9791e21623613a365a16/crates/near-mpc-crypto-types/src/sign.rs) — captured SHA256 `a1c1a442b6e473c0cf7de432bbb1b9da58b2d31de0467db180683b6d7796c965`.
- [src-9008f1dbb8f21e37fcc7](https://github.com/near/mpc/blob/3499a2a44207df89699c9791e21623613a365a16/crates/near-mpc-crypto-types/src/primitives.rs) — captured SHA256 `eb2430a39892667143764467dcce3bc057efc39a81b746109687517f82c2731c`.
- [src-969228e71efd216b6074](https://github.com/near/near-sdk-rs/blob/4ba939119ea3f9a559d6d45e82859419f5a09a20/examples/callback-results/src/lib.rs) — captured SHA256 `baeb194a63b077e16286bdf14610c47c7ee4f591a842868959d598f2b75e2446`.
- [src-bceba1009c10797eb0c9](https://github.com/defuse-protocol/one-click-sdk-rs/blob/f9741c3dcfe406dba2ce8c2ae73a6e025dbaafd9/docs/QuoteRequest.md) — captured SHA256 `cc74965c37da2778c84b00f8df6a2a862c0bff12a536b82b3a684f48a8ae4ab9`.
- [src-ec52de8c124d9343ca6a](https://github.com/near/docs/blob/c06865496870cdf8fa985424eca2b13ee2ac6ecf/chain-abstraction/intents/overview.mdx) — captured SHA256 `80d6e3d635cf57e82a2fe99ca9eba090667f6a9f49bce3d557a5265da55d25e3`.
- [src-6e8c911a3257a462c37c](https://docs.near-intents.org/integration/distribution-channels/1click-api/quickstart/signed-intent-execution) — captured SHA256 `5348bab6cdb992b312068fdb70c579967c622ea1fedfc0233eca4d0d07b29c48`.
- [src-604100171711b64f4125](https://docs.near-intents.org/integration/distribution-channels/1click-api/verify-quote-signature) — captured SHA256 `faa28576e0613a99877e4d72ad74ab6b34f5e707c0fc8af5ab7f56eec069dc01`.
- [src-6a3591a1e1a52bbb1b9f](https://docs.near-intents.org/integration/verifier-contract/signing-intents) — captured SHA256 `23ac45c8a3d13707bfc6d971d0ac30c522b6b1889137abddde93a95a434ae479`.
- [src-392156a7d47016310486](https://docs.near-intents.org/integration/distribution-channels/1click-api/about-1click-api) — captured SHA256 `f36e29db274aee88f57555463563e9f97695695a1b36b1f23211d7fa977288ac`.
- [src-49d8ad506ae5265fa684](https://docs.near-intents.org/integration/distribution-channels/1click-api/authentication) — captured SHA256 `1fd898e322e755222dc8c1278d70fc135e3cc7a004a74360f150e4dd4b9450e2`.
- [src-5bc9923699cf46ee3f5e](https://docs.near-intents.org/integration/market-makers/message-bus/introduction) — captured SHA256 `6f93e738325cd11e02de11cdaf929d3ea9e227636ba0cc808ecbc0ca9a2f602a`.
- [src-b12a25868fe0f1c112be](https://docs.near-intents.org/integration/market-makers/message-bus/rpc) — captured SHA256 `4357ba42c571ebb5e866ef255acd548763a7f81dd083422a657d0cffb2f7057b`.
- [src-62f21fdefe8edcd195a9](https://docs.near-intents.org/security-compliance/risk-and-compliance) — captured SHA256 `db1d241c4586384d7911c59923d7a31885217a7c36927db7d0bcc15b575bb946`.
- [src-73d3cf0d5335a6b1bb55](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/intents/mod.rs) — captured SHA256 `95d9ae3f8f99e6d1d78976f25ffbcefa8973cffa2a375fb4bbfc27806b9823df`.
- [src-465a961a59449b1f4e50](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/intents/state.rs) — captured SHA256 `cd25c8fb8619e0456dc2dbcdc18a441668ccd33ba6181a0777185ef20578ff4e`.
- [src-f122c29b1a89bde2127e](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/tokens/nep141/deposit.rs) — captured SHA256 `d2314633aa1553c958870569d9f0d1fff60c8ac8b3b8a2bb0daf033b0756c3fc`.
- [src-7c8be3a1e9f54e40420c](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/tokens/nep141/withdraw.rs) — captured SHA256 `c190ca5da85578557bede34cde975a0de8400bd1fc9fa92616e5d884d292b3f4`.
- [src-92578a1c271e1e460b4d](https://github.com/defuse-protocol/sdk-monorepo/blob/b6bab503e09027b40df3df14ca3610a11ff3d40d/packages/intents-sdk/src/intents/intent-payload-builder.ts) — captured SHA256 `7be160458c23cb0bc1fa68f0785fcbe4e2571de973d9fb26a026500d2308dbb9`.
- [src-afcd918a935c709da492](https://github.com/defuse-protocol/sdk-monorepo/blob/b6bab503e09027b40df3df14ca3610a11ff3d40d/packages/internal-utils/src/solverRelay/getQuote.ts) — captured SHA256 `5b4e19a15aabe73094f1ef84a5a8f40e2584bbe6032a1611e48135164264bd3a`.
- [src-ddf9c43e92a030789610](https://docs.near-intents.org/api-reference/oneclick/request-a-swap-quote) — captured SHA256 `62f29e581857cc967940abc0f3d72fbb1d8dbd984fdfc4ce88ce8a71a9591f92`.
- [src-e64eba86d3c0692a537d](https://docs.near-intents.org/api-reference/oneclick/generate-an-intent-for-signing) — captured SHA256 `23ec867696cc2e2ddd14a6fd823acffda08f0979d2c9eddc4ed489e367fc46ee`.
- [src-afccc84b0ce70f1e0b0b](https://docs.near-intents.org/api-reference/oneclick/submit-a-signed-intent) — captured SHA256 `7e9a32b418094b70b73577fddb487fd52a3bb6592f0062147c439a3c85d986ee`.
- [src-21a3537e952382724c5c](https://docs.near-intents.org/integration/distribution-channels/1click-api/quickstart/making-a-request) — captured SHA256 `92916a4b2aa4c3088eb8e003863ecfcb07842a3e4eeb1fcfb5317d67ee473807`.
- [src-33f811d930b2a7e1da8f](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/accounts/account/nonces.rs) — captured SHA256 `09f5a940f3e62ae736b5a5a05b4d6f5d0d509a0e227a731b6adf36760c07ccfb`.
- [src-45cefd62eab6d66c6302](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/core/src/intents/mod.rs) — captured SHA256 `47d9a5711e596373cc0b7c45f2b0416df0396ae016c98eaf0e923a1c5b597241`.
- [src-64528a2713d1347c3db1](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/core/src/intents/auth.rs) — captured SHA256 `e51b17069547a3d91d73844b9489e29e3cea3671a448424609c06e6392379f70`.
- [src-ff9f06464ef87924fd5f](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/intents/auth_call.rs) — captured SHA256 `1b9fa6df4408dfecffd0520c2469b52233033d63b6502285155b64e2a6d091ec`.
- [src-82fc0071a5d11ac0d0b7](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/core/src/engine/state/cached.rs) — captured SHA256 `28160001c13774234707066b6875e8af3e494993fa5137e3b6121a73e453d13f`.
- [src-937b2287c401aeb10e1e](https://github.com/defuse-protocol/gitbook-docs/blob/589b25d612161de1a463316d90ff19759dd994eb/market-makers/verifier/simulating-intents.md) — captured SHA256 `cd79686b86a9108258358355fd17ddc3ec590783e7ab47aa69cd4d5c0b35822f`.
- [src-aff217d6ec41cf87e4ad](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/tokens/mod.rs) — captured SHA256 `9ceacc45c79c529b7c5176cb562c983378487348c42bcd1dfbd8c90c42d77bb1`.
- [src-5af0899fdeae61383de5](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/admin.rs) — captured SHA256 `b171af34825bd93e8cdeacc2f147db230bfa3474e6003f708104b16e4b966340`.
- [src-baac35344d2e6398c0ba](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/upgrade.rs) — captured SHA256 `bde68ecc9dd92a2e54fcfa117eae20b2d37e9b5b95b20de85ddb92a637dd7188`.
- [src-0dd913a128d63e79e7e1](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/mod.rs) — captured SHA256 `82190723e5fe0b24e80d34b330f4f44c212898509017c3b848c7119c0e8bedac`.
- [src-a2912e324979e3e8637b](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/defuse/src/contract/intents/relayer.rs) — captured SHA256 `c6e8e218e6636408af906726b2aaab7366e04d3370c6f076d7fd820fdeb545f9`.
- [src-8bc0fceca08a3511cb99](https://github.com/defuse-protocol/near-intents-amm-solver/blob/0f29eb26df7bb36bcb6fad97919680edfc8bdebb/src/services/quoter.service.ts) — captured SHA256 `7e66342c8faaf057f9abb72244d1e22c51f51064b5e763b08e5f8ca697ba0e63`.
- [src-39ab8de4e2c5fa9c43b7](https://github.com/defuse-protocol/gitbook-docs/blob/589b25d612161de1a463316d90ff19759dd994eb/market-makers/verifier/introduction.md) — captured SHA256 `62e32a0facf3bb5661b45756075d0db1edb9a9000e8aa7f74d796667bb9b87c4`.
- [src-87599696b6e17936798c](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/state.rs) — captured SHA256 `86b3a72b5f113b7e1013f54189395097a3613016dd25cc620bf9d99d04f8bc9c`.
- [src-1eecb9963da1b4869eeb](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/contract/fund.rs) — captured SHA256 `a85b825359e49363dc0613e7ae13122f08871b5cd552be399f2d8f7417c4b427`.
- [src-5649e22903eae96621dc](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/contract/tokens/mod.rs) — captured SHA256 `3316a62c36d2cf59f2bbfe9912800c99f07d697c7735dce89d41bbfa047cb23c`.
- [src-49629810b2731716e78b](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/contract/fill.rs) — captured SHA256 `0d835d47bc053a881c276464ae81423f7e779e4ee116dfce82e0aad10e7cc94e`.
- [src-7f3947fff7909abbb124](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/contract/resolve.rs) — captured SHA256 `8bfc89fcb486692d7c74ef46d6886e9e517f798f51d6df7d9fde2123c9e08c3a`.
- [src-e678187dac45348d1020](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/contract/auth_call.rs) — captured SHA256 `9ef49697e6b9e5e9b5dd744128cb380db33365e3fa833f348790e05b0dc104be`.
- [src-83eb5b502e5c75127150](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/contract/close.rs) — captured SHA256 `9104e7d896bbb6d72b070a54ae27d20822023bf58bc11fbb216c92dac838b6b1`.
- [src-d02623459d4d7872b1e5](https://github.com/near/intents/blob/32a7836f825e8c984c26149f4456793ec7e3d49a/contracts/escrow-swap/src/contract/cleanup.rs) — captured SHA256 `e706f62cae96edeaffcae05ca9909947fcb766ce290f9b4d59e2a40a46375c15`.
- [src-670f98ae79e30d07311d](https://docs.near-intents.org/integration/verifier-contract/intent-types-and-execution) — captured SHA256 `6acac7ca680b4077c7fba8a63e4a699c779de707eea53096db0e764e9d9721a2`.
- [src-c62e8a68d3cc7a551305](https://docs.near-intents.org/integration/verifier-contract/introduction) — captured SHA256 `14eab7b859ac1230f18d74c95d24b9a87e136411dbcb31daf3f495c48bb1d7ad`.
- [src-a273b2cadceaa8698567](https://docs.near-intents.org/integration/verifier-contract/simulating-intents) — captured SHA256 `dde98d1f72346bc79a87d6cc5fd69e2de34e1cec1f3081da4f3f357c16d13a57`.
- [src-9c1a30c7a972832be80b](https://docs.near-intents.org/getting-started/what-are-intents) — captured SHA256 `1b35d50b3acb4222fe4077ecc72dcf4aeac0ac5693f950ffc52d9096e2148cfc`.
- [src-90f181483cd8dc85a1e6](https://docs.near-intents.org/integration/distribution-channels/1click-api/quickstart/introduction) — captured SHA256 `bc4cf20681db41a5b2c28e79273bd01c0808f56893b7405903791369fa44f8ce`.
- [src-1817f971b2e66202f9e4](https://docs.near-intents.org/integration/market-makers/confidential-intents) — captured SHA256 `b387fca8dd984143065b4541ac3712a4adf44c25e4b1cc9384a9b46b162ed0a5`.
- [src-fb34e560ed512e807355](https://docs.near-intents.org/integration/market-makers/introduction) — captured SHA256 `ec3a2ad6c0e35cb049a010fa0f1694e2a7b200f82b3e084445752cb950c068e9`.
- [src-78ee402b7c90608c3145](https://docs.near-intents.org/integration/market-makers/message-bus/guaranteed-delivery) — captured SHA256 `4eec54b913b3fdea1dea29fa98fd187ac75e9802e2a4e5c9c144710d1e0556a4`.
- [src-62c2a6bc6aa84d793220](https://docs.near-intents.org/integration/market-makers/solver-terms-of-use) — captured SHA256 `4824fa539b0c9133b555f77df2c8ca3d3fe6873dfdfa131c7add2bfd215340f7`.
- [src-bf71ebb4b274b966bedd](https://docs.near-intents.org/resources/fees) — captured SHA256 `1676d8f37d9ed923de27664fed2619c96f9e6d30471210b3917a533fcd94f884`.
- [src-1c22e624eb22b256ab6a](https://docs.near.org/chain-abstraction/intents/overview) — captured SHA256 `60bd0a8b42ab0c6e1f59efac16a5eb765fcb1466baa5f2ad5350b12ca9151e97`.
- [src-c0ce8431ff439b159688](https://docs.near.org/primitives/liquid-staking/liquid-staking) — captured SHA256 `14abb6221d28e00885d18889ce5f9de527c375d49d7972b6410149f2894e4ed5`.
- [src-1c3abaae9ab621b256cc](https://docs.near-intents.org/integration/bridging/overview) — captured SHA256 `4ca4d9010c9c45aa524e4d19f09c885fc5bf3d454a43e3d0a6afbb4be7bc2eb9`.
- [src-cf49ad82d40a484068d1](https://docs.near-intents.org/learn/omni-bridge/overview) — captured SHA256 `68bf8ff345e22c9ba386dc4d033191392c4c0e43de7940c41d72e3d568ee9f12`.
- [src-d848d5e193fd15ca4763](https://docs.near.org/chain-abstraction/chain-signatures) — captured SHA256 `b6c4f274f200d99a9185e5b1214d6040cc1f32add11eb8992f0ead01f11fb7c4`.
- [src-1ae5f94de8ce9ba9a966](https://docs.near.org/chain-abstraction/chain-signatures/implementation) — captured SHA256 `073a385496e684c2ef610f1f44423c886e3b306c3e47420a4170d324ddc09849`.
- [src-c38c37925c672a3573b4](https://docs.near.org/chain-abstraction/omnibridge/overview) — captured SHA256 `fb470e2357b3ffa7616d894f2ba72ee13a2ea85d14a51c2ae2944b397a132bd7`.
- [src-0aef6919b10df13398cd](https://docs.near.org/protocol/transactions/transaction-execution) — captured SHA256 `a6a5206d0a6f68f1cb715cb15e84840e3e88e15da616577cc1593e9e5fd70d2c`.
- [src-39c07df91d05ed0007aa](https://docs.near.org/smart-contracts/anatomy/crosscontract) — captured SHA256 `b01e6234d311e4ad206d1d949d2115dd80824d152f30cecc01f34250c1acda28`.
- [src-4dc9e9c22ca29bbd331b](https://discovery-domain.org/papers/nightshade.pdf) — captured SHA256 `a88117e9ed380364a71b7480dea8f4ccaf1a22092ad02c5bad018bcff2ed027d`.
- [src-e4a32986449b3fb83790](https://discovery-domain.org/papers/the-official-near-white-paper.pdf) — captured SHA256 `874e8bb2568644c8b7847bdf3500fdf740af57a2b0f86debd8546f88d2b43977`.
- [src-1ac11191a096f8968fee](https://discovery-domain.org/papers/doomslug.pdf) — captured SHA256 `0ebe7bc8ea21458897776650718c6b24bd7991ad56b7e533946fecf14874bb05`.
- [src-1705243c1b095e546c71](https://raw.githubusercontent.com/nearai/papers/main/ProofOfResponse.pdf) — captured SHA256 `30c543fad1a707aa9bb0bfd0cf72fbbd779e942388c0da2fe7b2e17c4d63f958`.
- [src-d6a1e297715f1699f2d0](https://raw.githubusercontent.com/nearai/papers/main/DecentralizedConfidentialMachineLearning.pdf) — captured SHA256 `0e9826334483d13e1c5380591cbfd51813d21e1bfd23cbc996ac5b876be47d9e`.
- [src-8fbf6e0ce1d944218a7a](https://xrpl.org/docs/concepts/payment-types/escrow) — captured SHA256 `120e3cc0e2b88126d619f9150dba2efee3a01fb4f8f0551698349ea4f39e6496`.
- [src-16eef9eee2f206cc5de4](https://xls.xrpl.org/xls/XLS-0100-smart-escrows.html) — captured SHA256 `533717e09614d64c453696b5a380de1ec413a09eb0122960d835535afadf559f`.
- [src-a5e0bd55d4b8401e6845](https://academy.iccwbo.org/trade-finance/article/11-questions-that-will-help-you-master-documentary-credits/) — captured SHA256 `577cce57ad68f8a2d1f8625fe4eb18e15e99304397723f8cb024fc0b1eabbc6c`.
- [src-cac45ed23d03cde62f19](https://www.bis.org/publications/aer-2023/blueprint-future-monetary-system-improving-old-enabling-new) — captured SHA256 `d9bbde6b129a622b9ea36f9942e914cd7cc884f17b5674a9b2e6ed2d31985723`.

[Full study result](../../../deliverables/near-teardown-2026-09-19/RESULT.md). Exact per-claim ranges are retained in the six study artifacts and claim ledger. Source inspection does not establish live protocol activation, deployed code hashes, admin holders or theorem discharge.
