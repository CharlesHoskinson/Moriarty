"""Fetch public primary sources with the user-requested Scrapling CLI."""
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import subprocess
SOURCES = [
 ('fflonk-gadget','Midnight in-circuit fflonk at PR 575 merge','https://raw.githubusercontent.com/midnightntwrk/midnight-zk/1e39c12/circuits/src/verifier/fflonk.rs'),
 ('fflonk-pcs','Midnight fflonk PCS at PR 575 merge','https://raw.githubusercontent.com/midnightntwrk/midnight-zk/1e39c12/proofs/src/poly/fflonk/mod.rs'),
 ('fflonk-benchmark','Midnight fflonk implementation PR 565','https://github.com/midnightntwrk/midnight-zk/pull/565'),
 ('micronova','MicroNova folding-based arguments','https://eprint.iacr.org/2024/2099'),
 ('neutronnova','NeutronNova folding','https://eprint.iacr.org/2024/1606'),
 ('vega','Vega prover implementation','https://github.com/microsoft/vega-prover'),
 ('vega-neutronnova','Vega NeutronNova folding implementation guide','https://microsoft.github.io/vega-prover/building-blocks/neutronnova.html'),
 ('flip','FLIP-and-Prove R1CS','https://eprint.iacr.org/2024/1364'),
 ('foreign-field','Efficient emulation of elliptic curve arithmetic','https://eprint.iacr.org/2025/695'),
]
ROOT=Path('raw/s7-recursion')
def collect(item):
 slug,title,url=item
 target=ROOT/(slug+'.md')
 result=subprocess.run(['scrapling','extract','get',url,str(target),'--timeout','40'],capture_output=True,text=True)
 if result.returncode:
  return {'slug':slug,'exit':result.returncode,'error':result.stderr[-500:]}
 body=target.read_text()
 target.write_text(f'# {title}\n> Source: {url}\n> Collected: 2026-10-26\n> Published: Unknown\n\n'+body)
 return {'slug':slug,'bytes':len(body),'status':result.stderr[-150:]}
if __name__=='__main__':
 with ThreadPoolExecutor(max_workers=4) as pool:
  for result in pool.map(collect,SOURCES): print(result,flush=True)
