# Independent Astra revision 2 design vote

**ACCEPT, source design only, with all mandatory conditions.** Proposal SHA256 `6b301ee1b441e9efb5112321d645d210fb0a1c1bc817fa527a7ea77bf0ef75b0`.

Explicit kore output plus protected exact syntaxDefinition.kore removes the output Java fallback for the supported retained LLVM path. Input Java fallback is replaced by pinned package-only Main -kast. These changes preserve the target interpreter/input and allow raw process/signal discrimination; final result must establish that discrimination rather than assume it. The native LD_LIBRARY_PATH difference and parser-child stack scope are now explicit.

Independent non-author reviewer. This is a new substantive review of revision 2 in the same independent context that reviewed revision 1, not a newly spawned context. Parent requested Astra medium; separate provider-returned model/effort attestation is not exposed. No Opus opinion read.

Revision1 remains REVISE. Its recommendation against an unreviewed output-mode change was not user authority; revision2 explicitly submits that change and is assessed on its merits.

Source observations:

- kore-print missing-syntax fallback precedes output-mode case; exact durable syntax presence is essential.
- With syntax present and mode kore, kore-print selects catNewline; catNewline calls cat then echo. Newline addition means stdout is not byte-identical to interpreter result file.
- Fixed backend.txt reads llvm; krun retains interpreter result, conditionally postprocesses result.kore, then exits result if postprocessing completes. set -e can make a postprocessing error replace outer status.
- The parser-child soft-stack raise cannot alter krun parent/interpreter limits when implemented only in that child. Native stack causal diagnosis remains an empirical question.

The original interpreter, trace106 and parser hashes still match. Exact parser Java argv is preserved in the JSON receipt. Full K conformance, semantic equivalence, successful diagnosis and source/admission approval are not established.

Mandatory conditions:

**A01.** Enforce the newly proposed diagnostic-only --output kore and new parser value as the only two changes to retained krun argv; preserve original recordedArgv. Bind exact supported argument shape and backend.txt=llvm. Require exact regular/pinned syntaxDefinition.kore and ensure its presence/identity remains protected through postprocessing under the adopted execution trust boundary. Missing, replaced or drifted syntax must fail before dispatch, since kore-print has an unconditional earlier Java fallback on absence. Do not depend on predicted crash or snapshot absence alone. Cover result-file absent, complete and partial/nonzero interpreter outcomes.

**A02.** Pin exact Java executable and complete package jar closure. Parser argv must be an array with no shell/eval: fixed JVM flags followed by org.kframework.main.Main -kast --input json --output kore --definition EXACT_RETAINED_DEFINITION EXACT_RAW_INPUT. Keep package Java wildcard as one argv element or justify any enumerated classpath ordering change. No HOME, relative, empty or unbound classpath entries; no ng process or server selection. Bind jar symlink targets as well as package map/requisites.

**A03.** Preserve fixed applicable fallback JVM options in source order: -Dfile.encoding=UTF-8 -Djava.awt.headless=true -Xms64m -Xmx8192m -Xss32m -XX:+TieredCompilation -ea -cp PACKAGE_JAVA_WILDCARD. TieredCompilation applies to pinned 64-bit Java; bind architecture/version evidence offline. Do not invoke Java --version as an unbudgeted probe. K_OPTS is absent in clean original diagnostic environment; accept no ambient K_OPTS or JAVA/JAVA_HOME override.

**A04.** Freeze explicit package-only LD_LIBRARY_PATH=<K>/lib/kframework/native/linux64 without the old empty CWD component. Bind absence of that currently absent directory under protected package ancestors and existing native dependencies through the retained requisite/NAR boundary. Retain applicable native PATH prefix in the parser-child environment (or document and audit a justified selection difference), LC_NUMERIC=C, K_COLOR_SUPPORT=1, original HOME, and absent TERM. Do not claim identical native environment or invent libraries. Full exact-source review must verify constructed environment and actual helper paths.

**A05.** In the parser child only, reproduce k shell soft-stack raise to inherited hard limit before Java exec, keeping hard limit unchanged. Record exact inherited hard limit and handle RLIM_INFINITY explicitly; reject incompatible drift/failed limit change before exec. Do not impose Java 8 MiB soft stack accidentally. JVM -Xss32m is distinct from OS RLIMIT_STACK. Outer krun and retained interpreter retain 8388608-byte soft stack and core0; parser subprocess stack mutation must not affect parent/interpreter. Record process lineage change from bypassing shell wrappers; never claim identical provenance.

**A06.** Construct an allowlisted environment before Python startup and Java invocation. Suppress PYTHONPATH/PYTHONHOME/user-site/startup selection, BASH_ENV/ENV and exported shell functions, JAVA_TOOL_OPTIONS/_JAVA_OPTIONS/JDK_JAVA_OPTIONS, CLASSPATH/K_OPTS/JAVA overrides and dynamic-loader injection. Keep HOME identity/value unchanged. Present HOME extension, local socket configuration and default nailgun configuration must not alter executable selections. No endpoint connection is an offline test.

**A07.** Keep original expression-parser.py, lock/binding bytes, full original compiled definition/interpreter and trace106 immutable. Bind new diagnostic-only parser separately. Preserve original recordedArgv and record amended argv; constrain every difference to expressly adopted loader changes. Input/definition/executable/closure drift rejects before exec. No host semantic parsing, evaluator, admission narrowing, type erasure, replacement JSON-to-KORE implementation, compile or copied/edited Nix distribution.

**A08.** Preserve accepted metadata depth5947/Core65536 and rejected depth5948/Core65547. Preserve --no-expand-macros and exact interpreter/trace bytes. Loader completion or parser failure is not interpreter failure evidence; actual review must distinguish parser, interpreter and output-conversion process outcomes from raw records.

**A09.** Repair E04-2..4 and retain D03-I1..I5. Verify actual reverse wrapper prepend/de-duplication and krun/setenv stages; pin every selected helper, env/shebang interpreter, Java/LLVM/kore and resolved target. Require host binding for links and targets, query package and trusted Nix database/ancestors; compare stat to stat and lstat to lstat. No unbound UID/overflow fallback. Revalidate mapping and effective nonwritability under actual strong containment with injected exec.

**A10.** Run offline actual callable construction tests with Java exec and limit mutation injected. Check exact argv/env, input/definition/executable/jar/helper drift, earlier PATH competitor, ownership and symlink failures, HOME extension and both server configs, Java/Python/dynamic-loader injection, finite/infinite hard stack and failed stack mutation. Exercise actual supplied output source with present and absent result files and absent/drifted syntax rejection; establish no reachable checkJava selection in supported admitted paths. catNewline executes pinned cat plus Bash builtin echo, adding a newline; bind script and shell, suppress exported-function/startup overrides, and do not assert byte-identical raw KORE stdout. Required strong containment controls remain separate from source reasoning.

**A11.** Freeze complete augmented candidate and obtain both fresh independent exact-source audits; preserve original audits. Update canonical runner/pointer/manifest/argv/candidate/debit identities without resets. Retain exact stored-receipt stream recovery, UTF8 hash/byte checks, rejection of overflow/ambiguity/124/missing or mismatched receipt, existing harmless runner controls and separate actual-result review.

**A12.** No resource or dispatch authority follows this vote. C7/D1/G60 stays prospective: one diagnostic, timeout20 including preflight, grace5, full60 charge, compile0/retry0/no refund/no slack. Separate native parser-equivalence testing would need separately reviewed bounded resources; do not spend this diagnostic on a canary. No new runner, collector or campaign.

**A13.** Preserve raw strace/exit evidence and existing runner recovery. krun saves interpreter result before output and exits with that result when postprocessing succeeds; a postprocessing failure under set -e may replace outer status, so neither outer exit nor KORE text alone classifies interpreter outcome. Parser failure, interpreter SIGSEGV, wrapper113 and output failure require distinct actual-result assessment. A changed or inconclusive crash is not equivalence proof or permission to retry. This design does not require a separate native parser canary; any subsequently required native equivalence test needs separate bounded resource review.

Only status CLI, local source reads/hashes, and these two review outputs. No K/Java/ng/strace/compile/native command, endpoint connection, source mutation, resource/accounting change, or Opus-opinion access. CLI status remains blocked for dependent dispatch and has no pending transactions.
