# F-2 / H15 oracle freeze note

- Author: Claude Opus 5.5 (`claude-opus-5-5`), test-source author. Not an independent reviewer.
- Candidate: `dd4e4298322666c140c74343a7827000f578c848`, clean. Baseline for the derivation: `fe01bc8b`.
- Frozen at 2026-10-15T06:43:12Z, before any assertion code was written. `test_source_exists_at_freeze: false`.
- Nothing was parsed, simulated, exported, built or run to produce these bytes. The fixtures were typed by hand.
- A format self-check (Python `json.dumps(indent=2)` round-trip) was declined by the harness and **not performed**.

## Source identity (the test constant)

```
language "moriarty/1"; agreement Identifiers { state count: UInt = 0; action run() { let await = 1; let when = await; next count = when; } }
```

Action `run`. Default scenario: `{"state":{},"inputs":{},"balances":[],"liabilities":[]}`.

## Fixtures

| File | Bytes | sha256 |
|---|---|---|
| `crates/mori-core/tests/fixtures/compat_v1_identifiers/program.json` | 881 | `27f7719725a0f2c0cf03efdb62586b11f328cc606309db6e245bd88c3f594699` |
| `crates/mori-core/tests/fixtures/compat_v1_identifiers/execution.json` | 2092 | `88315015f9b0eb7a6d10378d9eac4f48d77db9c56dd750f471daf76006367d7d` |

**Byte convention.** The bytes equal `mori_core::json::export(value)` followed by one `\n`, which is what CLI `core_io::print` emits via `println!`.

- At baseline, `json.rs:109-133` shows `export` is `serde_json::to_string_pretty`: 2-space indent, `": "` separator, and `[]` for an empty array.
- It escapes only non-ASCII and control characters, and every byte here is printable ASCII.
- The `fe01bc8b..HEAD` diff of `json.rs` touches only `parse`/`parse_limited`. `export` is unchanged.

## Hand derivation (git show fe01bc8b:crates/mori-core/src/…)

- **Program key order** (`ir.rs` `Program`): `language`, `semantics`, `name`, `states`, `assets`, `expressions`, `actions`.
  - `StateDecl`: `name`, `ty`, `initial`.
  - `Value` is adjacently tagged `{type, value}`. `UInt` serialises as a decimal string.
  - `Expr`/`Statement` are internally tagged by `kind`, followed by their fields in declaration order.
- **Parser** (`parser.rs`):
  - `language` is the `LANGUAGE`/`SEMANTICS` constants.
  - `word()` accepts `await`/`when`, because `is_word` = Ident or keyword. `statement()` dispatches on `let`/`next`.
  - `expr` pushes primaries in source order:
    - #0 `Literal UInt 1`
    - #1 `Name await`
    - #2 `Name when`
  - `;` ends each binary loop, so no `Binary` node is created.
- **Validate** (`runtime.rs:8-37,120-317`): `await` and `when` are not in the reserved list, and neither shadows `count`. `Next count` has type UInt.
- **Simulate** (`runtime.rs:697-891`):
  - `pre` comes from the declared initial value, because the scenario state is `{}`: `count = "0"`. Balances and liabilities are empty.
  - Each step records `index` (u16), the statement clone, `value` (`Some(v)` serialised bare), `snapshot` (the current state after the step) and its effects.
    - Step 0 `Let await`: value 1, count 0, no effects.
    - Step 1 `Let when`: value 1, count 0, no effects.
    - Step 2 `Next count`: value 1, count 1, one `State` effect `{kind, name, before "0", after "1"}`.
  - `post` is count 1. The top-level `effects` are the concatenated step effects.
  - Execution key order: `semantics`, `action`, `pre`, `post`, `steps`, `effects`.

**Not derived:** the current-tree behaviour. That behaviour is what the test checks.
