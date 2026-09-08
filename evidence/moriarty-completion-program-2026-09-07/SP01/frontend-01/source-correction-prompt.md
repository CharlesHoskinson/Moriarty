You are Grok4.6 high, author of the Moriarty frontend correction. Make one exact source correction now with Edit/Write in /home/charl/Moriarty/.worktrees/sp01-frontend-grok. Own only experiments/moriarty-language/src/validate.ts plus FOREMAN_REPORT.md/.json. No tests, shell, Git, agents, web, other edits. Finish within290sec.

Parent ran all33 frontend tests, all passed. Actual tsc7.0.2 failed:
experiments/moriarty-language/src/validate.ts(44,89): error TS2339: Property name does not exist on type SourceDeclaration.
Cause: StateDecl shares a union member with ConstDecl, so array filter does not produce a narrowed array element type. Replace only:
  const declaredStates=new Set(ast.declarations.filter(d=>d.tag==='StateDecl').map(d=>d.name));
with:
  const declaredStates=new Set(ast.declarations.flatMap(d=>d.tag==='StateDecl'?[d.name]:[]));
The conditional narrows inside the callback and collects precisely the same StateDecl names. All other source and frozen test bytes must remain unchanged. Update reports to record actual parent typecheck failure, this correction, and that parent reruns verification; do not claim new passing checks or run them.
