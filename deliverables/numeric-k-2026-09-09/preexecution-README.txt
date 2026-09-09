# Conversion rounding and ProRata in K

Status: implementation and source checks; actual K execution and result audits pending.

The definition now computes converted settlement with none/floor/ceil rounding,
rejects overflow before division and rejects free debt discharge through zero cash.
ProRata divides checked nominal*principal and preserves both residual components.
Scale checks precede exponentiation. The codec passes invalid financial inputs to
K rather than deciding success. Settled ProRata debt remains valid for Transfer-only.

[Independent cases](INDEPENDENT-CASES.md):22 new numeric records plus42 unchanged
prior distinct inputs. `check-source-cases.mjs` compares complete preparation
results; `check-cli-cases.mjs` exercises the actual successor simulation CLI.
Neither generates expected values or invokes K. The latter writes a new CLI
receipt, so do not run it over an immutable reviewed observation file.

The proposed allocation is one compile180s plus64krun20s each,1512s aggregate,
whole-tree4GiB zeroSwap immediateSIGKILL and flock. Both exact preexecution votes
and root admission are required. Previous five compile attempts and49krun calls
remain charged. No full SP03, semantic freeze, proof or Midnight settlement claim.
