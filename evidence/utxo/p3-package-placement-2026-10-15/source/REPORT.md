# P3S1 Aiken package placement source report

Status: **SOURCE WRITTEN; PLACEMENT UNRUN; INDEPENDENT REVIEW PENDING**.

## Scope and identity

This report covers only the isolated task in `TASK.md`: an ordinary fixed-input filesystem placement helper. The source author requested a new reviewed placement tool before A-1 in section 9 of the frozen P3S1 `REPORT.md`. No Aiken or cryptographic source was interpreted or changed.

Requested author identity: `gpt-6.1-sol`, high effort. Actual observed author interface: this delegated Codex implementation session. The parent assigned the requested identity; this worker has no independently authenticated provider/model/effort receipt and does not assert one. Requested independent nonsecurity reviewer: fresh actual `muse-wonder-1.0`, high effort, to be obtained by the root integrator. No reviewer was invoked by this worker. No self-approval is given. Opus retains all security and cryptographic analysis, tests, judgments and later execution reviews.

I read the installed `moriarty-dev:develop` skill and execution-focus reference, checkout `AGENTS.md` and product contract, the task brief, input pins, frozen source manifest and the author's section 9 request. Root reported that guarded status was refreshed on dispatch; I did not rerun that CLI or any other repository helper. The subagent-driven-development skill was read for this assigned implementation role; the explicit no-nested-agents and root-owned review constraints governed. No new worktree, branch, scheduler or campaign was created.

## Implementation

- `candidate/place_p3_aiken_package.py`: Python 3.9+ standard-library source with `--source-root`, `--source-manifest` and `--output`. Per the root's relocation clarification, input locations are caller supplied. The helper pins manifest SHA256 `5c5dc86d04e0ed0ba93e35d5fa45fd0a256f8f2492f5667f7d3883e8a7055d7f`, selects its exact 15 `aiken/` members and checks each length and digest before output creation. It retains the checked bytes, strips `aiken/`, writes the files, reads them back, and writes `PLACEMENT-RECEIPT.json` last with manifest identity and copied-file identities. Success stdout is one compact JSON summary.
- `candidate/README.md`: exact intended CLI, ordinary effects, failure behavior and future-consumer limitations. Its command is marked UNRUN and contains no native-tool execution sequence.
- `EXPECTATIONS.json`: frozen before helper execution, independently transcribed from supplied pins and the frozen manifest. It lists all 15 expected relative destination paths and identities, the absent future destination and unchanged-byte predicate. It was amended before any helper execution to record the root's input-relocation clarification. No helper execution followed.
- `PACKAGE-INPUT-PINS.json`: supplied input pins, unchanged.
- `SOURCE-MANIFEST.json`: exact manifest of the two candidate files, supplied input pins, expectations and this report, with byte lengths and SHA256. It excludes `TASK.md`, provider outputs and itself. `SOURCE-MANIFEST.sha256` identifies that manifest.

The 15 package files total **898891 bytes** by addition of the frozen listed lengths. The receipt is an additional output file, outside that package count and byte total. The helper's fixed manifest digest binds exact membership and content identities; it does not accept a different manifest selected by the caller. Source hashes are mechanical file identities only.

## Source-inspected ordinary effects

This inventory describes the source; it is not an observed placement result.

1. Read CLI arguments and import only `argparse`, `hashlib`, `json`, `pathlib` and `sys` from the Python standard library. No supplied package code is imported.
2. Resolve the supplied source root and manifest path, requiring their existence; inspect whether the root is a directory. Resolve the output parent and inspect whether it is a directory. Resolving paths can inspect ancestor path and symlink metadata.
3. Inspect the requested output with `lstat`; refuse any existing path, including an existing directory or file. The output parent must already exist. No parent of the output root is created.
4. Read the manifest bytes once; compare their SHA256 to the fixed pin before JSON parsing or package selection. Select only the manifest's 15 paths beginning with `aiken/`. Read exactly those selected source files and compare every length and SHA256 to the pinned entries. No recursive source traversal or full TASK-tree scan occurs.
5. Retain all checked package bytes in memory before the first write. Only after every input identity matches, create the fresh output directory with `mkdir` and no `exist_ok`; a path created since the freshness check also causes refusal.
6. Create the needed directories under that new output, write exactly the selected package bytes to their relative destinations with exclusive file creation, then read back each new file and check its length and digest. Record those copied identities.
7. Exclusively create `PLACEMENT-RECEIPT.json` under the output root. It records the resolved input/output locations, source manifest byte length and digest, prefix stripped, package file count and byte total, and each relative destination's actual copied length and digest. No clock or environment variable is read to construct the receipt.
8. Print one compact summary to stdout on success. On handled path, I/O, value or manifest-structure errors, print an error to stderr and return nonzero. Argument errors use `argparse`'s ordinary error/help output.

The helper source contains no subprocess, network, dependency/cache/toolchain installation, package-code import, environment-variable read, native tooling, compilation or key/PK/SRS/wallet access. It performs no write, chmod or timestamp-setting operation at the sources; ordinary reads can update access metadata according to the filesystem. It does not preserve source filesystem metadata in the copies.

Output overlap: the helper resolves the source root and output parent, then refuses a destination equal to or inside the resolved source root. Thus output writes cannot intentionally be placed in the supplied frozen candidate tree. An existing manifest path also cannot be the fresh output; an existing ancestor of an input cannot be a fresh output. The future destination in `EXPECTATIONS.json` is in this worker's own task directory, outside the original frozen source directory. Caller-supplied locations and their ancestry are assumed stable during this trusted-input operation. No source locking, concurrent-mutation guarantee or general extraction guarantee is claimed.

An I/O failure after output creation can leave a partial output without a completed receipt. The helper does not delete that directory or alter frozen inputs to recover. It refuses reuse of the partial output. These effects and limits are documented for the future operator; they are not hostile-test or security judgments.

## Evidence and unexecuted verification plan

Observed: the frozen manifest was read as data. A separate ordinary `sha256sum` on that exact manifest returned the pinned digest above. I inspected the final helper source and README; file generation, exact-file metadata hashing and an expectations-to-pins data comparison were used to prepare this delivery. Neither the helper nor package code was imported, compiled or executed. No placement directory or placement receipt was produced. No unit suite was added, as the task explicitly excludes a redundant suite for this reversible helper. Even `--help`, syntax compilation and an ordinary placement invocation remain unrun.

Falsifiable source predicate: for the exact frozen 15-member package and a fresh destination, the helper source implements byte-preserving placement with the declared file effects and no executable-tool invocation. The source and frozen expectations are supplied for independent review; no runtime pass is claimed.

Future ordinary verification, **UNRUN**:

1. Root obtains the required fresh full-candidate Meta Muse source audit. The reviewer receives this report, `TASK.md`, supplied pins, expectations, both candidate files and output manifest. Changed bytes require a fresh applicable audit.
2. A later reviewed operator verifies the actual helper and input identities and confirms the chosen destination is absent with an existing parent outside the input tree. Use the exact intended ordinary CLI in `candidate/README.md` or its input-relocated equivalent. This report grants no execution authority.
3. Compare actual destination membership with the 15 exact entries in `EXPECTATIONS.json`, plus only the added receipt; compare every placed length and SHA256 independently against the frozen manifest and expectations. Check that input file identities are unchanged and the receipt's copied-file identities match actual output bytes. Record the actual command, exit status and output identities separately from this source report.
4. A later Aiken consumer may use the prepared layout only under its separately reviewed execution request and Opus-owned boundary. No Aiken, Rust, UPLC, SDK, proof, setup, ledger or network result follows from placement.

## Unresolved concerns and acceptance

Fresh non-author Meta Muse review, actual ordinary placement and all native execution remain pending. Python syntax and runtime behavior have not been executed in this task. Actual model/effort identity has no worker-visible provider receipt. Partial-output behavior and stable trusted filesystem assumptions remain explicit operational limits.

Implementation is a source candidate. Evidence is source inspection and mechanical manifest identity only. Acceptance is pending; this report approves neither source nor behavior. Root is sole integrator. No other worker path, frozen P3 source, shared integration tree, wrapper, locator or provider body was changed. No private material was accessed and no nested agent or external model/network call was made.

Moriarty remains permissionless for public developers. This helper changes no P3 mechanism, financial duty, all20 family, recursion/private/multi-parent obligation, security/result review, native authority, resource counter, design/resource vote or complete U0–U7, M1–M17, MC/SP/G exit. All such obligations retain their existing open status. This report and frozen output manifest are the handoff; no repository checkpoint helper was run because this task owns only its isolated folder and forbids helper execution.
