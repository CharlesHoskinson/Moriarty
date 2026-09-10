/** Nonsecret offline reproduction. Uses only a newly-created temporary DB.
 * Run from any cwd; prints public JSON. Deletes only its own temporary fixture.
 * No wallet, real private-state reads, network, proof, or acceptance claim.
 */
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {mkdtempSync,readFileSync,readdirSync,statSync,rmSync,existsSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {createRequire} from 'node:module';
import {fileURLToPath,pathToFileURL} from 'node:url';
const packageRoot='/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world';
const nm=join(packageRoot,'node_modules');
const require=createRequire(join(packageRoot,'package.json'));
const sha256=bytes=>createHash('sha256').update(bytes).digest('hex');
const sourceFiles=[
 '@midnight-ntwrk/midnight-js-level-private-state-provider/package.json',
 '@midnight-ntwrk/midnight-js-level-private-state-provider/dist/index.mjs',
 '@midnight-ntwrk/midnight-js-utils/package.json',
 '@midnight-ntwrk/midnight-js-utils/dist/index.mjs',
];
const sources=sourceFiles.map(relativePath=>({path:join(nm,relativePath),sha256:sha256(readFileSync(join(nm,relativePath)))}));
const versions=Object.fromEntries(['midnight-js-level-private-state-provider','midnight-js-utils'].map(name=>[name,JSON.parse(readFileSync(join(nm,'@midnight-ntwrk',name,'package.json'),'utf8')).version]));
assert.equal(versions['midnight-js-level-private-state-provider'],'4.1.1');
assert.equal(versions['midnight-js-utils'],'4.1.1');
const {Level}=require('level');
const {levelPrivateStateProvider}=await import(pathToFileURL(join(nm,'@midnight-ntwrk/midnight-js-level-private-state-provider/dist/index.mjs')).href);
const fixture=mkdtempSync(join(tmpdir(),'moriarty-password-repro-'));
let result;
try {
 const provider=levelPrivateStateProvider({midnightDbName:fixture,privateStateStoreName:'sp05-loan',accountId:'synthetic-public-test-account',privateStoragePasswordProvider:()=> 'a1'.repeat(32)});
 provider.setContractAddress('a'.repeat(64));
 let failure;
 try {await provider.set('synthetic-test',{});}catch(error){failure={name:error.name,reason:error.reason};}
 assert.deepEqual(failure,{name:'PasswordValidationError',reason:'insufficient_classes'});
 const filesBeforeRead=readdirSync(fixture).sort().map(name=>({name,bytes:statSync(join(fixture,name)).size}));
 const db=new Level(fixture);let entries;
 try {await db.open();entries=[];for await(const [key,value] of db.iterator())entries.push([key,JSON.parse(value)]);}
 finally {await db.close();}
 assert.equal(entries.length,1);
 assert.deepEqual(Object.keys(entries[0][1]).sort(),['salt','version']);
 assert.equal(entries[0][1].version,1);
 assert.match(entries[0][1].salt,/^[a-f0-9]{64}$/);
 assert(entries[0][0].endsWith('__midnight_encryption_metadata__'));
 assert(!entries[0][0].includes('synthetic-test'));
 result={schema:'moriarty.password-provider-reproduction/1',status:'REPRODUCED',observedAt:new Date().toISOString(),
  scriptSha256:sha256(readFileSync(fileURLToPath(import.meta.url))),versions,sources,
  fixture:{syntheticPassword:true,passwordLength:64,passwordCharacterClasses:2,privateStateStoreName:'sp05-loan',newTemporaryDatabase:true},
  failure,entryCount:entries.length,onlySaltVersionMetadata:true,privateStateEntryWritten:false,filesBeforeRead,
  sourceTrace:[
   'midnight-js-level-private-state-provider/dist/index.mjs:797-804: set calls getOrCreateEncryption before state serialization and value write',
   'same file:389-414,424-439: salt/version metadata is created before getPasswordFromProvider',
   'same file:284-287: getPasswordFromProvider calls validatePassword',
   'midnight-js-utils/dist/index.mjs:319-335: validator requires at least three character classes; two yields insufficient_classes',
   'Moriarty prepare-deployment.mjs:57-63: successful finalized submit precedes private-state set and signing-key set',
   'Moriarty run-local.mjs:50-51: driver assigns contractAddress only after deployment adapter returns'
  ],
  qualification:'Exact synthetic provider failure reproduced. Original integration exception code was not emitted; original causal attribution combines this reproduction, root reported local validator rejection, and public failure sequence. Does not inspect original private state or prove absence of prior private writes.',
  networkAcceptance:false,proofAcceptance:false,financialAcceptance:false,
  scope:'Offline causal evidence only; no wallet, real private-state access, network, service, proof, or transaction operation'};
} finally {rmSync(fixture,{recursive:true,force:true});}
assert.equal(existsSync(fixture),false);
result.fixtureRemoved=true;
console.log(JSON.stringify(result,null,2));
