/** Inspect only a new private copy of a stopped Level store. Never opens the source DB.
 * Caller must complete the public recovery gate first and enforce the overall deadline.
 * Filesystem checks assume no concurrent mutation by the same trusted OS account.
 */
import * as fs from 'node:fs';
import {join,dirname,resolve,isAbsolute,sep} from 'node:path';
import {createHash} from 'node:crypto';
import {createRequire} from 'node:module';
import {PINNED_NM} from './providers.mjs';
import {assertMetadataOnlyEntries} from './recover-deployment.mjs';
const check=(ok,code)=>{if(!ok)throw Error('RECOVERY_STORE_'+code);};
const sha=b=>createHash('sha256').update(b).digest('hex');
const MAX_FILES=32,MAX_BYTES=1024*1024;
const require=createRequire(join(PINNED_NM,'../package.json'));
function pinnedLevel(){
 // Inspected installed Node entrypoints, not a transitive dependency attestation.
 for(const [name,version,digest] of [['level','10.0.0','14d1713253123c99a384aa66e79f871144216fe84d3b0362c3cf9f2ee3da7aa4'],['classic-level','3.0.0','6958a1d105be7eca861e18d244a42b3a6e7fc28c3cf9dbe6dd24720c529a171c']]){
  check(require.resolve(name)===join(PINNED_NM,name,'index.js'),'LEVEL_PATH_PIN');
  const pkg=JSON.parse(fs.readFileSync(join(PINNED_NM,name,'package.json'),'utf8'));
  check(pkg.name===name&&pkg.version===version&&sha(fs.readFileSync(require.resolve(name)))===digest,'LEVEL_PIN');
 }
 return require('level').Level;
}
function directory(path,privateMode=false){
 check(typeof path==='string'&&isAbsolute(path)&&resolve(path)===path,'ABSOLUTE_PATH');
 let cursor=path;
 for(;;){const s=fs.lstatSync(cursor);check(s.isDirectory()&&!s.isSymbolicLink(),'DIRECTORY_SYMLINK');if(cursor===path&&privateMode)check(s.uid===process.getuid()&&(s.mode&0o7777)===0o700,'DIRECTORY_PRIVATE');const parent=dirname(cursor);if(parent===cursor)break;cursor=parent;}
}
function fileStat(s){check(s.isFile()&&s.nlink===1&&s.uid===process.getuid()&&(s.mode&0o7777)===0o600,'FILE_PRIVATE');check(s.size<=MAX_BYTES,'BYTE_BOUND');}
function manifest(path){
 directory(path,true);const names=[];const d=fs.opendirSync(path);try{for(let e;(e=d.readSync())!==null;){names.push(e.name);check(names.length<=MAX_FILES,'FILE_BOUND');}}finally{d.closeSync();}
 check(names.length>0,'EMPTY');let total=0;return names.sort().map(name=>{const p=join(path,name),s=fs.lstatSync(p);fileStat(s);total+=s.size;check(total<=MAX_BYTES,'BYTE_BOUND');const fd=fs.openSync(p,fs.constants.O_RDONLY|fs.constants.O_NOFOLLOW);try{const actual=fs.fstatSync(fd);fileStat(actual);check(actual.ino===s.ino&&actual.dev===s.dev&&actual.size===s.size,'FILE_CHANGED');const b=Buffer.alloc(s.size);let n=0;while(n<b.length){const read=fs.readSync(fd,b,n,b.length-n,n);check(read>0,'FILE_CHANGED');n+=read;}check(fs.readSync(fd,Buffer.alloc(1),0,1,n)===0,'FILE_CHANGED');return{name,size:s.size,sha256:sha(b)};}finally{fs.closeSync(fd);}});
}
const digest=m=>sha(JSON.stringify(m));
function syncDirectory(path){const fd=fs.openSync(path,fs.constants.O_RDONLY|fs.constants.O_DIRECTORY|fs.constants.O_NOFOLLOW);try{fs.fsyncSync(fd);}finally{fs.closeSync(fd);}}
function copyFile(source,target,entry){const input=fs.openSync(join(source,entry.name),fs.constants.O_RDONLY|fs.constants.O_NOFOLLOW);let output;try{const s=fs.fstatSync(input);fileStat(s);check(s.size===entry.size,'FILE_CHANGED');output=fs.openSync(join(target,entry.name),fs.constants.O_WRONLY|fs.constants.O_CREAT|fs.constants.O_EXCL|fs.constants.O_NOFOLLOW,0o600);const b=Buffer.alloc(entry.size);let n=0;while(n<b.length){const read=fs.readSync(input,b,n,b.length-n,n);check(read>0,'FILE_CHANGED');n+=read;}check(sha(b)===entry.sha256&&fs.readSync(input,Buffer.alloc(1),0,1,n)===0,'FILE_CHANGED');fs.writeFileSync(output,b);fs.fsyncSync(output);}finally{if(output!==undefined)fs.closeSync(output);fs.closeSync(input);}}
export async function inspectFailedLoanStore({sourceDirectory,inspectionDirectory,accountId}){
 let db,originalDigest,result,failure;
 try{
  check(typeof accountId==='string'&&accountId.length>0&&accountId.length<=1024,'ACCOUNT');
  directory(sourceDirectory,true);check(typeof inspectionDirectory==='string'&&isAbsolute(inspectionDirectory)&&resolve(inspectionDirectory)===inspectionDirectory,'ABSOLUTE_PATH');
  check(sourceDirectory!==inspectionDirectory&&!inspectionDirectory.startsWith(sourceDirectory+sep)&&!sourceDirectory.startsWith(inspectionDirectory+sep),'OVERLAP');
  directory(dirname(inspectionDirectory),true);
  const Level=pinnedLevel(),before=manifest(sourceDirectory);originalDigest=digest(before);
  fs.mkdirSync(inspectionDirectory,{mode:0o700});syncDirectory(dirname(inspectionDirectory));
  for(const entry of before)copyFile(sourceDirectory,inspectionDirectory,entry);
  syncDirectory(inspectionDirectory);
  const copiedDigest=digest(manifest(inspectionDirectory));
  check(copiedDigest===originalDigest&&digest(manifest(sourceDirectory))===originalDigest,'SOURCE_CHANGED');
  const fd=fs.openSync(join(inspectionDirectory,'.recovery-copy-manifest.json'),fs.constants.O_WRONLY|fs.constants.O_CREAT|fs.constants.O_EXCL|fs.constants.O_NOFOLLOW,0o600);try{fs.writeFileSync(fd,JSON.stringify({sourceManifestSha256:originalDigest,files:before})+'\n');fs.fsyncSync(fd);}finally{fs.closeSync(fd);}syncDirectory(inspectionDirectory);
  db=new Level(inspectionDirectory,{createIfMissing:false,keyEncoding:'buffer',valueEncoding:'buffer'});
  await db.open();const entries=[];let total=0;
  // Read the entire root namespace; a second entry already disproves the predicate.
  for await(const [k,v] of db.iterator({limit:2,highWaterMarkBytes:1024,fillCache:false})){total+=k.length+v.length;check(k.length<=1024&&v.length<=256&&total<=4096,'ENTRY_BOUND');check(Buffer.from(k.toString('utf8')).equals(k)&&Buffer.from(v.toString('utf8')).equals(v),'ENTRY_ENCODING');entries.push([k.toString('utf8'),v.toString('utf8')]);}
  assertMetadataOnlyEntries(entries,accountId);
  result=Object.freeze({status:'METADATA_ONLY',sourceManifestSha256:originalDigest,copyManifestSha256:copiedDigest});
 }catch(e){failure=e.message?.startsWith('RECOVERY_STORE_')?e:Error('RECOVERY_STORE_INSPECTION_FAILED');}
 finally{
  if(db){try{await db.close();for(const name of fs.readdirSync(inspectionDirectory)){const p=join(inspectionDirectory,name),s=fs.lstatSync(p);check(s.isFile()&&!s.isSymbolicLink()&&s.nlink===1,'COPY_FILE');fs.chmodSync(p,0o600);const fd=fs.openSync(p,fs.constants.O_RDONLY|fs.constants.O_NOFOLLOW);try{fs.fsyncSync(fd);}finally{fs.closeSync(fd);}}syncDirectory(inspectionDirectory);}catch{failure=Error('RECOVERY_STORE_CLOSE_FAILED');}}
  if(originalDigest){try{check(digest(manifest(sourceDirectory))===originalDigest,'SOURCE_CHANGED');}catch{failure=Error('RECOVERY_STORE_SOURCE_CHANGED');}}
 }
 if(failure)throw failure;return result;
}
/** Check only the prepared destination's privacy and emptiness, before provider creation.
 * This does not authorize restoration; the caller still needs all recovery gates.
 * As above, the trusted OS account must keep the directory quiescent through use.
 */
export function assertEmptyRecoveryStore(path){
 try{
  directory(path,true);
  const handle=fs.opendirSync(path);
  try{check(handle.readSync()===null,'DESTINATION_NOT_EMPTY');}finally{handle.closeSync();}
  return true;
 }catch(e){throw e.message?.startsWith('RECOVERY_STORE_')?e:Error('RECOVERY_STORE_DESTINATION_INVALID');}
}
/** Require exactly the two pre-existing account scopes and v2 encrypted records.
 * Framing follows pinned private-state-provider 4.1.1 index.mjs:150-170,240-248,
 * 321-357,390-414,776-787,798-804,827-833. This does not decrypt or authenticate.
 * Missing metadata and v1/plaintext would make SDK reads write/migrate the source.
 */
function initializedEntries(entries,accountId,kind){
 const address=kind==='loan'?'ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713':'8824d69c9058f322b4f6da7e7cd8d49f3235db5fbe3d6080d25f239243812261',store='sp05-'+kind,account=sha(accountId).slice(0,32);
 check(entries.length===4,'INITIALIZED_NAMESPACE');const values=new Map(entries);check(values.size===4,'INITIALIZED_NAMESPACE');
 for(const [scope,key] of [[store,address+':'+store],['signing-keys',address]]){
  const prefix='!'+scope+':'+account+'!',metaKey=prefix+'__midnight_encryption_metadata__',valueKey=prefix+key;
  check(values.has(metaKey)&&values.has(valueKey),'INITIALIZED_NAMESPACE');
  let metadata;try{metadata=JSON.parse(values.get(metaKey));}catch{throw Error('RECOVERY_STORE_METADATA');}
  check(metadata&&Object.getPrototypeOf(metadata)===Object.prototype&&Object.keys(metadata).sort().join(',')==='salt,version'&&metadata.version===1&&typeof metadata.salt==='string'&&/^[0-9a-f]{64}$/.test(metadata.salt),'METADATA');
  const encoded=values.get(valueKey),data=Buffer.from(encoded,'base64');
  check(data.toString('base64')===encoded&&data.length>61&&data[0]===2&&data.subarray(1,33).toString('hex')===metadata.salt,'INITIALIZED_CIPHERTEXT');
 }
}
/** Preserve a raw snapshot first; open ONLY a second exclusive inspection copy.
 * Caller completes the public gate and keeps the trusted OS account quiescent.
 * No decryption: existing provider subsequently verifies the exact state/key.
 */
export function preserveInitializedLoanStore(options){return preserveInitializedStore(options,'loan');}
export function preserveInitializedSwapStore(options){return preserveInitializedStore(options,'swap');}
async function preserveInitializedStore({sourceDirectory,snapshotDirectory,inspectionDirectory,accountId},kind){
 let sourceDigest,snapshotDigest,db,failure,result;
 try{
  check(typeof accountId==='string'&&accountId.trim().length>0&&accountId.length<=1024,'ACCOUNT');directory(sourceDirectory,true);
  const paths=[sourceDirectory,snapshotDirectory,inspectionDirectory];
  for(const path of paths)check(typeof path==='string'&&isAbsolute(path)&&resolve(path)===path,'ABSOLUTE_PATH');
  for(let i=0;i<paths.length;i++)for(let j=i+1;j<paths.length;j++)check(paths[i]!==paths[j]&&!paths[i].startsWith(paths[j]+sep)&&!paths[j].startsWith(paths[i]+sep),'OVERLAP');
  for(const path of [snapshotDirectory,inspectionDirectory]){directory(dirname(path),true);try{fs.lstatSync(path);throw Error('RECOVERY_STORE_DESTINATION_EXISTS');}catch(e){if(e.code!=='ENOENT')throw e;}}
  const Level=pinnedLevel(),before=manifest(sourceDirectory);sourceDigest=digest(before);
  for(const path of [snapshotDirectory,inspectionDirectory]){
   fs.mkdirSync(path,{mode:0o700});syncDirectory(dirname(path));
   for(const entry of before)copyFile(sourceDirectory,path,entry);
   syncDirectory(path);check(digest(manifest(path))===sourceDigest,'COPY_CHANGED');
   if(path===snapshotDirectory)snapshotDigest=sourceDigest;
  }
  check(digest(manifest(sourceDirectory))===sourceDigest,'SOURCE_CHANGED');
  db=new Level(inspectionDirectory,{createIfMissing:false,keyEncoding:'buffer',valueEncoding:'buffer'});await db.open();
  const entries=[];let total=0;
  // Four exact entries required; a fifth disproves the predicate. Source <=1 MiB.
  for await(const [k,v] of db.iterator({limit:5,highWaterMarkBytes:1024,fillCache:false})){
   total+=k.length+v.length;check(k.length<=1024&&v.length<=4096&&total<=16384,'ENTRY_BOUND');
   check(Buffer.from(k.toString('utf8')).equals(k)&&Buffer.from(v.toString('utf8')).equals(v),'ENTRY_ENCODING');entries.push([k.toString('utf8'),v.toString('utf8')]);
  }
  initializedEntries(entries,accountId,kind);
  result=Object.freeze({status:'PRESERVED',sourceManifestSha256:sourceDigest,snapshotManifestSha256:snapshotDigest});
 }catch(e){failure=e.message?.startsWith('RECOVERY_STORE_')?e:Error('RECOVERY_STORE_PRESERVATION_FAILED');}
 finally{
  if(db){try{await db.close();for(const name of fs.readdirSync(inspectionDirectory)){const p=join(inspectionDirectory,name),s=fs.lstatSync(p);check(s.isFile()&&!s.isSymbolicLink()&&s.nlink===1,'COPY_FILE');fs.chmodSync(p,0o600);const fd=fs.openSync(p,fs.constants.O_RDONLY|fs.constants.O_NOFOLLOW);try{fs.fsyncSync(fd);}finally{fs.closeSync(fd);}}syncDirectory(inspectionDirectory);}catch{failure=Error('RECOVERY_STORE_CLOSE_FAILED');}}
  if(sourceDigest){try{check(digest(manifest(sourceDirectory))===sourceDigest,'SOURCE_CHANGED');}catch{failure=Error('RECOVERY_STORE_SOURCE_CHANGED');}}
  if(snapshotDigest){try{check(digest(manifest(snapshotDirectory))===snapshotDigest,'SNAPSHOT_CHANGED');}catch{failure=Error('RECOVERY_STORE_SNAPSHOT_CHANGED');}}
 }
 if(failure)throw failure;return result;
}
