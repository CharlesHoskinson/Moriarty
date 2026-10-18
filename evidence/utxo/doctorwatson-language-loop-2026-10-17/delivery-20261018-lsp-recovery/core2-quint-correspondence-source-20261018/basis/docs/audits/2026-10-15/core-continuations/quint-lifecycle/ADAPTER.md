# Normal first-prefix adapter specification — not implemented

This is a documented interpretation between the mathematical model and the frozen oracle. There is no executable Rust/model correspondence adapter, no candidate import, and no comparison result.

| Aspect | Quint representation | Oracle/Rust-side interpretation and limit |
|---|---|---|
| Numbers | Mathematical `int` | Fixed normal numeric values0..142 map to UInt values. No UInt128 overflow/underflow, field conversion, decimal-encoding or boundary-value rule is modeled. Do not generalize this arithmetic domain to Rust UInt128. |
| Balances | Complete account→int map for one specimen's asset, including observer | Source-static asset identity is supplied by specimen, not modeled cryptographically. Map equality compares values/key sets, not serialized vector order. |
| Program state | `programState` map | Escrow marker77; reserve allocated0/basis40→35/marker77. Amount/input is separate. State names are not captures. |
| Liability absence | `dutyPresent` plus `dutyAmount` | Escrow opening: false/0 means no release entry. Reserve opening: true/5 means allocation exists. Present-zero and absent are distinct via the bool; this first increment has no discharge to present-zero. The amount0 used with false is a model placeholder, not an encoded absent duty value. |
| Captures | Ordered `List[int]` | Escrow site1: `[amount UInt10]`. Reserve site1: `[extra UInt2, held UInt42]`. Names/types come from this table. No JSON decode, captures authentication or event binder is modeled. |
| Pending site | 0→1 | Local scalar boundary marker only; no frames of other actions, pending public ABI slots, hashes or predecessor statement are represented. Reserve's third zero padding slot belongs to `/4` ABI and is intentionally absent from the logical capture list. |
| Effects | Ordered list of Transfer/StateChange/LiabilityChange variants | Static identities are the table below. Reserve's let produces a capture and no effect. This is not the Rust Effect wire encoding. |
| Logical work | `logicalAttempted`, `logicalCommitted` | Fresh invocation0→5 for escrow,0→11 for reserve, as independently counted in the oracle. These are logical units, not circuit cost, DUST, fees or a measured runtime bound. |
| Grain | One model transition per successful prefix | No source statement interleaving or per-step snapshot is represented. Adapter ordinals must retain the original indices below; a model step index is not a source ordinal. |

Static asset/duty context:

- Escrow: asset `(network=local, issuer=continuation-escrow-oracle, reference=PAY, scale=0)`; release duty escrow→bob. Transfer effect is alice→escrow for10.
- Reserve: asset `(network=local, issuer=continuation-reserve-oracle, reference=UNIT, scale=0)`; allocation duty reserve→target. State effect targets basis. Opening duty5 accrues42 to47.

Original source ordinals, separate from the model's transition counter:

| Specimen | Executed source ordinals | Ordered model effect → original ordinal | Exit await reached but not executed |
|---|---|---|---|
| Escrow | `[0,1]` | effect0 Transfer→0; effect1 LiabilityChange→1 | ordinal2, delivery |
| Reserve | `[0,1,2]` | effect0 StateChange→1; effect1 LiabilityChange→2; let at0 has no effect | ordinal3, approval |

The exit await's single dispatch charge is included in the prefix work5/11 while its guard/step is absent. Work decomposition is escrow `(transfer1+name1)+(liability1+name1)+await-entry1=5`; reserve `(let1+basis1+extra1+add1)+(next1+basis1+literal1+sub1)+(liability1+held1)+await-entry1=11`.

Normal financial scope: escrow first-prefix gross debit10, fees0, net alice−10/escrow+10/bob0/observer0; reserve first-prefix gross debit0, fees0, no custody movement; dust0 in both frozen examples. Both account maps total142. These are fixed example facts, not security invariants or a conservation theorem. Positive residual duty at the first boundary is release10 or allocation47; neither prefix says settled.

Before a later correspondence test, independently specify extraction of complete boundary state, ordered financial effects, source indices and typed captures without borrowing candidate computations as the oracle. That work and later transitions are outside this first-step draft. Do not change frozen expected values to match candidate output.
