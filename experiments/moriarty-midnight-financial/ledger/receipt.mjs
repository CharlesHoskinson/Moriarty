import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';

const hash = bytes => createHash('sha256').update(bytes).digest('hex');
function requireThat(ok, code) { if (!ok) throw new Error(code); }
function hex(value, label) {
  requireThat(typeof value === 'string' && /^[0-9a-f]{64}$/.test(value), `INVALID_${label}`);
  return value;
}
function amount(value, label) {
  requireThat(typeof value === 'bigint' && value >= 0n && value < 1n << 128n, `INVALID_${label}`);
  return value.toString();
}
function decimal(value, label) {
  requireThat(typeof value === 'string' && /^(0|[1-9][0-9]*)$/.test(value), `INVALID_${label}`);
  return amount(BigInt(value),label);
}
function sorted(value) { return value.map(item=>JSON.stringify(item)).sort(); }

/** Closed public projection of the pinned ledger Effects type. */
export function projectTranscriptEffects(effects) {
  const arrays=['claimedNullifiers','claimedShieldedReceives','claimedShieldedSpends','claimedContractCalls'];
  const maps=['shieldedMints','unshieldedMints','unshieldedInputs','unshieldedOutputs','claimedUnshieldedSpends'];
  requireThat(effects && Object.keys(effects).length===arrays.length+maps.length,'EFFECT_SHAPE');
  for(const key of arrays) requireThat(Array.isArray(effects[key]) && effects[key].length===0,'UNSUPPORTED_EFFECT_'+key);
  for(const key of maps) requireThat(effects[key] instanceof Map,'EFFECT_MAP_'+key);
  requireThat(effects.shieldedMints.size===0,'UNSUPPORTED_SHIELDED_MINT');
  function token(type) {requireThat(type?.tag==='unshielded','UNSUPPORTED_EFFECT_TOKEN');return hex(type.raw,'EFFECT_ASSET');}
  function quantities(map,key) {
    const result={};for(const [type,value] of map) {const id=key(type);requireThat(!(id in result),'DUPLICATE_EFFECT_ASSET');result[id]=amount(value,'EFFECT_AMOUNT');}return result;
  }
  return {
    unshieldedMints:quantities(effects.unshieldedMints,x=>hex(x,'MINT_DOMAIN')),
    unshieldedInputs:quantities(effects.unshieldedInputs,token),
    unshieldedOutputs:quantities(effects.unshieldedOutputs,token),
    claimedUnshieldedSpends:[...effects.claimedUnshieldedSpends].map(([key,value])=>{
      requireThat(Array.isArray(key)&&key.length===2&&['user','contract'].includes(key[1]?.tag),'EFFECT_PAYOUT_SHAPE');
      return {type:token(key[0]),recipientKind:key[1].tag,recipient:hex(key[1].address,'PAYOUT_ADDRESS'),amount:amount(value,'PAYOUT_AMOUNT')};
    }),
  };
}

/** Decode native public transaction bytes; this does not verify ZK proofs or ledger acceptance. */
export function decodeNativeFinancialTransaction(raw, ledger) {
  requireThat(raw instanceof Uint8Array && raw.length > 0 && raw.length <= 16*1024*1024,'TRANSACTION_BYTES');
  const tx=ledger.Transaction.deserialize('signature','proof','binding',raw);
  requireThat(Buffer.from(raw).equals(Buffer.from(tx.serialize())),'NONCANONICAL_TRANSACTION');
  requireThat(!tx.rewards && !tx.guaranteedOffer && (!tx.fallibleOffer || tx.fallibleOffer.size===0),'UNSUPPORTED_SHIELDED_OR_REWARD');
  requireThat(tx.intents instanceof Map && tx.intents.size > 0,'MISSING_INTENTS');
  const inputs=[], outputs=[], actions=[], grossByAsset={}; const spent=new Set();
  let dustFee=0n;
  for (const [segment,intent] of tx.intents) {
    requireThat(Number.isInteger(segment) && segment>=0 && segment<=65535,'INVALID_SEGMENT');
    const intentHash=hex(intent.intentHash(segment),'INTENT_HASH');
    const signatureData=intent.signatureData(segment);
    for (const [section,offer] of [['guaranteed',intent.guaranteedUnshieldedOffer],['fallible',intent.fallibleUnshieldedOffer]]) {
      if (!offer) continue;
      requireThat(offer.signatures.length===offer.inputs.length,'INPUT_SIGNATURE_COUNT');
      for (const [index,input] of offer.inputs.entries()) {
        const owner=hex(ledger.addressFromKey(input.owner),'INPUT_OWNER');
        requireThat(ledger.verifySignature(input.owner,signatureData,offer.signatures[index])===true,'MISSING_OR_INVALID_INPUT_SIGNATURE');
        const type=hex(input.type,'INPUT_ASSET'), value=amount(input.value,'INPUT_VALUE');
        const origin=hex(input.intentHash,'INPUT_ORIGIN');
        requireThat(Number.isSafeInteger(input.outputNo) && input.outputNo>=0,'INVALID_OUTPUT_NUMBER');
        const id=`${origin}:${input.outputNo}`; requireThat(!spent.has(id),'DUPLICATE_INPUT'); spent.add(id);
        inputs.push({segment,section,owner,type,value,intentHash:origin,outputNo:input.outputNo});
        grossByAsset[type]=(BigInt(grossByAsset[type]??'0')+input.value).toString();
      }
      offer.outputs.forEach((output,index)=>outputs.push({segment,section,owner:hex(output.owner,'OUTPUT_OWNER'),type:hex(output.type,'OUTPUT_ASSET'),value:amount(output.value,'OUTPUT_VALUE'),intentHash,offerIndex:index}));
    }
    for (const action of intent.actions) {
      if (action instanceof ledger.ContractCall) {
        const entryPoint=typeof action.entryPoint==='string'?action.entryPoint:Buffer.from(action.entryPoint).toString('hex');
        const transcripts=[];
        for(const [section,transcript] of [['guaranteed',action.guaranteedTranscript],['fallible',action.fallibleTranscript]]) {
          if(transcript) transcripts.push({section,effects:projectTranscriptEffects(transcript.effects)});
        }
        actions.push({segment,kind:'call',address:hex(action.address,'CONTRACT_ADDRESS'),entryPoint,transcripts});
      } else if (action instanceof ledger.ContractDeploy) {
        actions.push({segment,kind:'deploy',address:hex(action.address,'CONTRACT_ADDRESS')});
      } else throw new Error('UNSUPPORTED_CONTRACT_ACTION');
    }
    if (intent.dustActions) {
      requireThat(intent.dustActions.registrations.length===0,'UNSUPPORTED_DUST_REGISTRATION');
      for (const spend of intent.dustActions.spends) dustFee+=BigInt(amount(spend.vFee,'DUST_FEE'));
    }
  }
  return {schema:'moriarty.native-financial-transaction/1',rawSha256:hash(raw),transactionHash:tx.transactionHash(),identifiers:[...tx.identifiers()],inputs,outputs,actions,grossByAsset,dustFee:dustFee.toString(),proofVerified:false,ledgerAccepted:false};
}

/** A shared absolute deadline; a timed-out observation is unknown, never successful. */
export async function beforeDeadline(operation, deadlineMs) {
  requireThat(Number.isSafeInteger(deadlineMs) && deadlineMs>Date.now(),'DEADLINE_EXPIRED');
  let timer;
  try {
    return await Promise.race([Promise.resolve().then(operation),new Promise((_,reject)=>{
      timer=setTimeout(()=>reject(new Error('OBSERVATION_TIMEOUT_UNKNOWN')),deadlineMs-Date.now());
    })]);
  } finally { clearTimeout(timer); }
}

function indexedRows(rows) {
  requireThat(Array.isArray(rows),'MISSING_INDEXED_UTXOS');
  return rows.map(x=>({owner:hex(x.owner,'INDEXED_OWNER'),type:hex(x.tokenType,'INDEXED_ASSET'),value:amount(x.value,'INDEXED_VALUE'),intentHash:hex(x.intentHash,'INDEXED_INTENT')}));
}
function nativeRows(rows) {
  return rows.map(({owner,type,value,intentHash})=>({owner,type,value,intentHash}));
}

/**
 * Read one confirmed stage through pinned SDK/RPC APIs. Observations come from
 * finalized bytes and block-pinned state, never from an expected financial fixture.
 * The RPC/indexer remain explicit external trust dependencies.
 */
export async function observeFinalizedStage({provider,rpc,ledger,txId,contractAddress,circuitId,decodeState,deadlineMs,expectedProtocolVersion}) {
  hex(contractAddress,'CONTRACT_ADDRESS');
  requireThat(Number.isSafeInteger(expectedProtocolVersion) && expectedProtocolVersion>=0,'EXPECTED_PROTOCOL_VERSION_REQUIRED');
  requireThat(typeof txId==='string' && txId.length>0,'TX_ID');
  const wait=op=>beforeDeadline(op,deadlineMs);
  const data=await wait(()=>provider.watchForTxData(txId));
  requireThat(data.status==='SucceedEntirely','TRANSACTION_STATUS');
  requireThat(data.tx instanceof ledger.Transaction,'NATIVE_TRANSACTION_REQUIRED');
  const decoded=decodeNativeFinancialTransaction(data.tx.serialize(),ledger);
  requireThat(decoded.identifiers.includes(txId) && data.txId===txId,'TRANSACTION_ID_MISMATCH');
  requireThat(isDeepStrictEqual([...data.identifiers].sort(),[...decoded.identifiers].sort()),'IDENTIFIERS_MISMATCH');
  requireThat(decoded.transactionHash===data.txHash,'TRANSACTION_HASH_MISMATCH');
  requireThat(decoded.actions.length===1,'CONTRACT_ACTION_COUNT');
  const action=decoded.actions[0];
  requireThat(action.address===contractAddress && (circuitId==='deploy'?action.kind==='deploy':action.kind==='call' && action.entryPoint===circuitId),'CONTRACT_ACTION_MISMATCH');
  requireThat(data.segmentStatusMap===undefined || [...data.segmentStatusMap.values()].every(s=>s==='SegmentSuccess'),'SEGMENT_FAILURE');
  requireThat(Number.isSafeInteger(data.blockHeight) && data.blockHeight>=0,'BLOCK_HEIGHT');
  const blockHash=hex(data.blockHash,'BLOCK_HASH');
  const finalizedHead=await wait(()=>rpc('chain_getFinalizedHead',[]));
  const header=await wait(()=>rpc('chain_getHeader',[finalizedHead]));
  requireThat(typeof header?.number==='string' && /^0x[0-9a-f]+$/i.test(header.number),'FINALIZED_HEADER');
  const finalizedHeight=Number.parseInt(header.number,16);
  requireThat(Number.isSafeInteger(finalizedHeight) && finalizedHeight>=data.blockHeight,'NOT_FINALIZED');
  const canonical=await wait(()=>rpc('chain_getBlockHash',[data.blockHeight]));
  requireThat(typeof canonical==='string' && canonical.replace(/^0x/,'')===blockHash,'NONCANONICAL_BLOCK');
  requireThat(Number.isInteger(data.protocolVersion) && data.protocolVersion===expectedProtocolVersion,'UNSUPPORTED_PROTOCOL');
  const paidFees=decimal(data.fees?.paidFees,'PAID_FEES');
  const estimatedFees=decimal(data.fees?.estimatedFees,'ESTIMATED_FEES');
  // Native vFee includes the wallet-selected fee resource debit. The indexer
  // strings have no verified encoding/overhead relationship to that debit.
  // Historical public replay demonstrates unequal values for a successful call.
  // Retain both; authorization charges the full native amount without refunds.
  requireThat(isDeepStrictEqual(sorted(indexedRows(data.unshielded?.spent)),sorted(nativeRows(decoded.inputs))),'INDEXED_INPUTS_MISMATCH');
  requireThat(isDeepStrictEqual(sorted(indexedRows(data.unshielded?.created)),sorted(nativeRows(decoded.outputs))),'INDEXED_OUTPUTS_MISMATCH');
  const config={type:'blockHash',blockHash};
  const state=await wait(()=>provider.queryContractState(contractAddress,config));
  requireThat(state!==null && state!==undefined,'MISSING_CONTRACT_STATE');
  const balances=await wait(()=>provider.queryUnshieldedBalances(contractAddress,config));
  requireThat(Array.isArray(balances),'MISSING_CONTRACT_BALANCES');
  const contractBalances={};
  for (const balance of balances) {
    const color=hex(balance.tokenType,'BALANCE_ASSET');
    requireThat(!(color in contractBalances),'DUPLICATE_BALANCE_ASSET');
    contractBalances[color]=amount(balance.balance,'BALANCE_AMOUNT');
  }
  // Keep decoded contract state separate and private to the comparator. No
  // uncontrolled caller object is spread into the publishable receipt.
  return {receipt:{schema:'moriarty.finalized-financial-stage/1',txId,contractAddress,circuitId,transaction:decoded,blockHash,blockHeight:data.blockHeight,finalizedHead,finalizedHeight,protocolVersion:data.protocolVersion,indexerIdentifiers:[...data.identifiers],fees:{nativeDebit:{asset:'DUST',unit:'SPECK',amount:decoded.dustFee},indexerReported:{paid:paidFees,estimated:estimatedFees,sourceUnitLabel:'DUST',encoding:'unresolved',nativeDebitRelationship:'unresolved'}},contractBalances,acceptance:'uncertified-I2-observation'},state:decodeState(state.data)};
}
