/**
 * The fixed counts are part of the brief, not an implementation detail. A
 * design that quietly shows "the main ones" fails; this is the gate that says so.
 *
 * Run: node --test site/src/data/completeness.test.mjs
 */
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const read = (f) => readFileSync(new URL(f, import.meta.url), 'utf8');

const counted = (src, section, marker, end) => {
  const block = end ? src.split(section)[1].split(end)[0] : src.split(section)[1];
  return (block.match(marker) ?? []).length;
};

test('seven economic families plus the cross-cutting group', () => {
  const src = read('./categories.ts');
  const ids = [...src.matchAll(/^    id: '(F[1-6]|P|X)',$/gm)].map((m) => m[1]);
  assert.deepEqual(ids, ['F1', 'F2', 'F3', 'F4', 'F5', 'F6', 'P', 'X']);
});

test('eight mandatory facets, and every family answers all of them', () => {
  const src = read('./categories.ts');
  const facets = [...src.matchAll(/^  '([^']+)',$/gm)].map((m) => m[1]);
  assert.equal(facets.length, 8, 'FACETS must list exactly eight');
  const blocks = src.split('facets: {').slice(1);
  assert.equal(blocks.length, 8, 'one facet profile per family');
  for (const b of blocks) {
    const body = b.split('},')[0];
    for (const f of facets) {
      assert.ok(body.includes(f), `facet "${f}" missing from a family profile`);
    }
  }
});

test('twenty-four action targets, each exactly once, DA01 through DA24', () => {
  const src = read('./actionTargets.ts');
  const ids = [...src.matchAll(/^    id: 'DA(\d\d)',$/gm)].map((m) => m[1]);
  assert.equal(ids.length, 24);
  assert.equal(new Set(ids).size, 24);
  assert.deepEqual(
    ids,
    Array.from({ length: 24 }, (_, i) => String(i + 1).padStart(2, '0')),
  );
});

test('every action target carries a requirement and a distinguishing test', () => {
  const src = read('./actionTargets.ts');
  assert.equal(counted(src, 'ACTION_TARGETS', /requirement:/g, 'COMPOSITION_OPERATORS'), 24);
  assert.equal(
    counted(src, 'ACTION_TARGETS', /distinguishingTest:/g, 'COMPOSITION_OPERATORS'),
    24,
  );
});

test('five composition operators', () => {
  const src = read('./actionTargets.ts');
  const ops = src.split('COMPOSITION_OPERATORS')[1].split(']')[0].match(/'[a-z]+'/g);
  assert.equal(ops.length, 5);
});

test('composition failure rates match the reproduced audit', () => {
  const src = read('./actionTargets.ts');
  const num = (k) => Number(src.match(new RegExp(`${k}: ([\\d.]+)`))[1]);
  assert.equal(num('eligiblePairs'), 1830);
  assert.equal(num('clean') + num('failures'), 1830);
  assert.equal(num('crossCategoryFailures') + num('withinCategoryFailures'), 185);
  assert.equal(num('withinCategoryPairs') + num('crossCategoryPairs'), 1830);
  const ratio = num('crossCategoryRate') / num('withinCategoryRate');
  assert.ok(Math.abs(ratio - num('ratio')) < 0.01, `stated ratio disagrees: ${ratio}`);
});

test('six assurance layers, twelve properties, four claims, thirteen threats', () => {
  const src = read('./assurance.ts');
  assert.equal(counted(src, 'ASSURANCE_LAYERS', /name:/g, '] as const'), 6);
  assert.equal(counted(src, 'export const PROPERTIES', /assumption:/g, 'MANDATORY_CLAIMS'), 12);
  assert.equal(counted(src, 'MANDATORY_CLAIMS', /name:/g, '] as const'), 4);
  assert.equal(counted(src, 'export const THREATS', /threat:/g, '] as const'), 13);
});

test('every property keeps its assumption and obligation separate', () => {
  const src = read('./assurance.ts');
  const block = src.split('export const PROPERTIES')[1].split('MANDATORY_CLAIMS')[0];
  assert.equal((block.match(/assumption:/g) ?? []).length, 12);
  assert.equal((block.match(/obligation:/g) ?? []).length, 12);
});

test('every threat keeps its residual risk', () => {
  const src = read('./assurance.ts');
  const block = src.split('export const THREATS')[1].split('] as const')[0];
  assert.equal((block.match(/residual:/g) ?? []).length, 13);
});

test('twelve sprints, and five specification layers', () => {
  assert.equal(counted(read('./roadmap.ts'), 'SPRINTS', /^    id: 'SP\d\d',$/gm, '] as const'), 12);
  assert.equal(counted(read('./language.ts'), 'SPEC_LAYERS', /^    layer: /gm, '] as const'), 5);
});

test('no vendor or tooling attribution anywhere in the data layer', () => {
  const forbidden = /\b(claude|anthropic|gemini|openai|gpt|copilot|chatgpt|llm|ai-generated)\b/i;
  for (const f of [
    './categories.ts',
    './actionTargets.ts',
    './assurance.ts',
    './language.ts',
    './intents.ts',
    './roadmap.ts',
  ]) {
    const hit = read(f).match(forbidden);
    assert.equal(hit, null, `${f} mentions ${hit?.[0]}`);
  }
});
