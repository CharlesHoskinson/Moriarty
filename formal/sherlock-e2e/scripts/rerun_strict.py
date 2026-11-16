import json, re, sys, pathlib, glob
S = pathlib.Path(sys.argv[1]); base = pathlib.Path('/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e')
rows = {}
for f in sorted(glob.glob(str(S/'res_*.json'))):
    mod = f.split('res_')[1][:-5]
    rep = (base/mod/'quint'/'REPORT.md').read_text().splitlines()
    out = []
    for r in json.load(open(f)):
        exp_pass = None
        for l in rep[r['line']:r['line']+6]:
            m = re.search(r'(\d+) passing', l)
            if m: exp_pass = int(m.group(1)); break
        s = r['summary']; c = r['cmd']; ok = r['rc'] == r['exp']; why = ''
        if c.startswith('quint run'):
            if r['exp'] == 1 and not ('[violation]' in s or 'Invariant violated' in s): ok = False; why = 'exit 1 without [violation]'
            if r['exp'] == 0 and '[ok]' not in s: ok = False; why = 'exit 0 without [ok]'
        elif c.startswith('quint test'):
            m = re.search(r'(\d+) passing', s)
            got = int(m.group(1)) if m else 0
            if r['exp'] == 0 and (got == 0 or 'failing' in s): ok = False; why = f'passing={got}'
            if exp_pass is not None and got != exp_pass and r['exp'] == 0: ok = False; why = f'passing {got} vs report {exp_pass}'
        elif c.startswith('quint typecheck'):
            if r['rc'] == 0 and re.search(r'error', s, re.I): ok = False; why = 'error text'
        out.append(dict(r, strict=ok, why=why, exp_pass=exp_pass))
    rows[mod] = out
json.dump(rows, open(S/'strict.json','w'), indent=1)
for m, o in rows.items():
    k = {}
    for r in o:
        t = r['cmd'].split()[1]; k.setdefault(t, [0,0]); k[t][0] += r['strict']; k[t][1] += 1
    bad = [ (r['line'], r['why'] or f"rc {r['rc']} vs {r['exp']}", r['summary'][:100]) for r in o if not r['strict']]
    print(m, sum(r['strict'] for r in o), '/', len(o), k, bad)
