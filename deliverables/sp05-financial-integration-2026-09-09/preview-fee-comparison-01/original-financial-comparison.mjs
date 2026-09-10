import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';

const EXPECTATION_HASH='693a8c60972050b6afbece5f2106682ed9966538eb60a21beaa9218b043df35f';
const RUNTIME_ROOT=PINNED_NM+'/@midnight-ntwrk/compact-runtime';
const RUNTIME_PACKAGE_SHA256='ac4f818510afca0d17758b4c38af613f7b1a489aec96633548d0d361683f124c';
const RUNTIME_ENTRY_SHA256='c55a8ab3e7533b3fa6e27b66ab742c783d912d5a00d6ecc23d9f6142ddb0ecde';
function checkRuntimePins(readFile){
 const metadata=readFile(RUNTIME_ROOT+'/package.json');
 check(createHash('sha256').update(metadata).digest('hex')===RUNTIME_PACKAGE_SHA256,'COMPACT_RUNTIME_PACKAGE_PIN');
 check(JSON.parse(metadata).version==='0.16.0','COMPACT_RUNTIME_VERSION');
 check(createHash('sha256').update(readFile(RUNTIME_ROOT+'/dist/index.js')).digest('hex')===RUNTIME_ENTRY_SHA256,'COMPACT_RUNTIME_ENTRY_PIN');
}
// Closed source-test inspection: shares the exact production byte predicates,
// but cannot import a runtime or return an executable financial comparator.
export function inspectFinancialRuntimePinsForSourceTest({sourceTestOnly,readFile}={}){
 check(sourceTestOnly===true&&typeof readFile==='function','SOURCE_TEST_ONLY_REQUIRED');
 checkRuntimePins(readFile);
 return {status:'SOURCE_TEST_ONLY',pinChecksPassed:true,runtimeImported:false,networkAcceptance:false,proofAcceptance:false};
}
const MAX=(1n<<128n)-1n;
const check=(condition,code)=>{if(!condition)throw Error(code);};
const hex=(x,label)=>{check(typeof x==='string'&&/^[0-9a-f]{64}$/.test(x),'HEX_'+label);return x;};
const bytes=x=>Uint8Array.from(Buffer.from(x,'hex'));
const pad=x=>Uint8Array.from(Buffer.from(x.padEnd(32,'\0')));
function record(x,label){check(x!==null&&typeof x==='object'&&!Array.isArray(x)&&!(x instanceof Uint8Array),'RECORD_'+label);}
function keys(x,wanted,label){record(x,label);check(Object.keys(x).sort().join('|')===[...wanted].sort().join('|'),'FIELDS_'+label);}
function decimal(x,label){check(typeof x==='string'&&/^(0|[1-9][0-9]*)$/.test(x),'DECIMAL_'+label);const n=BigInt(x);check(n<=MAX,'RANGE_'+label);return n;}
function sum(map,key,n){const total=(map[key]??0n)+n;check(total>=0n&&total<=MAX,'AGGREGATE_OVERFLOW');map[key]=total;}
function byteHex(x,label){check(x instanceof Uint8Array&&x.length===32,'BYTES_'+label);return Buffer.from(x).toString('hex');}
function compareNumeric(actual,expected,path){
 if(typeof expected==='string'){check(typeof actual==='bigint'&&actual>=0n&&actual<=MAX,'DECODED_INTEGER_'+path);check(actual.toString()===expected,'VALUE_'+path);return;}
 keys(actual,Object.keys(expected),path);
 for(const key of Object.keys(expected))compareNumeric(actual[key],expected[key],path+'.'+key);
}
function amounts(actual,allowed,label){record(actual,label);const out={};for(const [key,value] of Object.entries(actual)){hex(key,label);check(allowed.has(key),'UNEXPECTED_ASSET_'+label);out[key]=decimal(value,label);}return out;}
function equalAmounts(actual,expected,label){for(const key of new Set([...Object.keys(actual),...Object.keys(expected)]))check((actual[key]??0n)===(expected[key]??0n),'AMOUNT_'+label);}
const strings=map=>Object.fromEntries(Object.entries(map).map(([k,v])=>[k,v.toString()]));
function publicBindings(kind,roles,networkTag,programDigest,runtime){
 keys(roles,['firstSecret','secondSecret','firstAddress','secondAddress'],'ROLES');hex(networkTag,'NETWORK');
 const names=kind==='loan'?['borrower','lender']:['trader','provider'];const addresses={},capabilities={};
 check(roles.firstAddress!==roles.secondAddress,'DISTINCT_FINANCIAL_PARTICIPANTS_REQUIRED');
 for(const [i,name] of names.entries()){
  const address=hex(i?roles.secondAddress:roles.firstAddress,'ROLE_ADDRESS');check(address!=='0'.repeat(64),'ZERO_ROLE_ADDRESS');
  // These retained local-test identities are never real deployment evidence.
  check(![1,2,3,4].some(n=>address===`b00${n}`+`0${n}`.repeat(30)),'SYNTHETIC_ROLE_ADDRESS');
  const secret=i?roles.secondSecret:roles.firstSecret;byteHex(secret,'ROLE_SECRET');
  addresses[name]=address;
  capabilities[name]=byteHex(runtime.persistentHash(new runtime.CompactTypeVector(4,new runtime.CompactTypeBytes(32)),[pad(`moriarty:sp05:${kind}:${name}`),bytes(networkTag),bytes(programDigest),secret]),'CAPABILITY');
 }
 return {addresses,capabilities,networkTag,programDigest};
}

/**
 * Compare a fixed trace against immutable independent expectations. Inputs must
 * come from observeFinalizedStage; this comparison does not authenticate RPC,
 * replay proofs, establish ledger acceptance, or turn synthetic data into such
 * evidence. No role secret or expected financial value is emitted as observation.
 */
export async function createFinancialComparator({kind,roles,networkTag,expectedProtocolVersion}){
 check(kind==='loan'||kind==='swap','UNKNOWN_FINANCIAL_CASE');
 check(Number.isSafeInteger(expectedProtocolVersion)&&expectedProtocolVersion>=0,'EXPECTED_PROTOCOL_VERSION_REQUIRED');
 const raw=readFileSync(new URL('./financial-expectations.json',import.meta.url));
 check(createHash('sha256').update(raw).digest('hex')===EXPECTATION_HASH,'EXPECTATION_PIN');
 const expected=JSON.parse(raw).cases[kind];
 checkRuntimePins(readFileSync);
 const runtime=await import(pathToFileURL(RUNTIME_ROOT+'/dist/index.js').href);
 const bindings=publicBindings(kind,roles,networkTag,expected.bindingParameters.programDigest,runtime);
 roles=undefined; // The returned closure retains public commitments, not preimages.
 const roleNames=Object.keys(bindings.addresses),assets=expected.bindingParameters.assets;
 let position=0,stopped=false,contractAddress,colors,previousBalances={},previousHeight=-1;
 const txIds=new Set(),txHashes=new Set(),rawHashes=new Set(),identifiers=new Set(),spentInputs=new Set(),summaries=[];

 function compareState(s,e,deploy){
  const domainFields=kind==='loan'?{USD_TEST_ASSET:'usdDomain'}:{ASSET_A:'assetADomain',ASSET_B:'assetBDomain'};
  const wanted=['programDigest','networkTag','initialized','remaining','revision','kernelState',...Object.keys(e.lastResults),...roleNames.flatMap(r=>[r+'Address',r+'Capability']),...Object.values(domainFields),...Object.values(assets).map(a=>a.ledgerColorField)];
  keys(s,wanted,'PUBLIC_STATE');
  check(byteHex(s.programDigest,'PROGRAM')===bindings.programDigest,'PROGRAM_BINDING');
  check(byteHex(s.networkTag,'NETWORK')===bindings.networkTag,'NETWORK_BINDING');
  for(const role of roleNames){keys(s[role+'Address'],['bytes'],'ROLE_ADDRESS');check(byteHex(s[role+'Address'].bytes,'ROLE_ADDRESS')===bindings.addresses[role],'ADDRESS_BINDING');check(byteHex(s[role+'Capability'],'CAPABILITY')===bindings.capabilities[role],'CAPABILITY_BINDING');}
  for(const [asset,definition] of Object.entries(assets)){
   check(byteHex(s[domainFields[asset]],'DOMAIN')===definition.domainHex,'DOMAIN_BINDING');
   check(byteHex(s[definition.ledgerColorField],'COLOR')===(deploy?'0'.repeat(64):colors[asset]),'COLOR_BINDING');
  }
  check(typeof s.initialized==='boolean'&&s.initialized===e.initialized,'INITIALIZED');
  compareNumeric(s.remaining,e.remaining,'remaining');compareNumeric(s.revision,e.revision,'revision');compareNumeric(s.kernelState,e.kernelState,'kernelState');
  for(const [key,value] of Object.entries(e.lastResults))compareNumeric(s[key],value,key);
 }
 // Publish only checked, observed public fields. No expected amount is read here.
 function projectPublicState(s){
  const projectKernel=k=>Object.fromEntries(Array.from({length:kind==='loan'?9:7},(_,i)=>['f'+i,k['f'+i].toString()]));
  function projectResult(r,widths){
   const value={after:projectKernel(r.after),remaining:r.remaining.toString(),revision:r.revision.toString()};
   widths.forEach((width,i)=>{value['effect'+i]=Object.fromEntries(Array.from({length:width},(_,j)=>['v'+j,r['effect'+i]['v'+j].toString()]));});
   return value;
  }
  const value={programDigest:byteHex(s.programDigest,'PROGRAM'),networkTag:byteHex(s.networkTag,'NETWORK'),initialized:s.initialized,remaining:s.remaining.toString(),revision:s.revision.toString(),kernelState:projectKernel(s.kernelState)};
  for(const role of roleNames){value[role+'Address']={bytes:byteHex(s[role+'Address'].bytes,'ROLE_ADDRESS')};value[role+'Capability']=byteHex(s[role+'Capability'],'CAPABILITY');}
  if(kind==='loan'){
   value.usdDomain=byteHex(s.usdDomain,'DOMAIN');value.usdColor=byteHex(s.usdColor,'COLOR');
   value.lastAccrue=projectResult(s.lastAccrue,[5,5]);value.lastSettle=projectResult(s.lastSettle,[4,6,6]);
  }else{
   for(const field of ['assetADomain','assetBDomain','colorA','colorB'])value[field]=byteHex(s[field],field);
   value.lastSwap=projectResult(s.lastSwap,[4,4]);value.lastClose=projectResult(s.lastClose,[4,4]);
  }
  return value;
 }
 function projectNativeEffects(native){
  return {unshieldedMints:strings(native.unshieldedMints),unshieldedInputs:strings(native.unshieldedInputs),unshieldedOutputs:strings(native.unshieldedOutputs),claimedUnshieldedSpends:Object.entries(native.claims).map(([key,amount])=>{const [type,recipientKind,recipient]=key.split('|');return {type,recipientKind,recipient,amount:amount.toString()};})};
 }
 function nativeEffects(action,e){
  const colorSet=new Set(Object.values(colors)),domainSet=new Set(Object.values(assets).map(a=>a.domainHex));
  const total={unshieldedMints:{},unshieldedInputs:{},unshieldedOutputs:{},claims:{}};
  if(action.kind==='call'){
   check(Array.isArray(action.transcripts)&&action.transcripts.length>0&&action.transcripts.length<=2,'TRANSCRIPTS');const seen=new Set();
   for(const transcript of action.transcripts){
    keys(transcript,['section','effects'],'TRANSCRIPT');check(['guaranteed','fallible'].includes(transcript.section)&&!seen.has(transcript.section),'TRANSCRIPT_SECTION');seen.add(transcript.section);
    const f=transcript.effects;keys(f,['unshieldedMints','unshieldedInputs','unshieldedOutputs','claimedUnshieldedSpends'],'NATIVE_EFFECTS');
    for(const field of ['unshieldedMints','unshieldedInputs','unshieldedOutputs'])for(const [asset,n] of Object.entries(amounts(f[field],field==='unshieldedMints'?domainSet:colorSet,field)))sum(total[field],asset,n);
    check(Array.isArray(f.claimedUnshieldedSpends)&&f.claimedUnshieldedSpends.length<=128,'PAYOUTS');
    for(const p of f.claimedUnshieldedSpends){keys(p,['type','recipientKind','recipient','amount'],'PAYOUT');check(colorSet.has(hex(p.type,'PAYOUT_COLOR')),'PAYOUT_ASSET');check(['user','contract'].includes(p.recipientKind),'PAYOUT_KIND');hex(p.recipient,'PAYOUT_RECIPIENT');const payoutAmount=decimal(p.amount,'PAYOUT');check(payoutAmount>0n,'ZERO_PAYOUT');sum(total.claims,`${p.type}|${p.recipientKind}|${p.recipient}`,payoutAmount);}
   }
  }
  for(const field of ['unshieldedMints','unshieldedInputs','unshieldedOutputs']){
   const want=Object.fromEntries(Object.entries(e.nativeEffects[field]).map(([asset,n])=>[field==='unshieldedMints'?assets[asset].domainHex:colors[asset],BigInt(n)]));equalAmounts(total[field],want,field);
  }
  const wantClaims={};for(const p of e.nativeEffects.claimedUnshieldedSpends){const recipient=p.recipientKind==='contract'?contractAddress:bindings.addresses[p.recipientRole];check(recipient!==undefined,'EXPECTED_ROLE');sum(wantClaims,`${colors[p.asset]}|${p.recipientKind}|${recipient}`,BigInt(p.amount));}equalAmounts(total.claims,wantClaims,'PAYOUTS');
  return total;
 }
 function financialIo(tx,e,currentBalances,native){
  check(Array.isArray(tx.inputs)&&Array.isArray(tx.outputs)&&tx.inputs.length<=4096&&tx.outputs.length<=4096,'UTXO_COLLECTIONS');
  const colorSet=new Set(Object.values(colors)),ownerSet=new Set(Object.values(bindings.addresses));const gross={},out={},deltas={},newInputs=[],outputIds=new Set();
  for(const [direction,rows] of [['input',tx.inputs],['output',tx.outputs]])for(const row of rows){
   keys(row,direction==='input'?['segment','section','owner','type','value','intentHash','outputNo']:['segment','section','owner','type','value','intentHash','offerIndex'],'UTXO_'+direction);
   check(Number.isInteger(row.segment)&&row.segment>=0&&row.segment<=65535&&['guaranteed','fallible'].includes(row.section),'UTXO_SEGMENT');
   check(ownerSet.has(hex(row.owner,'UTXO_OWNER')),'UNAUTHORIZED_UTXO_OWNER');check(colorSet.has(hex(row.type,'UTXO_ASSET')),'UNEXPECTED_UTXO_ASSET');hex(row.intentHash,'UTXO_INTENT');
   const n=decimal(row.value,'UTXO_VALUE');check(n>0n,'ZERO_UTXO');
   const index=direction==='input'?row.outputNo:row.offerIndex;check(Number.isSafeInteger(index)&&index>=0,'UTXO_INDEX');
   if(direction==='input'){
    check(e.authorizedPayerRole!==null&&row.owner===bindings.addresses[e.authorizedPayerRole],'UNAUTHORIZED_PAYER');
    const id=`${row.intentHash}:${index}`;check(!spentInputs.has(id)&&!newInputs.includes(id),'REPLAYED_INPUT');newInputs.push(id);sum(gross,row.type,n);
   }else{const id=`${row.segment}:${row.section}:${index}`;check(!outputIds.has(id),'DUPLICATE_OUTPUT');outputIds.add(id);sum(out,row.type,n);}
   const key=`${row.owner}|${row.type}`;deltas[key]=(deltas[key]??0n)+(direction==='input'?-n:n);
  }
  equalAmounts(amounts(tx.grossByAsset,colorSet,'GROSS'),gross,'GROSS');
  for(const [role,values] of Object.entries(e.participantNetDeltas))for(const [asset,n] of Object.entries(values))check((deltas[`${bindings.addresses[role]}|${colors[asset]}`]??0n)===BigInt(n),'PARTICIPANT_NET_DELTA');
  for(const [asset,color] of Object.entries(colors)){
   const delta=(currentBalances[color]??0n)-(previousBalances[color]??0n);
   const mint=native.unshieldedMints[assets[asset].domainHex]??0n;
   check((out[color]??0n)-(gross[color]??0n)+delta===mint,'ASSET_CONSERVATION');
   // Native mint-to-self already contributes to unshieldedInputs.
   check(delta===(native.unshieldedInputs[color]??0n)-(native.unshieldedOutputs[color]??0n),'CONTRACT_RESERVE_CONSERVATION');
  }
  return {gross,newInputs,participantNetDeltas:Object.fromEntries(roleNames.map(role=>[role,Object.fromEntries(Object.entries(colors).map(([asset,color])=>[asset,(deltas[`${bindings.addresses[role]}|${color}`]??0n).toString()]))]))};
 }
 function verifyStage(stageName,observation){
  check(!stopped,'COMPARISON_STOPPED');
  try{
   check(position<expected.stageOrder.length&&stageName===expected.stageOrder[position],'STAGE_ORDER');keys(observation,['receipt','state'],'OBSERVATION');
   const r=observation.receipt,e=expected.stages[stageName],deploy=stageName==='deploy';
   keys(r,['schema','txId','contractAddress','circuitId','transaction','blockHash','blockHeight','finalizedHead','finalizedHeight','protocolVersion','indexerIdentifiers','fees','contractBalances','acceptance'],'RECEIPT');
   check(r.schema==='moriarty.finalized-financial-stage/1'&&r.acceptance==='uncertified-I2-observation','RECEIPT_SCOPE');
   hex(r.contractAddress,'CONTRACT_ADDRESS');check(r.contractAddress!=='0'.repeat(64)&&!Object.values(bindings.addresses).includes(r.contractAddress),'CONTRACT_ADDRESS');
   if(deploy){contractAddress=r.contractAddress;colors=Object.fromEntries(Object.entries(assets).map(([asset,definition])=>[asset,hex(runtime.rawTokenType(bytes(definition.domainHex),contractAddress),'DERIVED_COLOR')]));check(new Set(Object.values(colors)).size===Object.keys(colors).length,'COLOR_COLLISION');}
   check(r.contractAddress===contractAddress&&r.circuitId===stageName,'CONTRACT_STAGE_BINDING');
   check(typeof r.txId==='string'&&r.txId.length>0&&!txIds.has(r.txId),'TX_ID_REPLAY');hex(r.blockHash,'BLOCK_HASH');
   check(Number.isSafeInteger(r.blockHeight)&&r.blockHeight>=previousHeight&&Number.isSafeInteger(r.finalizedHeight)&&r.finalizedHeight>=r.blockHeight,'BLOCK_FINALITY');
   check(typeof r.finalizedHead==='string'&&/^0x[0-9a-f]{64}$/.test(r.finalizedHead),'FINALIZED_HEAD');check(r.protocolVersion===expectedProtocolVersion,'PROTOCOL_VERSION');
   const tx=r.transaction;keys(tx,['schema','rawSha256','transactionHash','identifiers','inputs','outputs','actions','grossByAsset','dustFee','proofVerified','ledgerAccepted'],'TRANSACTION');
   check(tx.schema==='moriarty.native-financial-transaction/1'&&tx.proofVerified===false&&tx.ledgerAccepted===false,'TRANSACTION_SCOPE');
   hex(tx.rawSha256,'RAW_HASH');hex(tx.transactionHash,'TX_HASH');check(!rawHashes.has(tx.rawSha256)&&!txHashes.has(tx.transactionHash),'TRANSACTION_REPLAY');
   check(Array.isArray(tx.identifiers)&&tx.identifiers.length>0&&tx.identifiers.length<=128&&tx.identifiers.includes(r.txId)&&new Set(tx.identifiers).size===tx.identifiers.length,'IDENTIFIERS');
   check(Array.isArray(r.indexerIdentifiers)&&r.indexerIdentifiers.length===tx.identifiers.length&&new Set(r.indexerIdentifiers).size===r.indexerIdentifiers.length&&r.indexerIdentifiers.every(id=>typeof id==='string'&&tx.identifiers.includes(id)),'INDEXER_IDENTIFIERS');
   for(const id of tx.identifiers){check(typeof id==='string'&&id.length>0&&!identifiers.has(id),'IDENTIFIER_REPLAY');}
   check(Array.isArray(tx.actions)&&tx.actions.length===1,'ACTION_COUNT');const a=tx.actions[0];keys(a,deploy?['segment','kind','address']:['segment','kind','address','entryPoint','transcripts'],'ACTION');
   check(Number.isInteger(a.segment)&&a.segment>=0&&a.segment<=65535&&a.address===contractAddress&&a.kind===(deploy?'deploy':'call')&&(deploy||a.entryPoint===stageName),'ACTION_BINDING');
   keys(r.fees,['nativeDebit','indexerReported'],'FEES');keys(r.fees.nativeDebit,['asset','unit','amount'],'NATIVE_FEE');check(r.fees.nativeDebit.asset==='DUST'&&r.fees.nativeDebit.unit==='SPECK','NATIVE_FEE_UNIT');
   check(decimal(r.fees.nativeDebit.amount,'NATIVE_FEE')===decimal(tx.dustFee,'TX_DUST_FEE'),'NATIVE_FEE_MISMATCH');
   keys(r.fees.indexerReported,['paid','estimated','sourceUnitLabel','encoding','nativeDebitRelationship'],'INDEXER_FEES');const indexer=r.fees.indexerReported;
   decimal(indexer.paid,'INDEXER_PAID');decimal(indexer.estimated,'INDEXER_ESTIMATED');check(indexer.sourceUnitLabel==='DUST'&&indexer.encoding==='unresolved'&&indexer.nativeDebitRelationship==='unresolved','INDEXER_FEE_ENCODING');
   compareState(observation.state,e,deploy);
   const actualBalances=amounts(r.contractBalances,new Set(Object.values(colors)),'CONTRACT_BALANCES');
   equalAmounts(actualBalances,Object.fromEntries(Object.entries(e.contractReserves).map(([asset,n])=>[colors[asset],BigInt(n)])),'CONTRACT_BALANCES');
   const native=nativeEffects(a,e),io=financialIo(tx,e,actualBalances,native);
   const summary={status:'PASS',kind,stage:stageName,publicState:projectPublicState(observation.state),nativeEffects:projectNativeEffects(native),txId:r.txId,contractAddress,blockHash:r.blockHash,blockHeight:r.blockHeight,grossByAsset:strings(io.gross),participantNetDeltas:io.participantNetDeltas,contractBalances:strings(actualBalances),nativeFee:structuredClone(r.fees.nativeDebit),indexerFees:structuredClone(indexer),scope:'Exact fixed financial trace comparison over supplied finalized observations; no proof or network acceptance',networkAcceptance:false,proofAcceptance:false};
   txIds.add(r.txId);txHashes.add(tx.transactionHash);rawHashes.add(tx.rawSha256);tx.identifiers.forEach(x=>identifiers.add(x));io.newInputs.forEach(x=>spentInputs.add(x));previousBalances=actualBalances;previousHeight=r.blockHeight;position++;summaries.push(summary);
   return structuredClone(summary);
  }catch(error){stopped=true;throw error;}
 }
 function finish(){check(!stopped,'COMPARISON_STOPPED');check(position===expected.stageOrder.length,'INCOMPLETE_FINANCIAL_TRACE');return {status:'PASS',kind,contractAddress,expectationsSha256:EXPECTATION_HASH,stages:structuredClone(summaries),scope:'All four stages match independent fixed loan/swap expectations; external observer, indexer and RPC trust remain; no PCD or proof acceptance',networkAcceptance:false,proofAcceptance:false};}
 return Object.freeze({verifyStage,finish});
}
