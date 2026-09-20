# Changelog

## [1.2.0](https://github.com/anoma/anoma-app-sdk/compare/v1.1.0...v1.2.0) (2026-08-27)


### Features

* add created resource timestamp ([#73](https://github.com/anoma/anoma-app-sdk/issues/73)) ([fd359a5](https://github.com/anoma/anoma-app-sdk/commit/fd359a5812bb682b60244f3b0ddc8639dfab8683))

## [1.1.0](https://github.com/anoma/anoma-app-sdk/compare/v1.0.0...v1.1.0) (2026-08-25)


### Features

* add Monad and Base and fix conversion error ([#65](https://github.com/anoma/anoma-app-sdk/issues/65)) ([006ed32](https://github.com/anoma/anoma-app-sdk/commit/006ed3297779a7a8a8e19b3bd3fb899c80dfdcad))

## [1.0.0](https://github.com/anoma/anoma-app-sdk/compare/v0.18.1...v1.0.0) (2026-08-25)


### ⚠ BREAKING CHANGES

* the ./api, ./lib, ./constants and ./domain subpath exports are gone. Two entrypoints remain — "." for the domain and "./primitives" for the encoding and assertion helpers.

### Features

* slim sdk role ([#69](https://github.com/anoma/anoma-app-sdk/issues/69)) ([245f4bd](https://github.com/anoma/anoma-app-sdk/commit/245f4bd44e7369eb09a9c6756e7c17a5ce6868c9))


### Bug Fixes

* improve install on expo apps ([#66](https://github.com/anoma/anoma-app-sdk/issues/66)) ([a7d38f9](https://github.com/anoma/anoma-app-sdk/commit/a7d38f94b3668a12c6304a971d03dc73a6a436c9))

## [0.18.1](https://github.com/anoma/anoma-app-sdk/compare/v0.18.0...v0.18.1) (2026-08-21)


### Bug Fixes

* map resource chain by id ([#67](https://github.com/anoma/anoma-app-sdk/issues/67)) ([1a11d9d](https://github.com/anoma/anoma-app-sdk/commit/1a11d9df6935d6ced98a1567c114101af1d4b605))

## [0.18.0](https://github.com/anoma/anoma-app-sdk/compare/v0.17.1...v0.18.0) (2026-08-20)


### ⚠ BREAKING CHANGES

* consumers must install viem, @wagmi/core, zod and @uniswap/permit2-sdk themselves. They already resolve these today via the SDK, so in practice this surfaces as unmet peer warnings until they are declared.

### Features

* add all chain resources endpoint ([#63](https://github.com/anoma/anoma-app-sdk/issues/63)) ([906fa75](https://github.com/anoma/anoma-app-sdk/commit/906fa75a1b3ab6746d2b05e41a8aa7203050c21c))
* export arm bindings to react-native ([#60](https://github.com/anoma/anoma-app-sdk/issues/60)) ([2c8c101](https://github.com/anoma/anoma-app-sdk/commit/2c8c101cb131dd214ca03cf996aab60b5673e1f6))
* update sdk to support react-native ([#62](https://github.com/anoma/anoma-app-sdk/issues/62)) ([4bef2cb](https://github.com/anoma/anoma-app-sdk/commit/4bef2cb18a000763b4d2f0da3e7173f7fa9cec6b))


### Bug Fixes

* declare viem, @wagmi/core, zod and permit2 as peerDependencies ([#45](https://github.com/anoma/anoma-app-sdk/issues/45)) ([6cf4fe4](https://github.com/anoma/anoma-app-sdk/commit/6cf4fe47f88e997cddc23cf70ed29c7643e26c3f))

## [0.17.1](https://github.com/anoma/anoma-app-sdk/compare/v0.17.0...v0.17.1) (2026-08-14)


### Bug Fixes

* send fee estimate resources as snake_case ([#58](https://github.com/anoma/anoma-app-sdk/issues/58)) ([4519022](https://github.com/anoma/anoma-app-sdk/commit/451902216a1ca22f7a6f964fe0c2a74b487fbfbd))

## [0.17.0](https://github.com/anoma/anoma-app-sdk/compare/v0.16.2...v0.17.0) (2026-08-13)


### Features

* install arm-bindings from npm ([#55](https://github.com/anoma/anoma-app-sdk/issues/55)) ([961c9f1](https://github.com/anoma/anoma-app-sdk/commit/961c9f1ada53357376d7b4e852f6a9c6150f6573))

## [0.16.2](https://github.com/anoma/anoma-app-sdk/compare/v0.16.1...v0.16.2) (2026-08-13)


### Bug Fixes

* add repository field so npm can verify provenance ([#53](https://github.com/anoma/anoma-app-sdk/issues/53)) ([1f97e9e](https://github.com/anoma/anoma-app-sdk/commit/1f97e9e6c9ebf1542c7ece93e978c7e1b4c7e4d7))

## [0.16.1](https://github.com/anoma/anoma-app-sdk/compare/v0.16.0...v0.16.1) (2026-08-13)


### Bug Fixes

* update sdk-released flag for release-please ([#51](https://github.com/anoma/anoma-app-sdk/issues/51)) ([77a38e9](https://github.com/anoma/anoma-app-sdk/commit/77a38e96cd1ccf0199a48f20b06e24057efdefe5))

## [0.16.0](https://github.com/anoma/anoma-app-sdk/compare/v0.15.0...v0.16.0) (2026-08-13)


### Features

* add arm-bindings package ([#44](https://github.com/anoma/anoma-app-sdk/issues/44)) ([c448fac](https://github.com/anoma/anoma-app-sdk/commit/c448fac679f941ad3a34d5230e049bfbeca811d9))
* add async attribute to add_keys api ([#600](https://github.com/anoma/anoma-app-sdk/issues/600)) ([9355537](https://github.com/anoma/anoma-app-sdk/commit/9355537d74eb1aed0bd6e1fac6750de8838b31bb))
* add balance example ([#32](https://github.com/anoma/anoma-app-sdk/issues/32)) ([b5be170](https://github.com/anoma/anoma-app-sdk/commit/b5be1700c66e1e671fb3ff876966365d0c30b42b))
* add beta redirect ([#562](https://github.com/anoma/anoma-app-sdk/issues/562)) ([528db9e](https://github.com/anoma/anoma-app-sdk/commit/528db9e37f33ad3871dc55df4a51e3ab523df8d7))
* add business mode toggle ([#708](https://github.com/anoma/anoma-app-sdk/issues/708)) ([db36977](https://github.com/anoma/anoma-app-sdk/commit/db36977eb4573a9828ee9d367e3beacec1315f96))
* add business shared routes ([#710](https://github.com/anoma/anoma-app-sdk/issues/710)) ([8edb83b](https://github.com/anoma/anoma-app-sdk/commit/8edb83bdbf419c6c894b269de77c231ec706f0d5))
* add CI and npm publishing ([#46](https://github.com/anoma/anoma-app-sdk/issues/46)) ([d01fb48](https://github.com/anoma/anoma-app-sdk/commit/d01fb48ebde3caca107a0dced848f6e6e440b682))
* add connect wallet to deposit form ([#72](https://github.com/anoma/anoma-app-sdk/issues/72)) ([165a228](https://github.com/anoma/anoma-app-sdk/commit/165a2287a66e5648d6c19538756e35bd35f86177))
* add dashboard page ([#19](https://github.com/anoma/anoma-app-sdk/issues/19)) ([4ae41c4](https://github.com/anoma/anoma-app-sdk/commit/4ae41c4f85cc4c38d82176f847cb997dd32f2563))
* add deposit page ([#26](https://github.com/anoma/anoma-app-sdk/issues/26)) ([c535c0d](https://github.com/anoma/anoma-app-sdk/commit/c535c0d1e00e555b03a38f0019ae96742d5e6bcd))
* add dev script ([bfb87c6](https://github.com/anoma/anoma-app-sdk/commit/bfb87c6a54fbf3484c88bc6ec8678ac0cfc4dde2))
* add docs and feedback button ([#449](https://github.com/anoma/anoma-app-sdk/issues/449)) ([3171e4d](https://github.com/anoma/anoma-app-sdk/commit/3171e4d76bc0c9f9b67218c4e946f5482ee6c610))
* add Keplr and OKX wallets ([#578](https://github.com/anoma/anoma-app-sdk/issues/578)) ([c352c06](https://github.com/anoma/anoma-app-sdk/commit/c352c06c822b8388936dd88c90ad117524a5a5f4))
* add main navigation ([#30](https://github.com/anoma/anoma-app-sdk/issues/30)) ([81d21ea](https://github.com/anoma/anoma-app-sdk/commit/81d21eabc7bccf3b9bc65540911f3cbfb70de57c))
* add mobile instructions ([#278](https://github.com/anoma/anoma-app-sdk/issues/278)) ([4768788](https://github.com/anoma/anoma-app-sdk/commit/47687889965e69a335582e618d45083ba83d48b1))
* add modal pages ([#151](https://github.com/anoma/anoma-app-sdk/issues/151)) ([f53dcab](https://github.com/anoma/anoma-app-sdk/commit/f53dcaba6390e5fec660973be123645d6a56a5de))
* add native token withdraw generic calls ([#41](https://github.com/anoma/anoma-app-sdk/issues/41)) ([764d86b](https://github.com/anoma/anoma-app-sdk/commit/764d86b74ab9b17390ab5afa50554ce93b0c08e1))
* add network name to token set and selector ([#638](https://github.com/anoma/anoma-app-sdk/issues/638)) ([befd1d4](https://github.com/anoma/anoma-app-sdk/commit/befd1d40a5a552316f5ea91eccd693ad8c045c94))
* add optimistic resources utils ([#21](https://github.com/anoma/anoma-app-sdk/issues/21)) ([c141a68](https://github.com/anoma/anoma-app-sdk/commit/c141a6899e507353b3f405b36c5b2bd0fa388275))
* add packaging, fix deps, remove browser features ([c7ed171](https://github.com/anoma/anoma-app-sdk/commit/c7ed17122a4f80745639c0d59bd100a178c40b47))
* add pay address to request page ([#50](https://github.com/anoma/anoma-app-sdk/issues/50)) ([ca12d4b](https://github.com/anoma/anoma-app-sdk/commit/ca12d4b698341b0aabf8ac6ece73c7bd74968293))
* add payroll form ([#691](https://github.com/anoma/anoma-app-sdk/issues/691)) ([d4168d9](https://github.com/anoma/anoma-app-sdk/commit/d4168d9e388758b7b22b2cbd4b71a9b293a16221))
* add payroll overview - part 1 ([#699](https://github.com/anoma/anoma-app-sdk/issues/699)) ([4caccd7](https://github.com/anoma/anoma-app-sdk/commit/4caccd74a415f33936918affd8c835038838d950))
* add payroll util supports ([#28](https://github.com/anoma/anoma-app-sdk/issues/28)) ([3ec9109](https://github.com/anoma/anoma-app-sdk/commit/3ec91099503e98fad5ada79cc097db317b079112))
* add pending transactions to history ([#491](https://github.com/anoma/anoma-app-sdk/issues/491)) ([47f226c](https://github.com/anoma/anoma-app-sdk/commit/47f226c9b04706f2b248f896dc490f19b2c633ca))
* add request and pay page ([#93](https://github.com/anoma/anoma-app-sdk/issues/93)) ([5b776ff](https://github.com/anoma/anoma-app-sdk/commit/5b776ffe57cb100ae4410f2caf50cfddada4ea77))
* add swap feature ([#35](https://github.com/anoma/anoma-app-sdk/issues/35)) ([9f67d4c](https://github.com/anoma/anoma-app-sdk/commit/9f67d4c9e9da19d05590550b65fb013796e68fbc))
* add total dollar value ([#137](https://github.com/anoma/anoma-app-sdk/issues/137)) ([8488ca9](https://github.com/anoma/anoma-app-sdk/commit/8488ca9aff1180af45a039f3565ebbc48a29a5be))
* add transaction receipt page ([#420](https://github.com/anoma/anoma-app-sdk/issues/420)) ([6ce7a8c](https://github.com/anoma/anoma-app-sdk/commit/6ce7a8cd5413091e6533acb3e20a5079f1e1e42c))
* add transactions history component ([#335](https://github.com/anoma/anoma-app-sdk/issues/335)) ([c736804](https://github.com/anoma/anoma-app-sdk/commit/c7368042e0136449afd11d3a8e8b830fc378f530))
* add usd amount below token amount ([#607](https://github.com/anoma/anoma-app-sdk/issues/607)) ([946ed0f](https://github.com/anoma/anoma-app-sdk/commit/946ed0fe464d7cff8949287834f69a72f1424184))
* add withdraw form ([#61](https://github.com/anoma/anoma-app-sdk/issues/61)) ([fa4d543](https://github.com/anoma/anoma-app-sdk/commit/fa4d5432d67596a6591c6860def2fd8aece9b402))
* add yield functionality ([#697](https://github.com/anoma/anoma-app-sdk/issues/697)) ([a02439a](https://github.com/anoma/anoma-app-sdk/commit/a02439ad9886883895e0c08fc6c0a0badb8b3952))
* adding binance logIn option ([#486](https://github.com/anoma/anoma-app-sdk/issues/486)) ([02d1ce4](https://github.com/anoma/anoma-app-sdk/commit/02d1ce4caa2a470ab5747a3bc9d346c18614eaae))
* adding generic rpc hook + temporary hotfix ([#574](https://github.com/anoma/anoma-app-sdk/issues/574)) ([b44a6a5](https://github.com/anoma/anoma-app-sdk/commit/b44a6a5c91525b909aa5c4be5381c978ff339d38))
* adding version ([cd42a8c](https://github.com/anoma/anoma-app-sdk/commit/cd42a8cdc48bdf2ee74b7ca269b1beb06260dd62))
* allow optional wasm bytes to be passed to builder/logic init ([db90e2a](https://github.com/anoma/anoma-app-sdk/commit/db90e2ad698ed47f956010741a6b5f9a5fe52a78))
* allow passing `salt` to keyring creation ([#26](https://github.com/anoma/anoma-app-sdk/issues/26)) ([1770f45](https://github.com/anoma/anoma-app-sdk/commit/1770f45c196b8c132d520374a55603591ee17020))
* Allowlist + success screen + review screen ([#114](https://github.com/anoma/anoma-app-sdk/issues/114)) ([bf88a69](https://github.com/anoma/anoma-app-sdk/commit/bf88a69461ef32e9b74884cee60eae26ddbe1858))
* AmountInput version 1 ([#23](https://github.com/anoma/anoma-app-sdk/issues/23)) ([efc133a](https://github.com/anoma/anoma-app-sdk/commit/efc133a5d1af7a033120333da8c00beea2ba1b82))
* asking Claude to improve itself ([#340](https://github.com/anoma/anoma-app-sdk/issues/340)) ([36dfc0f](https://github.com/anoma/anoma-app-sdk/commit/36dfc0ff45069320fb47866ee8d65915a08956a5))
* auto switch to Base network ([#132](https://github.com/anoma/anoma-app-sdk/issues/132)) ([e557b3c](https://github.com/anoma/anoma-app-sdk/commit/e557b3ce7480c3c415436d941acda6f7e4b6e907))
* avoid double encoding on session storage ([#630](https://github.com/anoma/anoma-app-sdk/issues/630)) ([81dbd2a](https://github.com/anoma/anoma-app-sdk/commit/81dbd2a6cabc6eb3136153226434a7a13e82b159))
* backing up passkeys vault ([#238](https://github.com/anoma/anoma-app-sdk/issues/238)) ([f25f7c8](https://github.com/anoma/anoma-app-sdk/commit/f25f7c86cbecbaf9f5568fad43179cd1b92ae58f))
* bring back the red-envelope ([#643](https://github.com/anoma/anoma-app-sdk/issues/643)) ([ff5a20f](https://github.com/anoma/anoma-app-sdk/commit/ff5a20fca309999be0f2994c258f6d44e86ab4b5))
* bumping the number of gpus ([#257](https://github.com/anoma/anoma-app-sdk/issues/257)) ([0772f2b](https://github.com/anoma/anoma-app-sdk/commit/0772f2b8d09dcc4e2a97f78f75785afe3820a72e))
* **business:** add AnomaPay business layout and sidebar ([#694](https://github.com/anoma/anoma-app-sdk/issues/694)) ([5e24ec4](https://github.com/anoma/anoma-app-sdk/commit/5e24ec40dad3ca7769b76db9a2d598378e341960))
* claim page improvements ([#129](https://github.com/anoma/anoma-app-sdk/issues/129)) ([9897adc](https://github.com/anoma/anoma-app-sdk/commit/9897adc22d3bf66a61e1e5afc3dc1eff15b5f9e3))
* claim with fee ([#453](https://github.com/anoma/anoma-app-sdk/issues/453)) ([da0d7b5](https://github.com/anoma/anoma-app-sdk/commit/da0d7b585441e2a54672824645de24cb83dd9c8e))
* claim, transfer and many other tweaks ([#109](https://github.com/anoma/anoma-app-sdk/issues/109)) ([e77ad50](https://github.com/anoma/anoma-app-sdk/commit/e77ad502f77acdedf5a7a2899cc5c8329440f6d6))
* clean up tsdown config for deps ([#17](https://github.com/anoma/anoma-app-sdk/issues/17)) ([539febe](https://github.com/anoma/anoma-app-sdk/commit/539febec8b1036ed5566141e2f584648787c3acf))
* create asset selector v2 ([#522](https://github.com/anoma/anoma-app-sdk/issues/522)) ([158a2ad](https://github.com/anoma/anoma-app-sdk/commit/158a2adaed7764570bff27707703ec4441196cf6))
* create enter route ([#622](https://github.com/anoma/anoma-app-sdk/issues/622)) ([b05826a](https://github.com/anoma/anoma-app-sdk/commit/b05826a9744db338cfc7f059d1796377fe4b0bec))
* create payment link ([#584](https://github.com/anoma/anoma-app-sdk/issues/584)) ([c33e6c8](https://github.com/anoma/anoma-app-sdk/commit/c33e6c87c38bef5a83218ff55467488bb5225a54))
* create placeholder business pages ([#690](https://github.com/anoma/anoma-app-sdk/issues/690)) ([449c96a](https://github.com/anoma/anoma-app-sdk/commit/449c96aef194c82d2609f61091cdb4f8fc878fc5))
* creating even more components ([#6](https://github.com/anoma/anoma-app-sdk/issues/6)) ([a60c48d](https://github.com/anoma/anoma-app-sdk/commit/a60c48dbf4570a65a962e6c30725c5a697ecd75d))
* Creating initial component structure ([#3](https://github.com/anoma/anoma-app-sdk/issues/3)) ([1063073](https://github.com/anoma/anoma-app-sdk/commit/10630731e6c1a9bbf545f957f66f67edbe307522))
* deterministic key hierarchy ([#396](https://github.com/anoma/anoma-app-sdk/issues/396)) ([123dc41](https://github.com/anoma/anoma-app-sdk/commit/123dc41b9d8f81449663d19a33ee789d3750bd15))
* embracing AI part 1 ([#289](https://github.com/anoma/anoma-app-sdk/issues/289)) ([f4fdea3](https://github.com/anoma/anoma-app-sdk/commit/f4fdea37eb113978af9f628c1205f5044df6487e))
* filter for only tokens allowed on backend ([#722](https://github.com/anoma/anoma-app-sdk/issues/722)) ([8097cc3](https://github.com/anoma/anoma-app-sdk/commit/8097cc3c735bb5a5095af176012b16ff4d5fda5e))
* fixing tests, final updates for build ([feefde2](https://github.com/anoma/anoma-app-sdk/commit/feefde2a4d1237485705ffb9029bb78020d0749e))
* fixing typescript errors ([#18](https://github.com/anoma/anoma-app-sdk/issues/18)) ([639bce1](https://github.com/anoma/anoma-app-sdk/commit/639bce1cd415446a8650f736eaa04a4db91a1a7a))
* get transactions from indexer ([#409](https://github.com/anoma/anoma-app-sdk/issues/409)) ([7be637b](https://github.com/anoma/anoma-app-sdk/commit/7be637b167a724647effc978fa6e3d9f420c680e))
* get user balance from backend ([#147](https://github.com/anoma/anoma-app-sdk/issues/147)) ([84a41da](https://github.com/anoma/anoma-app-sdk/commit/84a41da3288e6d83129000cee24402cbe2b68103))
* handling balances smaller than /bin/zsh.01 ([#214](https://github.com/anoma/anoma-app-sdk/issues/214)) ([f92323c](https://github.com/anoma/anoma-app-sdk/commit/f92323c2b97416c9f7b7e514b0492e43c629a15f))
* handling multiple tokens ([#385](https://github.com/anoma/anoma-app-sdk/issues/385)) ([2ccace0](https://github.com/anoma/anoma-app-sdk/commit/2ccace05f7f9ae87732f0284de5e3fb6d8dc0016))
* handling network id in different parts of the codebase ([#419](https://github.com/anoma/anoma-app-sdk/issues/419)) ([f3cd5f0](https://github.com/anoma/anoma-app-sdk/commit/f3cd5f08e1a537fe5cb304082bf247c99fcef92e))
* improve AmountInput paste ([#552](https://github.com/anoma/anoma-app-sdk/issues/552)) ([e50e5bc](https://github.com/anoma/anoma-app-sdk/commit/e50e5bc0bb37b69328914580966aa805ba617100))
* improve auth redirect flows ([#65](https://github.com/anoma/anoma-app-sdk/issues/65)) ([ebfe5ba](https://github.com/anoma/anoma-app-sdk/commit/ebfe5ba40645a170b3447549980da48da49af385))
* improve default token selection logic ([#639](https://github.com/anoma/anoma-app-sdk/issues/639)) ([a929011](https://github.com/anoma/anoma-app-sdk/commit/a9290113eb8c8643efb18d3819b6a4486f3e2a26))
* improve fee error message ([#519](https://github.com/anoma/anoma-app-sdk/issues/519)) ([6346f21](https://github.com/anoma/anoma-app-sdk/commit/6346f21d958e58e42ac585dbf89e527b24103013))
* improvements on deposit form part 1 + validation ([#64](https://github.com/anoma/anoma-app-sdk/issues/64)) ([381166c](https://github.com/anoma/anoma-app-sdk/commit/381166c8644a8bccf1d918bae3e52e53907a649d))
* improving transaction time estimate ([#111](https://github.com/anoma/anoma-app-sdk/issues/111)) ([7722351](https://github.com/anoma/anoma-app-sdk/commit/7722351bf1d0b328c6563159af3eef0f93e2e7a4))
* integrate app with new dev features ([#383](https://github.com/anoma/anoma-app-sdk/issues/383)) ([68b4afc](https://github.com/anoma/anoma-app-sdk/commit/68b4afc2c79511f0284388e45b364bc9115aa9ce))
* integrating keyblob ([#104](https://github.com/anoma/anoma-app-sdk/issues/104)) ([6080c9a](https://github.com/anoma/anoma-app-sdk/commit/6080c9a537ff36c3397646655f287fb3d3df47cd))
* internal audit fixes ([#20](https://github.com/anoma/anoma-app-sdk/issues/20)) ([a9ef448](https://github.com/anoma/anoma-app-sdk/commit/a9ef448bc26e5a4d804320b1153cc3593a79d85c))
* keep DOM for WebAssembly, remove DOM.Iterable ([c9c55bd](https://github.com/anoma/anoma-app-sdk/commit/c9c55bd72b7a6a800c9fb47d501e564b0c0c5589))
* keyblob backup retrieval ([#250](https://github.com/anoma/anoma-app-sdk/issues/250)) ([c383581](https://github.com/anoma/anoma-app-sdk/commit/c3835815c8936315b5694be8c26907393e7add55))
* limit balance formatting to MAX_DECIMALS ([#555](https://github.com/anoma/anoma-app-sdk/issues/555)) ([3a16e4e](https://github.com/anoma/anoma-app-sdk/commit/3a16e4e47d6acce573fa4931411b8e22b5525a19))
* linking to basescan ([#212](https://github.com/anoma/anoma-app-sdk/issues/212)) ([68fca95](https://github.com/anoma/anoma-app-sdk/commit/68fca9557ab8fd939fe0fe7fd4ce5642ac3dc841))
* merging pay-js-toolkit to pay-interface-app ([#12](https://github.com/anoma/anoma-app-sdk/issues/12)) ([42f4315](https://github.com/anoma/anoma-app-sdk/commit/42f4315d82348bddf28549f3b9e2dd4f1f160178))
* metamask on mobile ([#284](https://github.com/anoma/anoma-app-sdk/issues/284)) ([548e049](https://github.com/anoma/anoma-app-sdk/commit/548e04946ce288a4198eee9cbd157fefd2b35a50))
* migrate nullifier set from localStorage to indexedDB ([#500](https://github.com/anoma/anoma-app-sdk/issues/500)) ([8fbf5b2](https://github.com/anoma/anoma-app-sdk/commit/8fbf5b28b07082f23b8fe249d2becb97110d2dc6))
* move to pnpm, format, docs ([387c368](https://github.com/anoma/anoma-app-sdk/commit/387c368cf0df82e40665299817efbffa9c725945))
* moving token registry to json file ([#401](https://github.com/anoma/anoma-app-sdk/issues/401)) ([c3155b3](https://github.com/anoma/anoma-app-sdk/commit/c3155b3c37d8511fb131fde9f19bd5252a55d022))
* new backend integration with multi-chain support ([#587](https://github.com/anoma/anoma-app-sdk/issues/587)) ([0d624c3](https://github.com/anoma/anoma-app-sdk/commit/0d624c30470f3635b0d9d68b8d16b6d9e1d3956d))
* npm pack scripts, export tweaks for integration ([d61cda6](https://github.com/anoma/anoma-app-sdk/commit/d61cda6fcaae999ba1d706d487a571b0b4fed451))
* passkey sign up / sign in ([#34](https://github.com/anoma/anoma-app-sdk/issues/34)) ([3995a66](https://github.com/anoma/anoma-app-sdk/commit/3995a66484051a7bfbc9f28d98b9390a385d15c4))
* paste only for mobile devices + current network icon ([#210](https://github.com/anoma/anoma-app-sdk/issues/210)) ([878a9e3](https://github.com/anoma/anoma-app-sdk/commit/878a9e3d12caea3ee646db47743dbdcc982b3cbe))
* payroll csv import ([#709](https://github.com/anoma/anoma-app-sdk/issues/709)) ([ac50f39](https://github.com/anoma/anoma-app-sdk/commit/ac50f39be6eec69b340963a40a42e99b713dae6d))
* payroll tweaks on layout and validation ([#715](https://github.com/anoma/anoma-app-sdk/issues/715)) ([abad29c](https://github.com/anoma/anoma-app-sdk/commit/abad29c89fe537a6c20464051edeae5b0008bf6a))
* permit2 approval ([#125](https://github.com/anoma/anoma-app-sdk/issues/125)) ([abdcb50](https://github.com/anoma/anoma-app-sdk/commit/abdcb508d63cec9776bf7abf6b86a7a5a5dbd89f))
* Port and update wasm library to `arm-risc0` `v0.13.0` ([#43](https://github.com/anoma/anoma-app-sdk/issues/43)) ([444aee9](https://github.com/anoma/anoma-app-sdk/commit/444aee920a111c7247340a4e375ed30882cf4843))
* preparing ui for public beta ([#473](https://github.com/anoma/anoma-app-sdk/issues/473)) ([6fbdccd](https://github.com/anoma/anoma-app-sdk/commit/6fbdccdcc1b3fdc4ba9a0686a4d4421ae3f3cbe7))
* qrcode on claim link ([#580](https://github.com/anoma/anoma-app-sdk/issues/580)) ([86b7ce0](https://github.com/anoma/anoma-app-sdk/commit/86b7ce04338e2a96814f5381f176622565e865fc))
* redirect when access /enter + beta page ([#686](https://github.com/anoma/anoma-app-sdk/issues/686)) ([0e6acf3](https://github.com/anoma/anoma-app-sdk/commit/0e6acf3737d9668e3f340615d83941192437c7eb))
* redirecting users based on existing vault #ai ([#297](https://github.com/anoma/anoma-app-sdk/issues/297)) ([e1ea5d9](https://github.com/anoma/anoma-app-sdk/commit/e1ea5d99cac4d64cd66c25dba22cc9cd0a5d87ed))
* refactoring a few things and adding vault dto ([#94](https://github.com/anoma/anoma-app-sdk/issues/94)) ([2f99c27](https://github.com/anoma/anoma-app-sdk/commit/2f99c276081c86ecf9c321663087cc81ddbfee1f))
* refactoring forms to reuse more code + animation ([#73](https://github.com/anoma/anoma-app-sdk/issues/73)) ([983c065](https://github.com/anoma/anoma-app-sdk/commit/983c0651c005c746e09b7d300c1cb2e4fc1df895))
* refetching balances after transfers + refetch interval ([#177](https://github.com/anoma/anoma-app-sdk/issues/177)) ([7803e28](https://github.com/anoma/anoma-app-sdk/commit/7803e28446313a6bd9abd169d3bc4f7565594f65))
* remove beta references ([#650](https://github.com/anoma/anoma-app-sdk/issues/650)) ([2e96fc0](https://github.com/anoma/anoma-app-sdk/commit/2e96fc07be9766e4102c19d6fbb1e2c41818df75))
* remove deposit cap ([#646](https://github.com/anoma/anoma-app-sdk/issues/646)) ([1e32957](https://github.com/anoma/anoma-app-sdk/commit/1e32957bd0e347f7f5c639aa161458fdf07c4ac3))
* remove unused endpoints ([#517](https://github.com/anoma/anoma-app-sdk/issues/517)) ([3ae14ae](https://github.com/anoma/anoma-app-sdk/commit/3ae14ae5d973f8756380cf7f0007378a4d5603b3))
* removing base sepolia ([#332](https://github.com/anoma/anoma-app-sdk/issues/332)) ([f1f5063](https://github.com/anoma/anoma-app-sdk/commit/f1f5063ba8d76f6c0a1a2f88c3602022f427e42c))
* rename app constants ([#362](https://github.com/anoma/anoma-app-sdk/issues/362)) ([66000e2](https://github.com/anoma/anoma-app-sdk/commit/66000e2bd359f027f2ef490a36a52439c2b208c3))
* render vault errors to avoid gray screen ([#235](https://github.com/anoma/anoma-app-sdk/issues/235)) ([49b8ed6](https://github.com/anoma/anoma-app-sdk/commit/49b8ed65700eba4a3306b42c175b1b21938f5482))
* retry while querying trm api and also improving error message ([#209](https://github.com/anoma/anoma-app-sdk/issues/209)) ([82d2ac5](https://github.com/anoma/anoma-app-sdk/commit/82d2ac58203c45bc105b42ef675b04b1d3fb302a))
* review nullifier set ([#445](https://github.com/anoma/anoma-app-sdk/issues/445)) ([4edebfb](https://github.com/anoma/anoma-app-sdk/commit/4edebfb13da6ba9cba576343eced7a2bdbbcc4e2))
* reviewing TokenId type ([#406](https://github.com/anoma/anoma-app-sdk/issues/406)) ([9217228](https://github.com/anoma/anoma-app-sdk/commit/92172283b08d94958425cbdd5fe730db505f3a48))
* save claim link asap ([#674](https://github.com/anoma/anoma-app-sdk/issues/674)) ([f55e140](https://github.com/anoma/anoma-app-sdk/commit/f55e1401491767ce9e46d6cf40cf6dd7f6b2e2e1))
* Shielded Fees ([#295](https://github.com/anoma/anoma-app-sdk/issues/295)) ([20cfbd8](https://github.com/anoma/anoma-app-sdk/commit/20cfbd8f8b39b9f492c862960c33292a35de7656))
* show failed transactions error message ([#661](https://github.com/anoma/anoma-app-sdk/issues/661)) ([de00d02](https://github.com/anoma/anoma-app-sdk/commit/de00d0208f047a21226ae4f99767fad9349dd3a3))
* sign up flow iteration 2 ([#11](https://github.com/anoma/anoma-app-sdk/issues/11)) ([663aa6e](https://github.com/anoma/anoma-app-sdk/commit/663aa6e4921cf329ad6754fb0be3dff29a05122f))
* sign-in flow part 1 ([#9](https://github.com/anoma/anoma-app-sdk/issues/9)) ([5848714](https://github.com/anoma/anoma-app-sdk/commit/58487144634b4fd209a3b645fcf4bad9c0db8c1a))
* simplifying chain registry and adding bsc as default chain ([#487](https://github.com/anoma/anoma-app-sdk/issues/487)) ([4bb0979](https://github.com/anoma/anoma-app-sdk/commit/4bb097902f3de6b388bb0cfdbf978ef15f85ad73))
* store created claim links on indexedDB ([#598](https://github.com/anoma/anoma-app-sdk/issues/598)) ([50757d7](https://github.com/anoma/anoma-app-sdk/commit/50757d7875c41579e2c2cbb603c04d1d767e21ac))
* Support Base Mainnet / Sepolia token addresses ([#113](https://github.com/anoma/anoma-app-sdk/issues/113)) ([ad4d815](https://github.com/anoma/anoma-app-sdk/commit/ad4d81584dab809b8480f22c6e9f5a4b4e43b7b0))
* support resources from multiple networks ([#127](https://github.com/anoma/anoma-app-sdk/issues/127)) ([b98db76](https://github.com/anoma/anoma-app-sdk/commit/b98db76d5a1fedab706455834dad02f89a1c3a9b))
* tweaking a few instructions on Agents.md ([#303](https://github.com/anoma/anoma-app-sdk/issues/303)) ([38b57cc](https://github.com/anoma/anoma-app-sdk/commit/38b57cc34d052fcc7c199ec963cd19b7bba57a88))
* unifying claim page with send page ([#336](https://github.com/anoma/anoma-app-sdk/issues/336)) ([fd7f2dc](https://github.com/anoma/anoma-app-sdk/commit/fd7f2dc129c2c56990a96b0843411be3c807a4c1))
* update arm bindings ([#23](https://github.com/anoma/anoma-app-sdk/issues/23)) ([fd0782c](https://github.com/anoma/anoma-app-sdk/commit/fd0782caa58e6f62f16e57a345811c8e1792cfa7))
* update arm dependencies, fix breaking changes ([#280](https://github.com/anoma/anoma-app-sdk/issues/280)) ([3d8cca4](https://github.com/anoma/anoma-app-sdk/commit/3d8cca4393d4a231082ba23401db7ebe256a0400))
* update contents ([#165](https://github.com/anoma/anoma-app-sdk/issues/165)) ([20905a5](https://github.com/anoma/anoma-app-sdk/commit/20905a5f9b9f6518a46ba50a3b1d0e5ebcafa727))
* update estimate fee api ([#457](https://github.com/anoma/anoma-app-sdk/issues/457)) ([26a33b0](https://github.com/anoma/anoma-app-sdk/commit/26a33b0dc362d2c812aedccabc2a5dfa75c1e152))
* update layout for internal test ([#90](https://github.com/anoma/anoma-app-sdk/issues/90)) ([683a1a5](https://github.com/anoma/anoma-app-sdk/commit/683a1a5a4377a71ce77c8001b20c4af78a8e9e97))
* update progress bar animation ([#275](https://github.com/anoma/anoma-app-sdk/issues/275)) ([48402d9](https://github.com/anoma/anoma-app-sdk/commit/48402d92501ee347f56bc0590fe771a214e3d6ea))
* update success screen ([#142](https://github.com/anoma/anoma-app-sdk/issues/142)) ([1c6c62c](https://github.com/anoma/anoma-app-sdk/commit/1c6c62c5acbf045488b8c27705be4107e561112b))
* updating components and creating first route ([#8](https://github.com/anoma/anoma-app-sdk/issues/8)) ([a14bec7](https://github.com/anoma/anoma-app-sdk/commit/a14bec78ddd51dcf5ec4abb167606a0c71095641))
* updating logic ref address ([#105](https://github.com/anoma/anoma-app-sdk/issues/105)) ([2cede30](https://github.com/anoma/anoma-app-sdk/commit/2cede30107b705564121dcb876b2f3ec8b02a235))
* updating README ([#22](https://github.com/anoma/anoma-app-sdk/issues/22)) ([df05c14](https://github.com/anoma/anoma-app-sdk/commit/df05c1470f606facd0116c71839162a141ea425a))
* upgrade indexer add keys flow ([#145](https://github.com/anoma/anoma-app-sdk/issues/145)) ([2e7c8e4](https://github.com/anoma/anoma-app-sdk/commit/2e7c8e45b937dec9922a2affeb2c7f98e3d588b9))
* upgrade the selectTransferResources algo ([#675](https://github.com/anoma/anoma-app-sdk/issues/675)) ([a721c95](https://github.com/anoma/anoma-app-sdk/commit/a721c95917170be40d24fcdd76292dc2d5459977))
* upgrade wagmi 3.x ([#131](https://github.com/anoma/anoma-app-sdk/issues/131)) ([37f5ea0](https://github.com/anoma/anoma-app-sdk/commit/37f5ea09ac36fdd47920200b73064d8f796a8827))
* upgrading simple_transfer_id ([#323](https://github.com/anoma/anoma-app-sdk/issues/323)) ([b1602fd](https://github.com/anoma/anoma-app-sdk/commit/b1602fdd18df19dd769d95af05fb76431b4ea2e0))
* use online infra ([#54](https://github.com/anoma/anoma-app-sdk/issues/54)) ([afbf26d](https://github.com/anoma/anoma-app-sdk/commit/afbf26d6ad82baaffbc72a0694514329616adf21))
* use private discovery key to get resources ([#283](https://github.com/anoma/anoma-app-sdk/issues/283)) ([73a7ccb](https://github.com/anoma/anoma-app-sdk/commit/73a7ccbd698daf6f7903a6d688dcbf78a04134f4))
* user profile component ([#49](https://github.com/anoma/anoma-app-sdk/issues/49)) ([501e77b](https://github.com/anoma/anoma-app-sdk/commit/501e77b9f4d92eae707ccfc1140a96df791e16c3))
* using base58 for AnomaPay addresses ([#492](https://github.com/anoma/anoma-app-sdk/issues/492)) ([4924279](https://github.com/anoma/anoma-app-sdk/commit/492427926035e91edf555c9560d156c43c4c36fc))
* validating if deposit amount is higher than deposit limit ([#339](https://github.com/anoma/anoma-app-sdk/issues/339)) ([fe04fbf](https://github.com/anoma/anoma-app-sdk/commit/fe04fbfdbed68373306685a77c67044dad92dc63))
* wait for permit2 allowance receipt ([#695](https://github.com/anoma/anoma-app-sdk/issues/695)) ([93c9bf7](https://github.com/anoma/anoma-app-sdk/commit/93c9bf74b24aa1d1aa3a5c0ed2bab016bbf755cd))


### Bug Fixes

* add id-token to release-please ([#47](https://github.com/anoma/anoma-app-sdk/issues/47)) ([b99d782](https://github.com/anoma/anoma-app-sdk/commit/b99d782a32c8868892ea7b0344afd8907f35aab2))
* add relative url ([#571](https://github.com/anoma/anoma-app-sdk/issues/571)) ([2d63383](https://github.com/anoma/anoma-app-sdk/commit/2d633836195134b3f93b6219a49cacae9f57e2d2))
* audit review fixes ([#115](https://github.com/anoma/anoma-app-sdk/issues/115)) ([5beddf5](https://github.com/anoma/anoma-app-sdk/commit/5beddf5cd8c22431d2c23181746901c9d37a0828))
* biging conversion was keeping the comma ([#576](https://github.com/anoma/anoma-app-sdk/issues/576)) ([965bfd9](https://github.com/anoma/anoma-app-sdk/commit/965bfd908940abd0e06b6137e76e61f8bf00ad37))
* bump viem version to 2.52.2 ([361b094](https://github.com/anoma/anoma-app-sdk/commit/361b094195058eabfbdef66da72bcbb34e24e8dd))
* bump viem version to 2.52.2 ([974ae13](https://github.com/anoma/anoma-app-sdk/commit/974ae132ae4728f3db64e04f8c76b61b839ecd71))
* check and fix invalid witness data for buildTransferParameters ([#107](https://github.com/anoma/anoma-app-sdk/issues/107)) ([35e9c7b](https://github.com/anoma/anoma-app-sdk/commit/35e9c7b2effa28f7ee3557923687fb8cbcd0ec31))
* check nullification tx to build the history ([#481](https://github.com/anoma/anoma-app-sdk/issues/481)) ([0218f32](https://github.com/anoma/anoma-app-sdk/commit/0218f328d15a346e3242ce810756ba5d4b09c765))
* claim all resources from claim url ([#232](https://github.com/anoma/anoma-app-sdk/issues/232)) ([aec0e4f](https://github.com/anoma/anoma-app-sdk/commit/aec0e4f2c78ac70b39ae6217877e0cbbe85403fa))
* claim link on new sessions ([#632](https://github.com/anoma/anoma-app-sdk/issues/632)) ([7e6623c](https://github.com/anoma/anoma-app-sdk/commit/7e6623cdb75c877290ab5259a012a6e8df44fc9d))
* clean up ([6b3abd8](https://github.com/anoma/anoma-app-sdk/commit/6b3abd8bbd7b3dfd8e74c1cc5abce25c2b8c0b50))
* completely disable dark mode ([#208](https://github.com/anoma/anoma-app-sdk/issues/208)) ([bc35423](https://github.com/anoma/anoma-app-sdk/commit/bc354235e5a5b19b70d980450ff73428913b9acf))
* filter resources by current chain ([#525](https://github.com/anoma/anoma-app-sdk/issues/525)) ([9977d38](https://github.com/anoma/anoma-app-sdk/commit/9977d38566ade870c59885d65619b904aeb55271))
* fix add keys flow for claim link ([#471](https://github.com/anoma/anoma-app-sdk/issues/471)) ([85575da](https://github.com/anoma/anoma-app-sdk/commit/85575daada90c82dba3f5fcd93c1e8558149f699))
* fix claim loading state ([#279](https://github.com/anoma/anoma-app-sdk/issues/279)) ([53b0c5b](https://github.com/anoma/anoma-app-sdk/commit/53b0c5b98b4218bff14de9c3d502c5fded83971c))
* fix fee tests ([#450](https://github.com/anoma/anoma-app-sdk/issues/450)) ([79e982e](https://github.com/anoma/anoma-app-sdk/commit/79e982e42c63639cdbc410999f37561447196a17))
* fix flashing on changing screen + add jotai-minidb lib ([#512](https://github.com/anoma/anoma-app-sdk/issues/512)) ([dd54300](https://github.com/anoma/anoma-app-sdk/commit/dd54300f64319a07d1349e3d3c012c519cca8f20))
* fix icons request ([#169](https://github.com/anoma/anoma-app-sdk/issues/169)) ([a48bae6](https://github.com/anoma/anoma-app-sdk/commit/a48bae6dc965cebc5dd5d1e4c1a423a3508fd3a3))
* fix send form ([#58](https://github.com/anoma/anoma-app-sdk/issues/58)) ([01d0eeb](https://github.com/anoma/anoma-app-sdk/commit/01d0eeb72cf572653f6c056e1961eb1345af27b3))
* fix split quantity ([#149](https://github.com/anoma/anoma-app-sdk/issues/149)) ([ff94cb9](https://github.com/anoma/anoma-app-sdk/commit/ff94cb9de4026e50f111ee52d9d2f1ab76dcd483))
* fix token address for deposit ([#133](https://github.com/anoma/anoma-app-sdk/issues/133)) ([d49a838](https://github.com/anoma/anoma-app-sdk/commit/d49a838c59a752b4e6141fac696767871409bca7))
* fix token url and organize code a bit more ([#542](https://github.com/anoma/anoma-app-sdk/issues/542)) ([786b3b1](https://github.com/anoma/anoma-app-sdk/commit/786b3b1d9b7ba635070171e6576f635ab6494575))
* fix transfer for other accounts ([#70](https://github.com/anoma/anoma-app-sdk/issues/70)) ([895705c](https://github.com/anoma/anoma-app-sdk/commit/895705cd03091335538da76c89c961bde6edf58c))
* fix undefined business theme ([#721](https://github.com/anoma/anoma-app-sdk/issues/721)) ([eb9728b](https://github.com/anoma/anoma-app-sdk/commit/eb9728b9a3e60e18fd63da36507dea9c64219b8f))
* fixing icon import craziness ([#21](https://github.com/anoma/anoma-app-sdk/issues/21)) ([582da9c](https://github.com/anoma/anoma-app-sdk/commit/582da9ce1c680bf99d331ca2c47858b346cb9fab))
* fixing missing type ([#240](https://github.com/anoma/anoma-app-sdk/issues/240)) ([79e7252](https://github.com/anoma/anoma-app-sdk/commit/79e72528c4270f8339cf175467f0afa75d05e226))
* fixing token amount format ([#244](https://github.com/anoma/anoma-app-sdk/issues/244)) ([18e6c7e](https://github.com/anoma/anoma-app-sdk/commit/18e6c7e6d17d218a57ceb552ce1cc154d9dfd6b4))
* fixing type errors ([#27](https://github.com/anoma/anoma-app-sdk/issues/27)) ([ec15706](https://github.com/anoma/anoma-app-sdk/commit/ec15706c4169a58c8fa4ec78642ef66a7531d109))
* fixing variable name and cleaning up .env file ([#430](https://github.com/anoma/anoma-app-sdk/issues/430)) ([dbdacd9](https://github.com/anoma/anoma-app-sdk/commit/dbdacd9a070532f7310064f85d4a84b504596c7d))
* format bigint as string ([#526](https://github.com/anoma/anoma-app-sdk/issues/526)) ([3d1ffde](https://github.com/anoma/anoma-app-sdk/commit/3d1ffde4b93339b191a1ed05c12159172167eab4))
* move authorize to separate step, authorize all actions ([#176](https://github.com/anoma/anoma-app-sdk/issues/176)) ([d8c2c68](https://github.com/anoma/anoma-app-sdk/commit/d8c2c68652cbf43f8946d8a3a13c1d3241d75913))
* prefixing string with 0x ([#291](https://github.com/anoma/anoma-app-sdk/issues/291)) ([606e7c6](https://github.com/anoma/anoma-app-sdk/commit/606e7c6e56840d6719251ade558760ad84b1ecea))
* remove rpc settings & env vars ([#310](https://github.com/anoma/anoma-app-sdk/issues/310)) ([7c7330b](https://github.com/anoma/anoma-app-sdk/commit/7c7330bafb721305ca5e11356f5514cb0d1695fd))
* static type check ([#34](https://github.com/anoma/anoma-app-sdk/issues/34)) ([8b5e7a6](https://github.com/anoma/anoma-app-sdk/commit/8b5e7a682207a50bec2d62638fc437d268422b32))
* throwing an error if token prices were not properly loaded ([#456](https://github.com/anoma/anoma-app-sdk/issues/456)) ([759fba8](https://github.com/anoma/anoma-app-sdk/commit/759fba8776ac21e490ef1f009da448c9fb99b3bb))
* treat balances equal to 0.01 properly #ai ([#294](https://github.com/anoma/anoma-app-sdk/issues/294)) ([491248d](https://github.com/anoma/anoma-app-sdk/commit/491248da889534c6a47fc1d64adc1f98c1b9a126))
* tsdown should copy wasm on reload in watch mode ([1a4a174](https://github.com/anoma/anoma-app-sdk/commit/1a4a17415ddbd132e7f87e10bb39574c02b01d68))
* update deposit payload ([#36](https://github.com/anoma/anoma-app-sdk/issues/36)) ([30f1d13](https://github.com/anoma/anoma-app-sdk/commit/30f1d133e76b9666ba31702d182ded7642f7b76c))
* update heliax accounts ([#550](https://github.com/anoma/anoma-app-sdk/issues/550)) ([4b0b429](https://github.com/anoma/anoma-app-sdk/commit/4b0b429861a0a57792074191c1e704fd65cc4d23))
* update resource created in transaction hash ([#535](https://github.com/anoma/anoma-app-sdk/issues/535)) ([1bd0240](https://github.com/anoma/anoma-app-sdk/commit/1bd0240a8b3fa7863aa9455b4abed809a8e67b4a))

## Changelog
