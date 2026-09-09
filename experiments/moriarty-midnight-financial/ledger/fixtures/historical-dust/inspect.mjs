import fs from 'node:fs';
import {createHash} from 'node:crypto';
import * as ledger from '/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/midnight_ledger_wasm_fs.js';
import {decodeNativeFinancialTransaction} from '/home/charl/Moriarty/.worktrees/sp05-financial-integration/experiments/moriarty-midnight-financial/ledger/receipt.mjs';
const d='/home/charl/.local/state/moriarty/sp05-public-dust-fixture';
const data=JSON.parse(fs.readFileSync(d+'/response.json')).data.transactions[0];
const raw=Buffer.from(data.raw.replace(/^0x/,''),'hex');fs.writeFileSync(d+'/transaction.bin',raw,{flag:'wx'});
const tx=ledger.Transaction.deserialize('signature','proof','binding',raw);
const dust=[];for(const [segment,intent] of tx.intents??[]) { for(const s of intent.dustActions?.spends??[])dust.push({segment,nativeDustSpend:s instanceof ledger.DustSpend,vFee:s.vFee.toString(),oldNullifier:s.oldNullifier}); }
const fee=dust.reduce((n,s)=>n+BigInt(s.vFee),0n);
let decoder;try{decoder={status:'PASS',result:decodeNativeFinancialTransaction(raw,ledger)};}catch(e){decoder={status:'REJECTED',error:e.message};}
const result={scope:'Historical public hello-world call; no new transaction or proof verification',ledgerPackageVersion:JSON.parse(fs.readFileSync('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/package.json')).version,protocolVersion:data.protocolVersion,rawBytes:raw.length,rawSha256:createHash('sha256').update(raw).digest('hex'),canonical:Buffer.from(tx.serialize()).equals(raw),nativeTransactionHash:tx.transactionHash(),indexerHash:data.hash,identifiers:tx.identifiers(),block:data.block,status:data.transactionResult,fees:data.fees,dustSpends:dust,dustVFeeSum:fee.toString(),paidFeeEqualsVFee:fee===BigInt(data.fees.paidFees),decoder};
fs.writeFileSync(d+'/native-observation.json',JSON.stringify(result,null,2)+'\n',{flag:'wx'});console.log(JSON.stringify(result,null,2));
