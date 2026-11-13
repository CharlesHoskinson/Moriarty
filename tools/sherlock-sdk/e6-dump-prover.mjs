#!/usr/bin/env node
// E6 stub prover (E6_PROVER contract): reads the SDK opening {"binding_input": LE hex, "preimage": hex} on
// stdin, decodes the tagged ProofPreimage as sherlock_gp gp_call_prove does (private transcript must be
// empty; the communication commitment must be present), and writes the ledger's call vector
//   [binding_input, comm, public_transcript_inputs...]
// to the file named by E6_DUMP as {"words": ["<canonical decimal Fr>", ...]} (the lead's format, lane order).
// A sidecar E6_DUMP.meta.json holds counts and the preimage's other fields (no secrets: all public data).
// It prints nothing on stdout and exits 3, so the runner never gets a proof and submits nothing.
// Expected total for the E6 wrap key: 1,026 words (E6_EXPECT_WORDS overrides, 0 disables the check).
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {decodePreimage, callVector} from './e6-padding.mjs';

const dump = process.env.E6_DUMP;
assert.ok(dump, 'E6_DUMP (output file for the call vector)');
const opening = JSON.parse(readFileSync(0, 'utf8'));
assert.match(opening.binding_input ?? '', /^[0-9a-f]{64}$/, 'binding_input: 32-byte LE hex');
const raw = Buffer.from(opening.preimage, 'hex');
const p = decodePreimage(raw);
assert.equal(p.privateTranscript.length, 0, 'private transcript must be empty');
const words = callVector(p, opening.binding_input);
writeFileSync(dump, JSON.stringify({words: words.map(String)}) + '\n');
const zeros = (() => { let n = 0; for (let i = words.length - 1; i >= 2 && words[i] === 0n; i--) n++; return n; })();
const meta = {schema: 'e6-call-vector-dump/1', encoding: 'canonical decimal strings of BLS12-381 Fr, lane order',
  total: words.length, expected: Number(process.env.E6_EXPECT_WORDS ?? 1026),
  lane0_binding_input: String(words[0]), lane1_comm: String(words[1]),
  publicTranscriptInputs: p.publicTranscriptInputs.length, trailingZeroWords: zeros,
  publicTranscriptOutputs: p.publicTranscriptOutputs.map(String), inputs: p.inputs.map(String),
  preimageBindingInputField: String(p.bindingInput), keyLocation: p.keyLocation, preimageTag: p.tag,
  preimageBytes: raw.length, preimageSHA256: createHash('sha256').update(raw).digest('hex'), at: new Date().toISOString()};
writeFileSync(dump + '.meta.json', JSON.stringify(meta, null, 2) + '\n');
console.error(`E6 dump: ${words.length} words (transcript ${p.publicTranscriptInputs.length}, trailing zeros ${zeros}) -> ${dump}`);
if (meta.expected && words.length !== meta.expected) console.error(`E6 dump: MISMATCH, expected ${meta.expected}`);
process.exit(3);
