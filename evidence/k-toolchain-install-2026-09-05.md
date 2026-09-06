# K toolchain installation receipt, 2026-09-05

Installed with kup (nix profile) from the k-framework.cachix.org binary cache; no source build.

component	version	source	path
kup	0.2.6	github:runtimeverification/kup#kup	/nix/store/xqg41422lqk5b61sb86s91vqx9b479hn-kup-env/bin/kup
k	v7.1.337	github:runtimeverification/k/4a46d1231473b599c699160132fd6e76a5c46406#packages.x86_64-linux.k	/nix/store/y63xkr8pk2bqd5lh4889rlwldw26v9f4-k-7.1.337-4a46d1231473b599c699160132fd6e76a5c46406
pyk (kframework)	7.1.337	PyPI kframework==7.1.337, uv dependency group zkir-k	.venv/lib/python3.13/site-packages/pyk
nix	nix (Determinate Nix 3.21.1) 2.34.7	Determinate Nix	/nix/store/q6w31hnclw39dl8syvg0v45h4n8vzq7k-determinate-nix-3.21.1/bin/nix

kup install k --version v7.1.337: elapsed 2m 29s, exit 0 (log: kup install, 2026-09-05).

## experiments/zkir-k/toolchain-check/check_k_toolchain.sh output

```
date: 2026-09-05T17:04:31Z
host: Linux 6.18.33.2-microsoft-standard-WSL2 x86_64
kompile: /home/charl/.nix-profile/bin/kompile
kompile --version: K version:    v7.1.337 Build date:   Thu Aug 20 06:14:51 MDT 2026 
--- kompile (llvm)
LESSON-02-A-SYNTAX in definition.  Use --syntax-module to specify one. Using
LESSON-02-A as default.
--- krun banana.color
<k>
  Yellow ( ) ~> .K
</k>
--- krun blueberry.color
<k>
  Blue ( ) ~> .K
</k>
--- kompile (haskell)
LESSON-02-A-SYNTAX in definition.  Use --syntax-module to specify one. Using
LESSON-02-A as default.
--- krun banana.color (haskell)
<k>
  Yellow ( ) ~> .K
</k>
--- pyk (uv dependency group zkir-k of the Moriarty project)
pyk: 7.1.337
--- pyk kast round trip
parsed: colorOf ( Banana ( ) )
kast -> kore -> kast: unchanged
krun rc: 0
krun result: <generatedTop>   <k>     Yellow ( ) ~> .K   </k>   <generatedCounter>     0   </generatedCounter> </generatedTop>
pyk round trip: PASS
RESULT: PASS
```
