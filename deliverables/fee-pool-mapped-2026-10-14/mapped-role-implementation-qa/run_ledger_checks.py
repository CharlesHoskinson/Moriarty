from pathlib import Path
from datetime import datetime, timezone
import subprocess,time,json,hashlib
repo=Path('/home/charl/Moriarty/.worktrees/mapped-role-static-20261014')
out=Path(__file__).parent
prefix=['flock','/tmp/moriarty-language-wave1-build.lock','env','CARGO_TARGET_DIR=/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target','cargo','+1.99.0','test','--offline','--locked','-j2','-p','mori-ledger']
checks=[('mapped-role-ledger',['--test','mapped_role_static','--','--test-threads=1']),('legacy-static-regression',['--test','static_compile','--','--test-threads=1']),('native-all-policy-refusal',['--lib','static_compile::tests::native_all_action_policy_refuses_mapped_debits_before_emission','--','--test-threads=1']),('capability-doctests',['--doc','--','--test-threads=1']),('legacy-escrow-native-refusal',['--test','duty_reads_source','checked_in_full_escrow_refuses_one_public_payer_even_for_later_action','--','--test-threads=1'])]
files=['crates/mori-ledger/src/layout.rs','crates/mori-ledger/src/relation/mod.rs','crates/mori-ledger/src/relation/compiler.rs','crates/mori-ledger/src/static_compile.rs','crates/mori/src/commands/compile_call.rs','crates/mori-ledger/tests/mapped_role_static.rs']
for name,args in checks:
 a=prefix+args;start=datetime.now(timezone.utc).isoformat();before={n:hashlib.sha256((repo/n).read_bytes()).hexdigest() for n in files};t=time.monotonic();p=subprocess.run(a,cwd=repo,text=True,capture_output=True);elapsed=time.monotonic()-t
 (out/(name+'.stdout.log')).write_text(p.stdout);(out/(name+'.stderr.log')).write_text(p.stderr)
 (out/(name+'.json')).write_text(json.dumps({'argv':a,'cwd':str(repo),'started_utc':start,'elapsed_seconds':elapsed,'exit':p.returncode,'files_sha256_before':before,'files_unchanged_during_run':all(hashlib.sha256((repo/n).read_bytes()).hexdigest()==h for n,h in before.items()),'native_authority':0},indent=2)+'\n')
 print(name,'exit',p.returncode,'elapsed',round(elapsed,2),flush=True);print(p.stdout,flush=True)
 if p.stderr: print(p.stderr[-4000:],flush=True)
 if p.returncode: break
