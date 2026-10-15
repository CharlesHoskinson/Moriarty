# mori-cardano: current source inspection metadata

`inspect_cardano(source, accounts, InspectionTarget::Unit)` calls the actual
`mori_core::export_source` and describes the admitted current `/1` source subset.
It preserves the full parsed Program and returns deterministic state, asset,
account, duty, action and framed-cell metadata. Assets/actions preserve source
order; states/accounts/duties use bytewise name order; framed cells use full
AssetId order then account. Complete bound-account × asset counts and conservative
Core trace-cell upper bounds are informational metadata, not measured resources.

This first component is UNIT-only. Account bindings are symbolic classifications,
not physical credentials. Source AssetIds remain symbolic. There is no real target
identity, applied layout hash, trace, validator or admission. `PinnedCardano` and
`require_layout_hash` return explicit unresolved-interface errors. Output records
are public metadata, never caller-supplied proof or admission authority.

The crate has no SDK, ledger, Aiken, process, filesystem or network consumer.
It does not evaluate financial actions. Physical identity derivation, hash
preimages, bootstrap, proof bundles, emit/build and ledger operations remain
separate work with their applicable review and execution boundaries.

Run the bounded standalone consumer from the repository root:

```sh
cargo test --offline --locked \
  --manifest-path crates/mori-cardano/tests/metadata-consumer/Cargo.toml \
  --target-dir /tmp/mori-cardano-metadata-target --lib
```

That consumer has its own workspace/lock and selects the actual library while
avoiding parent workspace/SDK dev-dependency suites. Adjacent fixtures preserve
independent raw source, complete expected Core Programs, symbolic account metadata
and source-duty identities. They are not real Cardano target/ABI fixtures.
