import os
from pathlib import Path
prompt=Path('/tmp/moriarty-production-fable-20260913/correct-01-prompt.md').read_text()
os.execv('/home/charl/.local/bin/claude',['claude','-p','--resume','fa5644fc-e1d1-44a9-b4eb-654e94fc5baa','--model','claude-fable-5-1','--effort','medium','--permission-mode','acceptEdits','--allowedTools','Bash,Read,Write,Edit,Glob,Grep','--output-format','stream-json','--verbose',prompt])
