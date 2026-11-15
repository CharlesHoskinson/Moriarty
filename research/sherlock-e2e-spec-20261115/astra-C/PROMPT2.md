# Astra-C phase 2: adversarial review of the slice C specs

Same limits as phase 1: write only inside `research/sherlock-e2e-spec-20261115/astra-C/`; no git commands; no network; no builds, provers or transactions; never open a wallet seed, phrase or key file.

Fable-C wrote the slice C specs. Review them against your DRAFT.md and against the sources. Be adversarial: the goal is to find what is wrong or missing, not to confirm.

Files under review:
- `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-{wrap,ledger-verifier,ledger-contract,batched-settlement,governance,preview-deployment,wallet,sdk-cli,observability,recovery,helper-machines}/spec.md` (107 EARS: WRP, LVR, SLC, BAT, EGV, PVW, WAL, SCL, OBS, FRC, HLP)
- `research/sherlock-e2e-spec-20261115/bus/ddd-C.md` (contexts, context map, aggregates, events, terms)
- `research/sherlock-e2e-spec-20261115/bus/decisions-C.md` (MADR drafts D-070 to D-085)

Slice routing (from the bus): C owns rows T4, T10, T10a, T10m, T17, T19, T20, T22, T23 Part A and settle, T0/T1/T2 durability and record, T15 freeze and pushes, the T21 GUI page, on-ledger claim consumption and the on-ledger foreign registry. Slice B owns T25 keygen, the STARK layers, boxes, the daemon. Slice A owns Mycroft and the statement.

Write `research/sherlock-e2e-spec-20261115/astra-C/REVIEW.md` with numbered findings `F-01`, `F-02`, ... Each finding has: class, severity (HIGH / MEDIUM / LOW), the requirement or record ID, what is wrong, the source that shows it (file:line or commit), and the fix you propose. Classes:

1. MISSED: a component, LOOP row, failure case or hard rule of slice C that no requirement covers (compare with your DRAFT.md sections 1, 2 and 5).
2. WRONG: a claim that does not match the code, the measurement or the current ruling. Open the cited file:line and check it. Check every number.
3. UNTESTABLE: an EARS sentence with no observable response, a wrong EARS form (state against event, a hidden second obligation), or a Verification line that names a check that does not exist while the text says it exists.
4. AUTHORITY: a decision without authority, or with the wrong authority word (OWNER / LEAD PICK confirmed / LEAD PICK for T15); a pick stated as a rule; a ruling that the specs changed or narrowed.
5. DUPLICATE: a requirement that restates an existing ID (STL, RKY, MID, LW, LDG, GOV, ERT, ELD, CRR, ZK, ANC, GPW, PRD, LPC) with no added meaning, or contradicts it.
6. STATUS: a status word (BUILT / PARTIAL / TO BUILD) in a "Status of" scenario that the evidence does not support.

End with a table: for each of the 11 capabilities, one line "sound / needs fixes" and the count of findings. Your final message is a ten-line summary.
