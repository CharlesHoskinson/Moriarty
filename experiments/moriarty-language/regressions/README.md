# Public Rust Source/6 financial consumer regression

UNRUN source proposal. Predicate: for the frozen Invoice and NEW custodian-chosen Workshop scenarios, a reviewed feature-enabled Rust `mori --quiet --color never expand FILE --action pay --scenario JSON --emit-source6-transfer` emits bytes consumed unchanged by the existing experimental TypeScript Source/6 parser/lowerer and provisional local Core/5 preparer. The comparator checks the independent financial/input projection only.

Run explicitly from the repository root with Node.js24, after reviewing the exact binary and source candidate:

```sh
MORI_RUST_BIN=/absolute/reviewed/target/debug/mori MORI_SOURCE6_OBSERVATIONS=/absolute/existing-parent/new-observations node --test experiments/moriarty-language/regressions/rust-source6-transfer.mjs
```

Both environment paths are required. The observations directory must not exist; its parent must exist. A missing feature, failed invocation, rejected local consumer or financial mismatch fails rather than skipping. This separate opt-in directory is outside `npm test`'s `tests/*.test.mjs` glob; it neither changes normal package tests nor automatically builds Rust. Node24 strips the existing TypeScript files directly, without npm installation or SDK use.

If a reviewed binary is not already available, the integrator can separately inspect and authorize this ordinary nonnative build, with its final lock and dependency availability established first:

```sh
cargo build --locked --offline -p mori --bin mori --no-default-features --features source6-expansion
```

Do not omit `--no-default-features`: this crate defaults to `native`. The regression invokes exactly the supplied absolute executable and records its SHA256; the integrator must bind that binary to the reviewed source/build receipt. It cannot infer its source tree or build features from the file digest. This proposal executes neither command.

Each case records process.json, exact stdout.source6/stderr.txt and actual-consumer.json (actual parsed/lowered/prepared values, including observed work fields). Stdout is UTF8 decoded with fatal errors, without trimming, JSON unwrapping, replacement or host-generated Source/6. The unmodified string reaches both `parseAndLowerSource6` and `prepareSource6S0Unqualified`. The two calls share the actual lowerer; agreement between them is not an independent oracle. The financial comparator uses the independently frozen data, never the Node beta emitter or the Rust library emitter to generate expectations.

The fixture subset preserves exact input/fact bytes from custodian freeze `64a6b4a6e454bbc6bc2cf3582581e81e3dd5fd379f9ed3fc89bbfc8051180269`. FIXTURE-PINS.json pins all six selected files and identifies the full external freeze; it is not a replacement whole-packet freeze. The complete original packet is available under custodian-packet/ in the review handoff. Workshop is a newly chosen input, not a recovered missing historical scenario. The source/test adapters are author material, not independent goldens.

Assertions cover opening balances/allowance/head/round/replay and work counters; exact decimal/addition value, fee, caps and floor; ordered debit/credit/allowance/replay/head effects; and conditional provisional-S0 local post balances, allowance, replay, head and absent obligation. The effect adapter only renames fields into the custodian constructor/arguments shape and decodes the documented replay tuple. It calculates no expected effect or post-state. PreparedUnqualified is the callable local-classification predicate required to inspect a candidate, not a custodian-supplied accepted flag or actual admission. Its named external premises remain unresolved. Full nominal mapping, whole wire/DTO/origin golden, successful work policy/cost/post counters, a duty DTO, authenticated global frame and native/proof/ledger results remain absent. No complete UNI-019/020 observation or parent exit closes.

Full source-after/ includes the exact prior adopted30 plus nine new regression/fixture files. The delta patch applies only those nine files to an integrated adopted candidate; use it when root combines the security owner's separate repair. The combined39-file patch also includes the frozen adopted30 patch and applies to that proposal's reconstructed reviewed base; it is not a patch against current main. No production Rust/TypeScript/helper, lock, manifest or prior fixture is changed by this delta. The separately owned presentation repair is not included or judged. Final integrated source identity and fresh applicable audits are root-owned.
