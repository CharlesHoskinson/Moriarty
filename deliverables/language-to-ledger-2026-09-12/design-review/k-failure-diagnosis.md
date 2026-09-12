# Retained trace106 diagnosis

Repository observation from bounded read-only inspection, 2026-09-12. No K/parser/interpreter execution, compile, resource change, artifact mutation, service start or network operation. Fresh main plugin status remains blocked by stale source bindings, missing current accounting and unavailable live resource state. Those admission defects do not explain the retained native crash.

## Finding

The strongest concrete hypothesis is excessive native recursive traversal depth from the transport encoding, potentially exhausting the process stack. This is not yet a proved stack-limit diagnosis. The failure moved from `kore-expand-macros` to the LLVM `interpreter` when macro expansion was disabled; an input/parser syntax error or missing ConstructNone rule is much less consistent with the retained evidence.

Selected source/artifact root: `.worktrees/sp03-expression-k-macro05/experiments/moriarty-language/formal/k/`. Its `.build-expression-v1/trace-106.input` is identical to parse04's input: 3,278,570 bytes, 5947 Option tokens. The fixture builder at `fixtures/build-expression.mjs:99` constructs a single ConstructNone with a deeply nested elementType. Its Core is exactly 65,536 bytes, within the admitted byte limit. Depth5948 Core is 65,547 bytes and should reject INPUT_BOUND. This is metadata nesting, not permission to narrow the language's expression/value depth profile.

A simple read-only string/bracket scan found maximum JSON structural nesting 35,722, balanced to zero with no unclosed string or negative nesting. This is structural evidence, not a full parser validation. Encoding explains the amplification: each Option becomes ejArray(ejCons(ejString("Option"),ejCons(T,ejNil()))), adding roughly three K term layers and six JSON object/array layers. The Python codec's `_parse`, `_canonical`, `_encode` and term serialization are iterative; `expression-parser.py` only hash-checks the pinned kast executable then execs JSON→KORE conversion. The retained failures occur after that conversion.

The immediately preceding retained trace105 has 1500 Option tokens and 837,167 input bytes; both campaigns record 105 matches ending at metadata-depth-1500. A size/depth threshold fits the observations. It does not rule out a native heap/memory defect or recursive semantic evaluation inside the interpreter.

## Exact failing stages

- parse04: trace106 return code139, not timed out, elapsed2.461 seconds. Stderr explicitly reports SIGSEGV while running `kore-expand-macros <definition> <temporary-input>`. Stdout is empty. Macro file is the retained 10-byte empty macro module.
- macro05: trace106 return code113, not timed out, elapsed3.927 seconds. The guarded argv adds `--no-expand-macros`; stderr explicitly reports SIGSEGV in `<definition>/interpreter <temporary-input> -1 <result.kore>`. A later `kore-print` reports unexpected EOF. That EOF is downstream missing/incomplete interpreter output, not evidence that original input was malformed.
- Installed `bin-unwrapped/krun` source around lines570–575 either expands macros or copies input; around line630 it invokes the interpreter; line714 prints the resulting KORE. Thus the nonzero113 wrapper code must not conceal the preceding interpreter signal.
- `expression-types.k` explicitly handles nested Option in exTypeShape/exResolve. `expression-shape.k` lists ConstructNone, `expression-infer.k` resolves its elementType and constructs its Option type, and `expression-v1.k:119` returns its empty Option value. This is not an unsupported constructor. Functions still recurse on metadata in K, so semantic recursion remains a possible second site after ingestion.

## Smallest discriminator (specified, not executed)

Use the exact macro05 input, parser, compiled artifact and recorded argv under one separately recorded bounded diagnostic amendment. Preserve the original attempt debits and all current guards. Do not compile, replay the prefix or increase metadata limits. Retain current actual process stack limit and child stage/exit diagnostics; configure no automatic retries and cap output. Avoid enabling unbounded core dumps. A debugger/backtrace is useful only if already available and admitted within the same resource envelope.

First reproduction should preserve current stack and capture the interpreter failure location. Repeated native parser/traversal frames near stack exhaustion discriminate a stack-depth cause; semantic function frames discriminate evaluation recursion; corruption elsewhere requires a different targeted repair. Do not diagnose from return code alone. Missing debug symbols may leave location unresolved; record that limit instead of asserting stack overflow.

Only if that evidence supports stack exhaustion, a separately reviewed single-case stack-size amendment with unchanged memory/CPU/deadline and identical artifact can test whether bounded additional stack resolves the accepted maximum metadata case. No unlimited stack and no action in this inspection. A successful larger-stack run must still produce the complete expected expression value/type/work result; raw exit zero alone is insufficient. If a resource repair is selected, establish a safe maximum for the existing byte-bounded domain and re-run full current conformance before claiming the implementation fixed.

If safe bounded stack cannot cover the admitted domain, the semantic-preserving code route is iterative/native traversal or a reviewed transport representation that avoids depth amplification while retaining every metadata constructor and independent K validation. That is an implementation change requiring correspondence tests, not permission to flatten away semantics or move acceptance into a host flag. Do not begin that wider change before the one-case discriminator localizes the site. Macro bypass alone has already failed and is not the next proposed fix.

## Pin verification

Hashes below were recomputed from actual retained bytes. For parser, codec, runner, interpreter, definition and macros, the selected files match entries in their own retained binding. Command/input/stderr/binding files are separately pinned, not entries expected inside binding itself. This inspection checks the listed artifacts; a live admission must validate the complete closure and toolchain.


parse04 (all paths below relative to its K root):

- `expression-parser.py`: `8420d151ea9e510b96fc941e73f496b27d53e1e1913e3d270b8ce4f4c0701f1f`
- `expression_codec.py`: `9d8f32cfa778e246900873aa5c015b45ab1ae8bec067fb26709f4a2e2996590d`
- `run.py`: `5bf9c2693d0cae227d8017e9af53cabf579e6820dd3b9443e1c770951a9dc3f6`
- `.build-expression-v1/binding.json`: `91666b114cae7aa166cf0fb1892c68e8947df8eba20456421c56439e5d71a0d3`
- `.build-expression-v1/trace-106.input`: `6c29dc49a828956f5056605b672b519a515129383c7dd46b0dc34bba174c3c1b`
- `.build-expression-v1/trace-106.command.json`: `00ee78123fa099770bd10dcad10d72c94c82e299b24e32635c7d2655020cfe19`
- `.build-expression-v1/trace-106.stderr`: `24cfc53efd5879e25b251461f0be58ab6c799c338db215147e8c1741eb520289`
- `.build-expression-v1/expression-v1-kompiled/interpreter`: `eb28d6355454d86e57604222a921825b19d281ac49862cc08cd669d6672fceff`
- `.build-expression-v1/expression-v1-kompiled/definition.kore`: `88ccf04692630c3c2126f0654a3af15c61f0a68557dffda1bec77835ef25d537`
- `.build-expression-v1/expression-v1-kompiled/macros.kore`: `3585c084b81281ec26971b2e42c56b9968a802c5d645a81062296350493491cf`

macro05 (all paths below relative to its K root):

- `expression-parser.py`: `8420d151ea9e510b96fc941e73f496b27d53e1e1913e3d270b8ce4f4c0701f1f`
- `expression_codec.py`: `9d8f32cfa778e246900873aa5c015b45ab1ae8bec067fb26709f4a2e2996590d`
- `run.py`: `9adc7dcf0a66f9fdf9d754906f65df7ca1e64e0f10d0de360384c28c35ed4a79`
- `.build-expression-v1/binding.json`: `4c663a141042a0fd4ea2be1b00e95c0ea4c5b9e744cd940e0d5352feaafbbb27`
- `.build-expression-v1/trace-106.input`: `6c29dc49a828956f5056605b672b519a515129383c7dd46b0dc34bba174c3c1b`
- `.build-expression-v1/trace-106.command.json`: `22662c12bc58ac6c5bad58bc3e2f2fed50c73e73ccca0611f4f43826e6dd7f32`
- `.build-expression-v1/trace-106.stderr`: `5a5ef35093ea3f40495a70a519233de90f3108670e226132d8150ed7c277ff21`
- `.build-expression-v1/expression-v1-kompiled/interpreter`: `32869ebd481d5c6bd0d5b0d7e4a8697b4ce514677b3b0695805211a0fc4cb212`
- `.build-expression-v1/expression-v1-kompiled/definition.kore`: `43a905cf5cef480f0ad606a2207baddadca93c08f263a4d82ddb2a7227c4210d`
- `.build-expression-v1/expression-v1-kompiled/macros.kore`: `3585c084b81281ec26971b2e42c56b9968a802c5d645a81062296350493491cf`

Toolchain identity: `/nix/store/y63xkr8pk2bqd5lh4889rlwldw26v9f4-k-7.1.337-4a46d1231473b599c699160132fd6e76a5c46406`. The expression lock pins kast SHA256 `29e8f002b7454df4d5833ab75b4779c45af203ef3ca0a47a0a75c3f270023988`; its base toolchain-lock hash is `70e09f37636f6dfefdf39310c7793881f7ad2f64a3ce4d0ed7d67b5addb38522`. Use the exact retained trace106 command JSON as argv source after admission, not a reconstructed permissive runner. Both bindings retain krunInvocations106; this does not by itself authorize another invocation.
