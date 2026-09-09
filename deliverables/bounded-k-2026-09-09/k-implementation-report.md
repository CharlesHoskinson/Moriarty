# Bounded K implementation candidate

Author: GPT-6, scoped product implementation. Status: ready for preexecution audit; no K compiler, evaluator or prover was invoked.

Files implement staged K financial checks and results, a closed projection codec, bounded runner, pinned toolchain, 16 static independent expectations, codec unit tests and scoped documentation. K accepts two balances (checking distinct pairs itself), one allowance/obligation, empty tombstones, identity conversion, two ordered Transfer/Repay actions, and two allocation rules. Actual identifiers reach K. Absent receiver creation is represented but untested in K.

Checks actually run:

- Initial codec test failed because the codec was absent: `/tmp/moriarty-k-codec-red.txt`.
- `python3 -m unittest discover -s experiments/moriarty-language/formal/k -p test_codec.py`: 5 tests pass, including all six complete independent expected positive decodes, malformed output and binding rejection, and admission of financial failures. These are codec observations, not K execution.
- `python3 -m py_compile experiments/moriarty-language/formal/k/{codec,run,test_codec}.py`: passed.
- `python3 experiments/moriarty-language/formal/k/run.py prove`: exact PROOF_UNIMPLEMENTED HarnessError, exit 2, with no K dispatch.
- `git diff --check`: passed.
- Parent independently reported all 16 fixtures match actual source preparation, with six prior independent oracles preserved exactly.

Reviewed execution command: `python3 experiments/moriarty-language/formal/k/run.py compile-and-traces --all`, under the parent's whole-tree systemd memory/runtime scope. One compile attempt, maximum 16 krun attempts, timeouts 180/20 seconds and one 512-second aggregate deadline. Runner reports all full observations keyed by fixture ID in `.build/observations.json`. Compiled source/toolchain/file hashes reject stale output. Root retains whole-tree resource measurements.

Limits: no K compilation, traces, parser correspondence or proofs yet. No full Core K claim or SP03 completion. Raw K input outside the host-admitted projection is outside the claim domain. Python validates lexical UInt128 and schema; K owns financial predicates/arithmetic. The codec restores unchanged metadata, row order, effects and tombstones using K financial output; no TypeScript calls or financial fallback. Unexpected KAST JSON shapes fail closed and remain an integration uncertainty for first reviewed execution.

Early independent GPT-6 feedback was reproduced by the receiver-index regression (`/tmp/moriarty-k-receiver-red.txt`): changing K receiver index 1 to 0 or -1 was previously accepted. The codec now binds the index to the actual input recipient/asset lookup, a structural metadata check without financial arithmetic. Both mutations reject; the final five-test suite passes (`/tmp/moriarty-k-codec-final.txt`).

Additional independent codec findings were reproduced (`/tmp/moriarty-k-output-schema-red.txt`): duplicate outer JSON keys and Python Bool/Float numeric equality could pass output schema checks. Output JSON now rejects duplicate keys and non-integer version/arity values, including booleans. Targeted regressions and the final codec suite pass.

Frozen candidate SHA-256:

- `experiments/moriarty-language/formal/k/.gitignore`: `03824949a66ab6db1429bd078d10d1bc2d75706aa1b756c547f9a7ff541c77cb`
- `experiments/moriarty-language/formal/k/README.md`: `9876f84424fe04277f21dc54dfc3e709fc1bdb6b58726cb332ffddc4b121274f`
- `experiments/moriarty-language/formal/k/codec.py`: `fab1a6e1bcc122df389b6fd0a48fd45efd9141b09c54ea05a7ec063f1f4d0dbf`
- `experiments/moriarty-language/formal/k/fixtures/cases.json`: `d46679961af39a10adec03cb86b4481a39fc9393d15ceb48b37b10b8ce28f832`
- `experiments/moriarty-language/formal/k/moriarty.k`: `21260907574f5f688c799e059e63bfd56af1f9bed7b8d3f29177778940ba4273`
- `experiments/moriarty-language/formal/k/run.py`: `676fd43cf08af2d09a9b3be8efc035c4c6539876e16c0d290a7b2523215ef57a`
- `experiments/moriarty-language/formal/k/test_codec.py`: `e6b64f6cd6294ce0e40ff465f98d1aec2977922816fed1bca29c40cc155f74ef`
- `experiments/moriarty-language/formal/k/toolchain.lock.json`: `70e09f37636f6dfefdf39310c7793881f7ad2f64a3ce4d0ed7d67b5addb38522`
