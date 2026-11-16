import json, sys, subprocess, re, shlex, pathlib, time
S = pathlib.Path(sys.argv[1]); mod = sys.argv[2]
base = pathlib.Path('/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e')
cmds = json.load(open(S/'cmds.json'))[mod]
res = []
for c in cmds:
    cmd = c['cmd']
    cmd = re.sub(r"--out-itf[= ]'?([^' ]+)'?", lambda m: "--out-itf '%s/itf/%s/t_{seq}.itf.json'" % (S, mod), cmd)
    (S/'itf'/mod).mkdir(parents=True, exist_ok=True)
    t0 = time.time()
    try:
        p = subprocess.run(cmd, shell=True, cwd=base/mod/'quint', capture_output=True, text=True, timeout=900)
        rc = p.returncode; outp = (p.stdout + p.stderr)
    except subprocess.TimeoutExpired:
        rc = 'TIMEOUT'; outp = ''
    last = [l for l in outp.splitlines() if re.search(r'\[ok\]|\[violation\]|passing|failing|error|Error', l)]
    res.append({**c, 'rc': rc, 'agree': rc == c['exp'], 'secs': round(time.time()-t0,1), 'summary': (last[-1] if last else outp.strip().splitlines()[-1] if outp.strip() else '')[:200]})
    if rc != c['exp']:
        (S/'fail').mkdir(exist_ok=True)
        open(S/'fail'/f"{mod}_{c['line']}.log",'w').write(cmd+'\n'+outp)
json.dump(res, open(S/f'res_{mod}.json','w'), indent=1)
print(mod, sum(r['agree'] for r in res), '/', len(res))
