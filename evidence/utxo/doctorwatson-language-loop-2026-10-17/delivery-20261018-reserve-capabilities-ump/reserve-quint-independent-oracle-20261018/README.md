# Reserve Quint/Rust independent finite oracle

Inputs: original reserve source/scenario, cap36 and ordered Approval41, Approval43, Settlementfalse, Settlementtrue events. They are copied byte-for-byte from the pinned input-only handoff, without reading new tests or expected traces.

`reserve-common-projection.expected.json` contains complete initial/prefix FinancialFrame and five lifecycle Observation records under the separately pinned, proposed common JSON codec. `MODEL-TYPED-VALUES.qnt.literal` retains the corresponding typed mathematical values independently of that serialization. Split initial/prefix/observation JSONs are provided for scoped consumers. Actual Quint ITF serialization and its conversion remain unobserved.

`INDEPENDENT-FINANCIAL-WORK.json` additionally preserves every raw source prefix's complete balances/duties/typed state and ordered effects. Reserve ends56, target42, treasury2, observer42, positive allocation duty5. Attempted invocation work11/4/15/2/10 sums42; committed ends36. Lifecycle initialization has last=NotInvoked and attemptedTotal11, rather than an invented start Invocation.

`DERIVATION.md` and `PREMISES-AND-OMISSIONS.json` retain conditional schema/static identity assumptions, omitted raw Step/metadata/byte fields, original14gaps and previous exposure. The codec is authored schema, not independent compiler-wire proof or actual ITF bytes. All product/model/build execution remains UNRUN. No full native, proof, ledger or general-language correspondence is claimed. See FREEZE.json, PAYLOAD-INVENTORY.json and FREEZE.sha256 for exact custody.
