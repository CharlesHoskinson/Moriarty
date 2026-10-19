# Performance audit 09 — CEK engine, native runner, HTTP server

Auditor: Grok 4.7 xhigh Performance Auditor #9.
Scope is read-only. No source file was modified.

Targets:

- `server.js` (87 lines) — LRU trace cache, static-asset cache, compression
- `rust-runner/main.rs` (783 lines) and `rust-runner/machine_image.rs` (951 lines) — `rustc` flags and allocation cuts
- `src/engine/sherlockEngine.js` (2202 lines) — redundant `Map` copies, trace passes, and slot passes

Constraint: every replacement below is chosen so a successful trace stays byte-identical and the browser fallback stays field-identical. Panic/exit behavior, HTTP headers, and in-memory object identity are called out where they change. Do not "clean up" the preserved differences listed at the end while applying these patches. Those differences are observable output.

## Measured baseline (current tree, unmodified)

Native binary: `rust-runner/sherlock-runner`, ELF x86-64, rustc 1.99.1 (`58cf8f9c3`, 2026-10-13), LLD 23.1.1, **not stripped**, full DWARF (`.debug_info`, `.debug_line`, `.debug_str`). `size` reports text 661,548 bytes. `rustc --print cfg` on this toolchain defaults to `debug_assertions` and `panic="unwind"`, and `-C opt-level` defaults to **0**, `-C debuginfo` defaults to **0**. This artifact has DWARF and still links `attempt to add with overflow`, which is the debug overflow panic string. That matches **`-C opt-level=0 -C debug-assertions=yes` plus `-g`**, not a release build. `nm` shows `compile_and_trace_json` and 24 outlined `compile_and_trace_json::{closure}` iterator adapters left as separate functions (`json_escape` at `0x2b850`, `encode_snapshot` at `0x44c00`).

| Run | Wall time | stdout |
| --- | ---: | ---: |
| Factorial(4), fuel 200, stack 16, slice 4 | ~4 ms | 353,909 bytes, 145 steps, 37 slices, outcome `Done`, 13/145 steps with `aux2.arithmetic.active` |
| Factorial(8), fuel 2000, stack 32, slice 4 | ~7 ms | 836,645 bytes |
| `/bin/true` process spawn | ~1.9 ms | |
| Runner, invalid source | ~0.6 ms | one-line `ok:false` JSON, exit 0 |

Step 0 `machineImage.usage` on Factorial(4) is `{rows:27, cells:11, nameBytes:20}`. Slice 0 is steps 0–3, `prefixWork:[0,1,2,3,4]`, `terminal:false`. The last slice is step 144 only, `prefixWork:[0,0]`, `work:0`, `terminal:true`. `before` on the wire is only `{fuel, frameCount}` — there is no `before.frames` array until `compileAndRunSherlock` synthesizes one.

So the native preset path is already a few milliseconds. The waste is structural: a second full machine run, a snapshot image allocated only to read three integers, debug codegen, and the browser fallback repeating the same walks. The bytes that dominate the server are static: `vendor/three.webgpu.js` (2,284,850), `vendor/three.webgpu.nodes.js` (2,276,209), `vendor/three.core.js` (1,458,113), `public/graphify/graph.html` (6,990,174). Every GET `stat`s and streams them with no cache and no `Content-Encoding`.

Language arithmetic in the upstream machines is `checked_add` / `checked_sub` / `checked_mul` (`image_reference.rs` 489–491, `reference.rs` 334–335). There is no `debug_assert!` under `crates/mori-core/src/sherlock`. Turning debug assertions off does **not** change UInt128 overflow from error 10 into wrapping.

`DraftImagePreparation.image` is `Arc<ProgramImageDraft>` (`draft_image_bridge.rs` 15). `ParsedDraft.program` is `Arc<Program>` (`draft_source.rs` 65). `ImageMachine::new` takes that `Arc` (`image_reference.rs` 82). The `.clone()` calls at `main.rs` 226, 292, and 683 are atomic refcount bumps, not deep copies. Do not spend a patch on them.

## 1. `server.js`

### 1.1 Assemble the POST body once

Lines 26–29 append each `Buffer` chunk onto a string. Each `+=` UTF-8-decodes that chunk and copies the prefix. A code unit split across two TCP packets can also become U+FFFD before `JSON.parse`. Presets are ASCII and usually one chunk, so they do not hit the bug. The contract of `/api/trace` is the bytes the client sent.

Replace lines 26–33:

```javascript
    const chunks = [];
    let received = 0;
    req.on('data', (chunk) => {
      received += chunk.length;
      if (received > 256 * 1024) {
        req.destroy();
        return;
      }
      chunks.push(chunk);
    });
    req.on('end', () => {
      try {
        const body = Buffer.concat(chunks).toString('utf8');
        const payload = JSON.parse(body || '{}');
```

The 256 KiB cap, `JSON.parse('{}')` on an empty body, and the 400 path at lines 55–58 stay as they are. Destroying the socket still skips a normal `end` on most Node versions; that matches today's `req.destroy()`.

### 1.2 LRU for `/api/trace`

Lines 39–54 spawn `sherlock-runner` on every POST. The binary is a pure function of stdin source plus the three clamped argv strings (it prints JSON and exits 0, including `ok:false` compile errors). `app.js` 364–370 calls this on every execute. Re-selecting a preset or re-running an unchanged buffer pays a process spawn (~2 ms) plus the trace (~4–7 ms) plus a 0.3–0.8 MB JSON parse on the client.

Cache the **exact stdout string** (including the trailing newline from `println!` in `main.rs` 782). Replay it with the same 200 header as line 49.

Do not cache when `err != null`. A timeout can deliver a short `stdout` that today's code still answers as 200 (`if (err && !stdout)` at line 44). Caching that truncation would make the next identical POST return a partial body. Return it, uncached, exactly as now.

Key on the argv the child actually receives, not on a re-parsed number. `Number('foo')` is `NaN`, `String(NaN)` is `"NaN"`, and the binary's `parse().unwrap_or(...)` (`main.rs` 755–764) then substitutes the default. Two different payloads that clamp to the same argv and the same source string share one entry. That is the same stdout the child would print.

Insert above `const server` (after line 20):

```javascript
const TRACE_CACHE_ENTRIES = 32;
const TRACE_CACHE_BYTES = 64 * 1024 * 1024;
const traceCache = new Map();
let traceCacheBytes = 0;
const traceInflight = new Map();

function traceCacheGet(key) {
  const hit = traceCache.get(key);
  if (!hit) return undefined;
  traceCache.delete(key);
  traceCache.set(key, hit);
  return hit;
}

function traceCacheSet(key, stdout) {
  const size = Buffer.byteLength(stdout);
  if (size > TRACE_CACHE_BYTES) return;
  const prev = traceCache.get(key);
  if (prev) {
    traceCache.delete(key);
    traceCacheBytes -= prev.length;
  }
  traceCache.set(key, stdout);
  traceCacheBytes += size;
  while (traceCache.size > TRACE_CACHE_ENTRIES || traceCacheBytes > TRACE_CACHE_BYTES) {
    const oldest = traceCache.keys().next().value;
    if (oldest === undefined || oldest === key) break;
    traceCacheBytes -= traceCache.get(oldest).length;
    traceCache.delete(oldest);
  }
}

function runTrace(args, source) {
  const key = JSON.stringify([source, args]);
  const cached = traceCacheGet(key);
  if (cached !== undefined) return Promise.resolve({ ok: true, stdout: cached });
  const pending = traceInflight.get(key);
  if (pending) return pending;
  const job = new Promise((resolve) => {
    const child = execFile(
      RUST_BIN,
      args,
      { maxBuffer: 16 * 1024 * 1024, timeout: 4000 },
      (err, stdout) => {
        traceInflight.delete(key);
        if (!err) traceCacheSet(key, stdout);
        resolve({ ok: !err, err, stdout: stdout || '' });
      },
    );
    child.stdin?.write(source);
    child.stdin?.end();
  });
  traceInflight.set(key, job);
  return job;
}
```

Replace the `execFile` block at lines 39–54 with:

```javascript
        const args = ['--fuel', String(fuel), '--stack-limit', String(stackLimit), '--slice-width', String(sliceWidth)];
        runTrace(args, source).then(({ ok, err, stdout }) => {
          if (!ok && !stdout) {
            res.writeHead(500, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ ok: false, error: err.message }));
            return;
          }
          res.writeHead(200, { 'Content-Type': 'application/json; charset=utf-8' });
          res.end(stdout);
        });
        return;
```

`req.on('end')` becomes asynchronous. `return` after `runTrace(...).then` is required so the old fall-through cannot also serve a file. Coalescing in-flight jobs does not change bytes: both responses are the child's stdout. Evicting an LRU entry does not change a later response; it only misses.

`maxBuffer`, `timeout: 4000`, and the 500 body shape stay.

### 1.3 Static cache, validators, gzip / brotli

Lines 73–82 call `fs.stat` and `fs.createReadStream` on every request. No `ETag`, no `Cache-Control`, no `Content-Encoding`. The Three.js bundles and `graph.html` are re-read raw.

Semantics to keep:

- Path check at lines 66–71, 404 at 74–77, and MIME map at lines 11–20 stay.
- Decoded entity bytes stay identical to the file at the `mtime`+`size` `stat` saw.
- `Cache-Control: no-cache` forces revalidation. A long `max-age` would serve a stale bundle after an edit, which is not today's behavior.
- `Vary: Accept-Encoding`. A client that does not send `gzip` or `br` gets the raw file, same bytes as today's stream.
- Compress only `html/js/mjs/css/json/md/svg`, and only when the encoded buffer is smaller. Never compress `.png` (already compressed; the MIME map has no other binary types that benefit).
- Brotli quality 5, not the default 11. Quality does not change the decoded bytes; 11 would stall the event loop on a 7 MB HTML file the first time it is cached.
- Invalidate when `stats.mtimeMs` or `stats.size` changes. Residual window: a same-size rewrite that does not bump `mtimeMs`. Node's `mtimeMs` updates on normal writes. Re-`stat` on every request; do not serve memory blindly.

`304` changes the status line when `If-None-Match` hits. The entity the browser keeps is the body from the previous 200, which was the file bytes (or the same bytes after decompression). That is the caching behavior this finding is for. Requests without `If-None-Match` still get 200 and the full entity.

Add `import zlib from 'node:zlib';` at line 2.

```javascript
const COMPRESSIBLE = new Set(['.html', '.js', '.mjs', '.css', '.json', '.md', '.svg']);
const STATIC_CACHE_ENTRIES = 64;
const staticCache = new Map();

function loadStatic(safePath, stats, ext) {
  const cached = staticCache.get(safePath);
  if (cached && cached.mtimeMs === stats.mtimeMs && cached.size === stats.size) {
    staticCache.delete(safePath);
    staticCache.set(safePath, cached);
    return cached;
  }
  const raw = fs.readFileSync(safePath);
  const type = MIME_TYPES[ext] || 'application/octet-stream';
  const gzip = COMPRESSIBLE.has(ext) ? zlib.gzipSync(raw, { level: 6 }) : null;
  const br = COMPRESSIBLE.has(ext)
    ? zlib.brotliCompressSync(raw, {
        params: { [zlib.constants.BROTLI_PARAM_QUALITY]: 5 },
      })
    : null;
  const entry = { mtimeMs: stats.mtimeMs, size: stats.size, type, raw, gzip, br };
  staticCache.set(safePath, entry);
  while (staticCache.size > STATIC_CACHE_ENTRIES) {
    const oldest = staticCache.keys().next().value;
    staticCache.delete(oldest);
  }
  return entry;
}

function negotiated(entry, accept) {
  const header = String(accept || '');
  if (header.includes('br') && entry.br && entry.br.length < entry.raw.length) {
    return { body: entry.br, encoding: 'br', etag: `W/"${entry.size}-${entry.mtimeMs}-br"` };
  }
  if (header.includes('gzip') && entry.gzip && entry.gzip.length < entry.raw.length) {
    return { body: entry.gzip, encoding: 'gzip', etag: `W/"${entry.size}-${entry.mtimeMs}-gzip"` };
  }
  return { body: entry.raw, encoding: null, etag: `W/"${entry.size}-${entry.mtimeMs}"` };
}
```

Replace lines 73–82:

```javascript
  fs.stat(safePath, (err, stats) => {
    if (err || !stats.isFile()) {
      res.writeHead(404);
      res.end('Not Found');
      return;
    }
    const ext = path.extname(safePath).toLowerCase();
    let entry;
    try {
      entry = loadStatic(safePath, stats, ext);
    } catch (readErr) {
      res.writeHead(404);
      res.end('Not Found');
      return;
    }
    const chosen = negotiated(entry, req.headers['accept-encoding']);
    if (req.headers['if-none-match'] === chosen.etag) {
      res.writeHead(304, {
        ETag: chosen.etag,
        'Cache-Control': 'no-cache',
        Vary: 'Accept-Encoding',
      });
      res.end();
      return;
    }
    const headers = {
      'Content-Type': entry.type,
      'Content-Length': chosen.body.length,
      ETag: chosen.etag,
      'Cache-Control': 'no-cache',
      Vary: 'Accept-Encoding',
    };
    if (chosen.encoding) headers['Content-Encoding'] = chosen.encoding;
    res.writeHead(200, headers);
    res.end(chosen.body);
  });
```

`readFileSync` runs only on a cold or invalidated path, inside the `stat` callback. A first-hit compress of `graph.html` is slower than today's stream once; every later hit is a memory write. `Content-Length` replaces chunked transfer. The decoded body matches the file. A failed `readFileSync` (deleted between `stat` and `read`) becomes 404, which is the same status `stat` would have returned.

## 2. Rust runner

### 2.1 Release flags

Rebuild the same sources. Do not change `main.rs` logic for this finding. Keep whatever `--edition` and `--extern mori_lexer=rust-runner/libmori_lexer.rlib` the current binary was linked with (`extern crate mori_lexer` at `main.rs` 3, rlib sitting next to `main.rs`).

```text
rustc --edition 2021 \
  -C opt-level=3 \
  -C lto=thin \
  -C codegen-units=1 \
  -C debug-assertions=no \
  -C debuginfo=0 \
  -C target-cpu=native \
  -L rust-runner \
  --extern mori_lexer=rust-runner/libmori_lexer.rlib \
  -o rust-runner/sherlock-runner \
  rust-runner/main.rs
```

What stays identical: stdout JSON on success and on `ok:false` compile errors. `checked_add` / `checked_sub` / `checked_mul` still return error 10 on overflow. `debug_assertions=no` only drops debug overflow panics and `debug_assert` in libstd; this crate's Sherlock sources do not use `debug_assert!`.

What is intentionally not in that command:

- `-C panic=abort` changes a panic from unwind (exit 101, destructors run) to abort (exit 134). Success stdout is the same. Leave unwind in place so failure mode stays the same.
- `-C target-cpu=native` does not change JSON on this machine. The binary may `SIGILL` if it is copied to an older CPU. If the binary has to move, use `-C target-cpu=x86-64-v3` instead. JSON does not change either way.
- Leaving `-C debuginfo=1` (line tables only) is fine for profiles. It does not change stdout. Full `-g` is what made this 4.9 MB ELF mostly DWARF.

Gate: `diff` stdout of Factorial(4) and of an invalid program before and after the rebuild. Both should be empty diffs.

### 2.2 Stop encoding a snapshot image to read three integers

`main.rs` 478–495 calls `encode_snapshot(&record.after, limits)` and then uses only `img.usage()`. The `MachineImageDraft` (every expression row, value row, arm, function, parameter, environment, binding, frame, control, and every `name.to_owned()` at `machine_image.rs` 315–317) is dropped on the next iteration. Factorial(4) does this 145 times. Limits are the constants at `main.rs` 296–300 (`rows: 4096`, `cells: 8192`, `name_bytes: 16384`).

`decode_snapshot` (`machine_image.rs` 660) is not on this path. Do not rewrite it for the server hot path.

`encode_snapshot` is also not interning. The file header (`machine_image.rs` 5–7) says held expressions are copied structurally, and the graph test name `independent_encoder_rows_preserve_each_shared_tree_occurrence` confirms shared `Arc` nodes are re-walked, not pooled. A usage counter must re-walk the same way. Deduping by pointer would change `usage.rows` and then `mi_vals`.

The JSON fields derived from usage (`main.rs` 482–492) are:

- `mi_exprs` = `prep.image.expressions.len()` (the program image, not the encoded snapshot)
- `mi_vals` = `usage.rows - mi_exprs - frames.len()`
- `mi_envs` = `frames.len() + 1`
- `mi_binds` = `environment.len()` (active env only)
- `mi_frames` = `frames.len()`
- `mi_ctrls` = `control.is_some() as usize`
- `mi_rows/cells/names` = `usage.rows/cells/name_bytes`
- on `Err`, the fallback tuple at line 494: `(expr_len, 0, 1, 0, frames.len(), 1, 0, 0, 0)`

All three counters are monotonic. For a single monotonic counter, the total exceeds the limit if and only if some prefix does, in any order. `main.rs` treats every `Err` the same. So a counter matches the JSON if it adds the same multiset of row, cell, and name-byte increments, including increments of zero.

Row increments inside `Encoder`:

| Site | Lines | Rows |
| --- | --- | --- |
| Each expression node | 385 | 1 |
| `reserve_arms` | 353 | `arms.len()` |
| Each value node, including each list element as its own `UInt` row and each tree node | 465 | 1 |
| Each function | 509 | 1 |
| Each parameter | 516 | 1 per parameter |
| Each environment | 535 | 1, even when empty |
| Each binding | 542 | 1 |
| Each frame | 552 | 1, plus everything the frame's exprs / values / envs / arms add |
| Each control slot, including every `Push` link and the tail | 587 | 1 |

Cell increments:

| Site | Lines | Cells |
| --- | --- | --- |
| `expr_cells(len)` for every expression's children and for `expressions()` | 331 | `len` |
| `value_cells(len)` | 343 | `len` |
| Extra, once, before frames are encoded | 258 | `snapshot.frames.len()` |

Name bytes: `name.len()` at line 316 for every var, let binder, call name, pattern binder, function name, parameter name, binding name, and frame `Let` / `Args` name. `BTreeMap` order does not matter for the totals.

Outcome `Done` encodes one value (`machine_image.rs` 269). `Pending`, `Rejected`, and `Exhausted` add nothing.

Replacement at the call (`main.rs` 478–495):

```rust
let (mi_exprs, mi_vals, mi_envs, mi_binds, mi_frames, mi_ctrls, mi_rows, mi_cells, mi_names) =
    match account_usage(&record.after, limits) {
        Ok(u) => (
            prep.image.expressions.len(),
            u.rows.saturating_sub(prep.image.expressions.len() + record.after.frames.len()),
            record.after.frames.len() + 1,
            record.after.environment.len(),
            record.after.frames.len(),
            usize::from(record.after.control.is_some()),
            u.rows,
            u.cells,
            u.name_bytes,
        ),
        Err(_) => (
            prep.image.expressions.len(),
            0,
            1,
            0,
            record.after.frames.len(),
            1,
            0,
            0,
            0,
        ),
    };
```

`account_usage` should live next to `encode_snapshot` and return `EncodingResult<HostImageUsage>`. Implement it by running the same walks as `expression`, `value`, `function`, `environment`, `frame`, and `control`, calling `account` and not pushing rows. Keep a scratch `Vec` of `&Expr` / `&Value` if you want the walk to stay iterative; that vec is pointer-sized, which is the point.

Gate before deleting the `encode_snapshot` call from the hot loop: on every preset, `account_usage` and `encode_snapshot` must agree on `Ok` vs `Err` and, when `Ok`, on all three usage fields. Until that passes, do not switch the JSON writer over. A wrong counter changes `machineImage` in every step.

`ImageMachine::step` (`image_reference.rs` 139–171) still clones a full `before` and `after` `Snapshot` on every step (`snapshot()` at 114–124 clones the `BTreeMap` environment and every frame). `record.before` is required for aux/1 (rules 108/111/200/202) and for the rule-153 operand fetch (`main.rs` 497–595). Do not fork the K-A step function to skip those clones. The bench-side win is to stop walking `after` a second time into an owned image.

Decode-side clones (`machine_image.rs` 671, 678–682, 701, 753–754, 787, 830–837) are not on the trace path. `Tree` clone is an `Arc` bump (`reference_types.rs` 15–18: `Branch(Arc<Tree>, Arc<Tree>)`). `ExprRef` clone is an `Arc` bump. The expensive one, if decode is ever called, is `Environment` (`BTreeMap<String, Value>`) cloned at line 837 after it was already built at 666–673. A use-count prepass can `mem::take` an environment id that is referenced once and clone it only when the id is aliased. Encoder output never aliases environment ids (each `environment()` call at 534 pushes a fresh row), and a prepass keeps hand-built images correct too. That preserves `Snapshot` equality. It does not speed `/api/trace`.

### 2.3 One machine for steps, slices, and outcome

Today the runner executes the program twice, and sometimes walks the tail a third time:

1. `ImageMachine` up to 1024 steps (`main.rs` 302–681), building step JSON.
2. A fresh `Machine::new(prep.draft.program.clone(), ...)` (`main.rs` 683–712) re-executes from the start via `advance_m` (`reference_slice.rs` 62–104). Each `Machine::step` clones two snapshots (`reference.rs` 37–73). The JSON keeps only `records.len()`, `prefix_work`, and `boundaries[0].kind`. The snapshots are dropped.
3. `machine.run()` (`main.rs` 714, `image_reference.rs` 174–177) continues the image machine to the real outcome. If the loop already hit a terminal step, `run()` still calls `snapshot()` once more and discards it.

Slice JSON is a pure function of the charge tape and of whether the slice's end state is terminal. Both machines charge 0 for rules 80/82/84 and 1 otherwise. Factorial(4) at slice width 4 is 37 slices: 36×4 steps plus a one-step terminal slice whose `prefixWork` is `[0,0]`.

Caps that must stay independent:

- Step JSON stops at 1024 (`max_host_steps`).
- Slices stop at 512 groups (`main.rs` 688), or earlier on the terminal record.
- Outcome is the publication after the **full** run, even when steps or slices were truncated. Fuel from the server can be 2000 (`server.js` 35). Width 1 means 512 slice-groups cover only the first 512 steps, while outcome can be later. Do not mark the last emitted slice `terminal` unless the charge that was pushed is the publication step.

Replace the slice machine (lines 683–712) and `machine.run()` (line 714) by recording charges while stepping the image machine. Sketch of the loop head (the per-step JSON body stays; this is the control structure around it):

```rust
let width = slice_width.max(1);
let slice_charge_cap = 512usize.saturating_mul(width);
let mut charges: Vec<u8> = Vec::new();
let mut slices_closed_terminal = false;
let mut outcome_json: Option<String> = None;

let max_host_steps = 1024usize;
let mut step_idx = 0usize;
loop {
    let Some(record) = machine.step() else { break; };
    let terminal = record.after.outcome != Outcome::Pending;
    if charges.len() < slice_charge_cap {
        charges.push(record.charge);
        if terminal {
            slices_closed_terminal = true;
        }
    }
    if step_idx < max_host_steps {
        // existing per-step JSON, unchanged, pushed to steps_json
        step_idx += 1;
    }
    if terminal {
        outcome_json = Some(outcome_json_from(&record.after.outcome));
        break;
    }
}
let outcome_json = outcome_json.unwrap_or_else(|| {
    // Fresh machine always publishes within fuel. This arm is the same strings as today's Pending/Exhausted fallbacks.
    "{\"kind\":\"Pending\",\"summary\":\"Pending\"}".to_string()
});
let slices_body = format_slices(&charges, slices_closed_terminal, width);
```

`outcome_json_from` is the match at lines 715–728, moved onto `&Outcome`, not onto a freshly cloned `Snapshot`. `format_slices` must emit the same object text `advance_m` produces:

```rust
fn format_slices(charges: &[u8], tape_ends_terminal: bool, width: usize) -> String {
    let width = width.max(1);
    let mut out = String::new();
    let mut cursor = 0usize;
    let mut slice_idx = 0usize;
    while slice_idx < 512 && cursor < charges.len() {
        let end = (cursor + width).min(charges.len());
        let count = end - cursor;
        let start_step = cursor;
        let end_step = start_step + count - 1;
        let mut acc: u128 = 0;
        let mut pwork = String::from("0");
        for charge in &charges[cursor..end] {
            acc += u128::from(*charge);
            pwork.push(',');
            pwork.push_str(&acc.to_string());
        }
        let is_terminal = tape_ends_terminal && end == charges.len();
        if slice_idx > 0 {
            out.push(',');
        }
        // same key order and bool spelling as main.rs 704–707
        out.push_str(&format!(
            "{{\"index\":{slice_idx},\"startStep\":{start_step},\"endStep\":{end_step},\"count\":{count},\"prefixWork\":[{pwork}],\"work\":{acc},\"terminal\":{is_terminal}}}"
        ));
        cursor = end;
        slice_idx += 1;
        if is_terminal {
            break;
        }
    }
    out
}
```

`is_terminal = end == charges.len()` alone is wrong when the tape was cut at `slice_charge_cap` while the machine kept going. `tape_ends_terminal` is set only when the pushed charge itself is the publication. A full-width last slice of a still-running program stays `terminal:false`, matching `SliceBoundaryKind::TechnicalCut` (`reference_slice.rs` 84–90).

Delete the `Machine::new` / `advance_m` block. `prep.draft.program.clone()` goes away with it; it was only an `Arc` clone, the win is not doing the second run.

Gate: stdout `diff` on Factorial(4), Factorial(8) at fuel 2000, and a fuel-1 program (publication is `Exhausted` before 1024). Compare `slices` and `outcome` specifically.

### 2.4 Allocation cuts inside the step JSON

These keep the format string at `main.rs` 642–678 byte-identical. Apply them in the existing loop; they do not depend on 2.3.

**Inactive arithmetic text is a constant.** 132 of 145 Factorial(4) steps take the `None, None, None` witness (`main.rs` 546–553) and then allocate sixteen-element `Vec<String>`s at 608–637 to print zeros. `materialize_arithmetic(None, None, None)` (`arithmetic_witness.rs` 66–82) is pure: selectors false, `ovf` false, add carries and sub borrows false, mul limbs zero. The arithmetic object is therefore this fixed fragment (17 carry slots from `add.c`, 17 borrow slots, 32 diagonals = `mul.r` then `mul.h`):

```rust
const INACTIVE_ARITH_JSON: &str = concat!(
    "\"arithmetic\":{\"active\":false,\"op\":\"none\",\"lhs\":\"0\",\"rhs\":\"0\",\"result\":\"0\",\"overflow\":false,",
    "\"lhsBytes\":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],",
    "\"rhsBytes\":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],",
    "\"resultBytes\":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],",
    "\"carries\":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],",
    "\"borrows\":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],",
    "\"diagonals\":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}"
);
```

On the inactive path, splice `INACTIVE_ARITH_JSON` in place of the arithmetic object built at 662–671. On rule 153 with two `UInt`s, keep today's witness math unchanged (`arith_op_str`, limb arrays, `ovf`). Fuel is **not** constant; `materialize_fuel` (`main.rs` 626–629) still runs every step because `beforeBytes` / `afterBytes` follow `remaining_fuel`.

**Write comma-separated bytes into the output `String`.** `lhs_b_str` and the five sibling `Vec<String>`s (631–637) exist only to be `join`ed. `write!(out, "{b}")` for a `u8` matches `u8::to_string()`. Bool columns must stay `u8::from(b)` so they print `0`/`1`, not `true`/`false`.

```rust
fn push_u8_list(out: &mut String, bytes: &[u8]) {
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 { out.push(','); }
        let _ = std::fmt::Write::write_fmt(out, format_args!("{b}"));
    }
}
```

Use the same helper for fuel `post`, `zp`, and `bw` after mapping bools with `u8::from`. The characters appended are the same as `join(",")`.

**One output buffer instead of `steps_json: Vec<String>` plus a final `join` (293, 642, 736).** Reserve once (`String::with_capacity(steps * 2048)` is enough for these presets; 145 steps were 354 KB). Push `','` between steps. The final document at 730–738 concatenates the same fields in the same order; only the intermediate `String`s disappear. `expr_rows_json` / `fn_rows_json` (227, 276) are once per request; folding them into the same buffer is the same transformation, smaller.

**`frame_origins().to_vec()` at line 309 is a needless copy.** `frame_origins()` already returns `&[ExprId]` (`image_reference.rs` 132) and `ExprId` is `Copy`. The borrow ends before the next `machine.step()`.

```rust
let frame_origins = machine.frame_origins();
let f_origin = frame_origins.get(fidx).map(|e| e.0).unwrap_or(0);
```

Drop the `Vec` and the `.to_vec()`. Indexing is unchanged, so frame `originExprId` / `astPath` stay the same.

**Do not clone `Value` to format it.** Lines 313–355 own a `Value` in `ctrl_val_opt` and clone it again into `last_focus_value`. `Value::List` clone copies the `Vec<u128>` (`reference_types.rs` 21–26). `value_json` is then called on both (639–640), and on later steps it is called again on the same focus value even when the focus did not change. When `ctrl_val_opt` is `Some`, the focus JSON equals the control JSON (the assignment at 337–338 copies that same value). Format from the borrow and keep the previous focus **string**:

```rust
let mut ctrl_val_json = String::from("null");
let mut focus_is_ctrl = false;
match &record.after.control {
    Some(Control::Return(v)) => {
        ctrl_val_json = value_json(v);
        focus_is_ctrl = true;
        // ctrl_summary unchanged: format!("Return({})", value_summary(v))
    }
    Some(Control::Cases(v, _, _)) => {
        ctrl_val_json = value_json(v);
        focus_is_ctrl = true;
    }
    // CaseResult, PrimitiveResult(Ok), Outcome::Done: same, one value_json
    Some(Control::Invoke(_, args)) => {
        if let Some(v) = args.first() {
            ctrl_val_json = value_json(v);
            focus_is_ctrl = true;
        }
    }
    Some(Control::Enter(_, args)) => {
        if let Some(v) = args.first() {
            ctrl_val_json = value_json(v);
            focus_is_ctrl = true;
        }
    }
    _ => {}
}
if focus_is_ctrl {
    focus_val_json = ctrl_val_json.clone();
} else if let Some(v) = focus_from_frames_or_env(&record) {
    focus_val_json = value_json(v);
}
// else leave focus_val_json as the previous step's string
```

`focus_from_frames_or_env` is the loop at 340–357, returning `&Value` instead of `v.clone()` / `vals.last().cloned()`. The first step with no focus still emits `null` if you initialize `focus_val_json` to `"null"`, matching `.unwrap_or_else(|| "null".to_string())` at line 640. Cloning the JSON string is a byte copy of text that was going to be copied into the output anyway. It replaces a `Vec<u128>` clone plus a second tree walk.

**Line numbers.** `line_number` (205–208) counts newlines from byte 0 on every expression during image export and again on every step and every frame (231, 362, and the frame origin lookup). Build one prefix table:

```rust
fn line_starts(source: &str) -> Vec<usize> {
    let mut starts = Vec::with_capacity(64);
    starts.push(0);
    for (i, b) in source.bytes().enumerate() {
        if b == b'\n' {
            starts.push(i + 1);
        }
    }
    starts
}

fn line_number_at(starts: &[usize], source_len: usize, byte_offset: u32) -> usize {
    let end = (byte_offset as usize).min(source_len);
    starts.partition_point(|&s| s <= end)
}
```

`partition_point(|&s| s <= end)` equals `source[..end].bytes().filter(|&b| b == b'\n').count() + 1`:

| offset in `"ab\ncd\n"` | newlines strictly before it | `partition_point` |
| --- | ---: | ---: |
| 0 | 0 | 1 |
| 2 (on `\n`) | 0 | 1 |
| 3 (start of line 2) | 1 | 2 |
| 6 (just after the last `\n`) | 2 | 3 |

Call `line_starts` once after `prepare_draft_image` succeeds. `origin_line` and the expression-row `line` field stay the same numbers.

**`rule_name` strings need no `json_escape`.** The table at 87–129 has no quote, backslash, or code point below U+0020. `json_escape(rule_name(...))` at line 646 allocates a copy and returns the same bytes. Write `rule_name(...)` straight into the buffer. Do this only for that table. `origin_text`, names, and `ctrl_summary` still go through `json_escape` (71–85); source text can contain quotes.

**Per-step `owner_str` / `path_parts` (364–369 and again per frame at 375–380).** Those strings are determined by `origin_id` and `f_origin`, which repeat constantly during a factorial descend. A `HashMap<usize, String>` from expr id to the already escaped `astPath` fragment is safe: the path does not depend on machine state. Misses compute the same `format!` as today. This is a small win next to the JSON buffer; do it after the buffer exists so the cached value is the escaped text actually inserted.

## 3. `sherlockEngine.js`

The browser machine is the fallback in `compileAndRunSherlock` (1732–1736) and the only engine when `preferRust` is false. `buildStepState` (2151) runs from `app.js` `getOrBuildStepState` (910–942), which already caches one integer step index. The redundant work is inside a single compile and inside the first visit to each step index.

Live environments are not mutated in place. The only `.set` calls on runtime maps are `next.set` in `matchPattern` (1055–1070), `nextEnv.set` at 1158, and `callEnv.set` at 1276. Every one of those writes a map that was just created. `env = ...` always replaces the binding. Frame maps and the current `env` are therefore immutable after publication. Trace code reads them via `entries()` and copies names into plain objects (1478–1483, 1507–1510). Sharing the `Map` reference does not change any serialized `bindings` array.

### 3.1 One map per push, not two

`cloneEnv` is `new Map(e)` (1046). Each of Let, Binary, If, Match, and Call (1088–1136) calls it twice with the same `env`: once for `frame.env` and once for the `Push` environment. Unary (1103–1108) clones `env` into the `Push` even though the `Unary` frame does not store a map. `Bound` clones again at 1284 for the `Return` frame.

The `Push` handler (1144) does `env = ctrl.env` and does not write into the map. The later Let bind clones before it writes:

```javascript
const nextEnv = cloneEnv(top.env);
nextEnv.set(top.name, val);
frames[0] = { kind: 'Scope', env: top.env };
```

(lines 1157–1159). That clone stays. It is the copy that makes the new binding visible only to the body. The `Scope` frame keeps the pre-bind map.

Replace the five double clones with the current map. Example for Let (1088–1093); Binary, If, Match, and Call-with-args are the same shape:

```javascript
if (e.kind === 'Let') {
  return [114, {
    kind: 'Push',
    frame: { kind: 'Let', name: e.name, body: e.body, env },
    next: { kind: 'Eval', expr: e.value },
    env,
  }];
}
```

Unary (1103–1109):

```javascript
if (e.kind === 'Unary') {
  return [120, {
    kind: 'Push',
    frame: { kind: 'Unary', op: e.op },
    next: { kind: 'Eval', expr: e.operand },
    env,
  }];
}
```

`Bound` (1282–1286): store `env`, not `cloneEnv(env)`. The caller's map is installed on the `Return` frame and the current `env` becomes `ctrl.env` (the fresh `callEnv`). Nothing writes the caller map.

`cloneEnv` remains mandatory at 1049 (after the pattern actually matches and a binder is added) and at 1157.

### 3.2 `matchPattern` clones before it knows the arm matches

Lines 1048–1073 always copy `baseEnv`, then return `null` on a miss, including the duplicate-binder miss at 1054 and 1064. The copy is discarded. A miss is the common `cons`/`nil` and `leaf`/`branch` failure. Clone only after the arm matches and a new binder is required. `Nil` adds no binder; returning `baseEnv` itself serializes the same entries (the map is immutable, §3).

```javascript
function matchPattern(pattern, value, baseEnv) {
  if (pattern.kind === 'Nil' && value.kind === 'List' && value.elements.length === 0) {
    return baseEnv;
  }
  if (pattern.kind === 'Cons' && value.kind === 'List' && value.elements.length > 0) {
    if (pattern.binders[0] === pattern.binders[1]) return null;
    const next = cloneEnv(baseEnv);
    next.set(pattern.binders[0], { kind: 'UInt', value: BigInt(value.elements[0]) });
    next.set(pattern.binders[1], { kind: 'List', elements: value.elements.slice(1) });
    return next;
  }
  if (pattern.kind === 'Leaf' && value.kind === 'Tree' && value.tree.kind === 'Leaf') {
    const next = cloneEnv(baseEnv);
    next.set(pattern.binders[0], { kind: 'UInt', value: BigInt(value.tree.value) });
    return next;
  }
  if (pattern.kind === 'Branch' && value.kind === 'Tree' && value.tree.kind === 'Branch') {
    if (pattern.binders[0] === pattern.binders[1]) return null;
    const next = cloneEnv(baseEnv);
    next.set(pattern.binders[0], { kind: 'Tree', tree: value.tree.left });
    next.set(pattern.binders[1], { kind: 'Tree', tree: value.tree.right });
    return next;
  }
  if (pattern.kind === 'Var') {
    const next = cloneEnv(baseEnv);
    next.set(pattern.binders[0], value);
    return next;
  }
  return null;
}
```

Rule lines 214/217/220 do not change. `matchedEnv` is still `null` on a miss and still a map with the same bindings on a hit.

### 3.3 Top-of-stack at the end of the array

`frames.unshift` / `frameOrigins.unshift` (1142–1143) and the matching `.shift()` calls (1169, 1201, 1213, 1219, 1227, 1239, 1246, 1254) move every frame on every push and pop. The server clamps `stackLimit` to 64 (`server.js` 36); the browser function does not (`maxStack` at 1041). Depth is small on the eight presets, so this is quadratic but bounded. It is still a pure representation change if serialization stays top-first.

Internal convention after the patch: index `length - 1` is the top. `frames.push` / `pop` replace unshift / shift. Every current `frames[0]` read or write becomes the last index:

| Current | Lines | Replacement |
| --- | --- | --- |
| `frames.unshift(ctrl.frame); frameOrigins.unshift(controlOrigin)` | 1142–1143 | `frames.push(ctrl.frame); frameOrigins.push(controlOrigin)` |
| `const top = frames[0]` | 1151 | `const top = frames[frames.length - 1]` |
| `frameOrigins[0]` | 1153–1154 | `frameOrigins[frameOrigins.length - 1]` |
| `frames[0] = { kind: 'Scope', ...}` | 1159 | `frames[frames.length - 1] = { kind: 'Scope', env: top.env }` |
| `frames[0] = { kind: 'Right', ...}` | 1164 | write the last index |
| `frames[0] = { ...top, evaluated, remaining }` | 1235 | write the last index |
| `frames.shift(); frameOrigins.shift()` | 1169, 1201, 1213, 1219, 1227, 1239, 1246, 1254 | `frames.pop(); frameOrigins.pop()` |
| `beforeTopFrame = frames[0] \|\| null` | 1334 | `frames[frames.length - 1] \|\| null` |

Serialize top-first so `index: 0` is still the top of the stack. Replace the `frames.map` at 1441 with a downward loop that assigns `fidx` from 0:

```javascript
const serializedFrames = [];
for (let i = frames.length - 1, fidx = 0; i >= 0; i--, fidx++) {
  const fr = frames[i];
  const fOrigin = frameOrigins[i] ?? 0;
  // unchanged label / detail / bindings body, with index: fidx
  serializedFrames.push({ index: fidx, /* ...same fields... */ });
}
```

`before.frames`, ring telemetry (`[...frames].reverse()` at 1845 expects index 0 = top), and `buildRecursionTelemetry`'s top-to-bottom walk (2113) then see the same arrays they see today. `frames.length` checks (1141, 1245, 1332, 2170) do not care which end the top is on.

Do not change the `Args` label formula at 1467 (`evaluated.length + 1 + remaining.length`). The native label at `main.rs` 421 uses `evaluated.len() + remaining.len()` without the extra 1. Those strings differ across engines on purpose. See the preserved-differences list.

### 3.4 Do not copy the previous frame records

Line 1336:

```javascript
const beforeFramesSnapshot = prevSerializedFrames.map((f) => ({ ...f }));
```

The spread copies the frame object and shares `bindings`. Nothing in this function writes those objects after they are pushed (1505 stores the array, 1574 stores the snapshot). The next step's `before.frames` is a shallow twin of the previous step's `after.frames`. Field values are already equal. Reuse the array:

```javascript
const beforeFramesSnapshot = prevSerializedFrames;
```

`JSON.stringify` of a trace does not change. Object identity of `steps[i].before.frames` and `steps[i - 1].after.frames` becomes the same. `buildRingTelemetry` only reads both (1846–1850). That matches the focus-value aliasing that already exists: `lastFocusValue` is stored on every later step until it changes (1424–1425, 1568).

### 3.5 Inactive arithmetic witness

`materializeArithmeticWitness(null, null, null)` at 1536 runs on every non-arithmetic step and allocates six arrays (16+16+16+17+17+32 zeros) at 260–265. The active path (1537–1548) must keep building a fresh object; rule 153 writes `result`, `carries`, and `overflow`.

Hoist one inactive object next to the function (after line 320):

```javascript
const INACTIVE_ARITHMETIC = materializeArithmeticWitness(null, null, null);
```

and at 1536:

```javascript
let arith = INACTIVE_ARITHMETIC;
```

Readers (`designer5_circuit.js` 200–223) only read `lhsBytes`, `carries`, and `active`. Sharing the inactive object is the same pattern as the shared `focusValue`. Do not `Object.freeze` it if the goal is identical mutability; a freeze would throw on a write that today succeeds. No `src/` writer touches those arrays. The active path still returns a new object from `materializeArithmeticWitness(op, lhs, rhs)`.

Fuel witnesses (1550) depend on `beforeFuel` and on whether the rule is 80 or 82. Keep allocating them.

### 3.6 One walk for `controlSummary` and `controlValue`

`serializeValue` (913–944) already stores `summary` via `formatValueSummary` for lists and trees, and `String(big)` for `UInt`. `formatValueSummary` (898–904) produces that same text. The step loop calls both (1377–1406, and again inside the frame walk at 1455). For a `Return` / `Cases` / `CaseResult` / `PrimOk`:

```javascript
case 'Return':
  ctrlValObj = serializeValue(control.value);
  ctrlSummary = `Return(${ctrlValObj.summary})`;
  break;
```

Same for `Cases`, `CaseResult`, and the `ok` branch of `PrimitiveResult`. `Done` at 1417–1419 can use `outcome.summary` (already set from `formatValueSummary` at 1342) instead of formatting the value twice. `Invoke` / `Enter` still format every argument for the summary string; only argument 0 is serialized (1386, 1390). Leave that. The summary text is the same because `serializeValue`'s `summary` field is `formatValueSummary` for `List` and `Tree` and the decimal spelling for `UInt` and `Bool`.

While in that loop, accumulate `bindCount` inside the frame serializer instead of the second walk at 1555:

```javascript
const bindCount = activeEnvBindings.length + serializedFrames.reduce((a, f) => a + f.bindings.length, 0);
```

A running integer updated as each frame's `bindings` array is built is the same sum.

### 3.7 `buildRingTelemetry` builds the frontier slot twice

`buildSlot` (1852–1905) is called 12 times for the rings (1908–1911) and again at 1925–1928 for `frontierSlotIndex`, which is `chronoFrames.length - 1`. When the stack is 12 or shorter, that index is already `slotA` or `slotB` of `rings[index >> 1]`. The second call reruns `formatFrameFullLabel`, `formatSlotShortLabel`, and `parseArgsArity`. When the stack is longer than 12, the frontier is not in the rings and the extra call is required (`overflowTopFrame` at 1938).

Replace 1924–1928:

```javascript
const frontierSlotIndex = chronoFrames.length - 1;
let topSlot = null;
if (frontierSlotIndex >= 0 && frontierSlotIndex < 12) {
  const ring = rings[frontierSlotIndex >> 1];
  topSlot = (frontierSlotIndex & 1) === 0 ? ring.slotA : ring.slotB;
} else if (frontierSlotIndex >= 12) {
  topSlot = buildSlot(frontierSlotIndex, frontierSlotIndex % 2 === 0 ? 'A' : 'B');
}
```

`trueTop` (1929–1937) spreads `topSlot` into a new object immediately, so sharing the ring's slot object as the source does not alias `trueTop` to the ring. Field values match a second `buildSlot` call because `buildSlot` is a pure function of the two frame arrays and `step.ruleLine`.

The `[...frames].reverse()` copies at 1845 and 1850 are tiny (≤ stack depth). Replacing them with an index helper `frames[frames.length - 1 - chronoIdx]` avoids the copy and is the same lookup. Optional next to the double `buildSlot`.

### 3.8 One pass over the trace for recursion telemetry

`buildRecursionTelemetry` (2037–2146) scans `steps.length` on every call (2055–2081) to rebuild `calls`, then walks `calls` again for the playhead (2085–2094). `getOrBuildStepState` caches a step index, so playing through N steps calls this N times. Cost is O(N²) visits. N is 145 on Factorial(4) and can be ~2000 at the server fuel cap. The equation (2099–2130) depends on the current step. The call spans do not.

`maxDepth`, `invokeStep`, `returnStep`, `label`, `depth`, and `returnValue` are fixed for a given `trace` object. Cache them on the trace with a `WeakMap` so a replaced trace is collected and a mutated-in-place trace is not incorrectly reused. This compiler does not mutate `steps` after publish.

```javascript
const recursionSpanCache = new WeakMap();

function recursionSpans(trace) {
  const cached = recursionSpanCache.get(trace);
  if (cached) return cached;
  const steps = trace?.steps || [];
  const calls = [];
  const openStack = [];
  let maxDepth = 0;
  for (let i = 0; i < steps.length; i++) {
    const s = steps[i];
    const fLen = s.after?.frames?.length || 0;
    if (fLen > maxDepth) maxDepth = fLen;
    if (s.ruleLine === 200) {
      const prev = steps[Math.max(0, i - 1)];
      const raw = prev?.controlSummary || s.controlSummary || 'call()';
      const label = raw.replace(/^Invoke\((.*)\)$/, '$1').replace(/^Enter\((.*)\)$/, '$1');
      const callObj = {
        id: calls.length,
        label,
        invokeStep: i,
        returnStep: null,
        returnValue: null,
        depth: openStack.length,
      };
      calls.push(callObj);
      openStack.push(callObj);
    } else if (s.ruleLine === 185 && openStack.length > 0) {
      const finished = openStack.pop();
      finished.returnStep = i;
      finished.returnValue = s.controlValue?.summary ?? s.controlSummary?.replace(/^Return\((.*)\)$/, '$1') ?? '';
    }
  }
  const spans = { calls, maxDepth: Math.max(1, maxDepth) };
  recursionSpanCache.set(trace, spans);
  return spans;
}
```

Per `stepIndex`, project status into new objects (the function returns fresh `calls` today, and it writes `status`):

```javascript
const { calls: spans, maxDepth } = recursionSpans(trace);
const calls = spans.map((c) => {
  let status = 'future';
  if (stepIndex < c.invokeStep) status = 'future';
  else if (c.returnStep !== null && stepIndex >= c.returnStep) status = 'returned';
  else status = 'waiting';
  return { ...c, status };
});
const activeOpenCalls = calls.filter((c) => c.status === 'waiting');
if (activeOpenCalls.length > 0) {
  activeOpenCalls[activeOpenCalls.length - 1].status = 'active';
}
```

The status rules are the ones at 2086–2097, including "the last waiting call becomes `active`". `mode` (2132–2137) and the equation stay in the per-step function. `maxDepth` in the return (2144) becomes the cached value, which is `Math.max(1, max over every step)` — the same number the full scan computes, including on steps that are not the deepest.

Do not cache the returned object across step indexes. `status` and `equation` and `mode` depend on `stepIndex`.

### 3.9 Native traces: stop normalizing `before.frames` twice

`compileAndRunSherlock` 1705–1718:

```javascript
s.after.frames = s.after.frames.map(normalizeTraceFrame);
// ...
s.before.frames = rawBefore.map(normalizeTraceFrame);
```

Rust `before` has no `frames` (measured: keys are `fuel` and `frameCount`). `rawBefore` is therefore `data.steps[i - 1].after.frames`, which the previous iteration already normalized. `normalizeTraceFrame` (1672–1687) returns a new object for every frame, and for non-`Args` frames the spread does not change a field. Mapping it again re-runs `parseArgsArity` and allocates a second copy of every frame on every step.

`Args` normalization is idempotent (`parseArgsArity` prefers `evaluatedCount` / `totalArgs` once they exist, 1633–1638). A second pass produces the same fields. Skip it when the array was taken from the previous `after`:

```javascript
if (Array.isArray(s.after?.frames)) {
  s.after.frames = s.after.frames.map(normalizeTraceFrame);
}
if (s.before) {
  if (Array.isArray(s.before.frames)) {
    s.before.frames = s.before.frames.map(normalizeTraceFrame);
  } else if (i > 0) {
    s.before.frames = data.steps[i - 1].after?.frames || [];
  } else {
    s.before.frames = [];
  }
}
```

Step 0 still gets `before.frames = []`. Later steps share the previous step's normalized array. Field values match the double map. Identity matches the browser change in §3.4. `normalizeTraceFrame` for a real `before.frames` array (browser traces that already have one, or a future runner that emits one) still runs once.

Non-`Args` frames do not need `{ ...fr }` inside `normalizeTraceFrame`. Returning `fr` unchanged is the same fields. Keep the `Args` branch, which adds `fnName`, `evaluatedCount`, `totalArgs`, and a rewritten `label`.

### 3.10 Small, still identical

`parsed.functions.find` at 1262 is a linear scan on every `Invoke` (rule 200). Build one map after typecheck, first name wins, which is what `find` returns:

```javascript
const fnByName = new Map();
for (const f of parsed.functions) {
  if (!fnByName.has(f.name)) fnByName.set(f.name, f);
}
```

Lookup becomes `fnByName.get(ctrl.name) || null`. Duplicate names still resolve to the first function. The presets have unique names. The aux/1 `FindScan` probe list (1522–1527) must keep iterating `parsed.functions` in source order so probe indexes stay the same. Do not build the probes from the map.

`lineNumberAt` (403–409) rescans from byte 0 for every expression row (859). A `line_starts` prefix plus `partition_point`, same as §2.4, returns the same line numbers. Parse is not the hot loop; do this only while touching image construction.

`e.args.slice(1)` (1133) and `[...top.evaluated, val]` (1232) copy argument arrays whose length is the function arity. Replacing them with an index would change the frame object shape. The serialized trace only shows the lengths (1466–1469). Leave the copies; arity is tiny and an index rewrite is how label math gets drifted.

## Preserved differences (do not "fix" these in a perf patch)

These are observable today. A patch that makes the engines agree would fail the identical-semantics bar.

| Item | Browser | Native |
| --- | --- | --- |
| Rule 111 name | `RULE_NAMES` line 206: `Var missing -> error(3)` | `rule_name` line 94: `Var missing -> error(1)` |
| `Args` total in the label | `evaluated + 1 + remaining` (1467), the hole counts | `evaluated.len() + remaining.len()` (`main.rs` 421) |
| `machineImage` counts | Heuristic at 1552–1605 (`valCount = 2 + bindings + frames`, `nameBytes = bindCount * 4 + functions * 6`) | `encode_snapshot` usage, then the formula at 482–494 |
| Slice cap | `groupSlices` (991–1017) covers every browser step, no 512 cap. Terminal iff the last step of the whole trace has `charge === 0` | 512 groups, terminal iff that slice's end outcome is published |
| Step `before` from Rust | Synthesized in `compileAndRunSherlock` | Not in the runner JSON |

Also leave `ImageMachine::step`'s insert-at-front (`image_reference.rs` 185–186). Matching it is why the JS machine uses index 0 as the top today. Section 3.3 only flips the private JS storage and flips it back at serialization.

## Suggested order and gates

1. `rustc` flags (§2.1). `diff` Factorial(4) and a syntax error. No source change.
2. Slice/outcome single pass (§2.3) and `diff` `slices` plus `outcome` on fuel 1, fuel 200, and fuel 2000.
3. Usage counter (§2.2) behind a differential check against `encode_snapshot` on all eight presets, then switch the call.
4. JSON buffer, inactive arithmetic fragment, focus string, line-start table (§2.4). `diff` the whole stdout.
5. Server LRU and static cache (§1). Replay a preset POST twice and compare bytes. `curl --compressed` a cold `vendor/three.webgpu.js` and `cmp` the decompressed body to the file. Send `If-None-Match` and expect 304. Send no `Accept-Encoding` and `cmp` to the file.
6. JS map sharing, `matchPattern`, stack end, shared `before` frames, inactive arithmetic, telemetry (§3). Run `compileAndRunInBrowser` on the eight presets and deep-equal the traces against the pre-patch engine. Then run `buildRingTelemetry` / `buildRecursionTelemetry` at step 0, mid-trace, and the last step.

Until those diffs are empty, the corresponding patch is not semantics-preserving, even if it is faster.
