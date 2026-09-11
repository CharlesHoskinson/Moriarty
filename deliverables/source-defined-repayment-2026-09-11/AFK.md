# Source-defined repayment AFK execution record

The user authorized all four source-authoring steps and the native host goal.
No scheduler or orchestration framework was added.

1. Declare repayment units, assets, records and ordinary state in .mori.
2. Elaborate into the existing checked schema.
3. Check/format/funded simulate through the normal CLI without --schema.
4. Reject duplicate declarations, unknown types, mixed units and protected-operation redefinition.

Grok4.6 high authored the implementation in session
01a09288-cc1a-7871-9703-04544cf69322. The initial implementation and two repairs
returned terminal end_turn. Fresh Astra medium approved candidate-02.json.
The complete result, verification and audit links are in RESULT.md.

Branch: feat/source-defined-repayment.
Base: e6dc9f68bdb30adaed5adea6838549e7b457dc28, the reviewed funded adapter.
The reviewed source is integrated into /home/charl/Moriarty; unrelated user work
is preserved. Publication uses a PR stacked on feat/expression-funded-repayment.

This record is historical evidence, not process liveness. Query the native goal
and actual process when recovering. Do not resume obsolete exec handles or old
native/K/Preview campaigns from prior logs. Scope remains single-action local
source authoring; existing live admission/accounting stops remain unchanged.
