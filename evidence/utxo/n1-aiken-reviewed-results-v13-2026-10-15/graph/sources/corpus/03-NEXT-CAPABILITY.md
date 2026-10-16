# Next capability: adversarial internal witnesses on the held-out programs

This is a recommendation. It creates no authority, budget, campaign or counter, and it changes no frozen byte. Native/SDK authority stays 0.

## Why this step

r3 shows that honest completion of each frozen hostile claim fails, and that the public statement is bound. The step to arbitrary-witness evidence is still missing for the gadgets HW-01..HW-07 name and for computed Bool wires (HW-23). In E5-r3 the only tampered witnesses are two entry Bool wires (M-10/wire, HW-23-r3/H6-B1/wire-two). Tampered internal gadgets appear only in E4, on author programs. The existing `Tamper::set(label, value)` API and the `overrides_used` / `which_is_unsatisfied` reporting are enough to run the missing cases on held-out programs. No new framework is needed.

## Falsifiable predicate

For each case below, a witness built from the frozen claim, with the named internal wire or wires overridden through `Tamper`, gives `is_satisfied() == Ok(false)`, and `overrides_used` equals the number of intended overrides. **Any `Ok(true)` is a soundness defect.** An `overrides_used` mismatch means a wrong label and is a harness FAIL, never a pass.

| Case | Held-out base (relation) | Override, from the frozen HW text |
|---|---|---|
| HW-01 | H6-S2 (`sub`) | result limbs set to the field encoding of p-1, with the result bit wires chosen to satisfy the bit-sum equations |
| HW-02 | H6-M1 (`mul`) | product low limb 0, high limb wire hidden or nonzero, product bits adjusted |
| HW-03 | H6-A2 (`add`) | sum 0 with the output bit wires forged (there is no carry wire: record that) |
| HW-04 | H6-C2 / H6-C* (`cmp`) | `lt` flag flipped to 1 at x == y; flag wire = 2 |
| HW-06 | H5-R1 (`probeOr`) | rhs Sub/comparison wires set to garbage while the lhs is true |
| HW-07 | H5-R2 (`probeAnd`) | as HW-06 for `and` |
| HW-23-internal | H6-B* (`beq`) | computed Eq flag wire = 2 and = p-1 |

## Conditions (existing pattern only)

1. A fresh non-author freezes the case list, the exact wire labels (taken from the candidate's `lower.rs`/`gadgets.rs` role names, read from source) and the expected outcome **before** any harness edit. The freeze is a new dated addendum and is labelled AUTH unless a reviewer authors it.
2. The harness r4 adds only these rows. It keeps every r3 row and assertion and changes no candidate byte.
3. Fresh source review, two substantive votes on the step list, root authorization of exactly one attempt, and the same r3 operator pattern, limits and never list (no setup, keys, prove, verify, network, git, SDK or ledger).
4. A fresh result review of the actual logs.

## What it would and would not establish

- **If every case is UNSAT:** HW-01..04, 06, 07 and internal HW-23 move from honest-completion-only to "refused for the specific adversarial internal witness tried", on held-out programs. That is still not a proof for all witnesses.
- **It does not touch** F-04 (M-14 stays SAT), F-05, the ABI v1 byte image (M-11b), setup, VK, proof or ledger. Those remain the next boundaries after this one, and the setup/proof/ledger steps need native authority, which is currently 0.
