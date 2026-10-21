# Core JSON budget contract

Status: proposed contract documentation for `mori_core::json`. It describes
the supplied production source; it is not adoption, a backend contract, or
a guarantee for other serde feature combinations.

## Entry points

| Entry point | Byte bound | Node bound | Depth bound | Error type |
|---|---|---|---|---|
| `parse` | 1,048,576 bytes, message `JSON exceeds 1 MiB` | 65,536 | 32 | `mori_core::Error` |
| `parse_limited` | caller ceiling and caller message | 65,536 | 32 | `mori_core::Error` |
| `deserialize_strict` | none of its own | 65,536 | 32 | caller's serde error |
| `parse_budgeted` | caller `max_bytes` | caller `max_nodes` | caller `max_depth` | `BudgetedParseError` |
| `export` | legacy `parse` profile, including escaped output | 65,536 | 32 | `mori_core::Error` |

`parse_budgeted` does not inherit the legacy 32 / 65,536 / 1 MiB bounds.
Backend artifact budgets, including any canonical or clamped backend
depth, are separate contracts and are not defined here.

## Counting for `parse_budgeted`

- Bytes are `text.len()`, the UTF-8 byte length. Refusal occurs before any
  decoding iff the length is strictly greater than `max_bytes`.
- One node is counted at entry to each value: the root, every array
  element, and every object member value. Object keys are not nodes.
- Depth is 0 at the root. Array elements and object member values are at
  their parent depth plus one.
- All three limits are inclusive: a value is refused only when a count or
  depth is strictly greater than its limit.
- At a single value entry, Nodes is checked before Depth.
- A duplicate key is compared after JSON string decoding and refused
  before that key's value is entered. Escaped and literal spellings of the
  same decoded key therefore collide at every nesting level.
- After a successful root value, trailing non-whitespace is refused.
  Only space, tab, carriage return, and line feed are JSON whitespace.
- General `serde_json::Value` roots are accepted: null, booleans, numbers,
  strings, arrays, and objects. Unknown object members are retained. No
  object-only root or typed-schema rule belongs to this Core API.

## Upper bounds, not guaranteed acceptance

Meeting `max_bytes`, `max_nodes`, and `max_depth` does not guarantee that
`parse_budgeted` returns `Ok`. It can still return
`BudgetedParseError::Json` with Core error code `mori::core::json`, offset
`None`, and step `None` for malformed JSON, duplicate decoded keys,
trailing data, invalid escapes or surrogate pairs, raw control characters
in strings, and numbers that the pinned parser converts to an infinite
`f64` (`number out of range`).

The exact final `serde_json` line/column suffix is a property of the
pinned `serde_json` 1.0.151 error display. Core preserves that message
text; callers should use the typed budget variant or the Core error code
rather than string-matching suffixes.

## Interaction with the pinned serde_json recursion guard

With the recorded build features `default` and `std`, and without
`unbounded_depth` being enabled or `disable_recursion_limit` being called,
pinned `serde_json` starts each text deserializer with a container
recursion allowance of 128. Entering an array or object consumes one unit;
the 128th nested container is refused with `recursion limit exceeded`.
Scalars do not consume this allowance.

This container count is not the same as BudgetSeed depth. For example,
`[` repeated 127 times, then `1`, then `]` repeated 127 times has BudgetSeed
max depth 127 and 127 containers, so it can succeed under adequate budgets.
The corresponding 128-container text has BudgetSeed scalar depth 128, but
`serde_json` refuses the 128th container even when `max_depth` is 128 or
larger. That refusal is `BudgetedParseError::Json`, not `Budget` with kind
`Depth`. A caller `max_depth` of 126 or less refuses such a text earlier as
typed Depth. A caller `max_depth` of 127 admits the depth-127 container
entry to `serde_json`, which still applies its own guard.

Consequently there is no honest single `max_depth` clamp that expresses
the callee guard for every shape: the deepest empty container at BudgetSeed
depth `D` is container `D + 1`, while a scalar at depth `D` follows `D`
containers. Do not silently clamp caller depth to 32 or 127, and do not
disable the callee recursion guard.

Depths requiring 128 or more nested containers through this `from_str`
path are therefore unsupported by `parse_budgeted`. Different downstream
feature unification that enables `unbounded_depth`, `arbitrary_precision`,
or `preserve_order` can change recursion, number, or map behaviour and is
outside this contract.

## Numbers and strings

Without `arbitrary_precision`, unsigned integers up to `u64::MAX` and
negative integers down to `i64::MIN` retain integer representation.
Larger integer significands follow the pinned parser's finite `f64` path;
for example, `18446744073709551616` becomes the finite value 2^64. A
fraction or exponent whose finite conversion would be infinite is refused
as `number out of range`; extreme negative exponents can underflow to
finite `0.0`. Literal `NaN` and `Infinity` are not JSON values.

Strings are decoded but not Unicode-normalised. A valid surrogate pair
decodes to its supplementary code point. Lone or unpaired surrogates,
invalid hex escapes, and raw U+0000 through U+001F control characters are
refused. Precomposed and combining-character spellings remain distinct
string values and distinct object keys unless they decode to exactly the
same string.

## Refusal order

For `parse_budgeted`, the first refusal wins in this order:

1. Bytes precheck.
2. Streaming value entries in document order: Nodes, then Depth at the
   same entry; duplicate decoded key when that key is read and before its
   value; then callee syntax, string, number, and container-recursion
   refusals as encountered.
3. Trailing-data check after a successful root value.

No statement here is a universal acceptance, memory, stack, or complexity
theorem. Resource adequacy for a concrete caller remains that caller's
budget decision.
