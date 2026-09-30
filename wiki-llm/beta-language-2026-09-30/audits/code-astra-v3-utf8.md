# Additive v3 audit: CLI malformed UTF-8

Date: 2026-09-30. Requested reviewer: fresh GPT-6 Astra medium; this is the same independent `/root/code_astra_v3` review context. Identity-attestation limitations remain as recorded in `code-astra-v3.md`.

**Finding UTF8-01 — Low: the CLI replaces malformed file bytes before checking or hashing.** The full v3 report remains intact. This additive finding updates its earlier statement that no low defect was established. No high or medium defect was established by this additional probe; developer-trial entry remains supported with this tracked repair.

The parent supplied a focused consumer observation and requested independent reproduction and severity assessment. No peer or Grok report was read. All 58 frozen file hashes were rechecked and match the v3 manifest, aggregate `c66b683b47aec11f3670f93510a73f706ee37369bb848164559edf122287ade9`.

## Location and reproduction

`packages/moriarty-beta/src/cli.ts:19` reads files with `readFileSync(path, 'utf8')`. Node replaces malformed UTF-8 with U+FFFD. The resulting valid string reaches the frontend's scalar validation and hashing at `frontend.ts:394`. The formatter write at `cli.ts:82` can then persist that replacement.

Independently wrote a temporary file consisting of these bytes:

```js
Buffer.concat([
  Buffer.from('profile "moriarty-beta/1"; agreement Demo { // '),
  Buffer.from([255]),
  Buffer.from('\n const x=1; }')
])
```

Ran the actual CLI source with `check FILE --json`, then `fmt FILE --write`. Observations:

- Check exited 0 and returned `AuthoringChecked`.
- Original file SHA256 was `1bb7ac7620a6f2d23222eb3a39accfa78dd9171175b017adcf38f2f496fe5ff7`.
- Reported sourceHash was `ccd2b113576140dce66a472196387530c5b60e39947d02f689cf0649144bd881`, exactly the hash of the replacement-decoded/re-encoded text.
- Format exited 0; the resulting file contained the UTF-8 encoding of U+FFFD and no original 0xFF byte.

Temporary reproduction files were removed. No product bytes were changed.

## Severity reasoning

This is a real CLI input-integrity defect against the exact-source-byte and strict-invalid-input contract. A caller cannot use this digest as the original malformed file's byte commitment, and `fmt --write` accepts an input that should have been rejected without a write. The common file reader also serves scenario and test-manifest files, so repair should cover those callers.

I classify it **Low**, rather than Medium, because the established trigger requires malformed UTF-8 at a local file boundary. Valid UTF-8 inputs are unaffected; the source/scenario text APIs receive and hash the actual strings they process, and stdio transport has fatal UTF-8 decoding already. The probe does not establish altered acceptance of a valid financial request, bypassed authentication or ledger acceptance, or a mismatch between the processed string and its reported digest. Local results remain unqualified. This is not an exemption from repair: the documented CLI integrity behavior should be fixed before release.

## Requested repair and acceptance checks

Read bounded file bytes and decode with a fatal UTF-8 decoder, returning a nonzero structured or clear CLI error on decoding failure before source/scenario/manifest processing. Preserve valid leading Unicode content deliberately; when using TextDecoder, account for BOM handling so successful decoding does not silently remove an original BOM and change the hashed input bytes.

Add focused invalid UTF-8 source and scenario probes; check that malformed input fails and that `fmt --write` leaves the original bytes intact. Include a valid UTF-8 digest regression and an explicit BOM policy regression. Use the same reader for case-manifest dependencies. No repair has been reviewed or accepted in this addendum; the parent reports it as planned for the final developer-experience batch.
