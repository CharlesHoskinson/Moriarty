# One-task future-stream AGY adapter

Source proposal only. This author ran no tests, imported no candidate code and made no provider request. Root reported the unchanged v2 selector's earlier eight tests passed in0.004s; that is not a result for this new runner. The exact v2 selector SHA is ee3536dab1c2e33ec69ce734756973be0019dfa5a8622b27b58fda8371e9051a, with only literal SUCCESS positive.

The falsifiable predicate is that a schema-valid review embedded only in progress/tool arguments never becomes a terminal review. Eight focused tests also cover chunked new stdout, retention/rejection of every parsed nonobject, parse/incomplete-capture failures, duplicates/response-only lookalikes, exit/pin conditions, fixed argv, and immutable public input rechecks. Test source was frozen before future_runner.py existed.

## Explicit input contract

Supply one absolute TASK.json path and its externally frozen SHA-256. TASK must contain:

- id: string; model: exact gemini-4-argon-high; reviewToken: nonempty string.
- packet: absolute public packet directory containing MANIFEST.json; manifestSha256: exact hash of that manifest.
- promptFile and promptSha256: absolute public prompt path and exact byte hash.
- schemaFile and schemaSha256: absolute externally reviewed schema path and exact byte hash.
- wallSeconds: explicit positive integer outer bound, chosen under root's existing authority. The adapter does not approve or raise any resource limit.

MANIFEST.json uses the existing `{ "files": [{ "path": "packet-relative-public-path", "sha256": "exact hash" }] }` shape. The caller supplies only authorized public pins; the runner does not discover extra files. It verifies TASK, manifest, all listed file bytes, prompt, schema and selector before launch, then rechecks all pins plus its own source after the child exits. The manifest does not include the new output directory. The prompt retains the earlier strict less-than125000-byte boundary.

Only decoded schema object shape and byte identity are checked at load; final selection validates the exact externally supplied schema using v2. The provider's json_schema is not substituted. The caller remains responsible for the reviewed TASK/schema/prompt, root-approved bound and source hashes. Setup/input errors propagate without provider export; no retry or replay fallback exists.

## Source inspection and future execution

Root can run only the format tests from this directory:

```sh
python3 -B -m unittest -v test_future_runner.py
```

After separate root inspection/authorization, one future task is invoked with:

```sh
python3 -B future_runner.py --task /absolute/new/TASK.json --task-sha256 ROOT_FROZEN_TASK_SHA256 --output-dir /absolute/new/output-directory
```

The output directory must not exist and its parent must exist. An existing directory is refused rather than reused. There is no batch queue, scheduler, automatic retry, resume, export-only or old-capture-read mode.

The exact child argv starts `/usr/bin/timeout --signal=TERM --kill-after=5s <wallSeconds>s /home/charl/.local/bin/agy`, then fixed `--model gemini-4-argon-high --effort high --mode plan --sandbox --disable-slash-commands --add-dir <packet> --output-format stream-json --json-schema <schemaFile> --print-timeout 0 --log-file <new output>/agy.private.log --print <complete prompt>`. Cwd is packet, stdin DEVNULL, environment inherited without exporting its values. The internal print timeout is0; the explicit outer bound remains in force.

## New-stream handling and public outputs

The runner creates only new files in its new output directory. Raw stdout bytes are written to provider.private.jsonl and simultaneously parsed from the live pipe in memory. stderr.private receives only the new child's stderr; AGY is directed to a new agy.private.log. None of these private files is reopened/read by this runner. Capture files use exclusive create and0600; the new directory uses0700. These are literal implementation facts, not a security certification of AGY or its tools.

LiveCapture retains **every** parsed top-level JSON value. It does not discard arrays, null, strings or numbers before selector validation. Parse errors are counted. Only EOF marks capture complete; incomplete capture cannot export. No response/progress/text/tool-argument content is searched for reviews.

After terminal process completion and pin recheck, exact v2 selection requires exit0, immutable pins, zero parse errors and a single final top-level event=result with child result.status exactly SUCCESS and result.structured_output a schema-valid/token-matching dictionary. It exports the full selected object to TERMINAL-REVIEW.json only on EXPORT. The receipt is written after the review. Otherwise no review file is created. CLI init.model is only a label and actualBackendIdentityAttested remains false. A selected review is not a substantive correctness/merge/security decision.

Public outputs are START.json (pins/argv excluding prompt text), CHILD.json (new owned group/pid), CLEANUP.json and TERMINAL-EXPORT-RECEIPT.json (decision/condition metadata), plus the selected whole review only on EXPORT. START retains prompt hash instead of prompt text. No raw progress, response, tool arguments or stderr content is exported in public receipts. Only the selector's allowlisted receipt fields and explicit task/process/custody metadata are exported.

The process starts a new owned process group; the loop uses the outer wall+5s deadline and signals only that group during cleanup. CLEANUP records whether that group remains and whether capture reached EOF. The runner does not inspect global process tables or claim that every detached/nested OS process is gone. Process setup/read/write/wait exceptions can leave new diagnostic captures without a terminal export; they do not trigger a retry or alternate extraction. Root must assess actual cleanup/receipt conditions independently.

No original helper, prior probe/selector files, old private capture, shared checkout/Git/cache/config/auth files were changed/read by this authoring task. Develop skill, K AGENTS/product contract and read-only status were restored; historical SP01 blockers remain operational history. All old failures, caps, allocations and U0–U7/M/MC/SP/G requirements remain unchanged. Provider/tool side effects, resource review, actual identity and source-review truth remain outside this source proposal's claims.
