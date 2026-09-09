import assert from 'node:assert/strict';
import {mkdtempSync,writeFileSync,readFileSync,rmSync} from 'node:fs';
import {join} from 'node:path';
import {tmpdir} from 'node:os';
import {spawnSync} from 'node:child_process';
import {pathToFileURL} from 'node:url';
const root=process.argv[2],cli=join(root,'experiments/moriarty-language/src/successor/syntax-cli.ts');
const tmp=mkdtempSync(join(tmpdir(),'mori-syntax-independent-'));let cases=0;
function run(args){const r=spawnSync(process.execPath,[cli,...args],{encoding:'utf8',timeout:3000});assert.equal(r.error,undefined,String(r.error));return r;}
function rejected(file,code){const r=run(['check-syntax',file]);assert.notEqual(r.status,0);assert.equal(r.stdout,'');assert.match(r.stderr,new RegExp(code));cases++;}
try{
 const source='profile "moriarty-successor-syntax/0";agreement A{}';const file=join(tmp,'source.mori');writeFileSync(file,source);
 const r=run(['format',file]);assert.equal(r.status,0,r.stderr);assert.equal(readFileSync(file,'utf8'),source);cases++;
 const inert=spawnSync(process.execPath,['--input-type=module','-e',`await import(${JSON.stringify(pathToFileURL(cli).href)})`],{encoding:'utf8',timeout:3000});assert.equal(inert.status,0,inert.stderr);assert.equal(inert.stdout,'');assert.equal(inert.stderr,'');cases++;
 writeFileSync(file,source+' '.repeat(65536-source.length));assert.equal(run(['check-syntax',file]).status,0);cases++;
 writeFileSync(file,source+' '.repeat(65537-source.length));rejected(file,'SOURCE_BOUND');
 writeFileSync(file,Buffer.from([0xc0,0xaf]));rejected(file,'INVALID_UTF8');
 writeFileSync(file,'\uFEFF'+source);rejected(file,'UNEXPECTED_CHAR');
 rejected(tmp,'CLI_IO');rejected('/dev/zero','CLI_IO');
 const fifo=join(tmp,'input.fifo');assert.equal(spawnSync('mkfifo',[fifo]).status,0);rejected(fifo,'CLI_IO');
 console.log(JSON.stringify({scope:'Independent direct-source CLI controls',cases,pass:true}));
}finally{rmSync(tmp,{recursive:true,force:true});}
