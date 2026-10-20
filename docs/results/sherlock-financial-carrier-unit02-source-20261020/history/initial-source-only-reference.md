# Financial carrier source boundary

The crate-private `helper_machine::shared::fin_carrier` component adds checked source bindings and financial snapshot carriers. It resolves strict source names, distinguishes state rows from name IDs, checks plan arithmetic and action identity, and preserves typed state, asset, balance and liability rows through Core snapshot conversion. Mutation helpers validate the source/snapshot binding and return typed errors for invalid state, transfer and liability updates. Production callers outside this component are not yet wired.

`FinBoundSource`, `FinAdmittedExtras` and `FinSnapshot` keep their construction fields private. Their identity is structural over the producing names, states, assets, actions and capacities; it is not a program digest, authentication result, complete ProgramView validator or public finance admission. `apply_set_state`, `apply_transfer` and `apply_liability_add` retain their literal reviewed checks and atomic refusal behavior.

Two fresh independent Muse source/design/security/API opinions approve this sealed carrier prerequisite. Root adopted only that source prerequisite. Sixteen tests are source-exposed assertions; at this publication preparation cut they have not run. The proposed qualification builds current merged typed-input Core normally and with its real full cfg(test) root, then selects only these sixteen tests. No successful compile or runtime result is inferred from the source opinions.

The direct indexes require their proven bounds and sealed production construction; the owner's old prose saying there are no direct indexes is inaccurate. The tests do not separately assert from-Core liability overcapacity; some refusal preservation checks are grouped, and a fixed fixture helper ignores its `rows` argument. These residuals remain disclosed without changing the approved source.

Full financial lowering remains required: all twenty XFin transitions, financial scalar entry, recorded source/ref/env provenance, selection and frame invariants, public invocation, genesis, stored admission capacities, continuations, canonical wire/digests, independent schedules and complete financial outcomes. This component does not close the fourteen whole-finance findings or all twelve original counterexample predicates. Native constraints, proof, target acceptance and EARS closure remain open.

The linked source archive records exact source and independent review provenance. It contains no compiled artifact or private oracle data. These project review records are not developer compile, prove or deployment permissions.

See [source qualifications](../../results/sherlock-financial-carrier-unit02-source-20261020/SOURCE-QUALIFICATION.json), [review A](../../results/sherlock-financial-carrier-unit02-source-20261020/A-FINAL-RESPONSE.txt) and [review B](../../results/sherlock-financial-carrier-unit02-source-20261020/B-FINAL-RESPONSE.txt).
