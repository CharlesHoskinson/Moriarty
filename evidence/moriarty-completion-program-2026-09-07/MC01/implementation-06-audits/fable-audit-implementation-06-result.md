I'll audit the supplied MC01 packet against the frozen profile (grammar, semantics, typed schemas, numeric profile, bounds), then record the verdict. First a quick read-only check of whether the candidate worktree is reachable so I can bind hashes independently.


<invoke name="Bash">
<invoke name="command">cd /home/charl/Moriarty-wt-mc01-implementation 2>/dev/null && git rev-parse HEAD && sha256sum experiments/moriarty-language/src/validate.ts experiments/moriarty-language/src/checker.ts experiments/moriarty-language/src/evaluate.ts experiments/moriarty-language/src/lower-compact.ts experiments/moriarty-language/spec/semantics.md experiments/moriarty-language/spec/examples/loan.moriarty 2>/dev/null; echo "exit=$?"; git -C /home/charl/Moriarty-wt-mc01-implementation status --short 2>/dev/null | head -5</invoke>
</invoke>
<invoke name="description">Read-only: confirm worktree HEAD and hash key files</invoke>
</invoke>
<invoke name="timeout">30000</invoke>
</invoke>
<invoke name="function_calls">
</invoke>