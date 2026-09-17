# WSL diagnostic result

First build: failed in3.686seconds with missing K equality syntax. Second build after importing K-EQUAL: passed in7.446seconds. First diagnostic: interpreter SIGSEGV in native KORE recursive parser, followed by wrapper113 and EOF converting the absent output. Elapsed3.023seconds. Original metadata-depth-5947 input unchanged.

Backtrace repeats application_pattern_internal, application_pattern, pattern_internal and patterns_ne at KOREParser.cpp324–435. This localizes the native failure before semantic rewriting. It supports testing stack exhaustion; it does not itself establish successful execution under another stack size or lifecycle conformance.

Cumulative recorded attempts: original8compiles/219krun +2newcompiles/1newkrun =10compiles/220krun. Failure and success command receipts, compiled hashes and transcripts remain retained.
