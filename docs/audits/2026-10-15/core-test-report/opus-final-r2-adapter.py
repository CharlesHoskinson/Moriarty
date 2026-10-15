"""One-shot report adapter: retain only final public advice and model identity."""
import json,subprocess,sys
from pathlib import Path
prompt,report=sys.argv[1:3]
cmd=['claude','-p','--model','claude-opus-5-5','--effort','high','--output-format','json','--tools','','--permission-mode','plan','--safe-mode','--no-session-persistence','--strict-mcp-config','--mcp-config','{"mcpServers":{}}','--add-dir','/home/charl/Moriarty/.moriarty-dev/runtime/core-test-report-20261015','--add-dir','/home/charl/Moriarty/.worktrees/core-test-report-20261015']
p=subprocess.run(cmd,input=Path(prompt).read_bytes(),stdout=subprocess.PIPE,stderr=subprocess.PIPE)
try:d=json.loads(p.stdout)
except Exception:d={}
result=d.get('result','');usage=d.get('modelUsage',{});identity=usage.get('claude-opus-5-5',{}).get('canonicalModel');ok=p.returncode==0 and not d.get('is_error') and bool(result) and identity=='claude-opus-5-5'
if ok:
 Path(report).write_text(result+'\n');Path(report).with_suffix('.json').write_text(json.dumps({'report_kind':'independent-security-full-candidate-and-actual-result-review','requested_model':'claude-opus-5-5','requested_effort':'high','returned_model_usage':usage,'is_error':False,'source_commit':'95df73558ab4fb0b6d80618a76e88c3a52c38888','text':result},indent=2)+'\n')
public={'text':result if result else 'No completed public advice returned','modelUsage':usage,'isError':not ok,'stopReason':'completed' if ok else 'error'}
print(json.dumps(public));sys.exit(0 if ok else 1)
