from pathlib import Path
import subprocess
import tempfile
import json
import sys

candidate = Path(sys.argv[1])
baseline = Path('/home/charl/Moriarty/.worktrees/financial-postconditions')
rows = []
with tempfile.TemporaryDirectory(prefix='moriarty-legacy-type-') as directory:
    for label, checkout in [('baseline', baseline), ('candidate', candidate)]:
        package = checkout / 'experiments/moriarty-language'
        consumer = Path(directory) / (label + '.mts')
        consumer.write_text(
            'import {createFinancialAgreementSourceV4} from '
            + json.dumps(str(package / 'src/successor/financial-agreement-source-v4.ts'))
            + ";\nconst result=createFinancialAgreementSourceV4().evaluate('', '', '', '');\n"
            + "if(result.status==='FundedExpressionPrepared'){for(const effect of result.effects){"
            + "switch(effect.kind){case 'Transfer':break;case 'Repayment':break;"
            + 'default:{const impossible:never=effect;void impossible;}}}}\n'
        )
        config = Path(directory) / (label + '.json')
        config.write_text(json.dumps({
            'extends': str(package / 'tsconfig.json'),
            'include': [str(package / 'src/**/*.ts'), str(consumer)],
        }))
        result = subprocess.run(['tsc', '-p', str(config)], capture_output=True, text=True)
        rows.append({'tree': label, 'exitCode': result.returncode,
                     'diagnostics': result.stdout + result.stderr})
print(json.dumps({'passed': all(row['exitCode'] == 0 for row in rows), 'results': rows}, indent=2))
sys.exit(0 if all(row['exitCode'] == 0 for row in rows) else 1)
