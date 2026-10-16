# Fresh wallet operator — outer-bound scope addendum

## Status

Additive proposal only. No wallet exists, no keypair was created, and this addendum authorizes nothing by itself. The original five candidate files stay byte-identical in a new sibling candidate R2 root; only these four addendum files are new. Authorship under the indexed user Gemini/Muse fallback is not independent approval.

## What changes

Only scope qualification changes. Original clause P-25 claimed a 120 s whole-execution cap; the reviewed code starts that clock after archive extraction and adds up to 10 s per-call kill cleanup, so that claim is historically preserved but not true. New clause P-29 (source author: FOR) adds the concrete outer bound: `RLIMIT_CORE=(0,0)` inherited, 180 s from operator spawn including staging/extraction, TERM then kill after 10 s grace, absolute maximum 190 s. The exact launch argv, minimal environment, `/dev/null` stdin, and caveats (separate bwrap sessions, D-state, non-transitive CPU) are in `OUTER-EXECUTION-PROPOSAL.json` for fresh-audit checking.

New clauses P-30 to P-32 (source author: FOR) record that the historical P-02/P-11 null/unresolved wording is superseded only by the reviewed root fills and ELF/source pins (operator `7944d08f…`, member `01b13170…`, static, no PT_INTERP/PT_DYNAMIC), that leaf argv/defaults and top dispatch are source-closed by the pinned Haskell supplement with no runtime probe or reproducible-build claim, and that the secret/public boundary is unchanged.

## What does not change

One attempt, zero retries, at most four CLI calls, no existing-key access, no signing, network, faucet, transaction, or later spend authority. Root must not read, stat, hash, or walk the private root; only existence/mode booleans leave the operator. Raw native outputs are never public.

## Next boundary

Fresh full candidate+addendum source audit and two FOR/AGREE votes on P-29 to P-32, then a separate actual `AUTH.json` (`AUTHORIZED_ONE_RUN`, exact existing key set, no extras), then exactly one wallet generation, then a fresh actual audit. No funding advice before that audit. All 20 UTxO families, ACTUS/DeFi/general obligations, recursion, private handoff, bounded multi-parent PCD, universal certificates, and real Preprod transactions remain open.
