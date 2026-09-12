# Independent exact-source exec shim audit 04

Verdict: **blocked pending same-author repair**. Source, dispatch and actual-result approval are all false.

Candidate `7c0c4c7abe5b784f0c4c484db9979811e96bc4f767a8614e498c46241e4dcfae`. All six frozen files match root freeze04. Author timeout124 is retained; no successful author terminal result is inferred.

Fresh actual strong containment preflight passed with exec injected. All18 author tests passed. Independently hashed778 files, checked all488 compiled artifacts, compared actual255-file K package map and ran actual159-requisite set/NAR queries. No K or strace was invoked.

## E04-1: Selected Java loader still consumes uncommitted HOME and nailgun code

The full-map lib/kframework/checkJava first invokes ng against $HOME/.kserver/socket, then default ng endpoint, and otherwise constructs Java classpath with $HOME/.local/lib/kframework/java/*. child_env preserves HOME and only suppresses injection environment variables; no preflight predicate excludes these runtime selections. Both HOME paths happen to be absent now; absence is not checked by shim. No server connection was attempted by audit.

Same author must bind and fail closed on the actual loader selection, including both nailgun endpoint branches and HOME classpath. Preserve HOME. Use a bounded reviewed suppression/absence policy that actually covers the unchanged loader, or present a minimal loader-selection adjustment for review if absence cannot safely establish it; do not just add environment names the loader ignores.

## E04-2: PATH-selected host helpers and selected Java are not committed; PATH model has wrong prepend order

Actual krun wrapper prepends each directory, yielding reversed wrapperPathDirs order, then bin-unwrapped/krun prepends its own directory. verify_wrapper_and_parser instead concatenates the forward list and ignores bin-unwrapped. Python resolves to the same pinned Python3.11 today, so this is no demonstrated current Python runtime failure. Selected host dirname, basename, mktemp, date, rm, cat, cp, fold, uname, grep, head, cut and sed paths/symlink targets are absent from selectedExecutables. Java executable is absent from direct byte hashes. LLVM llvm-krun bytes are covered indirectly by the full K map symlink; kore-print and K loaders also are in full map, so do not claim their bytes wholly unverified.

Pin actual selected helpers, symlink targets and final resolved targets; add explicit selected Java/LLVM/kore identities while retaining map coverage. Compute stage-specific actual PATH prepend order and check actual selections rather than merely parsing directory text. Include loader helper paths under setenv/checkJava and shell-selected commands.

## E04-3: Ownership verification silently falls back for unobserved paths and omits trusted Nix database

verify_requisites uses verify_nonwritable when bind_path_for_ownership returns None. That branch accepts an unobserved root/overflow-owned selected file without pinned host inode/device evidence. Actual /usr/bin/env takes that branch; tests explicitly endorse it. os.access follows symlink target while owner check uses lstat(link), so target host-root ownership is not established there. The pinned query binary is likewise outside the K requisites and lacks a bound root entry; /nix/var/nix/db and db.sqlite are not referenced or checked. Actual inspected DB is currently UID0 and nonwritable, but the required preflight predicate is absent.

Extend host evidence to selected links, resolved executable targets, query-tool package and trusted database/ancestor paths. Reject missing host binding rather than falling back to UID/nonwritability only. Validate mapping, correct stat identity and effective nonwritability for those concrete paths under strong containment.

## E04-4: Host evidence target mode is compared with link mode

Host entries.mode is followed stat mode, but verify_ownership compares path.lstat().st_mode. Independent full evidence revalidation found no actual host drift using the declared stat fields;28 symlink entries would fail the shim mode comparison. Current preflight avoids these entries, so this is a latent false rejection, not a proven current execution failure.

Keep link and target stat fields separate, bind both identities where needed, and compare like fields. Exercise real selected symlink ownership in namespace tests before freezing repair.

## Remaining admission

No current runtime-blocking failure was demonstrated. The mandatory gaps above block source acceptance despite a successful preflight. Root six-case existing-runner and temporary-SQLite evidence was inspected, including30 identity mismatch rejections; this auditor did not rerun those six cases. Actual-result acceptance remains unperformed.

Final source/candidate/runner/accounting records remain prospective. Bind all consumed files, use20-second timeout/grace5/full60 debit and suppress Python startup injection before shim loading. Preserve exact one-case/no-compile/no-retry scope and perform fresh source review after repairs. Host evidence must compare target and link fields correctly and be refreshed before final admission.

Detailed hashes, check dispositions and limitations: [audit-exec-shim-04.json](audit-exec-shim-04.json). Complete file hashes: [audit-exec-shim-closure-04.json](audit-exec-shim-closure-04.json). Fresh probes: [audit-exec-shim-probes-04.json](audit-exec-shim-probes-04.json).
