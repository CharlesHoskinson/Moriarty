/** Offline conservation regression for the actual generated page data + typed loader.
 * Reads public JSON only. No React, esbuild, SDK, proof, provider or ledger import.
 */
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {loadForDisplay} from './catalogue.ts';

const expected=JSON.parse(await readFile(new URL('./testdata/catalogue-membership.json',import.meta.url),'utf8'));
const sourceText=await readFile(new URL('../../../../evidence/utxo/cardano-catalogue-inventory-graph-2026-10-17/attribution-packet/CATALOGUE-v1.2.json',import.meta.url),'utf8');
const source=JSON.parse(sourceText);
const generated=JSON.parse(await readFile(new URL('./data.generated.json',import.meta.url),'utf8'));
const view=loadForDisplay(generated.catalogueText,generated.catalogueSource);

test('actual generated page data supplies the exact retained catalogue to the loader',()=>{
  assert.equal(createHash('sha256').update(sourceText).digest('hex'),expected.source.sha256);
  assert.equal(generated.catalogueText,sourceText);
  assert.deepEqual(generated.catalogueSource,expected.source);
  assert.equal(view.catalogueId,source.catalogueId);
  assert.equal(view.catalogueVersion,source.catalogueVersion);
});

test('loader preserves all143 versioned identities and89 source heads in frozen source order',()=>{
  assert.equal(view.entries.length,expected.expectedVersionedRows);
  assert.deepEqual(view.entries.map(entry=>entry.identity.versionedId),expected.orderedVersionedIds);
  const heads=view.entries.filter(entry=>entry.identity.sourceHead).map(entry=>entry.identity.versionedId);
  assert.equal(heads.length,expected.expectedSourceHeads);
  assert.deepEqual(heads,expected.sourceHeadIds);
  for(const [index,entry] of view.entries.entries()){
    const row=source.rows[index];
    assert.deepEqual(entry.sourceRow,row,row.versionedId);
    assert.equal(entry.identity.id,row.id,row.versionedId);
    assert.equal(entry.identity.version,row.version,row.versionedId);
    assert.equal(entry.evidence.sourceStatus,row.status,row.versionedId);
    assert.equal(entry.assumptions.sourceDescription,row.anchorAndAssumptions,row.versionedId);
    assert.equal(entry.inputs.publicDescription,row.publicInputs,row.versionedId);
    assert.equal(entry.inputs.privateWitnessDescription,row.privateWitness,row.versionedId);
    assert.equal(entry.outputs.description,row.resultSchema??null,row.versionedId);
    assert.equal(entry.source.jsonPointer,`/rows/${index}`);
  }
});

test('loader retains every ordered typed field and source attribute without inventing a schema',()=>{
  const typed=view.entries.filter(entry=>entry.publicStatement.schema!==null);
  assert.deepEqual(typed.map(entry=>entry.identity.versionedId),expected.typedRows.map(row=>row.versionedId));
  for(const frozen of expected.typedRows){
    const entry=typed.find(item=>item.identity.versionedId===frozen.versionedId);
    assert.deepEqual(entry.publicStatement.schema,frozen.statementSchema,frozen.versionedId);
  }
  for(const entry of view.entries){
    assert.deepEqual(entry.publicStatement.schema,entry.sourceRow.statementSchema??null,entry.identity.versionedId);
    assert.equal(entry.inputs.typedInputFields,null);
    assert.equal(entry.outputs.typedFields,null);
  }
});
