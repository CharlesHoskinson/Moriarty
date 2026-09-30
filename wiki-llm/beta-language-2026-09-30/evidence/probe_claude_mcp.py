import subprocess,json,time,hashlib,shutil
from pathlib import Path
root=Path('/home/charl/research/moriarty-beta-2026-09-30');project=Path('/tmp/moriarty-beta-final-consumer-v5-20260930')
source=Path('/tmp/moriarty-beta-developers-v3-20260930/S1/inv/invoice.mori').read_text()
config=project/'mcp-probe.json';config.write_text(json.dumps({'mcpServers':{'moriarty':{'command':shutil.which('node'),'args':[str(project/'node_modules/@moriarty-lang/beta/dist/cli.js'),'mcp']}}})+'\n')
prompt='Use the configured Moriarty MCP check and inspect tools on the EXACT source text below. Make exactly these two read-only calls. Return only status and open-gate summary. Do not use files, shell, other tools, or alter source. This is a local authoring activation probe, never signing or settlement. SOURCE:\n'+source
args=['claude','--model','claude-sonnet-5-5','--tools','','--allowed-tools','mcp__moriarty__check,mcp__moriarty__inspect','--permission-mode','bypassPermissions','--setting-sources','','--disable-slash-commands','--strict-mcp-config','--mcp-config',str(config),'--no-session-persistence','--max-turns','4','--output-format','stream-json','--verbose','-p',prompt]
start=time.time();r=subprocess.run(args,cwd=project,capture_output=True,text=True)
records=[]
for line in r.stdout.splitlines():
 try:d=json.loads(line)
 except json.JSONDecodeError:continue
 if d.get('type')=='system' and d.get('subtype')=='init':records.append({k:d.get(k) for k in ('type','subtype','model','mcp_servers','session_id')})
 if d.get('type') in ('assistant','user'):
  for block in d.get('message',{}).get('content',[]):
   if isinstance(block,dict) and block.get('type')=='tool_use' and block.get('name','').startswith('mcp__moriarty__'):records.append({k:block.get(k) for k in ('type','id','name','input')})
   if isinstance(block,dict) and block.get('type')=='tool_result':records.append({k:block.get(k) for k in ('type','tool_use_id','is_error','content')})
 if d.get('type')=='result':records.append({k:d.get(k) for k in ('type','subtype','is_error','result','num_turns','session_id','modelUsage')})
out={'schema':'moriarty-native-provider-activation/1','requested_model':'claude-sonnet-5-5','requested_effort':'default','process_exit_code':r.returncode,'elapsed_seconds':round(time.time()-start,1),'configuration':'explicit ephemeral project MCP config; no global edits; built-in tools disabled; only check/inspect allowed','input_sha256':hashlib.sha256(source.encode()).hexdigest(),'scope':'native Claude Code MCP activation for authoring check/inspect only; financial execution gates remain open','records':records}
(root/'claude-mcp-activation-v5.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({'exit_code':r.returncode,'record_types':[x.get('type') for x in records],'tool_names':[x.get('name') for x in records if x.get('type')=='tool_use']}))
