# S0 cross-artifact implementation review

**Status:** independent read-only review of the provisional Source/6, K, Quint and TypeScript S0 slice. This report records findings and local dispositions. It is not a vote on revised final bytes.

| Severity | Finding | Disposition |
| --- | --- | --- |
| High | Quint submission amount and fee were independent of the signed action. | Resolved: the signed record now binds exact quantities; Quint typecheck passed. |
| High | K required round zero at failure and commit despite accepting a validity interval. | Resolved: nonzero valid rounds pass the rule pattern; K syntax compilation passed. |
| High | K expected `useWork(1)` in the source submitted vector. Source/6, Quint and TypeScript omitted that line. | Resolved: K meters work in state and omits it from the submitted vector; K syntax compilation passed. |
| High | K took an unauthenticated current round from the request. | Resolved: the trusted premise now binds that round to the authenticated snapshot; K syntax compilation passed. |
| Medium | Nominal caps and obligation fields disagreed across artifacts. | Resolved in the prototypes; Source/6 range rejection and Core judgment placement still need a parser and correspondence evidence. |
| Medium | K repayment required a third balance and fee recipient absent from the Source/6 repayment grammar. | Resolved: K uses two balance rows and empty unused tuple positions; K syntax compilation passed. |
| Medium | TypeScript classified some complete-vector mismatches at authority or history. | Resolved: the preparer compares the full ordered vector at effect; TypeScript typecheck passed. |
| Medium | Work budget was absent from TypeScript and Quint. | Resolved: all three prototypes check and consume one unit on success; that unit cost is provisional. |
| Medium | Repay floor and Quint negative floor disagreed with Source/6. | Resolved: repayment requires zero floor and nominal floor bounds are checked. |
| Medium | K rejected aggregate allowance overflow while TypeScript and Quint allowed it. | Resolved: TypeScript checks the aggregate and Quint seedAccount checks it; both typechecks passed. |
| Medium | Quint accepted negative validity endpoints. | Resolved: Quint checks both interval endpoints as unsigned; typecheck passed. |

The review was static. No semantic trace, simulation, test or proof was run. The revised artifacts require fresh frozen-byte audit before Sprint 0 exit.
