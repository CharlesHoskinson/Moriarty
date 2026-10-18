# Forward an explicit custody oracle-results path

Prepared source-only fix against immutable64c6dedff1d99b33c23832330d7911a5fad9ed59. Full candidate/ and candidate.patch contain exactly three files: one CLI production edit and two new ordinary argument-transport test files. Existing compiler implementation, oracle binding, Q1 publication flow, owner/security tests and security fixtures are unchanged.

The existing public `compileCustody` API accepts `oracleResultsPath = DEFAULT_ORACLE` at src/compile-custody.mjs202 and reads that selected file at312. The actual public CLI never supplied the property at src/cli.mjs307–316, so `--oracle-results FILE` was ignored. The fix forwards `oracleResultsPath: arg('--oracle-results', undefined)` and advertises the optional flag in usage. Explicit relative/absolute path text is forwarded unchanged, as for existing input options. Omission delegates to the existing API default; its historical absolute string remains unchanged and was not followed.

The ordinary regression executes the actual CLI entry, argument parser, manifest read and compile-custody dispatch in a child. A Node `registerHooks` preload substitutes the CLI's direct operation imports with argument-capture probes to avoid loading compiler/native dependency graphs. The compileCustody probe records its options and returns an explicit non-success marker. Assertions cover exact path A/B, spaces, relative spelling and omitted/undefined delegation, plus preservation of every other supplied option and the existing false-result exit mapping. Removing the forwarding property makes explicit-path comparisons fail. These are argument-handoff expectations only; no oracle content, file existence, compiler validation, output-path policy or generated spending result is tested. Probes supply no compile-success result.

Astra supplied exact CLI/API source pins and independently specified routine path/omission expectations (copied ASTRA-EXPECTATIONS.json SHA2becde9f83740f3269251a821cd5908acf2216123c661b07c8e073ba7fc189a7). This corroboration is source-exposed, not a fresh provider audit or financial oracle. The ordinary author inspected those pins and actual CLI/API source; no oracle content/trust judgment is supplied. Prior wave compiler finding and all original packets remain untouched.

Proposed root command, UNRUN (Node24 with built-in registerHooks):

```sh
node --test experiments/moriarty-eutxo/tests/compile-custody-cli-args.test.mjs
```

The selected test uses only Node built-ins, reads the actual CLI entry and its capture preload, creates its own temp manifest/options capture, spawns Node children with the isolated operation-import probes, captures stdout/stderr and removes only its temp roots. Other operation imports throw if called unexpectedly. It does not load compiler/native operations, read the supplied oracle path, traverse the excluded external tree, invoke Aiken/SDK/VM/proof/key/wallet/ledger tools or write shared output. Broader `npm test` is outside this command and would select existing native/security/external-fixture tests; it was neither changed nor run. The production compile-custody command still has its existing compiler/Aiken/read/write graph, so this path option alone is not runtime eligibility or generated-spend qualification.

All tests and syntax/runtime checks remain UNRUN. Pure text/hash/patch materialization checks alone are recorded in STATIC-CHECKS.json; they are not a test pass. No Git/shared checkout mutation, provider dispatch, resource/cache reset, native allocation or acceptance change occurred. EUC012/EUC013 and all148requirements/292scenarios/20families/U0–U7/M1–M17/MC/SP/G/financial/composition/native/target obligations remain open with original limits.
