from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import subprocess,json
root=Path(__file__).resolve().parent
repos=[("https://github.com/minswap/sdk","minswap-sdk"),("https://github.com/SundaeSwap-finance/sundae-sdk","sundae-sdk"),("https://github.com/fallen-icarus/cardano-swaps","cardano-swaps"),("https://github.com/fallen-icarus/cardano-aftermarket","cardano-aftermarket"),("https://github.com/fallen-icarus/cardano-options","cardano-options"),("https://github.com/spectrum-finance/cardano-dex-contracts","spectrum-contracts"),("https://github.com/spectrum-finance/cardano-dex-backend","spectrum-backend"),("https://github.com/MuesliSwapTeam/muesliswap-liquidity-efficient-dex","muesli-hybrid-dex"),("https://github.com/Liqwid-Labs/agora","agora")]
def acquire(item):
 url,name=item
 command=["python3",str(root/"acquire.py"),"--dir",str(root/"reference-app-snapshots"),"repo",url,name]
 result=subprocess.run(command,capture_output=True,text=True,timeout=260)
 print(json.dumps({"name":name,"exit_code":result.returncode,"output":result.stdout[-800:]}),flush=True)
with ThreadPoolExecutor(max_workers=3) as pool:list(pool.map(acquire,repos))
