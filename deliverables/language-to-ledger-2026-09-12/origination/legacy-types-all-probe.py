from pathlib import Path
import subprocess,tempfile,json,sys
candidate=Path(sys.argv[1]);baseline=Path('/home/charl/Moriarty/.worktrees/financial-postconditions')
exports=[('funded-expression-source-v1.ts','FundedExpressionPrepared',False),('funded-expression-source-v1.ts','FundedExpressionResult',False),('funded-expression-source-v1.ts','createFundedFinancialExpressionSourceV1',True),('financial-expression-v1.ts','createFinancialExpressionContractV1',True),('financial-expression-v1.ts','createFinancialExpressionContractV2',True),('financial-expression-v1.ts','createFinancialExpressionContractV3',True)]+[(f'financial-agreement-source-v{v}.ts',f'createFinancialAgreementSourceV{v}',True) for v in range(1,5)]
with tempfile.TemporaryDirectory(prefix='moriarty-all-legacy-types-') as temp:
 p=Path(temp);lines=['type Equal<A,B>=(<T>()=>T extends A?1:2) extends (<T>()=>T extends B?1:2)?true:false;','type Assert<T extends true>=T;']
 for i,(file,name,factory) in enumerate(exports):
  for label,root in [('Base',baseline),('New',candidate)]:
   path=root/'experiments/moriarty-language/src/successor'/file
   lines.append(f'import type {{ {name} as {label}{i} }} from {json.dumps(str(path))};')
  t=lambda label:f'ReturnType<ReturnType<typeof {label}{i}>["evaluate"]>' if factory else f'{label}{i}'
  lines.append(f'type Check{i}=Assert<Equal<{t("Base")},{t("New")}>>;')
 consumer=p/'consumer.mts';consumer.write_text('\n'.join(lines)+'\n')
 config=p/'tsconfig.json';config.write_text(json.dumps({'extends':str(candidate/'experiments/moriarty-language/tsconfig.json'),'include':[str(consumer)]}))
 run=subprocess.run(['tsc','-p',str(config)],text=True,capture_output=True)
 print(json.dumps({'passed':run.returncode==0,'checkedExports':[x[1] for x in exports],'exitCode':run.returncode,'diagnostics':run.stdout+run.stderr},indent=2));sys.exit(run.returncode)
