# feat: Parallelised folding in Gemini
https://github.com/AztecProtocol/barretenberg/pull/550
merged: 2023-06-22T16:39:38Z

# Description

The folding operation in Gemini now used parallelism. I also split gemini.hpp into gemini.cpp and gemini.hpp for faster compilation.

# Checklist:

- [ ] I have reviewed my diff in github, line by line.
- [ ] Every change is related to the PR description.
- [ ] The branch has been merged with/rebased against the head of its merge target.
- [ ] There are no unexpected formatting changes, superfluous debug logs, or commented-out code.
- [ ] There are no circuit changes, OR a cryptographer has been assigned for review.
- [ ] New functions, classes, etc. have been documented according to the doxygen comment format. Classes and structs must have `@brief` describing the intended functionality.
- [ ] If existing code has been modified, such documentation has been added or updated.
- [ ] No superfluous `include` directives have been added.
- [ ] I have [linked](https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue) to any issue(s) it resolves.
- [ ] I'm happy for the PR to be merged at the reviewer's next convenience.

