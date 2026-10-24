

**Obligations map** (source-level only; nothing was run, applied or hashed)

- **B1 (builtin rows by position).**
  - `typ` maps rows 0..3 to `UInt`/`Bool`/`List`/`Tree` by position. Each such row must have `decl None`, args exactly `{start:0,len:0}`, `name_id` 0 (never looked up in `names`), and the ctor ranges (0,0),(0,0),(0,2),(2,2).
  - A user row at index 4 or higher must have a declaration, and its argument count must equal `decl_param_counts`.
  - `_builtin_rows` checks `entry == 0` and the exact `new_checked` seed: constructors 0..3 and the five field rows.
  - `ctor` refuses builtin TypeIds.
  - The existing nominal owner, field and generic checks are unchanged.
- **R4 (single buffer).** `checked` hashes and returns the same buffer from one `pin_raw` read and compares with `same`. Canonical realpath pins are unchanged. `project_command` now reads the bindings file once.
- **R3 (located failures).**
  - `capture_case` wraps everything from `binding["capture"]` onward. It catches `AttributeError`, `ValueError` and `OSError` in addition to the existing errors.
  - Every receipt carries a location: the `Gap` path, or `where` (file name or `records.jsonl:n`).
  - Producer errors and accepted pins collected before a failure are kept. Only accepted pins are rechecked later.
  - `main` also catches `ValueError` and `AttributeError`.
- **Efficiency.**
  - Reference and inline admissions are normalized to one immutable admission object. Inline is first checked for equality.
  - `same` short-circuits on `is`, so per-record continuity no longer walks the Program. Full image continuity and terminal equality are unchanged.
  - `jsonl` streams one record at a time from the checked buffer and requires the final LF.
  - The unlocated value and environment ledger rows are removed. Raw env roots and the scrutinee ref are now attached to the call and match rows.
  - `ReturnCallValid` uses a direct row handle instead of rescanning the ledger.
- **Lifecycle.**
  - Exactly one `KAdminPublishValue`/`Published`/`Publish` event is allowed, and no event may follow it.
  - Publication also requires `enter` and `plan` to be empty. `finish` requires the publication.
  - All five symbolic fields, their orders, the checkpoints and `first_difference` are unchanged.
- **Empty slices.** Strict normalization applies only to occurrence paths and to the positional builtin args. Every other `cut` checks bounds only.

**Residual finite prerequisites**

- **Separate test delivery.** Fixtures must use the owner layout: builtins 0..3, the seed constructors and fields, and user rows from 4. The old `sample()` violates this.
- **Lifecycle tests.** These must drive all five files through `capture_case`, including Publish and Terminal. The old tests also need these changes:
  - `test_body_entry…` mocks `p.enter`, so its rows now lack the new `"row"` key.
  - `finish` now needs `published`.
- **New fail-closed shape requirements to note:**
  - binding item keys must be exact;
  - the JSONL final LF is required;
  - an inline admission still costs one Program walk per boundary that uses it.
- **Not changed here.** `compare_command` keeps its pin-then-`json_pin` double read, but each parse still uses the hashed buffer. There is no fit claim against the 30 s / 25 s CPU / 2 GiB / 64 MiB caps.
- **Next gates.** Fresh nonauthor review, then the revised synthetic PASS, then Root pin/binding freezes. Ops02/03 stay UNRUN.