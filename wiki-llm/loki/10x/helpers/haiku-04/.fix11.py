p='/home/charles/repos/Moriarty-factory/wiki-llm/loki/10x/helpers/haiku-04/astra-1-11.md'
s=open(p).read()
def rep(old,new):
    global s
    assert s.count(old)==1, old[:70]
    s=s.replace(old,new)
rep('1. **ICICLE "up to 25x ... expressly excluding witness optimization" (Line 48) is not in the capture.**',
'1. **[RETRACTED by astra-1 review; my earlier flag was wrong.]** The capture contains both statements: line 19 "up to 25x performance boost" (for ICICLE\'s ezkl-halo2 fork; baseline is not named as CPU), and line 31 "In this work we didn\'t focus on optimizing the witness generation." Draft line 48 is supported. Add the baseline caveat: the 25x is against the ezkl-halo2 fork, not a CPU baseline.')
rep('2. **"Prove interval includes PK destruction" (Line 27, 60).** Inference.',
'2. **[RESOLVED: draft correct.]** `docs/results/sherlock-native-source44-20261022/source/sherlock_link.rs:1230-1234` starts the Prove phase before `prove::<...>`, then `drop(pk)` at 1233 and ends the phase at 1234, so the receipt prove phase includes PK drop. (Earlier inference superseded.)')
rep('3. **"Read-only sherlock-next reports A2 first failing" (Line 123).** Not verified here. I did not run it.',
'3. **[RESOLVED by astra-1 rerun at 22:55Z: A2 first failing.]** Not rerun by me.')
rep('4. **F-table incomplete for the factory schema (Lines 125-134).**',
'4. **F-table incomplete for the factory schema (Lines 125-134). [Accepted; T1 point narrowed.]** Heavy routing: `factory/cli.py:99` sends heavy predicates through `heavy` even under `check`; `CONTRACT.md:43-44` requires `budget` for heavy checks. So heavy at T1 is not illegal. `CONTRACT.md:106` bounds T1 at under 20 min, so F202 stays T1 only if its heavy GPU differential fits that bound; otherwise make it T2.')
rep('5. **F201 dependency differs from my proposal (A1 vs A2).**',
'5. **[ACCEPTED by astra-1: F201 depends on A2 intentionally.]** Original concern:')
rep('- **Ledger reader: `chain/rc5check/ledger-rc5` is the rc.5 tag source; recl9\'s decider verify is rc.6 (`REPORT.md`).',
'- **Ledger reader (corrected per astra-1).** `chain/rc5check/ledger-rc5` is at git HEAD `3997c6c8d1`, not a tag (`git describe --tags` returns nothing). `chain/REPORT.md:33` describes it as built from `ledger-9.1.0.0-rc.5`; that label is unverified by git. Resolve the reader from `ledger-verify` Cargo.lock/Cargo.toml, not the directory name. recl9 measured decider verify is rc.6 (`REPORT.md:160-166`).\n- (superseded) Ledger reader: `chain/rc5check/ledger-rc5` is the rc.5 tag source; recl9\'s decider verify is rc.6 (`REPORT.md`).')
s += '''
## Corrections after astra-1 review (2026-10-24 22:55Z)

- ICICLE 25x and witness-scope statement: both present in the capture (lines 19 and 31). Draft line 48 is supported; add the ezkl-halo2 baseline caveat.
- Prove interval includes PK drop: confirmed at `docs/results/sherlock-native-source44-20261022/source/sherlock_link.rs:1230-1234`.
- sherlock-next gives A2 first failing: from astra-1's rerun; not rerun by me.
- F201 depends on A2: accepted as intentional.
- F202 at T1 is allowed only within the 20-min T1 bound (`CONTRACT.md:106`); heavy routing exists (`cli.py:99`).
- rc5check HEAD `3997c6c8d1` is not a tag; the rc.5 label comes from `chain/REPORT.md:33` and is unverified by git.
'''
open(p,'w').write(s); print('ok')
