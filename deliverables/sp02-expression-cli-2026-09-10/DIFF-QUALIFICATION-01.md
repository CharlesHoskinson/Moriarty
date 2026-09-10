# Additional diff qualification after candidate01

The earlier `git diff --check` ran before newly created files were staged and
returned0; it did not inspect those new files. After staging, a full diff check
returned2 for whitespace in the retained raw failed-test and npm build outputs.
Those original outputs remain unchanged. No product-file whitespace error was
reported. This qualifies the earlier check rather than treating it as complete
staged-change validation.

The committed comparison
`git diff 1124735 HEAD --check -- experiments/moriarty-language` returned0;
its output is `product-diff-check-01.txt`. The full committed comparison
`git diff 1124735 HEAD --check` returned2; its output is
`full-diff-check-01.txt`. Both were run with HEAD at the CLI implementation
commit bc2afdd. These supplemental records do not alter candidate01's pinned
source, tests, contract, result or original verification bytes.
