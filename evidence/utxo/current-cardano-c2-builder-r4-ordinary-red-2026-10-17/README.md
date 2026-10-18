# R4 artifact-builder test baseline

The first ordinary run executed 18 tests. R00 passed. R01–R17 each failed with the exact message `missing implementation: build_k10_inputs_r4.py`. There were zero errors, exit status 1 and empty stdout. All eight input files and six governance pins retained their recorded hashes and modes. Scratch remained empty with mode 0700.

This was the planned baseline with the builder absent. It confirms that the pinned artifact consumer and public fixtures satisfy the prerequisite test. The complete stderr includes all seventeen missing-implementation tracebacks; the result was captured once, without a retry. An independent actual-result review is pending at this snapshot. No result adoption is included here.

Two fresh nonauthor Opus 4.8 source reviews agreed on R00–R17 and all seven effects/meaningfulness checks before the run. They are independent contexts from the same provider. Both found manifest VALUE assertions missing from the future successful-run oracle; those findings remain prerequisites for GREEN acceptance. Presence of all 33 keys does not establish their values.

The suite is the byte concatenation of the Opus 5.5 prefix and three Opus 4.8 fragments. Its SHA-256 is `c9375097ccf1e98a84230619c3b2a8cdd611c8ec39fdc2d0d79e5aa4f76d02ad`. The whole source, fragments, public fixtures, normative documents, both whole reviews and root admission/result records are retained. Python copies use `.py.txt` to preserve bytes without adding runnable test modules to repository discovery. The source file names and isolated layout are recorded in the envelope; the JSON read-segment alternatives are omitted as duplicate delivery material.

The command used Python 3.14 with isolation and bytecode writes disabled, a read-only input mount and a scratch mount, under the recorded CPU, wall-clock, address-space and output-file bounds. Stdout/stderr were captured in separate regular files. No native prover, VM, wallet or ledger action was launched.

This is a Python artifact-preparation baseline. Builder implementation, manifest coverage, Groth16 verification, current-language compiler correspondence, all twenty UTXO families, financial obligations, recursion, private handoff, multiparent composition and actual Cardano Preprod transactions remain open. The K6 proof is a prior local proof with unauthenticated context; the K2 setup is single-party and TEST ONLY. Builder reliance remains false.
