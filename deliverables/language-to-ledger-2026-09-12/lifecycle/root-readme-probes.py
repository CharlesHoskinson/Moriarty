import pathlib,sys,re,shlex,subprocess,json
root=pathlib.Path(sys.argv[1]);text=(root/'README.md').read_text();commands=[]
for block in re.findall(r'^```(?:sh|bash)\n([\s\S]*?)^```$',text,re.M):
 for line in block.splitlines():
  if line.startswith('node experiments/moriarty-language/src/cli.ts ') and 'examples/loan-lifecycle.mori' in line:commands.append(shlex.split(line))
  if line=='npm --prefix experiments/moriarty-language run loan-lifecycle-demo':commands.append(shlex.split(line))
assert len(commands)==4 and {c[2] for c in commands if c[0]=='node'}=={'check','format','simulate'}
rows=[]
for cmd in commands:
 r=subprocess.run(cmd,cwd=root,text=True,capture_output=True,timeout=30);assert r.returncode==0,r.stderr
 if cmd[0]=='npm':
  payload=json.loads(r.stdout[r.stdout.index('{'):]);assert payload['settled']['financialPost']['obligations'][0]['outstanding']=='0';assert len(payload['calls'])==6
 elif cmd[2]=='check':assert json.loads(r.stdout)['judgmentResult']=='SourceChecked'
 elif cmd[2]=='simulate':assert json.loads(r.stdout)['result']['financialPost']['obligations'][0]['outstanding']=='100'
 rows.append({'command':cmd,'exitCode':r.returncode,'stdout':r.stdout,'stderr':r.stderr})
print(json.dumps({'passed':True,'commandsPassed':len(rows),'rows':rows},indent=2))
