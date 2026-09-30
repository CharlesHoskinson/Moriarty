#!/usr/bin/env node
import { readFileSync,writeFileSync,statSync,mkdirSync,realpathSync } from 'node:fs';
import { resolve,relative,isAbsolute,join } from 'node:path';
import { isDeepStrictEqual } from 'node:util';
import { check,format,inspect,expand,simulate } from './index.ts';
import { LocalError,parseBoundedJson } from './json.ts';
import { starterSource,starterScenario,starterCases,repaymentSource,repaymentScenario,repaymentCases } from './starter.ts';
import { runLsp,runMcp } from './servers.ts';
import { byteToPosition, type Analysis } from './frontend.ts';
function output(value:unknown):void{process.stdout.write(JSON.stringify(value,null,2)+'\n');}
type CheckSummary = Pick<Analysis,'status'|'agreement'|'diagnostics'|'actions'>;
function readableCheck(source:string,result:CheckSummary):void{
 let text=`${result.status}${result.agreement ? ` ${result.agreement}` : ''}\n`;
 for(const d of result.diagnostics){const p=byteToPosition(source,d.span.start);text+=`  ${d.code} at ${p.line+1}:${p.character+1}: ${d.message}\n`;}
 for(const a of result.actions)text+=`  ${a.name}: ${a.support}${a.support==='SpecifiedOnly'?' (execution unsupported; financial relations open)':''}\n`;
 if(result.actions.some(a=>a.support==='LocalS0'))text+='Local preparation remains PreparedUnqualified.\n';
 process.stdout.write(text);
}
function read(path:string):string{if(statSync(path).size>65536)throw new Error('File exceeds 65536 bytes');const bytes=readFileSync(path);if(bytes.length>=3&&bytes[0]===0xef&&bytes[1]===0xbb&&bytes[2]===0xbf)throw new LocalError('BETA_BOM','UTF-8 BOM is unsupported; remove it explicitly');let text:string;try{text=new TextDecoder('utf-8',{fatal:true,ignoreBOM:true}).decode(bytes);}catch{throw new LocalError('BETA_UTF8','File must contain valid UTF-8');}return text;}
function closed(v:unknown,required:string[],optional:string[]=[]):Record<string,unknown>{
 if(!v||typeof v!=='object'||Array.isArray(v))throw new LocalError('BETA_CASE_SCHEMA','Expected record');
 const r=v as Record<string,unknown>;
 for(const k of Object.keys(r))if(!required.includes(k)&&!optional.includes(k))throw new LocalError('BETA_CASE_SCHEMA',`Unknown field ${k}`);
 for(const k of required)if(!Object.hasOwn(r,k))throw new LocalError('BETA_CASE_SCHEMA',`Missing field ${k}`);
 return r;
}
function casePath(root:string,value:unknown):string{
 if(typeof value!=='string'||isAbsolute(value))throw new LocalError('BETA_CASE_PATH','Expected relative project file');
 const target=resolve(root,value),rel=relative(root,target);
 if(rel.startsWith('..')||isAbsolute(rel))throw new LocalError('BETA_CASE_PATH','Case file outside project');
 const actual=realpathSync(target),realRel=relative(root,actual);
 if(realRel.startsWith('..')||isAbsolute(realRel))throw new LocalError('BETA_CASE_PATH','Case symlink outside project');
 return actual;
}
type Mismatch={pointer:string;expected:unknown;actual:unknown};
const pointerPart=(key:string):string=>key.replaceAll('~','~0').replaceAll('/','~1');
function brief(v:unknown):unknown{
 if(v===undefined)return {type:'missing'};
 if(typeof v==='string'&&v.length>256)return {type:'string',length:v.length,prefix:v.slice(0,256)};
 if(Array.isArray(v))return {type:'array',length:v.length};
 if(v&&typeof v==='object')return {type:'record',fields:Object.keys(v).length};
 return v;
}
function firstMismatch(actual:unknown,expected:unknown,pointer:string):Mismatch|null{
 if(isDeepStrictEqual(actual,expected))return null;
 if(Array.isArray(actual)&&Array.isArray(expected)){
  const n=Math.min(actual.length,expected.length);
  for(let i=0;i<n;i++){const d=firstMismatch(actual[i],expected[i],`${pointer}/${i}`);if(d)return d;}
  return {pointer:`${pointer}/length`,expected:expected.length,actual:actual.length};
 }
 if(actual&&expected&&typeof actual==='object'&&typeof expected==='object'&&!Array.isArray(actual)&&!Array.isArray(expected)){
  const a=actual as Record<string,unknown>,e=expected as Record<string,unknown>;
  for(const k of new Set([...Object.keys(e),...Object.keys(a)])){
   const d=firstMismatch(a[k],e[k],`${pointer}/${pointerPart(k)}`);if(d)return d;
  }
 }
 return {pointer,expected:brief(expected),actual:brief(actual)};
}
function runCases(directory:string):object{
 const root=realpathSync(directory),manifest=closed(parseBoundedJson(read(join(root,'mori.tests.json'))),['profile','cases']);
 if(manifest.profile!=='moriarty-beta-tests/1'||!Array.isArray(manifest.cases)||manifest.cases.length===0||manifest.cases.length>64)throw new LocalError('BETA_CASE_SCHEMA','Expected 1..64 local cases');
 const names=new Set<string>(),results=manifest.cases.map((raw)=>{
  const c=closed(raw,['name','source','action','scenario','expect']);
  if(typeof c.name!=='string'||!c.name||names.has(c.name)||typeof c.action!=='string')throw new LocalError('BETA_CASE_SCHEMA','Invalid or duplicate case name/action');
  names.add(c.name);const expected=closed(c.expect,['status'],['code','effects','post']);
  if(!['PreparedUnqualified','CoreRejected','SourceRejected','AuthoringRejected','FormationRejected','Unsupported'].includes(String(expected.status)))throw new LocalError('BETA_CASE_SCHEMA','Unknown expected status');
  const r=simulate(read(casePath(root,c.source)),c.action,read(casePath(root,c.scenario)));
  const core='result' in r?r.result:null;
  const code=core?.status==='CoreRejected'?core.rejection.code:core?.status==='SourceRejected'?core.code:'diagnostics' in r?r.diagnostics[0]?.code:undefined;
  const mismatches:Mismatch[]=[];
  for(const [key,actual] of [['status',r.status],['code',code],['effects',core?.status==='PreparedUnqualified'?core.candidate.effects:undefined],['post',core?.status==='PreparedUnqualified'?core.candidate.candidatePost:undefined]] as const){
   if(Object.hasOwn(expected,key)){const d=firstMismatch(actual,expected[key],`/${key}`);if(d)mismatches.push(d);}
  }
  return {name:c.name,passed:mismatches.length===0,status:r.status,code:code??null,mismatches};
 });
 const passed=results.every(x=>x.passed);if(!passed)process.exitCode=1;
 const report={status:passed?'TestsPassed':'TestsFailed',qualification:'local-stipulation-only',cases:results};
 if(Buffer.byteLength(JSON.stringify(report))>524288)throw new LocalError('BETA_CASE_RESULT_BOUND','Case report exceeds 524288 bytes');
 return report;
}
function main():void{
 const [command,...raw]=process.argv.slice(2);
 if(!command||command==='help'||command==='--help'){process.stdout.write('Moriarty beta: init DIR [--template transfer|repay] | check FILE [--json] | fmt FILE [--write] | inspect FILE | expand/simulate FILE --action NAME --scenario FILE | test DIR | lsp | mcp\nLocal financial results remain PreparedUnqualified.\n');return;}
 if(raw.includes('--help')||raw.includes('-h')){const help:Record<string,string>={init:'init DIR [--template transfer|repay]',check:'check FILE [--json]',fmt:'fmt FILE [--write]',inspect:'inspect FILE',expand:'expand FILE --action NAME --scenario FILE',simulate:'simulate FILE --action NAME --scenario FILE',test:'test DIR',lsp:'lsp',mcp:'mcp'};if(!Object.hasOwn(help,command))throw new Error(`Unknown command ${command}`);process.stdout.write(`mori ${help[command]}\nLocal financial results remain PreparedUnqualified.\n`);return;}
 if(command==='lsp'||command==='mcp'){if(raw.length)throw new Error('Stdio command accepts no arguments');command==='lsp'?runLsp():runMcp();return;}
 const allowed:Record<string,string[]>={init:['--template'],check:['--json'],fmt:['--write'],inspect:[],expand:['--action','--scenario'],simulate:['--action','--scenario'],test:[]};
 if(!Object.hasOwn(allowed,command))throw new Error(`Unknown command ${command}`);
 const positional:string[]=[],options=new Map<string,string|boolean>();
 for(let i=0;i<raw.length;i++){
  const arg=raw[i];if(arg.startsWith('--')){
   if(!allowed[command].includes(arg)||options.has(arg))throw new Error(`Unknown or duplicate option ${arg}`);
   if(arg==='--action'||arg==='--scenario'||arg==='--template'){const v=raw[++i];if(!v||v.startsWith('--'))throw new Error(`Missing value for ${arg}`);options.set(arg,v);}else options.set(arg,true);
  }else positional.push(arg);
 }
 if(positional.length!==1)throw new Error(`${command} requires one path`);
 const path=resolve(positional[0]);
 if(command==='init'){
  const template=options.get('--template')??'transfer';if(!['transfer','repay'].includes(String(template)))throw new Error('Template must be transfer or repay');
  const repay=template==='repay',filename=repay?'repayment.mori':'invoice.mori',action=repay?'repay_loan':'pay';
  try{mkdirSync(path);}catch(e){if((e as NodeJS.ErrnoException).code==='EEXIST')throw new LocalError('BETA_INIT_EXISTS','Target already exists; choose a new directory');throw e;}writeFileSync(join(path,filename),repay?repaymentSource:starterSource,{flag:'wx'});
  writeFileSync(join(path,'scenario.json'),JSON.stringify(repay?repaymentScenario:starterScenario,null,2)+'\n',{flag:'wx'});
  writeFileSync(join(path,'mori.tests.json'),JSON.stringify(repay?repaymentCases:starterCases,null,2)+'\n',{flag:'wx'});
  writeFileSync(join(path,'README.md'),`# Moriarty local starter\n\nRun \`mori check ${filename}\`, \`mori test .\`, or \`mori simulate ${filename} --action ${action} --scenario scenario.json\`.\n\nThe fixture is an untrusted local stipulation. No signing, authentication, proof or ledger commit occurs.\n`,{flag:'wx'});
  output({status:'Initialized',directory:path,qualification:'local-stipulation-only'});return;
 }
 if(command==='test'){output(runCases(path));return;}
 if((command==='expand'||command==='simulate')&&(!options.has('--action')||!options.has('--scenario')))throw new Error(`${command} requires --action and --scenario`);
 const source=read(path);
 if(command==='fmt'){
  const result=format(source);if(result.text===null){output({status:'AuthoringRejected',diagnostics:result.diagnostics});process.exitCode=1;return;}
  if(options.has('--write'))writeFileSync(path,result.text);else process.stdout.write(result.text);return;
 }
 const result=command==='check'?check(source):command==='inspect'?inspect(source):command==='expand'?expand(source,options.get('--action') as string,read(resolve(options.get('--scenario') as string))):simulate(source,options.get('--action') as string,read(resolve(options.get('--scenario') as string)));
 if(command==='check'&&!options.has('--json'))readableCheck(source,result as CheckSummary);else output(result);
 const status=(result as {status?:string}).status;
 if(status&&!['AuthoringChecked','Expanded','PreparedUnqualified'].includes(status))process.exitCode=1;
}
try{main();}catch(error){
 process.exitCode=1;
 if(error instanceof LocalError)output({status:'FormationRejected',diagnostics:[{code:error.code,message:error.message}],publishedEffects:null,publishedPost:null});
 else process.stderr.write(`mori: ${error instanceof Error?error.message:String(error)}\n`);
}
