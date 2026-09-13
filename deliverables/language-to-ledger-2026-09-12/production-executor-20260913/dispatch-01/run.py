import os
from pathlib import Path
prompt=Path('/tmp/moriarty-production-fable-20260913/prompt.md').read_text()
os.execv('/home/charl/.local/bin/claude',['claude','-p','--model','claude-fable-5-1','--effort','medium','--permission-mode','acceptEdits','--allowedTools','Bash,Read,Write,Edit,Glob,Grep','--output-format','stream-json','--verbose',prompt])
