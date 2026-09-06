import { evaluate, type Action, type CoreState, type Evaluation } from "./language/core.js";
import { defaultAction, elaborate, exampleSource, type AgreementSource, type ElaboratedBundle } from "./language/packages.js";
import { checkIntentEffects, type IntentPolicy } from "./language/policy.js";
import { generateLocalKey, prepareLocalPlan, signLocalPlan, verifyRequiredClaims, verifySignedPlan, type Domain, type PreparedPlan, type SignedPlan } from "./language/claims.js";

type Example = "loan" | "swap";
const domain: Domain = { network:"local-demo", deployment:"browser-r2", semantics:"moriarty-r2/1", verifierProfile:"midnight-native-pcd/1" };
const clone = <T>(value:T):T => structuredClone(value);
const pretty = (value:unknown):string => JSON.stringify(value, null, 2);
const parse = <T>(text:string):T => JSON.parse(text) as T;
const hex = (bytes:ArrayBuffer):string => Array.from(new Uint8Array(bytes),byte=>byte.toString(16).padStart(2,"0")).join("");
const el = <K extends keyof HTMLElementTagNameMap>(tag:K, className="", text?:string):HTMLElementTagNameMap[K] => { const node=document.createElement(tag); node.className=className; if(text!==undefined) node.textContent=text; return node; };
const status = (id:string, text:string, kind="idle"):HTMLElement => { const node=el("p",`language-status ${kind}`,text); node.id=id; node.setAttribute("aria-live","polite"); return node; };

let example:Example="loan";
let sourceText=exampleSource(example);
let bundle:ElaboratedBundle|null=null;
let currentState:CoreState|null=null;
let actionText="";
let policyText="";
let result:Evaluation|null=null;
let policyPassed=false;
let prepared:PreparedPlan|null=null;
let signed:SignedPlan|null=null;
let keys:CryptoKeyPair|null=null;
let trustedPublicKey:string|null=null;
let generation=0;
let sourceMessage="Not elaborated";
let evaluationMessage="Not evaluated";
let policyMessage="Not checked";
let signatureMessage="No current signature";
let claimsMessage="Required claims not checked";
let simulationMessage="No local state advance yet";

function invalidated(reason:string):void {
  generation++; result=null; policyPassed=false; prepared=null; signed=null;
  evaluationMessage="Not evaluated — inputs changed"; policyMessage="Not checked — inputs changed";
  signatureMessage="No current signature — inputs changed"; claimsMessage="Required claims not checked"; simulationMessage=reason;
}
function clearRenderedEvidence(clearSource:boolean):void {
  for(const id of ["before-state","after-state","effects-json"]) { const node=document.getElementById(id); if(node) node.textContent="—"; }
  if(clearSource) for(const id of ["current-state","core-json"]) { const node=document.getElementById(id); if(node) node.textContent="—"; }
  if(clearSource) document.getElementById("quantization-note")?.remove();
}
function showInvalidation(clearSource=false):void {
  const values:Record<string,string>={"evaluation-status":evaluationMessage,"policy-status":policyMessage,"signature-status":signatureMessage,"claims-status":claimsMessage,"simulation-status":simulationMessage};
  for(const [id,value] of Object.entries(values)) { const node=document.getElementById(id); if(node) { node.textContent=value; node.className="language-status idle"; } }
  clearRenderedEvidence(clearSource);
}

function policyFor(source:AgreementSource, before:CoreState, action:Action):IntentPolicy {
  const t=source.terms;
  if(source.package==="Actus.LAM.FirstPeriod") {
    const accounting={borrowerCash:{asset:t.asset,account:t.borrower},lenderCash:{asset:t.asset,account:t.lender}};
    if(action.name==="accrue") return { profile:"loan", accounting, transfers:[], minimumCredits:[], fees:[], dues:[
      {kind:"DueCreated",bucket:"principal",dueId:`${t.instance}:principal`,debtor:t.borrower,creditor:t.lender,denomination:t.denomination,maxAmount:t.principalPayment},
      {kind:"DueCreated",bucket:"interest",dueId:`${t.instance}:interest`,debtor:t.borrower,creditor:t.lender,denomination:t.denomination,maxAmount:t.notional},
    ], allowedWrites:["notional","principalDue","interestDue","cursor"] };
    const total=(BigInt(before.values.principalDue)+BigInt(before.values.interestDue)).toString();
    return { profile:"loan", accounting, transfers:[{asset:t.asset,from:t.borrower,to:t.lender,maxAmount:total}], minimumCredits:[{asset:t.asset,to:t.lender,minAmount:total}], fees:[], dues:[
      {kind:"DueSettled",bucket:"principal",dueId:`${t.instance}:principal`,debtor:t.borrower,creditor:t.lender,denomination:t.denomination,asset:t.asset,maxAmount:before.values.principalDue},
      {kind:"DueSettled",bucket:"interest",dueId:`${t.instance}:interest`,debtor:t.borrower,creditor:t.lender,denomination:t.denomination,asset:t.asset,maxAmount:before.values.interestDue},
    ], allowedWrites:["borrowerCash","lenderCash","principalPaid","interestPaid","principalDue","interestDue","cursor","closed"] };
  }
  const accounting={reserveA:{asset:t.assetA,account:t.instance},reserveB:{asset:t.assetB,account:t.instance},traderA:{asset:t.assetA,account:t.trader},traderB:{asset:t.assetB,account:t.trader},providerA:{asset:t.assetA,account:t.provider},providerB:{asset:t.assetB,account:t.provider}};
  if(action.name==="close") return {profile:"swap",accounting,transfers:[
    {asset:t.assetA,from:t.instance,to:t.provider,maxAmount:before.values.reserveA}, {asset:t.assetB,from:t.instance,to:t.provider,maxAmount:before.values.reserveB}
  ],minimumCredits:[],fees:[],dues:[],allowedWrites:["reserveA","reserveB","providerA","providerB","closed"]};
  return {profile:"swap",accounting,transfers:[
    {asset:t.assetA,from:t.trader,to:t.instance,maxAmount:action.args.amountIn}, {asset:t.assetB,from:t.instance,to:t.trader,maxAmount:before.values.reserveB}
  ],minimumCredits:[{asset:t.assetB,to:action.args.recipient,minAmount:action.args.minOut}],fees:[],dues:[],allowedWrites:["traderA","reserveA","reserveB","traderB"]};
}

function selectExample(next:Example):void { example=next; sourceText=exampleSource(next); elaborateSource("Example reset"); }

function elaborateSource(message="Agreement elaborated. Prepared evidence was cleared."):void {
  generation++; bundle=null; currentState=null; result=null; policyPassed=false; prepared=null; signed=null;
  try { const outcome=elaborate(parse(sourceText)); if(outcome.outcome==="rejected") { sourceMessage=`${outcome.code}: ${outcome.message}`; actionText=""; policyText=""; }
    else { bundle=outcome; currentState=clone(outcome.initialState); const action=defaultAction(outcome,currentState); actionText=pretty(action); policyText=pretty(policyFor(outcome.source,currentState,action)); sourceMessage=`Elaborated · ${outcome.description}`; }
  } catch(error) { sourceMessage=`INVALID_JSON: ${error instanceof Error?error.message:"Invalid JSON"}`; actionText=""; policyText=""; }
  evaluationMessage="Not evaluated"; policyMessage="Not checked"; signatureMessage="No current signature"; claimsMessage="Required claims not checked"; simulationMessage=message; render();
}

function runEvaluation():void {
  generation++;
  if(!bundle||!currentState) { evaluationMessage="Elaborate a valid agreement first"; render(); return; }
  try { result=evaluate(bundle.program,currentState,parse<Action>(actionText)); evaluationMessage=result.outcome==="evaluated"?`Evaluated · ${result.steps} bounded instructions`:`${result.code}: ${result.message}`; }
  catch(error) { result=null; evaluationMessage=`INVALID_JSON: ${error instanceof Error?error.message:"Invalid JSON"}`; }
  policyPassed=false; prepared=null; signed=null; policyMessage="Not checked"; signatureMessage="No current signature"; claimsMessage="Required claims not checked"; render();
}

function checkPolicy():void {
  generation++;
  if(!result||result.outcome!=="evaluated") { policyMessage="Evaluate successfully before checking intent"; render(); return; }
  try { const checked=checkIntentEffects(result.before,result.after,result.effects,parse(policyText)); if(checked.outcome==="checked") { policyPassed=true; policyMessage="Local intent/effect check passed · local check only"; } else { policyPassed=false; policyMessage=`${checked.code}: ${checked.message}`; } }
  catch(error) { policyPassed=false; policyMessage=`INVALID_JSON: ${error instanceof Error?error.message:"Invalid JSON"}`; }
  prepared=null; signed=null; signatureMessage="No current signature"; claimsMessage="Required claims not checked"; render();
}

async function sign():Promise<void> {
  const token=++generation;
  if(!bundle||!currentState||!result||result.outcome!=="evaluated"||!policyPassed) { signatureMessage="Evaluate and pass the intent check before signing"; render(); return; }
  const action=parse<Action>(actionText), policy=parse<IntentPolicy>(policyText), snapshot=pretty({source:sourceText,action:actionText,policy:policyText,state:currentState});
  signatureMessage="Generating an in-memory Ed25519 key…"; render();
  try { const nextKeys=await generateLocalKey(); if(token!==generation||snapshot!==pretty({source:sourceText,action:actionText,policy:policyText,state:currentState})) return;
    const nextPrepared=await prepareLocalPlan(bundle.program,currentState,action,result,policy,domain); if(token!==generation) return;
    if(nextPrepared.outcome!=="prepared") { signatureMessage=`${nextPrepared.code}: ${nextPrepared.message}`; render(); return; }
    const independentlyTrusted=hex(await crypto.subtle.exportKey("raw",nextKeys.publicKey));
    const nextSigned=await signLocalPlan(nextPrepared.plan,nextKeys); const checked=await verifySignedPlan(nextSigned,nextPrepared.plan,independentlyTrusted); if(token!==generation) return;
    if(checked.outcome==="rejected") { signatureMessage=`${checked.code}: ${checked.message}`; render(); return; }
    keys=nextKeys; trustedPublicKey=independentlyTrusted; prepared=nextPrepared.plan; signed=nextSigned; signatureMessage="Ed25519 signature verified · signature and bindings are a local check only";
  } catch(error) { if(token!==generation) return; signatureMessage=`Ed25519 unavailable in this browser: ${error instanceof Error?error.message:"unsupported browser feature"}`; }
  render();
}

async function verifyClaims():Promise<void> {
  const token=generation;
  if(!signed||!prepared||!trustedPublicKey) { claimsMessage="Sign a current plan before checking required claims"; render(); return; }
  const checked=await verifyRequiredClaims(signed,prepared,trustedPublicKey); if(token!==generation) return;
  claimsMessage=checked.outcome==="unavailable"?`Real PCD acceptance unavailable · ${checked.missing.join(", ")} · ${checked.reason}`:`${checked.code}: ${checked.message}`; render();
}

function advanceLocal():void {
  if(!bundle||!result||result.outcome!=="evaluated"||!policyPassed) { simulationMessage="Local advance requires a successful evaluation and intent check"; render(); return; }
  currentState=clone(result.after); generation++; result=null; policyPassed=false; prepared=null; signed=null;
  const action=defaultAction(bundle,currentState); actionText=pretty(action); policyText=pretty(policyFor(bundle.source,currentState,action));
  evaluationMessage="Not evaluated — state advanced"; policyMessage="Not checked — state advanced"; signatureMessage="No current signature — state advanced"; claimsMessage="Required claims not checked";
  simulationMessage="Local simulation advanced. This is not ledger acceptance; stale signatures were cleared."; render();
}

function exportPlan():void { if(!prepared) return; const payload=signed??prepared; const url=URL.createObjectURL(new Blob([pretty(payload)],{type:"application/json"})); const a=el("a"); a.href=url;a.download=`moriarty-r2-${example}-local-plan.json`;a.click();URL.revokeObjectURL(url); }

function injectFailure(kind:"event"|"minimum"|"overflow"|"malformed"):void {
  if(kind==="malformed") actionText="{ malformed JSON";
  else if(kind==="overflow"&&example==="loan") { const source=parse<AgreementSource>(sourceText); source.terms.rateNumerator=((1n<<128n)-1n).toString(); sourceText=pretty(source); elaborateSource("Overflow example loaded. Evaluate to inspect checked intermediate arithmetic."); return; }
  else { const action=parse<Action>(actionText); if(kind==="event") action.args.event="WRONG_EVENT"; if(kind==="minimum") action.args.minOut="19744"; if(kind==="overflow") action.args.amountIn=(1n<<128n).toString(); actionText=pretty(action); }
  invalidated("Failure example loaded. Evaluate to inspect the rejection."); render();
}

function editor(id:string,label:string,value:string,onInput:(value:string)=>void):HTMLElement { const wrap=el("label","json-editor",label); const area=el("textarea") as HTMLTextAreaElement; area.id=id; area.value=value; area.spellcheck=false; area.addEventListener("input",()=>onInput(area.value)); wrap.append(area); return wrap; }
function jsonView(id:string,value:unknown):HTMLElement { const pre=el("pre"); pre.id=id; pre.textContent=value===null?"—":pretty(value); return pre; }
function card(title:string,...children:HTMLElement[]):HTMLElement { const section=el("section","card language-card"); const head=el("div","card-head"); head.append(el("h2","",title)); const body=el("div","card-body"); body.append(...children); section.append(head,body); return section; }
function button(id:string,label:string,handler:()=>void|Promise<void>,secondary=false):HTMLButtonElement { const b=el("button",secondary?"secondary":"",label);b.id=id;b.addEventListener("click",()=>void handler());return b; }

function render():void {
  const root=document.querySelector<HTMLElement>("#app"); if(!root)return; root.replaceChildren();
  const shell=el("main","shell language-shell"); const top=el("header","topbar"); const brand=el("a","brand"); brand.href="/"; brand.append(el("span","mark","M"),el("strong","","Moriarty")); top.append(brand,el("span","demo-badge","Executable slice · local only"));
  const hero=el("section","hero language-hero"); const intro=el("div"); intro.append(el("p","eyebrow","R2 language workspace"),el("h1","","Run a bounded agreement."),el("p","","Edit typed JSON, inspect generic Core execution, constrain effects independently, and sign a local plan. Required real proofs remain unavailable."));
  const controls=el("div","language-controls"); const label=el("label","","Example"); const select=el("select") as HTMLSelectElement;select.id="example"; for(const [value,name] of [["loan","Loan · first period"],["swap","Pool · swap and close"]]) {const option=el("option","",name);option.value=value;option.selected=value===example;select.append(option);} select.addEventListener("change",()=>selectExample(select.value as Example));label.append(select); controls.append(label,button("reset-example","Reset example",()=>selectExample(example),true)); hero.append(intro,controls);
  const boundary=el("div","notice"); boundary.append(el("strong","","Boundary."),document.createTextNode(" JSON is the first authoring subset, not final textual syntax. Evaluation and signatures are local checks. Advance changes only this page's simulation state; it never means ledger acceptance."));
  const grid=el("div","language-grid");
  const sourceCard=card("1 · Agreement",editor("source-json","Agreement JSON",sourceText,value=>{sourceText=value; invalidated("Agreement edited. Elaborate again."); sourceMessage="Not elaborated — source edited"; bundle=null;currentState=null; const node=document.getElementById("source-status");if(node){node.textContent=sourceMessage;node.className="language-status idle";}showInvalidation(true);}),button("elaborate","Elaborate",()=>elaborateSource()),status("source-status",sourceMessage,bundle?"ok":sourceMessage.startsWith("Not")?"idle":"fail"));
  const actionCard=card("2 · Action and intent",editor("action-json","Action JSON",actionText,value=>{actionText=value;invalidated("Action edited. Prepared evidence was cleared.");showInvalidation();}),editor("policy-json","Independent IntentPolicy JSON",policyText,value=>{policyText=value;invalidated("Policy edited. Prepared evidence was cleared.");showInvalidation();}),el("p","caption","The default policy is derived from agreement terms, the authored action, and predecessor state. It does not use evaluator output."));
  const run=card("3 · Execute",el("div","language-buttons"),el("div","failure-tools"),status("evaluation-status",evaluationMessage,result?.outcome==="evaluated"?"ok":result?"fail":"idle"),status("policy-status",policyMessage,policyPassed?"ok":policyMessage.includes(":")?"fail":"idle"),status("simulation-status",simulationMessage,"idle")); const rb=run.querySelector(".language-buttons")!;rb.append(button("evaluate","Evaluate",runEvaluation),button("check-policy","Check intent effects",checkPolicy,true),button("advance-local","Advance local simulation",advanceLocal,true)); const failures=run.querySelector(".failure-tools")!; failures.append(el("span","caption","Load a visible rejection:"),button("try-malformed","Malformed JSON",()=>injectFailure("malformed"),true),button("try-overflow","UInt128 overflow",()=>injectFailure("overflow"),true),button("try-profile-failure",example==="loan"?"Wrong event":"Minimum 19,744",()=>injectFailure(example==="loan"?"event":"minimum"),true));
  const signCard=card("4 · Sign and verify",el("div","language-buttons"),status("signature-status",signatureMessage,signed?"ok":signatureMessage.includes("unavailable")?"warn":"idle"),status("claims-status",claimsMessage,claimsMessage.includes("unavailable")?"warn":"idle"),el("p","caption","The private key is nonextractable, remains in memory, and is never exported. Verification trusts a separately retained generated public key, not a key copied from an edited envelope.")); const sb=signCard.querySelector(".language-buttons")!;sb.append(button("sign-local","Sign with local demo key",sign),button("verify-claims","Verify required claims",verifyClaims,true),button("export-plan","Export local plan JSON",exportPlan,true));
  const before=result?.outcome==="evaluated"?result.before:currentState; const after=result?.outcome==="evaluated"?result.after:null; const effects=result?.outcome==="evaluated"?result.effects:null;
  const stateCard=card("State transition",el("h3","","Current simulation state"),jsonView("current-state",currentState),el("h3","","Evaluation before"),jsonView("before-state",before),el("h3","","Evaluation after"),jsonView("after-state",after),el("h3","","Effects"),jsonView("effects-json",effects));
  const coreCard=card("Elaborated generic Core",el("p","caption",bundle?.description??"Elaborate a valid agreement to inspect Core JSON."),jsonView("core-json",bundle?.program??null)); if(bundle?.source.package==="Actus.LAM.FirstPeriod") {const note=el("p","quantization-note",bundle.description);note.id="quantization-note";const link=el("a","","Open imported reference workspace");link.href="/";coreCard.querySelector(".card-body")!.prepend(note,link);}
  grid.append(sourceCard,actionCard,run,signCard,stateCard,coreCard); shell.append(top,hero,boundary,grid,el("footer","footer","Moriarty R2 executable slice · local evaluation and cryptographic signing · real PCD and ledger acceptance unavailable"));root.append(shell);
}

elaborateSource("Loan example loaded.");
