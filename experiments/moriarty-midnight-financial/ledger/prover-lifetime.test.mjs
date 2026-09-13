import test from 'node:test';
import assert from 'node:assert/strict';
import {
 PROVER_LATEST_START_SECONDS,PROVER_KILL_DEADLINE_SECONDS,PROVER_MAX_RUNTIME_SECONDS,
 PROVER_CONTROL_SCHEMA,PROVER_CONTROL_MAX_BYTES,computeProverLifetimeBounds,
 validateWrapperEntry,computeWrapperKillDeadline,encodeProverControlRecord,
 decodeProverControlRecord,validateClockIdentity
} from './prover-lifetime.mjs';

const bootId='01234567-89ab-4def-8123-456789abcdef',invocationDigest='ab'.repeat(32);
const control=()=>({bootId,timeNamespaceInode:42n,latestStartMonotonicNs:120000000123n,killDeadlineMonotonicNs:1620000000123n,invocationDigest});

test('literal prover lifetime constants and BigInt bounds remain anchored to outer start',()=>{
 assert.equal(PROVER_LATEST_START_SECONDS,120);assert.equal(PROVER_KILL_DEADLINE_SECONDS,1620);assert.equal(PROVER_MAX_RUNTIME_SECONDS,1500);
 assert.equal(PROVER_CONTROL_SCHEMA,'moriarty.prover-lifetime/1');assert.equal(PROVER_CONTROL_MAX_BYTES,512);
 assert.deepEqual(computeProverLifetimeBounds(123n),{latestStartMonotonicNs:120000000123n,killDeadlineMonotonicNs:1620000000123n});
 for(const value of [-1n,0,1,'0',null])assert.throws(()=>computeProverLifetimeBounds(value),{message:'PROVER_LIFETIME_OUTER_START'});
});

test('wrapper entry accepts only times strictly before the absolute latest start',()=>{
 assert.equal(validateWrapperEntry(119999999999n,120000000000n),true);
 assert.throws(()=>validateWrapperEntry(120000000000n,120000000000n),{message:'PROVER_LIFETIME_LATE_ENTRY'});
 assert.throws(()=>validateWrapperEntry(120000000001n,120000000000n),{message:'PROVER_LIFETIME_LATE_ENTRY'});
 assert.throws(()=>validateWrapperEntry(-1n,120000000000n),{message:'PROVER_LIFETIME_ENTRY'});
 assert.throws(()=>validateWrapperEntry(1n,-1n),{message:'PROVER_LIFETIME_LATEST_START'});
});

test('wrapper deadline is the earlier of entry plus 1500 seconds and the control deadline',()=>{
 assert.equal(computeWrapperKillDeadline(50n,2000000000000n),1500000000050n);
 assert.equal(computeWrapperKillDeadline(50n,1400000000000n),1400000000000n);
 for(const value of [-1n,1])assert.throws(()=>computeWrapperKillDeadline(value,2n),{message:'PROVER_LIFETIME_ENTRY'});
 for(const value of [-1n,2])assert.throws(()=>computeWrapperKillDeadline(1n,value),{message:'PROVER_LIFETIME_CONTROL_DEADLINE'});
});

test('control record encoding is the exact six-line ASCII format and round trips canonically',()=>{
 const expected='moriarty.prover-lifetime/1\n01234567-89ab-4def-8123-456789abcdef\n42\n120000000123\n1620000000123\n'+invocationDigest+'\n';
 const encoded=encodeProverControlRecord(control());
 assert.equal(Buffer.isBuffer(encoded),true);assert.equal(encoded.toString('ascii'),expected);
 assert.deepEqual(decodeProverControlRecord(encoded),control());
 assert.deepEqual(decodeProverControlRecord(encodeProverControlRecord({...control(),timeNamespaceInode:42})),control());
});

test('control record encoder rejects every malformed field and unknown fields',()=>{
 const cases=[
  [{...control(),bootId:bootId.toUpperCase()},'PROVER_LIFETIME_CONTROL_BOOT_ID'],
  [{...control(),timeNamespaceInode:'42'},'PROVER_LIFETIME_CONTROL_TIME_NAMESPACE'],
  [{...control(),timeNamespaceInode:Number.MAX_SAFE_INTEGER+1},'PROVER_LIFETIME_CONTROL_TIME_NAMESPACE'],
  [{...control(),latestStartMonotonicNs:-1n},'PROVER_LIFETIME_CONTROL_LATEST_START'],
  [{...control(),killDeadlineMonotonicNs:1},'PROVER_LIFETIME_CONTROL_KILL_DEADLINE'],
  [{...control(),invocationDigest:'A'.repeat(64)},'PROVER_LIFETIME_CONTROL_INVOCATION_DIGEST'],
  [{...control(),extra:true},'PROVER_LIFETIME_CONTROL_FIELDS']
 ];
 for(const [value,code] of cases)assert.throws(()=>encodeProverControlRecord(value),{message:code});
 const accessor=control();Object.defineProperty(accessor,'bootId',{enumerable:true,get(){throw Error('GETTER_RAN');}});
 assert.throws(()=>encodeProverControlRecord(accessor),error=>error.message==='PROVER_LIFETIME_CONTROL_FIELDS');
 const hidden=control();Object.defineProperty(hidden,'hidden',{value:true});
 assert.throws(()=>encodeProverControlRecord(hidden),{message:'PROVER_LIFETIME_CONTROL_FIELDS'});
});

test('control record decoder rejects size, CR, framing, schema and malformed fields distinctly',()=>{
 const lines=encodeProverControlRecord(control()).toString('ascii').split('\n');
 const record=xs=>Buffer.from(xs.join('\n'),'ascii');
 const cases=[
  [Buffer.alloc(PROVER_CONTROL_MAX_BYTES+1,0x61),'PROVER_LIFETIME_CONTROL_SIZE'],
  [Buffer.from(encodeProverControlRecord(control()).toString('ascii').replace('\n','\r\n')),'PROVER_LIFETIME_CONTROL_CR'],
  [Buffer.from(encodeProverControlRecord(control()).subarray(0,-1)),'PROVER_LIFETIME_CONTROL_LINES'],
  [Buffer.concat([encodeProverControlRecord(control()),Buffer.from('extra\n')]),'PROVER_LIFETIME_CONTROL_LINES'],
  [record(['wrong',...lines.slice(1)]),'PROVER_LIFETIME_CONTROL_SCHEMA'],
  [record([lines[0],'not-a-uuid',...lines.slice(2)]),'PROVER_LIFETIME_CONTROL_BOOT_ID'],
  [record([...lines.slice(0,2),'01',...lines.slice(3)]),'PROVER_LIFETIME_CONTROL_TIME_NAMESPACE'],
  [record([...lines.slice(0,3),'01',...lines.slice(4)]),'PROVER_LIFETIME_CONTROL_LATEST_START'],
  [record([...lines.slice(0,4),'-1',...lines.slice(5)]),'PROVER_LIFETIME_CONTROL_KILL_DEADLINE'],
  [record([...lines.slice(0,5),'A'.repeat(64),'']),'PROVER_LIFETIME_CONTROL_INVOCATION_DIGEST'],
  [Buffer.from([0xff,0x0a]),'PROVER_LIFETIME_CONTROL_ASCII']
 ];
 for(const [value,code] of cases)assert.throws(()=>decodeProverControlRecord(value),{message:code});
 assert.throws(()=>decodeProverControlRecord('not-a-buffer'),{message:'PROVER_LIFETIME_CONTROL_BUFFER'});
});

test('clock identity compares boot ID and normalized namespace inode without I/O',()=>{
 assert.equal(validateClockIdentity({parentBootId:bootId,parentTimeNamespaceInode:42n,childBootId:bootId,childTimeNamespaceInode:42}),true);
 for(const value of [
  {parentBootId:bootId,parentTimeNamespaceInode:42n,childBootId:'11234567-89ab-4def-8123-456789abcdef',childTimeNamespaceInode:42n},
  {parentBootId:bootId,parentTimeNamespaceInode:42n,childBootId:bootId,childTimeNamespaceInode:43n}
 ])assert.throws(()=>validateClockIdentity(value),{message:'PROVER_LIFETIME_CLOCK_IDENTITY'});
});
