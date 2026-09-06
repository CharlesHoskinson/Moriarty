import { generateLocalKey } from "./language/claims.js";
import { readIntent, renderIntentSummary, signIntent, type IntentIR, type PlanIR, type SignedIntent, type TrustContext } from "./language/outcome.js";
import { createOutcomeDemo } from "./language/outcome-runtime.js";

type Example = "swap" | "loan";
type OutcomeRuntime = Awaited<ReturnType<typeof createOutcomeDemo>>["runtime"];
type Result = { outcome:string; code?:string; message?:string; receipt?:unknown; missing?:string[] };
const pretty=(value:unknown):string=>JSON.stringify(value,null,2);
const el=<K extends keyof HTMLElementTagNameMap>(tag:K,className="",text?:string):HTMLElementTagNameMap[K]=>{const node=document.createElement(tag);node.className=className;if(text!==undefined)node.textContent=text;return node;};

let example:Example="swap";
let runtime:OutcomeRuntime;
let intentText="";
let signed:SignedIntent|null=null;
let trust:TrustContext|null=null;
let plan:PlanIR|null=null;
let receipt:unknown=null;
let planDraftInvalid=false;
let simulating=false;
let proposing=false;
let loadingDemo=false;
let generation=0;
let intentMessage="Loading local outcome demo…";
let signatureMessage="Sign the outcome before choosing a plan";
let planMessage="No plan selected";
let previewMessage="Not checked";
let simulationMessage="Not simulated";
let claimsMessage="Required real claims not checked";

function resultText(result:Result):string {
  if(result.outcome==="rejected") return `Rejected · ${result.code??"Invalid"}: ${result.message??"The request was rejected."}`;
  if(result.outcome==="unavailable") return `Real PCD acceptance unavailable · ${(result.missing??[]).join(", ")}`;
  return result.outcome==="simulated"?"Simulated complete · one-shot unused authority extinguished":`Eligible local simulation · ${result.outcome}`;
}
function clearChecks(reason:string):void { receipt=null; previewMessage="Not checked — "+reason; simulationMessage="Not simulated — "+reason; claimsMessage="Required real claims not checked"; }
function intentEdited(value:string):void {
  generation++;intentText=value; signed=null;trust=null;plan=null;planDraftInvalid=false;signatureMessage="No current signature — intent edited";planMessage="No plan selected — intent edited";clearChecks("intent edited");
  try { readIntent(JSON.parse(value)); intentMessage="Valid editable IntentIR"; } catch(error) { intentMessage=`Invalid JSON or intent · ${error instanceof Error?error.message:"invalid input"}`; }
  updateEvidence();
}
function planEdited(value:string):void {
  generation++;clearChecks("plan edited");
  try { plan=JSON.parse(value) as PlanIR;planDraftInvalid=false;planMessage="Editable PlanIR — recheck required"; } catch(error) { plan=null;planDraftInvalid=true;planMessage=`Invalid JSON · ${error instanceof Error?error.message:"invalid plan"}`; }
  updateEvidence();
}
async function reset(next:Example):Promise<void>{
  const token=++generation;loadingDemo=true;signed=null;trust=null;plan=null;receipt=null;planDraftInvalid=false;signatureMessage="No current signature — loading fresh local world";planMessage="No plan selected — loading fresh local world";clearChecks("loading fresh local world");intentMessage=`Loading ${next} local outcome demo…`;updateEvidence();
  try{const demo=await createOutcomeDemo(next);if(token!==generation)return;example=next;runtime=demo.runtime;intentText=pretty(demo.intent);intentMessage="Valid editable IntentIR";signatureMessage="Sign the outcome before choosing a plan";planMessage="No plan selected";previewMessage="Not checked";simulationMessage="Not simulated";claimsMessage="Required real claims not checked";loadingDemo=false;render();}
  catch(error){if(token!==generation)return;loadingDemo=false;intentMessage=`Demo creation rejected · ${error instanceof Error?error.message:"unable to create local world"}`;const select=document.querySelector<HTMLSelectElement>("#example");if(select)select.value=example;updateEvidence();}
}
async function sign():Promise<void>{
  const token=++generation;signed=null;trust=null;plan=null;receipt=null;planDraftInvalid=false;clearChecks("signing started");let intent:IntentIR;
  try{intent=readIntent(JSON.parse(intentText));}catch(error){intentMessage=`Invalid JSON or intent · ${error instanceof Error?error.message:"invalid input"}`;signed=null;trust=null;updateEvidence();return;}
  signatureMessage="Generating in-memory local key…";updateEvidence();
  try { const keys=await generateLocalKey();const next=await signIntent(intent,keys);if(token!==generation)return;signed=next;trust={domain:structuredClone(intent.domain),principal:intent.principal,publicKey:next.publicKey};signatureMessage=`Signed outcome ${next.intentHash}`;plan=null;planMessage="Signed before plan choice · choose a route";clearChecks("new signature");updateEvidence(); }
  catch(error){if(token!==generation)return;signed=null;trust=null;signatureMessage=`Signing rejected · ${error instanceof Error?error.message:"unknown error"}`;updateEvidence();}
}
async function propose(route:number):Promise<void>{
  if(!signed||proposing||simulating)return;const token=++generation;plan=null;receipt=null;planDraftInvalid=false;proposing=true;clearChecks("proposal pending");planMessage=`Proposing route ${route} · prior executable plan cleared…`;updateEvidence();
  try{const next=await runtime.propose(signed.intentHash,route);if(token!==generation)return;plan=next;planDraftInvalid=false;clearChecks("new proposal");planMessage=`Proposed route ${route} · ${next.steps[0]?.agreementId??"registered agreement"} · signed intent retained`;}
  catch(error){if(token!==generation)return;plan=null;planMessage=`Rejected · ${error instanceof Error?error.message:"proposal failed"}`;}
  finally{if(token===generation){proposing=false;updateEvidence();}}
}
async function run(kind:"preview"|"simulate"|"real"):Promise<void>{
  if(!signed||!plan||!trust||loadingDemo||proposing||simulating)return;const token=++generation;let result:Result;
  if(kind==="simulate"){simulating=true;receipt=null;simulationMessage="Local simulation pending · prior receipt cleared; inputs locked until atomic commit finishes";updateEvidence();}
  try { result=await (kind==="preview"?runtime.preview(signed,plan,trust):kind==="simulate"?runtime.simulate(signed,plan,trust):runtime.verifyRealAcceptance(signed,plan,trust)) as Result; }
  catch(error){result={outcome:"rejected",code:"UIError",message:error instanceof Error?error.message:"operation failed"};}
  if(kind==="simulate")simulating=false;
  if(token!==generation)return;
  if(result.outcome==="rejected")receipt=null;
  if(result.receipt)receipt=result.receipt;
  if(kind==="preview")previewMessage=resultText(result);else if(kind==="simulate")simulationMessage=resultText(result);else claimsMessage=resultText(result);
  updateEvidence();
}
function advanceTime():void { const input=document.querySelector<HTMLInputElement>("#logical-time");if(!input)return;generation++;try{const result=runtime.advanceTime(input.value) as unknown as Result;if(result?.outcome==="rejected")previewMessage=resultText(result);else{clearChecks("logical time advanced");previewMessage=`Not checked — logical time is ${runtime.snapshot().now}`;}}catch(error){previewMessage=`Rejected · ${error instanceof Error?error.message:"invalid time"}`;}updateEvidence(); }
function exportOutcome():void { try{const intent=readIntent(JSON.parse(intentText));const payload={kind:"MoriartyOutcomeDemoExport",example,snapshot:runtime.snapshot(),intent,signedIntent:signed,plan,receipt,realAcceptance:claimsMessage};const url=URL.createObjectURL(new Blob([pretty(payload)],{type:"application/json"}));const link=el("a");link.href=url;link.download=`moriarty-${example}-outcome.json`;link.click();URL.revokeObjectURL(url);}catch(error){intentMessage=`Export rejected · Invalid JSON or intent · ${error instanceof Error?error.message:"invalid input"}`;updateEvidence();} }
function status(id:string,text:string):HTMLElement{const kind=/rejected|invalid/i.test(text)?"fail":/signed|eligible|simulated complete/i.test(text)?"ok":/unavailable/i.test(text)?"warn":"idle";const node=el("p",`language-status ${kind}`,text);node.id=id;node.setAttribute("aria-live","polite");return node;}
function editor(id:string,label:string,value:string,handler:(value:string)=>void):HTMLElement{const wrap=el("label","json-editor",label);const area=el("textarea") as HTMLTextAreaElement;area.id=id;area.value=value;area.spellcheck=false;area.addEventListener("input",()=>handler(area.value));wrap.append(area);return wrap;}
function button(id:string,label:string,handler:()=>void|Promise<void>,secondary=false):HTMLButtonElement{const b=el("button",secondary?"secondary":"",label);b.id=id;b.addEventListener("click",()=>void handler());return b;}
function card(title:string,...children:HTMLElement[]):HTMLElement{const section=el("section","card language-card");const head=el("div","card-head");head.append(el("h2","",title));const body=el("div","card-body");body.append(...children);section.append(head,body);return section;}
function summary():string{try{return renderIntentSummary(JSON.parse(intentText));}catch(error){return `Invalid intent: ${error instanceof Error?error.message:"invalid input"}`;}}
function updateEvidence():void {
  const values:Record<string,string>={"intent-status":intentMessage,"signature-status":signatureMessage,"plan-status":planMessage,"preview-status":previewMessage,"simulation-status":simulationMessage,"claims-status":claimsMessage,"intent-summary":summary(),"signed-intent":signed?pretty(signed):"—","receipt-json":receipt?pretty(receipt):"—"};
  for(const [id,value] of Object.entries(values)){const node=document.getElementById(id);if(node){if(node instanceof HTMLTextAreaElement)node.value=value;else{node.textContent=value;if(node.classList.contains("language-status"))node.className=`language-status ${/rejected|invalid/i.test(value)?"fail":/signed|eligible|simulated complete/i.test(value)?"ok":/unavailable/i.test(value)?"warn":"idle"}`;}}}
  const planEditor=document.querySelector<HTMLTextAreaElement>("#plan-json");if(planEditor&&document.activeElement!==planEditor&&!planDraftInvalid)planEditor.value=plan?pretty(plan):"";
  for(const id of ["example","intent-json","plan-json","sign-intent","propose-route-0","propose-route-1","preview-plan","simulate-plan","verify-real","logical-time","advance-time","export-outcome"]){const control=document.getElementById(id) as HTMLButtonElement|HTMLInputElement|HTMLTextAreaElement|HTMLSelectElement|null;if(control)control.disabled=simulating||proposing||loadingDemo;}
}
function render():void{
  const root=document.querySelector<HTMLElement>("#app");if(!root)return;root.replaceChildren();const shell=el("main","shell language-shell outcome-shell");
  const top=el("header","topbar");const brand=el("a","brand");brand.href="/";brand.append(el("span","mark","M"),el("strong","","Moriarty"));const nav=el("nav","top-actions");for(const [href,label] of [["/","Fixtures"],["/language","Exact plans"],["/intents","Outcome intents"]]){const a=el("a","demo-badge",label);a.href=href;nav.append(a);}top.append(brand,nav);
  const hero=el("section","hero language-hero");const intro=el("div");intro.append(el("p","eyebrow","R2b outcome workspace"),el("h1","","Sign the outcome. Then choose the route."),el("p","","Authorize bounded results across two pools or a loan settlement. Every check is local; real proof-carrying acceptance remains unavailable."));const controls=el("div","language-controls");const label=el("label","","Example");const select=el("select") as HTMLSelectElement;select.id="example";for(const [value,name] of [["swap","Two pool routes"],["loan","Loan settlement"]]){const option=el("option","",name);option.value=value;option.selected=value===example;select.append(option);}select.addEventListener("change",()=>void reset(select.value as Example));label.append(select);controls.append(label);hero.append(intro,controls);
  const boundary=el("div","notice");boundary.append(el("strong","","Local boundary."),document.createTextNode(" No real PCD, verifier, wallet, network, or ledger is connected. Changing the example or reloading creates a fresh local world and resets its nonce history. Atomic completion extinguishes all unused one-shot authority."));
  const grid=el("div","outcome-grid");
  const intentCard=card("1 · Outcome authority",editor("intent-json","Editable IntentIR JSON",intentText,intentEdited),status("intent-status",intentMessage),el("h3","","Canonical semantic summary"),(()=>{const p=el("pre");p.id="intent-summary";p.textContent=summary();return p;})(),button("sign-intent","Sign outcome with local key",sign),status("signature-status",signatureMessage));
  const routeButtons=el("div","language-buttons");if(example==="swap")routeButtons.append(button("propose-route-0","Propose pool A",()=>propose(0)),button("propose-route-1","Propose pool B",()=>propose(1),true));else routeButtons.append(button("propose-route-0","Propose loan settlement",()=>propose(0)));
  const planArea=editor("plan-json","Editable PlanIR JSON",plan?pretty(plan):"",planEdited);const planCard=card("2 · Concrete plan",routeButtons,status("plan-status",planMessage),planArea);
  const actions=el("div","language-buttons");actions.append(button("preview-plan","Preview independently",()=>run("preview")),button("simulate-plan","Simulate locally",()=>run("simulate")),button("verify-real","Verify real claims",()=>run("real"),true));const time=el("div","logical-time");const timeLabel=el("label","","Logical time");const timeInput=el("input") as HTMLInputElement;timeInput.id="logical-time";timeInput.value=runtime.snapshot().now;timeLabel.append(timeInput);time.append(timeLabel,button("advance-time","Advance logical time",advanceTime,true));const execution=card("3 · Check and consume",actions,time,status("preview-status",previewMessage),status("simulation-status",simulationMessage),status("claims-status",claimsMessage),el("p","caption","Preview is read-only. Simulation rechecks current balances, the agreement anchor, expiry, signature, and nonce immediately before atomic tab-local consumption."));
  const evidence=card("Public evidence",el("h3","","Signed intent"),(()=>{const p=el("pre");p.id="signed-intent";p.textContent=signed?pretty(signed):"—";return p;})(),el("h3","","Latest receipt"),(()=>{const p=el("pre");p.id="receipt-json";p.textContent=receipt?pretty(receipt):"—";return p;})(),button("export-outcome","Export public outcome JSON",exportOutcome,true),el("p","caption","The generated private key is nonextractable, stays in memory, and is never exported."));
  grid.append(intentCard,planCard,execution,evidence);shell.append(top,hero,boundary,grid,el("footer","footer","Moriarty R2b local outcome simulation · tab-only nonce · no ledger · mandatory real claims unavailable"));root.append(shell);
}
void reset("swap");
