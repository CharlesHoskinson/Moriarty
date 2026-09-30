import {spawnSync} from 'node:child_process';
import fs from 'node:fs';
const log=[];
export function run(id,args,expected,cwd='merchant'){
  const r=spawnSync('../node_modules/.bin/mori',args,{cwd,encoding:'utf8'});
  const out=`outputs/${id}.txt`;
  fs.writeFileSync(out,`$ mori ${args.join(' ')}\n--- stdout\n${r.stdout}\n--- stderr\n${r.stderr}\n--- exit ${r.status}\n`);
  const first=(r.stdout||r.stderr).split('\n').filter(l=>/status|code|"message"/.test(l)).slice(0,4).map(s=>s.trim()).join(' | ');
  log.push({command:'mori '+args.join(' ')+` (cwd ${cwd})`,exit_code:r.status,expected,observed:first||'(see file)',output_file:out});
}
const a=JSON.parse(process.argv[2]);
for(const c of a) run(...c);
const p='command-log.json';
const prev=fs.existsSync(p)?JSON.parse(fs.readFileSync(p)):[];
fs.writeFileSync(p,JSON.stringify(prev.concat(log),null,2));
