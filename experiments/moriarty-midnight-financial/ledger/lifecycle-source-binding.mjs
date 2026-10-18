/** Offline source/Core binding for the new lifecycle wrapper. No compiled claim. */
import {createHash} from 'node:crypto';
import pins from './lifecycle-source-pins.json' with {type:'json'};
import {createFinancialAgreementSourceV6} from '../../moriarty-language/src/successor/financial-agreement-source-v6.ts';
import {canonical} from '../../moriarty-language/src/successor/expression-wire-v1.ts';
const sha=s=>createHash('sha256').update(s).digest('hex');
const requireThat=(v,c)=>{if(!v)throw Error(c);};
export function lowerLifecycleSourceBinding(source) {
 // This is one selected source, not a general-purpose financial lowerer.
 requireThat(typeof source==='string'&&sha(source)===pins.sourceSha256,'LIFECYCLE_SOURCE_PIN');
 const elaborated=createFinancialAgreementSourceV6().elaborate(source);
 requireThat(elaborated.judgmentResult==='SourceElaborated','LIFECYCLE_SOURCE_REJECTED');
 const action=elaborated.actions.find(a=>a.action==='originate');
 requireThat(action,'LIFECYCLE_ORIGINATE_REQUIRED');
 const emits=action.core.statements.filter(s=>s.constructor==='Emit');
 requireThat(emits.map(e=>e.operands.operation).join(',')==='Transfer,Originate,Fee','LIFECYCLE_EFFECT_SHAPE');
 const fields=e=>Object.fromEntries(e.operands.fields.operands.fields.map(f=>[f.name,f.value]));
 const transfer=fields(emits[0]),loan=fields(emits[1]),fee=fields(emits[2]);
 requireThat(transfer.transferAmount?.constructor==='ConstructAmount'&&transfer.transferAmount.operands.value?.constructor==='LitUInt'&&transfer.transferAmount.operands.asset==='Cash','LIFECYCLE_AMOUNT_LITERAL');
 const text=(v,s)=>v?.constructor==='LitText'&&v.operands.value===s;
 requireThat(text(transfer.from,'Lender')&&text(transfer.to,'Borrower')&&text(transfer.settlementAsset,'Cash')
  &&text(loan.debtor,'Borrower')&&text(loan.creditor,'Lender')&&text(loan.settlementAsset,'Cash')
  &&text(fee.from,'Borrower')&&text(fee.to,'Lender')&&text(fee.settlementAsset,'Cash')
  &&fee.transferAmount?.constructor==='ReadArg'&&fee.transferAmount.operands.name==='feeAmount','LIFECYCLE_ROLE_MAPPING');
 const sourceDigest=sha(source),coreDigest=sha(canonical(elaborated.actions));
 const binding={sourceProfile:elaborated.sourceProfile,coreContract:elaborated.contract,stateSchema:'moriarty-financial-lifecycle-state/2',
  sourceDigest,coreDigest,sourceProfileDigest:sha(elaborated.sourceProfile),coreContractDigest:sha(elaborated.contract)};
 return Object.freeze({...binding,programDigest:sha(canonical(binding)),disbursementAmount:transfer.transferAmount.operands.value.operands.value,
  scope:'authority-fee-disbursement-only',compiled:false});
}
