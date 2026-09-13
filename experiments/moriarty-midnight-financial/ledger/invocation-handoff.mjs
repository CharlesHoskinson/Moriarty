/** Working JS model of the one-shot runner handoff, not the production Python runner.
 * Peer credentials remain an injected Node boundary; sockets and /proc checks are real.
 */
import {randomBytes as cryptoRandomBytes,createHash,timingSafeEqual} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {mkdir as fsMkdir,chmod as fsChmod} from 'node:fs/promises';
import {isAbsolute,join,resolve} from 'node:path';
import * as nodeNet from 'node:net';

export const HANDOFF_ENV_SOCKET='MORIARTY_INVOCATION_SOCKET';
export const HANDOFF_ENV_NONCE='MORIARTY_INVOCATION_NONCE';
export const HANDOFF_EVENT_SCHEMA='moriarty.loan-invocation/1';
export const HANDOFF_DENIAL_EVENT_SCHEMA='moriarty.loan-invocation-denial/1';
export const HANDOFF_NONCE_BYTES=32;
export const HANDOFF_OUTER_SECONDS=1800;
export const HANDOFF_ANCESTRY_MAX_DEPTH=32;
export const HANDOFF_SOCKET_NAME='invocation.sock';
export const HANDOFF_EVENT_FIELDS=['schema','allocationId','actionId','candidateHash','runnerDigest','chargeId','reservationId','storeIdentity','executionContextSha256','projectionSha256','correspondenceSha256','authoritySha256','bootId','outerStartMonotonic','outerDeadlineMonotonic','blockDeadlineUtc','parentPid','parentStartTicks','launcherPid','launcherStartTicks','nonceSha256'];

const HANDOFF_DENIAL_CLOSE_GRACE_MS=250;
const EVENT_INPUT_FIELDS=HANDOFF_EVENT_FIELDS.slice(1),HEX=/^[0-9a-f]{64}$/,UUID=/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/;
const check=(ok,code)=>{if(!ok)throw Error(code);};
const plain=value=>value!==null&&typeof value==='object'&&Object.getPrototypeOf(value)===Object.prototype;
function fields(value,code){check(plain(value),code);const descriptors=Object.getOwnPropertyDescriptors(value);check(Reflect.ownKeys(descriptors).length===Object.keys(descriptors).length&&Object.values(descriptors).every(d=>Object.hasOwn(d,'value')&&d.enumerable),code);return Object.keys(descriptors);}
function exact(value,keys,code){const actual=fields(value,code);check(actual.length===keys.length&&actual.every(key=>keys.includes(key)),code);}
const digest=value=>typeof value==='string'&&HEX.test(value),uuid=value=>typeof value==='string'&&UUID.test(value),nonnegativeBigInt=value=>typeof value==='bigint'&&value>=0n,positiveInteger=value=>Number.isSafeInteger(value)&&value>0,nonnegativeInteger=value=>Number.isSafeInteger(value)&&value>=0;
const identifier=value=>typeof value==='string'&&value.length>0&&!value.includes('\0')&&!value.includes('\n');
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');

export function generateNonce(randomBytesFn=cryptoRandomBytes){const nonce=randomBytesFn(HANDOFF_NONCE_BYTES);check(Buffer.isBuffer(nonce)&&nonce.length===HANDOFF_NONCE_BYTES,'HANDOFF_NONCE_LENGTH');return nonce;}
export function nonceHex(nonceBuffer){check(Buffer.isBuffer(nonceBuffer)&&nonceBuffer.length===HANDOFF_NONCE_BYTES,'HANDOFF_NONCE_LENGTH');return nonceBuffer.toString('hex');}
export function nonceSha256Hex(nonceBuffer){check(Buffer.isBuffer(nonceBuffer)&&nonceBuffer.length===HANDOFF_NONCE_BYTES,'HANDOFF_NONCE_LENGTH');return hash(nonceBuffer);}
export function constantTimeEqual(bufferA,bufferB){check(Buffer.isBuffer(bufferA)&&Buffer.isBuffer(bufferB),'HANDOFF_COMPARE_TYPE');/* Protocol nonce and digest lengths are fixed, so a length mismatch is not secret and may return early. */if(bufferA.length!==bufferB.length)return false;return timingSafeEqual(bufferA,bufferB);}
export function buildInvocationEnv(value){exact(value,['socketPath','nonceHex'],'HANDOFF_ENV_FIELDS');check(typeof value.socketPath==='string'&&value.socketPath.length>0&&isAbsolute(value.socketPath)&&resolve(value.socketPath)===value.socketPath&&!value.socketPath.includes('\0'),'HANDOFF_SOCKET_PATH');check(digest(value.nonceHex),'HANDOFF_NONCE_HEX');return Object.freeze({[HANDOFF_ENV_SOCKET]:value.socketPath,[HANDOFF_ENV_NONCE]:value.nonceHex});}
export async function createRuntimeDirectory({parentDir,mkdir=fsMkdir,chmod=fsChmod,randomHex=()=>cryptoRandomBytes(16).toString('hex')}){try{check(typeof parentDir==='string'&&parentDir.length>0&&isAbsolute(parentDir)&&resolve(parentDir)===parentDir&&!parentDir.includes('\0'),'HANDOFF_RUNTIME_DIR');check(typeof mkdir==='function'&&typeof chmod==='function'&&typeof randomHex==='function','HANDOFF_RUNTIME_DIR');const suffix=randomHex();check(typeof suffix==='string'&&/^[0-9a-f]{32}$/.test(suffix),'HANDOFF_RUNTIME_DIR');const runtimeDir=join(parentDir,'handoff-'+suffix);await mkdir(runtimeDir,{mode:0o700});await chmod(runtimeDir,0o700);return runtimeDir;}catch{throw Error('HANDOFF_RUNTIME_DIR');}}
export function socketPathFor(runtimeDir){check(typeof runtimeDir==='string'&&runtimeDir.length>0&&isAbsolute(runtimeDir)&&resolve(runtimeDir)===runtimeDir&&!runtimeDir.includes('\0'),'HANDOFF_SOCKET_PATH');return join(runtimeDir,HANDOFF_SOCKET_NAME);}
export async function createInvocationSocket({runtimeDir,net=nodeNet,chmod=fsChmod}){const socketPath=socketPathFor(runtimeDir);check(net&&typeof net.createServer==='function'&&typeof chmod==='function','HANDOFF_SOCKET_CREATE');const server=net.createServer();try{await new Promise((resolveListening,reject)=>{const failed=error=>{server.off('listening',ready);reject(error);},ready=()=>{server.off('error',failed);resolveListening();};server.once('error',failed);server.once('listening',ready);server.listen(socketPath);});await chmod(socketPath,0o600);return Object.freeze({server,socketPath});}catch(error){try{server.close();}catch{}throw error;}}

export function readProcStatReal(pid){check(positiveInteger(pid),'HANDOFF_ANCESTRY_FIELDS');let raw;try{raw=readFileSync(`/proc/${pid}/stat`,'utf8');}catch(error){if(error?.code==='ENOENT'||error?.code==='ESRCH')return null;throw error;}const close=raw.lastIndexOf(')');check(close>0,'HANDOFF_PROC_STAT');const tokens=raw.slice(close+1).trim().split(/\s+/);check(tokens.length>19,'HANDOFF_PROC_STAT');const parsedPid=Number(raw.slice(0,raw.indexOf(' '))),ppid=Number(tokens[1]);let startTicks;try{startTicks=BigInt(tokens[19]);}catch{throw Error('HANDOFF_PROC_STAT');}check(parsedPid===pid&&nonnegativeInteger(ppid)&&nonnegativeBigInt(startTicks),'HANDOFF_PROC_STAT');return Object.freeze({pid,ppid,startTicks});}
export function verifyAncestry({peerPid,launcherPid,launcherStartTicks,readProcStat=readProcStatReal,maxDepth=HANDOFF_ANCESTRY_MAX_DEPTH}){check(positiveInteger(peerPid)&&positiveInteger(launcherPid)&&nonnegativeBigInt(launcherStartTicks)&&typeof readProcStat==='function'&&positiveInteger(maxDepth),'HANDOFF_ANCESTRY_FIELDS');let currentPid=peerPid;for(let depth=0;depth<maxDepth;depth++){const stat=readProcStat(currentPid);if(stat===null)throw Error('HANDOFF_ANCESTRY_PID_GONE');check(stat&&stat.pid===currentPid&&nonnegativeInteger(stat.ppid)&&nonnegativeBigInt(stat.startTicks),'HANDOFF_ANCESTRY_FIELDS');if(currentPid===launcherPid){check(stat.startTicks===launcherStartTicks,'HANDOFF_ANCESTRY_START_TICKS_MISMATCH');return Object.freeze({ok:true,depth});}if(stat.ppid===currentPid)throw Error('HANDOFF_ANCESTRY_NOT_FOUND');currentPid=stat.ppid;}throw Error('HANDOFF_ANCESTRY_BOUND_EXCEEDED');}
export function verifyPeerUid({expectedUid,actualUid}){check(nonnegativeInteger(expectedUid)&&nonnegativeInteger(actualUid),'HANDOFF_PEER_UID_TYPE');return expectedUid===actualUid;}
export function verifyBootId({expected,actual}){check(uuid(expected)&&uuid(actual),'HANDOFF_BOOT_ID_TYPE');return expected===actual;}
export function classifyEofBeforeSetup(){return Object.freeze({code:'HANDOFF_EOF_BEFORE_SETUP',refuseStartup:true});}
export function classifyEofAfterStartup({retainedKnownMainFailure}){check(typeof retainedKnownMainFailure==='boolean','HANDOFF_EOF_FIELDS');return retainedKnownMainFailure?Object.freeze({code:'HANDOFF_EOF_AFTER_STARTUP_RETAINED',refuseStartup:false,parentLost:false}):Object.freeze({code:'HANDOFF_EOF_AFTER_STARTUP_UNKNOWN',refuseStartup:false,parentLost:true});}

function validatePayload(payload){exact(payload,HANDOFF_EVENT_FIELDS,'HANDOFF_EVENT_FIELDS');check(payload.schema===HANDOFF_EVENT_SCHEMA,'HANDOFF_EVENT_SCHEMA');for(const key of ['allocationId','actionId','chargeId','reservationId','storeIdentity'])check(identifier(payload[key]),'HANDOFF_EVENT_IDENTIFIER');for(const key of ['candidateHash','runnerDigest','executionContextSha256','projectionSha256','correspondenceSha256','authoritySha256','nonceSha256'])check(digest(payload[key]),'HANDOFF_EVENT_DIGEST');check(uuid(payload.bootId),'HANDOFF_EVENT_BOOT_ID');for(const key of ['outerStartMonotonic','outerDeadlineMonotonic','parentStartTicks','launcherStartTicks'])check(nonnegativeBigInt(payload[key]),'HANDOFF_EVENT_MONOTONIC');/* blockDeadlineUtc is milliseconds since the Unix epoch because the source spec leaves its wire unit open. */check(nonnegativeInteger(payload.blockDeadlineUtc),'HANDOFF_EVENT_BLOCK_DEADLINE');for(const key of ['parentPid','launcherPid'])check(positiveInteger(payload[key]),'HANDOFF_EVENT_PID');return payload;}
export function buildInvocationEventPayload(value){exact(value,EVENT_INPUT_FIELDS,'HANDOFF_EVENT_FIELDS');const payload={schema:HANDOFF_EVENT_SCHEMA};for(const key of EVENT_INPUT_FIELDS)payload[key]=value[key];validatePayload(payload);return Object.freeze(payload);}
/** Canonical wire bytes are ASCII: schema plus newline, then key=value newline in
 * HANDOFF_EVENT_FIELDS order; integer and BigInt values use canonical String().
 */
export function canonicalizeInvocationPayload(payload){validatePayload(payload);let text=HANDOFF_EVENT_SCHEMA+'\n';for(const key of EVENT_INPUT_FIELDS)text+=`${key}=${String(payload[key])}\n`;return Buffer.from(text,'ascii');}
// The requirement names the persisted digest but not its adjacent field name;
// this model chooses payloadSha256 wherever the digest accompanies the payload.
export function invocationPayloadSha256(payload){return hash(canonicalizeInvocationPayload(payload));}

function validateServerOptions(options){exact(options,['server','timeoutMs','expectedNonce','expectedAuthoritySha256','expectedBootId','expectedUid','invocationPayloadSha256','ancestry','readBootId','getPeerUid','getPeerPid','confirmCurrentState','buildResponse','recordDenialEvent'],'HANDOFF_SERVER_FIELDS');check(options.server&&options.server.listening===true&&typeof options.server.on==='function','HANDOFF_SERVER_FIELDS');check(positiveInteger(options.timeoutMs)&&Buffer.isBuffer(options.expectedNonce)&&options.expectedNonce.length===HANDOFF_NONCE_BYTES&&digest(options.expectedAuthoritySha256)&&uuid(options.expectedBootId)&&nonnegativeInteger(options.expectedUid)&&digest(options.invocationPayloadSha256),'HANDOFF_SERVER_FIELDS');exact(options.ancestry,['launcherPid','launcherStartTicks','readProcStat','maxDepth'],'HANDOFF_SERVER_FIELDS');check(positiveInteger(options.ancestry.launcherPid)&&nonnegativeBigInt(options.ancestry.launcherStartTicks)&&typeof options.ancestry.readProcStat==='function'&&positiveInteger(options.ancestry.maxDepth),'HANDOFF_SERVER_FIELDS');for(const key of ['readBootId','getPeerUid','getPeerPid','confirmCurrentState','buildResponse','recordDenialEvent'])check(typeof options[key]==='function','HANDOFF_SERVER_FIELDS');}
const denial=code=>Object.freeze({status:'HANDSHAKE_DENIED',code});
const denialEvent=(invocationPayloadSha256,code)=>Object.freeze({schema:HANDOFF_DENIAL_EVENT_SCHEMA,invocationPayloadSha256,code});
const jsonLine=value=>JSON.stringify(value,(_key,item)=>typeof item==='bigint'?String(item):item)+'\n';
function writeSocketSafely(socket,line,done=()=>{}){socket.on('error',()=>{});const finish=()=>{try{done();}catch{}};try{socket.write(line,finish);}catch{finish();}}
function denySocket(socket,code){if(socket&&!socket.destroyed)writeSocketSafely(socket,jsonLine({denied:code}),()=>{socket.end();const graceTimer=setTimeout(()=>{if(!socket.destroyed)socket.destroy();},HANDOFF_DENIAL_CLOSE_GRACE_MS);socket.once('close',()=>clearTimeout(graceTimer));});}
/** One-line JSON carries nonce/authority to the parent and payload/digest back.
 * recordDenialEvent must complete its durable append before resolving; rejection
 * fails closed without sending denial bytes. BigInt payload fields become decimal
 * JSON strings because JSON has no BigInt.
 * Call this only after the durable invocation event append has already succeeded;
 * a failed append must mean this function is never called, yielding no handshake.
 */
export async function runHandoffServer(options){
 validateServerOptions(options);
 return new Promise((resolveResult,rejectResult)=>{
  let settled=false,consumed=false,active=null;
  const recordAndDeny=async(socket,code)=>{await options.recordDenialEvent(denialEvent(options.invocationPayloadSha256,code));if(socket)denySocket(socket,code);};
  const finish=(value,socket=null)=>{if(settled)return;settled=true;clearTimeout(timer);if(value.status==='HANDSHAKE_DENIED'){options.server.off('connection',connection);void recordAndDeny(socket,value.code).then(()=>resolveResult(value),error=>{socket?.destroy();rejectResult(error);});return;}resolveResult(value);};
  const timer=setTimeout(()=>{const socket=active;finish(denial('HANDOFF_SETUP_TIMEOUT'));socket?.destroy();},options.timeoutMs);
  const refuseReplay=socket=>{let observed=false;const replay=()=>{if(observed)return;observed=true;clearTimeout(replayTimer);socket.removeListener('data',replay);socket.removeListener('end',replay);socket.removeListener('close',replay);void recordAndDeny(socket,'HANDOFF_REPLAY_DENIED').catch(()=>socket.destroy());};const replayTimer=setTimeout(replay,options.timeoutMs);socket.once('data',replay);socket.once('end',replay);socket.once('close',replay);};
  const connection=socket=>{
   if(consumed||active){refuseReplay(socket);return;}active=socket;let input='',lineReceived=false;
   const eof=()=>{if(!settled&&!input.includes('\n'))finish(denial(classifyEofBeforeSetup().code));};socket.once('end',eof);socket.once('close',eof);
   socket.on('data',chunk=>{
    if(settled||lineReceived)return;input+=chunk.toString('utf8');if(input.length>65536){finish(denial('HANDOFF_HANDSHAKE_MALFORMED'),socket);return;}const newline=input.indexOf('\n');if(newline<0)return;lineReceived=true;socket.removeListener('end',eof);socket.removeListener('close',eof);
    void (async()=>{
     let child;try{check(newline===input.length-1,'HANDOFF_HANDSHAKE_MALFORMED');child=JSON.parse(input.slice(0,newline));exact(child,['nonce','authoritySha256'],'HANDOFF_HANDSHAKE_MALFORMED');check(typeof child.nonce==='string'&&HEX.test(child.nonce)&&typeof child.authoritySha256==='string','HANDOFF_HANDSHAKE_MALFORMED');}catch{finish(denial('HANDOFF_HANDSHAKE_MALFORMED'),socket);return;}
     const receivedNonce=Buffer.from(child.nonce,'hex');if(!constantTimeEqual(receivedNonce,options.expectedNonce)){finish(denial('HANDOFF_NONCE_MISMATCH'),socket);return;}if(child.authoritySha256!==options.expectedAuthoritySha256){finish(denial('HANDOFF_AUTHORITY_MISMATCH'),socket);return;}
     let peerUid,peerPid,ancestryResult;try{const actualBootId=await options.readBootId();if(settled)return;if(!verifyBootId({expected:options.expectedBootId,actual:actualBootId})){finish(denial('HANDOFF_BOOT_ID_MISMATCH'),socket);return;}peerUid=await options.getPeerUid(socket);if(settled)return;if(!verifyPeerUid({expectedUid:options.expectedUid,actualUid:peerUid})){finish(denial('HANDOFF_PEER_UID_MISMATCH'),socket);return;}peerPid=await options.getPeerPid(socket);if(settled)return;ancestryResult=verifyAncestry({peerPid,...options.ancestry});await options.confirmCurrentState();if(settled)return;}catch(error){finish(denial(typeof error?.message==='string'?error.message:'HANDOFF_SERVER_ERROR'),socket);return;}
     let response;try{response=await options.buildResponse();exact(response,['payload','payloadSha256'],'HANDOFF_RESPONSE_FIELDS');check(digest(response.payloadSha256)&&response.payloadSha256===options.invocationPayloadSha256,'HANDOFF_RESPONSE_FIELDS');}catch(error){finish(denial(typeof error?.message==='string'?error.message:'HANDOFF_SERVER_ERROR'),socket);return;}
     consumed=true;settled=true;clearTimeout(timer);
     // The durable append and one-shot consumption commit before this response;
     // a peer-abandoned write must not retroactively turn success into denial.
     writeSocketSafely(socket,jsonLine(response));
     resolveResult(Object.freeze({status:'HANDSHAKE_OK',socket,peerPid,peerUid,ancestryDepth:ancestryResult.depth}));
    })();
   });
  };
  options.server.on('connection',connection);
 });
}

/** Node's net module does not expose getsockopt(SO_PEERCRED); callers must inject
 * real peer UID/PID retrieval and otherwise fail loudly at that platform boundary.
 */
export function peerCredentialsUnavailable(){throw Error('HANDOFF_PEER_CREDENTIALS_UNAVAILABLE');}
