# New WSL single-case diagnostic

Root adopts the independently approved Astra medium successor diagnostic: one fresh LLVM compile of unchanged recovered expression K source (180 seconds), then one recovered metadata-depth-5947 input under GDB (20 seconds). Aggregate ceiling 600 seconds, one compile and one interpreter attempt. Historical counts remain 8 compiles and 219 krun attempts; new attempts are additive. No replay of the 105 previously matched cases and no automatic retry.

Each command runs in a systemd user service with MemoryMax=8G, MemorySwapMax=0, TasksMax=256, KillMode=control-group, LimitCORE=0 and LimitSTACK=8388608:8388608. Compile RuntimeMaxSec=200; diagnostic RuntimeMaxSec=30. Both stack limits are explicit because the installed K launcher raises the soft limit to the hard limit. Capture actual unit properties and raw outputs before cleanup.

The Debian K 7.1.337 package is a new WSL build provenance. Historical Nix artifact equivalence is neither assumed nor asserted. Freeze exact source/tool and debugger command hashes before execution.

The independent Astra reviewer inspected the installed debugger implementation: --debugger-command FILE followed by --debugger-batch invokes GDB once around the LLVM interpreter. Commands run once, then capture bt 64, registers and program state. GDB exit zero is not interpreter success. Preserve signal and downstream wrapper failures separately.

No K action is registered in the current development plugin. Its skill requires CLI dispatch for registered actions; the reviewer agrees the existing K runner under reviewed systemd containment is appropriate for this explicitly authorized unregistered successor diagnostic. No registered denial is bypassed and old campaign accounting is untouched. This does not approve semantic conformance or lifecycle completion.

Votes: root adopts; independent gpt-6-astra medium k_recovery_design approves the bounded resource amendment subject to frozen commands and pins. Its substantive findings are retained in diagnostic-review-astra.md.
