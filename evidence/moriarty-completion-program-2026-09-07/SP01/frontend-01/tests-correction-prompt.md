You are Grok4.6 high, the implementation author. Make this small test correction with Edit/Write now. Parent independently reviewed your previous fixtures. The extra covering-zero target weakens required test3 and was rejected. No new design is needed.

Own ONLY experiments/moriarty-language/tests/frontend.test.mjs and FOREMAN_REPORT.md/.json in /home/charl/Moriarty/.worktrees/sp01-frontend-grok. No implementation edits, shell, tests, Git, agents, web or installs. Parent executes verification. Finish in290sec.

Required exact edits:
1. Change signature `function sp01PolicyOrdinalSource(ordinal,policyAfterAction,extraCoveringZero=false){` to `function sp01PolicyOrdinalSource(ordinal,policyAfterAction){`.
2. Replace targets ternary with `const targets=`effect(run,${ordinal},amount)`;`.
3. Change `sp01PolicyOrdinalSource(sp01Uint128Overflow,policyAfterAction,policyAfterAction)` to `sp01PolicyOrdinalSource(sp01Uint128Overflow,policyAfterAction)`.
4. Update both reports: original covering-zero adjustment was rejected; single-target policy-before and policy-after fixture restored. Parent will run RED, you did not run tests. Preserve all other test content.

Rationale: dependent uncovered-emit coverage must be deferred narrowly when the invalid ordinal's action/field/unit match. A covering valid target bypasses that exact required behavior. Other tests retain unrelated errors. The source-only phase already has independent GPT6 reviewed algorithm, so do not adjust expected UINT_RANGE or add targets to hide defects.
