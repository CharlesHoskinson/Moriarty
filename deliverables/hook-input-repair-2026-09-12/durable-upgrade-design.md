# Durable hook upgrade design

The September 12 reinstall removed a cache directory while the calling session
still referenced its hook script. PreToolUse then prevented the restoration
command, and Stop repeated the missing-file error. The prior instructions put
restoration in the next tool call. That boundary made recovery unreachable.

## Proposed repair

1. Keep a small Python bootstrap in the registered command itself (`python3 -c`).
   It therefore starts without reading a file from the plugin cache. Its source
   is tracked in the plugin; hook JSON is generated from that source.
2. Pin each generated registration to a SHA-256 digest of the complete Python
   runtime inventory. The bootstrap verifies that inventory before execution.
   Resolve the selected native plugin root first, or the legacy root only when
   the native variable is empty. If that runtime is absent or mismatched, use
   only the same digest under `~/.local/share/moriarty-dev/runtimes/DIGEST`.
   Never follow a mutable latest-version pointer.
3. Stage runtime copies through a temporary sibling and atomic rename. Existing
   digests must match; never overwrite retained runtime content. Runtime storage
   contains code only. Repository state and admission remain in existing stores.
4. When neither exact runtime can execute, emit a bounded common diagnostic with
   no explicit allow, deny, or Stop continuation. This preserves the existing
   unavailable-input contract and repair access. The guarded CLI remains the
   mandatory execution gate; host interception remains a scoped observation.
5. Provide a local upgrade command. Validate source, hook generation and runtime
   digest before invoking `codex plugin add`. Snapshot all retained cache roots
   outside the cache. Run the installer and restore any removed roots in one
   subprocess `try/finally`, before returning to the host's PostToolUse hook.
   Never overwrite a surviving but modified retained root; report conflict.
6. Verify the installed candidate bytes and report the installer exit status.
   Keep backups and runtimes. No garbage collection, trust edits, permission
   changes, marketplace rewrites, or automatic retries. A killed legacy upgrade
   can still require host recovery; the stable bootstrap prevents that dependency
   for sessions using the new registration.

## Required controls

## Clarifications after independent design review

The initial Opus and Astra votes requested revision. Their full outputs are
retained alongside this design. The following clarifications constrain the
implementation and its claims.

- This is a workflow guard, not an adversarial security boundary. The existing
  hook already returns a diagnostic without a decision for malformed or
  unavailable records; arbitrary shell encodings, code-mode composition and
  native delegation are already outside its recognition guarantee. A raw command
  launched outside the CLI is not independently intercepted by the CLI. This
  repair must not claim universal enforcement. Registered workflow execution
  remains required to use the independently enforcing CLI. Its admission logic
  and negative tests remain unchanged. Missing-runtime diagnostics explicitly
  report degraded interception and are not counted as a successful guard test.
- The upgrade command runs from the checked-out or marketplace source package,
  independently of the cache and retained runtime. Bootstrap generation and
  staging also work when both installation locations are absent.
- The POSIX shell registration checks `python3` availability, runs it with `-I`
  to ignore Python import environment and user site packages, and emits common
  diagnostic JSON if the interpreter is absent. A broken interpreter executable
  or missing shell remains a host repair issue. JSON round-trip and missing
  interpreter cases are tests. Runtime code can import the standard library and
  its verified package. No custom loader or hostile same-user filesystem threat
  model is introduced; verification and execution are not race-free against an
  adversarial concurrent writer. This limitation already applies to CLI code.
- Inventory consists of every `.py` file below `scripts/moriarty_dev`, with
  relative path and content hash, including additions. Reject symlinks. Ignore
  bytecode caches and metadata outside the Python runtime. Runtime copies omit
  bytecode. Runtime publication and upgrades share a POSIX lock. Existing runtime
  digests are verified and reused; publication uses same-filesystem staging.
- Restore each missing snapshot file, even when its root survives partially.
  Restore by staged copy before atomic rename; preserve any conflicting surviving
  content and report recovery failure. Compare against the same invocation's
  original snapshot, not the newly reviewed version. Report installer status,
  restoration status and installed candidate identity separately.
- Snapshot failures and stale generated registrations prevent installer launch.
  Capture expected installed bytes before invoking the installer. `try/finally`
  covers ordinary exceptions, not SIGKILL or power loss. Retained runtime pinning
  protects new registrations even when a legacy cache restoration is interrupted.
  Backups remain available for recovery from the source package.

The controls below apply to the bounded contract above. They do not establish
host trust, malicious-agent containment, or full filesystem crash durability.

- Exercise the actual registered shell command with spaces in paths.
- Delete the entire selected cache while retaining its runtime: permitted hook
  still returns valid JSON, raw registered dispatch still returns deny, Stop `{}`.
- Stage two versions, remove caches, prove each registration executes its own
  pinned bytes. Tamper a runtime and prove it is rejected rather than executed.
- Remove both copies: bounded diagnostic and no continuation; never report
  coverage as verified merely because the diagnostic succeeds.
- A fake installer deletes old caches then exits successfully or unsuccessfully.
  In both cases, assert restoration has completed before the upgrade returns.
- Prove snapshot/source failures prevent installer invocation. Preserve originals
  and detect conflicts rather than overwriting user work.
- Run package and repository tests, fresh independent exact-candidate audits,
  then a normal-host smoke against the installed candidate. Direct subprocess
  tests cannot establish actual host interception.

## Foreman footguns applied

Read all 24 entries in `/home/charl/foreman/AGENT_TRAPS.md` on September 12.
Sections 9 and 12 require restoration within a single invocation and verification
against fixed bytes. Sections 2, 3, 15 and 17 require negative controls, content
identity, and assertions rather than test names or successful exit codes.
Sections 6, 7, 14, 21 and 24 distinguish CLI terminal reasons, permissions,
offloaded reads, edit payload limits, and actual completion from log silence.
Historical CLI recipes are observations, not authority to override current
permissions or substitute models selected by the user.
