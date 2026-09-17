# Independent Astra medium resource review

Reviewer: gpt-6-astra, medium, /root/k_recovery_design. Read-only inspection; no author edits or native execution.

Verdict: approve the bounded successor diagnostic amendment subject to concrete command and source/tool pins frozen before dispatch. The missing historical compiled artifact necessitates one fresh build to test the recovered input here. Preserve historical 8 compiles/219 krun and add new attempts.

180-second compile, 20-second diagnostic, 8GiB memory, swap zero, TasksMax256, finite 8MiB stack, 600-second aggregate, no prefix replay and no automatic retry are reasonable. Add LimitCORE=0, bounded cleanup/output and verify service properties.

Installed krun supports --debugger-command FILE followed by --debugger-batch, yielding gdb --batch -x FILE --args interpreter. Use one run, bt 64, registers and program state; never continue or rerun. Bind or exclude GDB startup files. GDB may exit zero after inferior SIGSEGV, so transcript must distinguish inferior signal, debugger exit and downstream conversion failure.

Direct existing K runner under reviewed systemd containment is acceptable for this newly authorized unregistered diagnostic. AGENTS/skill restrict registered campaign actions to CLI; they do not require creating campaigns for every new command. Do not bypass an existing registered denial. Preserve all expression and lifecycle acceptance obligations and independent actual-result audits.
