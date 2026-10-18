from pathlib import Path
import json,hashlib
R=Path(__file__).resolve().parent
parts=[];blocks=[]
def put(s):parts.append(s)
def block(title,path,lang='text',ranges=None,aliases=None):
    p=R/path;full=p.read_bytes();source=full
    if ranges:
        lines=full.splitlines(keepends=True);source=b''.join(b''.join(lines[a-1:b]) for a,b in ranges)
    assert not source or source.endswith(b'\n'),path
    sha=hashlib.sha256(source).hexdigest()
    put(f'\n### {title}\n\n`{path}`; ' + ('complete body' if not ranges else f'original lines {ranges}') + f'; SHA256 `{sha}`.\n\n```{lang}\n')
    start=len(''.join(parts).encode());put(source.decode());end=len(''.join(parts).encode());put('```\n')
    blocks.append({'path':path,'ranges':ranges,'aliases':aliases or [],'startByte':start,'endByteExclusive':end,'sha256':sha,'bytes':len(source)})
put('''# Independent review input — PR227 shared source checker

Review target: https://github.com/CharlesHoskinson/Moriarty/pull/227 (stored draft metadata; no fresh provider/GitHub request). Exact head `823ce081c00786a0c55b92af81009f9e7c5e6a16`, tree `aa593783a8717ac838ae556e24c187997ab3547e`, direct parent `e4d7a2b5709caa18a94f5256d03a0302364b8235` (E4D). Source bodies come from exact Git objects, never mutable refs. The runtime baseline is a separate `97c9b1565d71404413aa35e9f95fe7661b8a09b1`, tree `e20de64d1e0ada6872b80ddd941857145305dfe0`; its entire delta from E4D is the exact15 test-only paths recorded below. This distinction must survive review.

Packet preparation and prior source author: GPT-6.1 Sol. This packet is NOT an independent review, approval or design vote. A fresh non-author reviewer must read and assess it. Requested current ordinary route is Muse Wonder1.0 high under root steering; record requested and actual returned identity/effort/status honestly. No provider invocation occurred in this preparation lane.

Falsifiable review predicate: each reported whole-byte equality for the eight normal public CLI check cases binds to the actual stored baseline and candidate head, correct source bytes/original filename, complete stdout including final newline, actual argv, exits and stderr. Source inspection must separately assess preservation of actual Beta/Core1/Core2 dispatch, diagnostics/spans, report order, lazy human reports and Core2 qualification labels. Passing eight cases does not prove general semantic equivalence.

## Review contract

''')
put((R/'REVIEW-PROMPT.md').read_text())
put('\nSource predicates requiring individual substantive assessment:\n\n')
for k,v in json.loads((R/'REVIEW-PREDICATES.json').read_text()).items():put(f'- `{k}`: {v}\n')
put('''
## Scope, limits and provenance

The real consumer is public `mori check FILE [--json]` → existing command routing → `check::run` → shared `check_source::dispatch` → unchanged actual Beta parser/checker or Core1 parse/Core2 stage loader → the CLI consumer → existing report/exporter. No new public profile, dependency/feature, Cargo file, editor, protocol, evaluator or SDK server is added. The Rust MCP command remains TODO and is included as context. Public use remains permissionless; internal reviews do not gate individual developers.

There are exactly21 changed paths: two modified production files, one new shared production module, one public CLI regression target, eight .mori fixtures, eight stdout goldens and one fixture manifest. All21 full bodies are represented losslessly below: the eight golden-file bodies alias the independently byte-equal baseline stdout blocks, with identical hashes. All four Rust files are inline in full. Context excerpts are explicit; parser/dependency internals, global guideline conformance and full runtime/semantic audit remain unestablished. Full preserved context bodies and hashes remain in `context/` and `CONTEXT-PINS.json`; excerpt selection is explicit, not a claim to full-module audit.

Execution results below are attributed stored root observations, not processes run by this packet author. Baseline build:41.284s/exit0. Candidate selected target:22.640s/eightPASS/zero filtered. Each test calls JSON and quiet human CLI modes. Eight direct candidate JSON stdout/exits equal baseline; all direct JSON stderr empty. Quiet human checks establish stdout silence/exits/diagnostic presence only, not exact renderer bytes, verbose timing, allocation counts or memory bounds. Lazy report construction and Parsed/check ordering are source predicates, not measured allocation/timing results.

The goldens are captured original-implementation compatibility oracles. The original frozen input also contains historical/authored specified DTO expectations and an unrelated proposed MCP envelope; only CLI expectations were run here. No MCP envelope, oracle provider, financial evaluator, native proof, wallet, network or ledger action was executed by these source checks. The full original21,209-byte freeze is preserved at `receipts/INPUT-CASES-FROZEN.json`; MCP-only fields are not hidden or relabeled as execution evidence. Each actual source text and complete baseline/after CLI JSON is inline below.

The only test selection was `--test check_source_dispatch -- --test-threads=1` with `--no-default-features`; it did not run the whole package or workspace. Clippy, broad suites, allocation/verbose measurements and full supported-grammar coverage are not established. Root formatted the candidate before commitment; this head's four Rust bodies, rather than the earlier unformatted proposal, are the review target. The initial broad precheck wrongly required unrelated UMP test paths; it failed before changed-source execution. The corrected exact15 test-only delta and final receipt are retained. `CURRENT-SOURCE.json` is a pre-execution snapshot and still says UNRUN_CHANGED_SOURCE; later FINAL-RESULT and START/RESULT logs establish the scoped observations. Do not treat that historical status string as a post-test verdict.

No full MCP first delivery is claimed. The separate35-path official-SDK candidate, two substantive design votes, designated dependency/build-script/resource judgments, lockfile resolution, independent result review and runtime delivery remain open. U0–U7, M1–M17 and MC/SP/G/native/target exits are not closed. Security authoring, hostile-test design and new security conclusions are outside this ordinary lane; existing source qualification labels are attributed facts only.

Startup restored the installed Moriarty develop skill and ran the required read-only status from K; its SP01 blockers are historical operational state, not this work queue. `STARTUP.json` preserves that snapshot. No Foreman, scheduler, provider, build, Git mutation or shared write was invoked by this preparation lane. No root-owned temporary build path was accessed; cleanup is a stored root receipt.

## Exact changed paths

| Path | Kind | Bytes | Head SHA256 |
|---|---|---:|---|
''')
changes=json.loads((R/'CHANGED-PATHS.json').read_text())['paths']
for x in changes:put(f"| `{x['path']}` | {x['kind']} | {x['headBytes']} | `{x['headSha256']}` |\n")
put('\n## Complete four Rust files at the exact review head\n')
for x in changes:
 if x['path'].endswith('.rs'):block(x['path'],'head/'+x['path'],'rust')
put('\n## Complete original production checker for comparison\n')
block('E4D commands/check.rs before refactor','base/crates/mori/src/commands/check.rs','rust')
put('\n## All eight source inputs and exact fixture bindings\n')
block('Committed fixture bindings','head/crates/fixtures/check-source/cases.json','json')
cases=json.loads((R/'head/crates/fixtures/check-source/cases.json').read_text())
for c in cases:block(c['id']+' complete source','head/crates/fixtures/check-source/'+c['source'],'text')
put('\n## Full baseline and after stdout, with actual invocation identities\n\nEvery JSON block below is verbatim stored stdout and includes its final newline. Both phase stderr files are zero bytes. Each baseline block also represents the byte-identical committed expected/*.stdout body named beside it. `BYTE-CLAIMS.json` and every raw START/RESULT/STDOUT/STDERR copy retain complete command/environment provenance.\n')
claims=json.loads((R/'BYTE-CLAIMS.json').read_text())['cases']
for c in claims:
 put(f"\n### Case {c['case']} invocation binding\n\nSource SHA256 `{c['sourceSha256']}`; original filename `{c['fileLabel']}`; baseline and after complete stdout bytes/exit equal; stored golden also equals both.\n")
 for phase in ['baseline-cases','changed-cases']:
  p=c['pair'][phase]
  put(f"\nPhase `{phase}`; head `{p['head']}`; tree `{p['tree']}`; cwd `{p['cwd']}`; actual argv `{json.dumps(p['argv'])}`; exit `{p['exitCode']}`; stdout bytes `{p['stdoutBytes']}`, SHA256 `{p['stdoutSha256']}`; stderr bytes `{p['stderrBytes']}`; START `{p['start']}`; RESULT `{p['result']}`.\n")
  block(c['case']+' '+phase+' complete stdout',p['stdout'],'json',aliases=[c['goldenPath']] if phase=='baseline-cases' else [])
put('\n## Exact scoped execution receipts, with no hidden test filter\n')
for p in ['BASELINE-PARENT-DELTA.json','FINAL-RESULT.json','changed-public-tests/START.json','changed-public-tests/RESULT.json','changed-public-tests/STDOUT.log','changed-public-tests/STDERR.log']:
 block(p,'receipts/'+p,'json' if p.endswith('.json') else 'text')
put('\n## Pinned unmodified context\n\nAll context below is at the exact review head and unchanged from E4D, verified against pinned Git bodies. Full file hashes identify excerpts; quoted line numbers use original source lines. This is selected dependency/interface context, not an independent audit of every implementation internals.\n')
context=[('crates/mori/src/json.rs',[(1,199)]),('crates/mori/src/commands/core_io.rs',[(1,22),(247,251)]),('crates/mori/src/commands/simulate.rs',[(150,159)]),('crates/mori/src/shell.rs',[(21,52),(68,79),(99,108)]),('crates/mori/src/cli.rs',[(1,64),(122,129)]),('crates/mori-checker/src/lib.rs',[(33,48),(61,92)]),('crates/mori-checker/src/checker.rs',[(1,44)]),('crates/mori-parser/src/lib.rs',None),('crates/mori-core/src/json.rs',[(117,140)]),('crates/mori-core/src/stage/mod.rs',[(27,94)]),('crates/mori-core/src/lib.rs',None),('crates/mori/src/commands/mcp.rs',None),('crates/mori/Cargo.toml',None),('Cargo.toml',[(1,24),(53,61)]),('RUST_GUIDELINES.md',[(77,114),(274,292)]),('docs/MORIARTY-PRODUCT-CONTRACT.md',[(1,13)]),('ROADMAP.md',[(239,246)]),('openspec/changes/consolidated-language-kernel/requirements.md',None)]
for p,ranges in context:block(p,'context/'+p,'toml' if p.endswith('.toml') else 'rust' if p.endswith('.rs') else 'text',ranges=ranges)
put('\n## Required typed response schema — unfilled, no self-approval\n\nThis schema describes the fresh reviewer response. It is not a returned review. Every exact quote must be checked against the cited pinned path/hash and original line range. No evidence field is a substitute for substantive independent reading.\n')
block('Typed independent-review response schema','REVIEW-RESPONSE.schema.json','json')
put('\n## Packet integrity and remaining empirical boundary\n\n`INLINE-BLOCKS.json` gives exact byte offsets/hashes for every inline body; `PACKET-VERIFICATION.json` records lossless recovery of all21 changed paths and all16 output bodies. `CHANGED-PATHS.json`, `CONTEXT-PINS.json`, `RECEIPT-PINS.json`, `BYTE-CLAIMS.json` and `BASELINE-DELTA-VERIFIED.json` preserve provenance. Packet generation only read exact Git blobs and stored receipts and wrote this exclusive review directory; it reran no product or compiler. Review source disposition and stored-results disposition separately. Nothing in this packet authorizes or approves integration, a new dependency/resource choice or full MCP delivery.\n')
body=''.join(parts).encode();(R/'PRIMARY.md').write_bytes(body);(R/'INLINE-BLOCKS.json').write_text(json.dumps(blocks,indent=2)+'\n')
print('PRIMARY bytes',len(body),'blocks',len(blocks))
assert len(body)<125000
