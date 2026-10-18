# Independent Astra loader-amendment design vote

**REVISE.** Proposal SHA256 `6072e78aba3608d89bb3081a97ea06dae40728df3ecae2bf808edd7225d1ff80`.

Direct pinned Java parser is a sound narrow direction, but parser-only selection leaves JSON output conversion reaching original checkJava. Native environment empty-component handling also needs explicit disposition. These omissions prevent acceptance of this exact proposal.

Independent delegated non-author context; parent requested Astra medium. No separate provider-returned model/effort attestation exposed. No Opus opinion read.

Source fact: `krun` calls `kore-print --output json` if `result.kore` exists. `kore-print` lines 230–231 dispatch absolute sibling `kast`, which reaches `k`, `setenv`, and `checkJava`. This happens even though the input parser was replaced. A predicted crash is not durable exclusion of that branch.

Source fact: the Linux64 native directory named by `setenv` is absent on this installed package. The original append from an unset LD_LIBRARY_PATH introduces an empty CWD component. Removing it needs an explicit selection disposition rather than an identical-environment claim.

Exact expected parser Java argv (specified only):

```json
[
  "/nix/store/kjb3a2ip8ydb3zm604agim5ccjja3vfj-openjdk-headless-minimal-jre-21+35/bin/java",
  "-Dfile.encoding=UTF-8",
  "-Djava.awt.headless=true",
  "-Xms64m",
  "-Xmx8192m",
  "-Xss32m",
  "-XX:+TieredCompilation",
  "-ea",
  "-cp",
  "/nix/store/y63xkr8pk2bqd5lh4889rlwldw26v9f4-k-7.1.337-4a46d1231473b599c699160132fd6e76a5c46406/lib/kframework/java/*",
  "org.kframework.main.Main",
  "-kast",
  "--input",
  "json",
  "--output",
  "kore",
  "--definition",
  "/home/charl/Moriarty/.worktrees/sp03-expression-k-macro05/experiments/moriarty-language/formal/k/.build-expression-v1/expression-v1-kompiled",
  "/home/charl/Moriarty/.worktrees/sp03-expression-k-macro05/experiments/moriarty-language/formal/k/.build-expression-v1/trace-106.input"
]
```

Mandatory conditions:

**A01.** Resolve the output path before design acceptance. krun invokes kore-print --output json whenever result.kore exists; kore-print invokes absolute sibling kast for json (lines 230-231), reaching unchanged checkJava. Expected interpreter crash does not exclude successful output or a partial result file after failure. Specify a minimal reviewed suppression route for all reachable Java loader entries, including output, or a durable justified exclusion. Do not change output mode or claim parser-only suppression covers the whole diagnostic.

**A02.** Pin exact Java executable and complete package jar closure. Parser argv must be an array with no shell/eval: fixed JVM flags followed by org.kframework.main.Main -kast --input json --output kore --definition EXACT_RETAINED_DEFINITION EXACT_RAW_INPUT. Keep package Java wildcard as one argv element or justify any enumerated classpath ordering change. No HOME, relative, empty or unbound classpath entries; no ng process or server selection. Bind jar symlink targets as well as package map/requisites.

**A03.** Preserve fixed applicable fallback JVM options in source order: -Dfile.encoding=UTF-8 -Djava.awt.headless=true -Xms64m -Xmx8192m -Xss32m -XX:+TieredCompilation -ea -cp PACKAGE_JAVA_WILDCARD. TieredCompilation applies to pinned 64-bit Java; bind architecture/version evidence offline. Do not invoke Java --version as an unbudgeted probe. K_OPTS is absent in clean original diagnostic environment; accept no ambient K_OPTS or JAVA/JAVA_HOME override.

**A04.** Explicitly resolve native environment difference. Linux64 setenv prepends K/lib/kframework/native/linux64 to PATH, appends that directory to LD_LIBRARY_PATH, exports LC_NUMERIC=C, and with absent TERM sets K_COLOR_SUPPORT=1. Under original clean environment LD_LIBRARY_PATH becomes a leading colon plus the directory, an empty CWD search component. That native directory currently does not exist. Do not invent libraries, silently copy the unsafe empty component, or claim identical environment when removing it. Freeze a package-only native library setting without empty components, document this selection change, and bind actual needed native dependencies through existing closure trust. Preserve applicable locale/color behavior and original HOME. Do not add a replacement Java library path without source justification.

**A05.** In the parser child only, reproduce k shell soft-stack raise to inherited hard limit before Java exec, keeping hard limit unchanged. Record exact inherited hard limit and handle RLIM_INFINITY explicitly; reject incompatible drift/failed limit change before exec. Do not impose Java 8 MiB soft stack accidentally. JVM -Xss32m is distinct from OS RLIMIT_STACK. Outer krun and retained interpreter retain 8388608-byte soft stack and core0; parser subprocess stack mutation must not affect parent/interpreter. Record process lineage change from bypassing shell wrappers; never claim identical provenance.

**A06.** Construct an allowlisted environment before Python startup and Java invocation. Suppress PYTHONPATH/PYTHONHOME/user-site/startup selection, BASH_ENV/ENV and exported shell functions, JAVA_TOOL_OPTIONS/_JAVA_OPTIONS/JDK_JAVA_OPTIONS, CLASSPATH/K_OPTS/JAVA overrides and dynamic-loader injection. Keep HOME identity/value unchanged. Present HOME extension, local socket configuration and default nailgun configuration must not alter executable selections. No endpoint connection is an offline test.

**A07.** Keep original expression-parser.py, lock/binding bytes, full original compiled definition/interpreter and trace106 immutable. Bind new diagnostic-only parser separately. Preserve original recordedArgv and record amended argv; constrain every difference to expressly adopted loader changes. Input/definition/executable/closure drift rejects before exec. No host semantic parsing, evaluator, admission narrowing, type erasure, replacement JSON-to-KORE implementation, compile or copied/edited Nix distribution.

**A08.** Preserve accepted metadata depth5947/Core65536 and rejected depth5948/Core65547. Preserve --no-expand-macros and exact interpreter/trace bytes. Loader completion or parser failure is not interpreter failure evidence; actual review must distinguish parser, interpreter and output-conversion process outcomes from raw records.

**A09.** Repair E04-2..4 and retain D03-I1..I5. Verify actual reverse wrapper prepend/de-duplication and krun/setenv stages; pin every selected helper, env/shebang interpreter, Java/LLVM/kore and resolved target. Require host binding for links and targets, query package and trusted Nix database/ancestors; compare stat to stat and lstat to lstat. No unbound UID/overflow fallback. Revalidate mapping and effective nonwritability under actual strong containment with injected exec.

**A10.** Before source freeze run offline real callable argv/environment/limit construction with Java exec and limit mutation injected. Cover positive exact selection and negative input/definition/java/jar/helper/ownership drift, earlier PATH competitor, symlink replacement and correct unchanged symlinks, HOME extension plus both server settings, Java/Python/loader injection, missing/incorrect native environment, finite/infinite hard stack and failed stack change. Exercise success-output and partial-output branches to establish no remaining checkJava reachability. Tests must inspect actual supplied source, not a separate idealized helper.

**A11.** Freeze complete augmented candidate and obtain both fresh independent exact-source audits; preserve original audits. Update canonical runner/pointer/manifest/argv/candidate/debit identities without resets. Retain exact stored-receipt stream recovery, UTF8 hash/byte checks, rejection of overflow/ambiguity/124/missing or mismatched receipt, existing harmless runner controls and separate actual-result review.

**A12.** No resource or dispatch authority follows this vote. C7/D1/G60 stays prospective: one diagnostic, timeout20 including preflight, grace5, full60 charge, compile0/retry0/no refund/no slack. Separate native parser-equivalence testing would need separately reviewed bounded resources; do not spend this diagnostic on a canary. No new runner, collector or campaign.

Local source inspection and byte hashing only; source reachability is not runtime observation. Prior offline reproducer inspected, not rerun. No K, Java, ng, strace, compile, native command, endpoint connection, accounting/resource change, source edit or Opus-opinion access. Only status CLI, file reads/hashes and these two review outputs. CLI status blocks dependent dispatch; no pending transactions.

Original parser, trace106 and interpreter hashes were checked and match retained pins. Full proposed implementation does not yet exist; semantic equivalence and native success are unproven. This vote grants no source, execution, resource or result approval.
