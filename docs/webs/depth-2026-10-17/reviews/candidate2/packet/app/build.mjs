import {build} from 'esbuild';
import {readFile,writeFile} from 'node:fs/promises';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {applyPresentationPatches} from './src/presentation.mjs';
import {applyEditorialRevision} from './src/editorial.mjs';
const app=path.dirname(fileURLToPath(import.meta.url));
const root=path.resolve(app,'..');
const contribution=(name)=>path.join(root,'contributions',name);
const digest=(s)=>createHash('sha256').update(s).digest('hex');
const concept=path.join(root,'reference-inputs/concept');
const inputPaths=[path.join(concept,'MANIFEST.json'),...['integrated-r2/CONCEPT.md','integrated-r2/MESSAGING.md','integrated-r2/DEVELOPER-EXPERIENCE.md','root-checks/STUDY10-CLARIFIED-INTAKE.json','sources/S03-MORIARTY-PRODUCT-CONTRACT.md'].map(x=>path.join(concept,x)),...['BRIEF.md','PLAN.md','SOURCE-PRECONDITION-CLARIFICATION.json','DEPENDENCY-REGISTRY-PINS.json','V2-PRESENTATION-BRIEF.md','USER-V2-PRESENTATION-STEERING.json'].map(x=>path.join(root,x)),contribution('03-visual/design.css'),contribution('03-visual/HeroWeb.jsx'),contribution('13-sevenlayer/FACT-CHECK.json'),contribution('13-sevenlayer/SECTION-NOTES.md')];
// Retain the source document and exact pointer without embedding absolute host paths.
const clean=(value)=>Array.isArray(value)?value.map(clean):value&&typeof value==='object'?Object.fromEntries(Object.entries(value).map(([key,item])=>key==='path'?['document',path.basename(item)]:[key,clean(item)])):value;
const json=async(p)=>JSON.parse(await readFile(p,'utf8'));
let data={};
for(const [key,name] of [['scenarios','04-scenarios/scenarios.json'],['invoice','04-scenarios/invoice-fixtures.json'],['security','06-security/SECURITY-COPY.json'],['learning','08-learning/learning.json'],['v2','11-v2-security/V2-COPY.json'],['groth16','12-groth16/GROTH16-SECTION.json'],['sevenLayers','13-sevenlayer/SEVEN-LAYER-SECTION.json'],['bookFindings','13-sevenlayer/FINDINGS.json']]){const p=contribution(name);data[key]=clean(await json(p));inputPaths.push(p)}
const patchPath=contribution('14-review-corrections/PRESENTATION-PATCHES.json');
const presentationPatches=await json(patchPath);
if(presentationPatches.pending.length>0)throw new Error('Presentation corrections are not complete.');
data=applyPresentationPatches(data,presentationPatches);
inputPaths.push(patchPath,contribution('14-review-corrections/COUNT-CLARIFICATION.json'));
const editorialPath=path.join(root,'editorial-2026-10-17/COPY.json');
data=applyEditorialRevision(data,await json(editorialPath));
inputPaths.push(editorialPath,path.join(root,'../assets/moriarty-banner.png'),path.join(root,'assets/moriarty-web-strands.png'),path.join(root,'depth-2026-10-17/COPY.json'));
// V2-COPY is authored by the designated security author. No fallback maturity copy.
if(!data.v2.main||!data.v2.technical)throw new Error('V2 copy must supply main and technical sections.');
await writeFile(path.join(app,'src/data.generated.json'),JSON.stringify(data));
const result=await build({entryPoints:[path.join(app,'src/main.jsx')],bundle:true,write:false,minify:true,loader:{'.png':'dataurl'},format:'iife',platform:'browser',target:['chrome110','firefox115','safari16'],nodePaths:[path.join(app,'node_modules')],define:{'process.env.NODE_ENV':'"production"'},legalComments:'inline',logLevel:'warning'});
const script=result.outputFiles[0].text.replace(/<\/script/gi,'<\\/script');
const css=(await readFile(contribution('03-visual/design.css'),'utf8'))+'\n'+(await readFile(path.join(app,'src/app.css'),'utf8'))+'\n'+(await readFile(path.join(app,'src/brand.css'),'utf8'))+'\n'+(await readFile(path.join(app,'src/transaction-diagram.css'),'utf8'))+'\n'+(await readFile(path.join(app,'src/depth.css'),'utf8'))+'\n'+(await readFile(path.join(app,'src/concern-graph.css'),'utf8'));
const notices=[];
for(const name of ['react','react-dom','scheduler','esbuild']){const notice=await readFile(path.join(app,'node_modules',name,name==='esbuild'?'LICENSE.md':'LICENSE'),'utf8');notices.push(`${name}\n${notice}`)}
const notice=notices.join('\n\n');
await writeFile(path.join(app,'THIRD-PARTY-NOTICES.txt'),notice);
const escapeHTML=(s)=>s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');
const html=`<!doctype html>\n<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="description" content="How Moriarty Webs carry proofs for Cardano financial transactions. Explore an invoice payment, contract checks and Groth16 trusted setup."><meta name="color-scheme" content="dark"><title>Moriarty — Webs and Strands for Cardano</title><style>${css.replace(/<\/style/gi,'<\\/style')}</style></head><body><div id="root"></div><noscript><main style="max-width:48rem;margin:4rem auto;padding:1rem"><h1>Moriarty Webs</h1><p>This interactive guide needs JavaScript for its explorer and arithmetic worksheet.</p><p>${escapeHTML(data.v2.main.heroBody)}</p><h2>Implementation status</h2><p>${escapeHTML(data.security.maturity.body)}</p></main></noscript><script>${script}</script>\n<!-- Third-party license notices\n${notice.replace(/-->/g,'-- >')}\n-->\n</body></html>`;
await writeFile(path.join(root,'index.html'),html);
const localSources=['package.json','package-lock.json','build.mjs','src/App.jsx','src/WebDiagram.jsx','src/TransactionDiagram.jsx','src/transaction-diagram.css','src/DepthContent.jsx','src/CryptographicDetail.jsx','src/depth.css','src/ConcernGraph.jsx','src/concern-graph.css','src/SevenLayers.jsx','src/Groth16.jsx','src/invoice.mjs','src/presentation.mjs','src/editorial.mjs','src/main.jsx','src/app.css','src/brand.css','src/data.generated.json','check-artifact.mjs','THIRD-PARTY-NOTICES.txt'].map(x=>path.join(app,x));
const pins=[];for(const p of [...new Set([...inputPaths,...localSources])]){const bytes=await readFile(p);pins.push({path:path.relative(root,p),bytes:bytes.length,sha256:digest(bytes)})}
const packageJson=await json(path.join(app,'package.json'));
const report={format:'moriarty-webs-page-build/2',status:'V4_DEEP_EXPLANATION_FOR_FRESH_QA',builtAt:new Date().toISOString(),scope:'Standalone explanatory frontend and invoice arithmetic. No product/proof/ledger or security acceptance.',referenceInputsManifest:'reference-inputs/concept/MANIFEST.json',dependencies:{...packageJson.dependencies,...packageJson.devDependencies},node:process.version,presentationCorrections:{path:'contributions/14-review-corrections/PRESENTATION-PATCHES.json',applied: presentationPatches.patches.map(patch=>({id:patch.id,findingId:patch.findingId})),originalInputsPreserved:true},inputs:pins,output:{path:'index.html',bytes:Buffer.byteLength(html),sha256:digest(html)},build:{tool:'esbuild',result:'SUCCESS',selfContained:true,externalRequestsConfigured:false,persistentStorageConfigured:false,editableSource:true,thirdPartyNotices:'app/THIRD-PARTY-NOTICES.txt'},validation:{browser:'Root owns fresh actual browser QA of this exact HTML.',securityWording:'Fresh nonauthor Opus review of final bytes required.',editorial:'Fresh nonauthor Muse review of final bytes required.'},authorship:{implementationRequestedModel:'gpt-6.1-sol',effortRequested:'high',actualProviderModel:'not independently observed',actualEffort:'not independently observed',securityCopy:'Frozen technical evidence is unchanged. Visible explanatory copy is revised by the user-authorized editorial overlay; old page reviews do not cover these new bytes.'},limits:['Product design explanation is distinct from observed implementation evidence.','Local invoice arithmetic is not Moriarty execution.','No wallet, signing, cryptography, native SDK, network fetch, telemetry or persistent storage behavior.','Frozen v1, original contribution files and all persona inputs remain unchanged.'],staticArtifactCheck:{command:'npm run check:artifact',receipt:'app/ARTIFACT-CHECK.json',scope:'Packaging, shared arithmetic against independent fixtures and input pin checks.'},startup:{guardedStatusExit:0,historicalCampaignNotDispatched:true}};
await writeFile(path.join(root,'PAGE-REPORT.json'),JSON.stringify(report,null,2)+'\n');
console.log(JSON.stringify({build:'SUCCESS',bytes:report.output.bytes,sha256:report.output.sha256},null,2));
