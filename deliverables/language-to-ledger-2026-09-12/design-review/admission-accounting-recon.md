# Admission accounting reconstruction

Repository observation, 2026-09-12 UTC. **Insufficient for current runtime accounting or dispatch; sufficient to preserve identified debits and prepare a bounded amendment.** No runtime file, admission, service, wallet, provider, campaign or original record was changed. The companion JSON carries exact source paths, SHA-256 digests, per-allocation counters, source/artifact checks and the selected read-only SQLite rows. Latest parent-supplied routing applies: Grok 4.6 high author and fresh Astra medium audit; no approval is issued here.

## Concrete time ledger

The original `/home/charl/.local/state/moriarty/mc01-supervised-20260907/budget.json` exists and matches the earlier diagnosis hash `a12ca70bceacd912a6681caee0d7fe1c627ffc188fbe77bbfa7ca0c2ce0f0748`. It contains 177 charge rows totaling 108960 seconds, 228 external charge rows totaling 99135 seconds, and 8 reservation rows totaling 300 seconds. Preserve all 405 debit rows and 8 reservations, including their IDs and candidate/action/runner commitments. The schema permits copying these fields into a separately reviewed current record; copying alone cannot make their coverage current.

Actual reader arithmetic is:

```
master_used =2323.0473305040214 planning +165 overhead +108960 +300 +99135
            =210883.04733050402 seconds
master_remaining =210890 -master_used =6.9526694959786255 seconds
package_remaining =117710 -(2323.0473305040214 +165 +108960)
                  =6261.952669495979 seconds
```

Worker dispatches are 52/52; aggregate recorded dispatches are 83/83; all successor envelopes are exhausted. Elapsed process time cannot refund advance full-bound debits. Later K and financial histories are absent from this unchanged September8 ledger. Its `preview_submissions:0` is historical and cannot replace current submission evidence.

The missing current record requires schema `moriarty.supervised-accounting/1`, pinned authority SHA `f6b7f17d1bda437d5f83abc656ccf4a0d9c7a57a83995668387c29bd0328f492`, package matching the campaign owner, preserved planning/overhead/debit/reservation/dispatch fields, and only explicitly reviewed additive limit changes. Unknown amounts must remain unresolved outside an executable candidate; the reader requires numeric seconds and cannot safely encode unknown as 0. Companion `candidateRuntimeFields` gives the precise minimum mapping. Missing post-September8 charge/dispatch/amendment lineage prevents a complete executable reconstruction.

The common SQLite store has two finished reservations (report and static loan verification), and one claimed debit, `plugin-real-loan-design-host-check-01`. The original budget contains its 120-second debit with matching action, candidate and runner digest. The claim belongs to reservation `46c14ac4-a8d0-4167-9e1c-a2d8834de9aa`; it cannot be reused for K or another loan run. The database is a dispatch/exit store, not the global time or financial ledger. Its exact queried-row digest is in JSON; a changing database file hash is not used as an immutable receipt.

## Actual financial reservation sources

Thirteen distinct current local `sp05*/run/reservations.json` sources were read, without wallet or network access. Twelve byte-match retained repository copies. The remaining stale-loan record matches its public projection SHA `2d3a294a9f3c7942fbe61920471967e3e3f8760d6563598ab79feaad8f3818c5`. Totals:

```
11 earlier local +16 Preview +4 later local loan completion =31 reservations
31 *300000000000001 =9300000000000031 DUST SPECK reserved
```

The additional 4 belong to `sp05-local-loan-completion-20260910-01`; the older 27 narrative predates that allocation. Preserve its retained command result under `local-command-execution-proposal-01/loan/`. This resolves the numerical difference without resetting any allocation. Current reservation files are the provider counter source of truth for their allocation; immutable copies/projections establish retained history. They do not prove global completeness or paid economic fees. Older hello-world, unquantified charges, financial attempts outside the inspected allocations and their original limits still need evidence.

Provider schema is `moriarty.financial-reservations/1` with exactly `schema,binding,reservedSubmissions,reservedDustFee,reservedGrossByAsset,identifiers,active,stopped`. The `.allocation` binding marker prevents recreation after missing state. `providers.mjs` persists a reservation after signing and before finalize/submit; therefore reservedSubmissions counts charged reservations, not necessarily successful or attempted network submissions. `active.phase=submitting` persists in three inspected historical allocations; retain that unresolved state. Never release charges because finality, exit or success is missing. Obtain actual submit attempts from original per-run events/receipts keyed by allocation and transaction identifiers, not from this counter alone.

## K lineage and minimum one-case amendment

parse04 and macro05 each retain one distinct compile and 106 krun command records. Compile starts are 1789091693.5656447 and 1789100334.0692973; argv, timings and runner hashes differ. These are two actual histories, totaling212 inspected krun attempts, not duplicate 106 snapshots. Both complete source and artifact manifests were rehashed: no mismatches. The final command exits 139 versus 113; all exact pins and commands are in JSON. Recorded aggregate observations 358.0397113850049 and 346.2238577800017 seconds are observations, not refundable allocation charges.

The copied original proposal02 specifies 113 calls/2512 seconds and original worktree `sp03-expression-k`, while both retained artifact runners encode 115/2552. It also preserves an earlier numeric history of 113 calls / 6 compiles. Those three meanings must remain separate. Original proposal resource/source votes do not establish the missing parse04/macro05 amendment and root-admission lineage. Do not infer 7 unused calls, collapse 212 to 106, count original numeric 113 as the expression budget, or reset a compile marker.

A later reviewed amendment can select one exact existing artifact and retained trace 106 input, authorize **one additional krun, zero compiles, at most 20 seconds child execution**, and state separately bounded startup/diagnostic/output/cleanup costs. Charge its full new bound before dispatch; preserve original 113/2512 allocation commitment, both actual histories and all earlier unknown charges. Verify current containment prerequisites only at admitted execution. The runner requires an exact action/candidate/runner debit and sufficient remaining authority; its reproduce assertion exit range 2–123 also means raw 139 cannot be the declared behavioral assertion without an explicitly reviewed diagnostic adapter. No adapter or new action is implemented here.

Required amendment evidence is narrowly defined: original resource authority and debit links for both sibling executions; complete later charges or a reviewed conservative preservation disposition that grants no old credit; selected artifact/input/tool pins; exact one-case action, containment and new debit; current source/candidate binding and substantive fresh review. Until these exist, neither repairing the stale SP01 binding nor writing a syntactically valid accounting file admits the reproduction.
