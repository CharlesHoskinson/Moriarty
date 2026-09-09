import assert from 'node:assert/strict';
import { readFileSync, writeFileSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';
const w=new URL('../../',import.meta.url).pathname;
const d=join(w,'deliverables/transfer-only-k-2026-09-09');
const cases=JSON.parse(readFileSync(join(d,'source-observations.json'))).observations;
const tmp=mkdtempSync(join(tmpdir(),'moriarty-transfer-cli-'));
const observations=[];
try {
 for(const c of cases){
  writeFileSync(join(tmp,'input.mori'),c.source);
  writeFileSync(join(tmp,'invocation.json'),JSON.stringify(c.invocation));
  const argv=[join(w,'experiments/moriarty-language/src/successor/simulate-cli.ts'),'simulate',join(tmp,'input.mori'),join(tmp,'invocation.json')];
  const r=spawnSync(process.execPath,argv,{encoding:'utf8',timeout:10000});
  assert.equal(r.status,c.result.status==='Prepared'?0:1,c.id);
  assert.equal(r.stderr,'',c.id);
  assert.deepEqual(JSON.parse(r.stdout),c.result,c.id);
  observations.push({id:c.id,argv,exitCode:r.status,stdout:r.stdout,stderr:r.stderr});
 }
 writeFileSync(join(d,'cli-observations.json'),JSON.stringify({observations},null,2)+'\n');
 console.log('All 16 actual successor simulate CLI results and exit codes match the independent/source observations.');
} finally {rmSync(tmp,{recursive:true});}
