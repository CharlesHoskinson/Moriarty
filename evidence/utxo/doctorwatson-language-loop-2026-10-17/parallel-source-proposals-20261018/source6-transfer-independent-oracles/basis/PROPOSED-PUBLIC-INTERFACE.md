# Proposed explicit Rust beta Source/6 transfer expansion

Status: unadopted interface/source proposal. No compile, runtime, native or release qualification is established. The `source6-expansion` Cargo feature enables the explicit local transfer presentation path; it is separate from `native` and does not alter default features.

Existing default command, unchanged:

```sh
mori expand core.mori --action noop --scenario core.json
```

This still validates a Core action against its Core scenario and prints its generic Core Program JSON. A Rust beta file supplied without the new option still reaches the existing Core parser and refuses; no default beta output is replaced. The separate Node beta package and its default JSON expansion output are unchanged.

Proposed explicit command, in a build with `source6-expansion` enabled:

```sh
mori check invoice.mori --json
mori --quiet expand invoice.mori --action pay --scenario scenario.json --emit-source6-transfer
```

The explicit command reads a typed local transfer scenario and calls `mori-source6::expand_transfer`, which uses the real Rust beta parser and checker. Stdout is the actual generated Source/6 text with its existing final newline; `--quiet` leaves stderr empty. Without `--quiet`, stderr reports `Expanded Source/6 transfer proposal (local-stipulation-only)`. It does not emit a Core success result. Without the build feature, the explicit option reports `mori::command::source6-unavailable`.

The proposed scenario JSON is the closed **transfer subset** of the existing `moriarty-local-scenario/1` / `local-stipulation` shape. It requires domain, asset, head, predecessor, round, exactly three ordered balances, allowance, replay, work_remaining, work_spent and post_head. Numeric values are canonical decimal strings parsed through the existing Core UInt decoder; no new decimal evaluator is introduced. Balance order is owner, recipient, fee recipient. Replay is `unused` or `consumed`. Obligation and candidate_effects overrides are outside this option. The current strict CLI reader and existing read/JSON bounds apply.

This profile-token reuse, transfer subset, option name, feature name, raw stdout/status output, and JSON-decode-before-helper error ordering are **new unadopted public interface choices**. They require the planned independent design/interface votes; they do not establish full Node beta wire or diagnostic parity. The typed borrowed local scenario passed to the library is an internal input, not proof or authentication.

Boundary: Rust beta/1 → proposed transfer presentation → Source/6 text. The real experimental TypeScript `parseAndLowerSource6` must still consume actual emitted bytes; its Core/5 preparation remains separately unqualified. This option provides no Source/6/Core/5→Core/1/Core/2/Sherlock/native embedding, no handwritten target IR and no target/proof/post-state success. The default Core and separate Node consumers retain their own profiles.
