# Proposed command and checks

Source checks (already performed without K dispatch):

```sh
python3 -m unittest discover -s experiments/moriarty-language/formal/k -p 'test_*.py'
node --test experiments/moriarty-language/tests/expression-v1*.test.mjs experiments/moriarty-language/tests/k-expression-fixtures.test.mjs
```

After exact source/resource approvals and root admission only, use the complete argv in resource-proposal-02.json. Its financial/proof-free child command is:

```sh
/usr/bin/python3 /home/charl/Moriarty/.worktrees/sp03-expression-k/experiments/moriarty-language/formal/k/run.py --suite expression-v1 compile-and-traces --all
```

Do not run that child outside the admitted systemd memory/swap/runtime/CPU limit and nonblocking shared flock. Require a previously absent `.build-expression-v1/compile-attempt.json`; never remove an attempt to retry. Check source/toolchain/build pins before dispatch, current memory/disk availability and no competing heavy command. Preserve exact argv, actual process exit, command logs, compile attempt, binding, each input/raw K output/decoded result and complete comparisons. Stop at the first failed compile, trace, decoder, expected-result check or deadline. Verify terminal cgroup absence independently. A source error is evidence for a corrected separately reviewed candidate, not authority for automatic recompilation.

No command in this document has been operationally dispatched for this candidate.
