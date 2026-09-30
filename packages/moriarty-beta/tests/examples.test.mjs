import test from 'node:test';import assert from 'node:assert/strict';import {readFileSync} from 'node:fs';
import {check} from '../src/frontend.ts';import {expand,simulate} from '../src/bridge.ts';
for(const name of ['amm','lending','stablecoin','derivatives','oracle','governance','bridge','staking'])test(`${name} authoring example is checked and explicitly unsupported for execution`,()=>{
 const source=readFileSync(new URL(`../examples/${name}.mori`,import.meta.url),'utf8');const report=check(source);
 assert.equal(report.status,'AuthoringChecked',JSON.stringify(report));assert.ok(report.actions.length>0);
 for(const action of report.actions){assert.equal(action.support,'SpecifiedOnly');for(const run of [expand,simulate]){const r=run(source,action.name,'{}');assert.equal(r.status,'Unsupported');assert.equal(r.diagnostics[0].code,'BETA_PROFILE_UNSUPPORTED');assert.equal(r.publishedEffects,null);}}
});
