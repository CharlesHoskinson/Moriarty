# Candidate A compilation-view timeout

Experiment observation: the original pilot compilation view reached its
900-second child timeout with 4096 MiB Node/JVM settings at dispatch base
`43f33721da7a3bfb11dd1a91f1f548d5110ab93c`. The series stopped. Factored
compilation and solver execution did not occur; H1 remains unresolved.

| Preserved outcome | Recorded value |
| --- | --- |
| Reaped `/usr/bin/time` wrapper status | -15 |
| Recording helper timeout return | 124 |
| Actual outer command exit | 1 |
| Generated JSON | Empty original, zero bytes; no valid input |
| Stderr | Empty original, zero bytes |
| GNU-time resources | Empty original, zero bytes |
| Native compiler elapsed time / maximum RSS | Unavailable |

The helper's recorded 901.218350434996 seconds includes post-child verification;
it is not an independently measured native compiler duration. No zero CPU or
memory measurement is inferred from an empty resource file. Empty stderr does
not establish a compiler phase, successful alias resolution, a flattening or
InlinePass cause, or a semantic counterexample. There is no paired input-size
comparison and no successful zero-byte baseline.

Experiment observations: the four preceding gates passed with actual child,
helper and outer exits zero. Recursive typecheck passed; the unchanged
`allPilotPrefixesTest` reported one passing quantified test covering 22 prefix
pairs; both 100-sample runs reported all eight required witnesses positive.
Witness counts in each sample were prepared 100, signed 100, proposed 100,
verified 50, committed 50, rejected 50, afterCompleted 53 and beforeCompleted 47.
These are bounded prefix/sample observations, not exhaustive model checking.

Repository observation: root independently admitted those gates and the timeout
preservation. Their exact intake records and the independent terminal review are
archived, together with the adopted plan, source/command review and adoption
record. The independent terminal review admits evidence preservation while
marking the original compilation gate failed. All original files remain frozen.

`original-inputs.json` and `archive-manifest.json` describe 91 original regular
files. They include all 83 control files: the exact 81 files in the author's
original index, that unchanged index itself, and the handoff's actual outer
record created afterward. The post-index record is preserved separately without
rewriting the original index. Five complete source-and-runner archives remain
byte-for-byte originals; their unique member counts are 146, 156, 166, 176 and
186. The last complete archive contains 79,309,039 bytes.

The lossless outer tar is 154,388,480 bytes, SHA256
`9cd6ff42940804ab9aa68d0e90bc9f5faec28993ad1e1ed186a48f19264dc133`.
Ten sequential `original-receipts.tar.part-*` files, each at most 16 MiB, retain
that exact archive. The outer tar is uncompressed to avoid recompressing the
original compressed source archives. Part and full-member hashes are in the
manifest. No complete extra runtime or Python environment archive is included.

Run the standalone preservation audit with Python 3.11 or later from any working
directory. Its explicit startup guard rejects enabled Python optimization so
that every assertion executes:

```sh
PYTHONOPTIMIZE=0 python3 -B /home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-candidate-a-completion/a5/compilation-view-timeout/audit.py
```

The audit reconstructs the tar in a temporary file, verifies every part/archive/
member hash, checks all five nested source closures against their own input pins,
and checks the exact 24-file copied import closure, two reversible import additions
and 30 frozen Task2 originals. Historical recorder/test-first pins are checked
inside the retained earlier source archive where their original bytes reside.
It also checks actual emitted/terminal command distinctions, raw prefix/witness
outputs, both root intakes and the unchanged original index. It reads archived
evidence only; it does not rerun native tools or rehash the live runtime.

`validation-final.json` is the actual final successful audit output; it records
optimization zero, disabled bytecode and the original Python argv.
`audit-final-command.json` retains the actual `PYTHONOPTIMIZE=0` invocation and
tool result. The preceding successful audit source, output and tool record remain
unchanged as `audit-before-optimize-guard.py`, `validation.json` and
`audit-command.json`. An initial archive-audit
assumption incorrectly required every historical pin to appear directly in the
latest source archive. Its exact source and KeyError tool record are preserved
as `audit-original-failure.py` and `audit-original-failure-command.json`. The
corrected audit traverses the preserved historical archive. This correction
changed only the package audit; no experimental command or source was retried.

The existing 4,758-file runtime and 3,329-file Python archives remain admitted
external references, with original full paths and digests in `validation-final.json`
and the archived author index. The runtime archive SHA256 is
`f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c`;
the Python archive SHA256 is
`7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac`.
The original runtime store is retained by the earlier A5 evidence owner,
including the [kernel-task1 runtime parts](../kernel-task1/). Bootstrap remains
`900bb2051225b4a3d99bf422c3b2e5e386e3e7bc`, distinct from actual dispatch.

No full A5 completion, actual-model repair success, Council approval or new
experimental hypothesis is claimed by this package.
