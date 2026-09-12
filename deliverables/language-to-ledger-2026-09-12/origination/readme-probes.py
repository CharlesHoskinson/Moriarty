import sys,pathlib,re,subprocess,json,shlex
root=pathlib.Path(sys.argv[1]);text=(root/'README.md').read_text();pkg=root/'experiments/moriarty-language'
grammar=(pkg/'spec/successor/financial-agreement-source-v5-grammar.ebnf').read_text();blocks=re.findall(r'^```ebnf\n([\s\S]*?)^```$',text,re.M)
assert grammar in blocks,'README does not reproduce canonical /5 EBNF exactly'
commands=[]
for block in re.findall(r'^```(?:sh|bash)\n([\s\S]*?)^```$',text,re.M):
 for line in block.splitlines():
  if line.startswith('node experiments/moriarty-language/src/cli.ts ') and 'moriarty-financial-agreement-source/5' in line:
   commands.append(shlex.split(line))
assert {c[2] for c in commands}>={'check','format','simulate'},'README missing real /5 CLI forms'
results=[]
for command in commands:
 run=subprocess.run(command,cwd=root,text=True,capture_output=True,timeout=30)
 assert run.returncode==0,{'command':command,'exit':run.returncode,'stderr':run.stderr}
 if command[2]=='check':assert json.loads(run.stdout)['judgmentResult']=='SourceChecked'
 if command[2]=='simulate':assert json.loads(run.stdout)['result']['status']=='FundedExpressionPrepared'
 results.append({'command':shlex.join(command),'exitCode':run.returncode,'stdout':run.stdout,'stderr':run.stderr})
demo=subprocess.run(['npm','--prefix','experiments/moriarty-language','run','financial-lifecycle-demo'],cwd=root,text=True,capture_output=True,timeout=30)
assert demo.returncode==0,demo.stderr
results.append({'command':'npm --prefix experiments/moriarty-language run financial-lifecycle-demo','exitCode':demo.returncode,'stdout':demo.stdout,'stderr':demo.stderr})
print(json.dumps({'canonicalGrammarMatched':True,'commandsPassed':len(results),'results':results},indent=2))
