import re, sys, json, pathlib
root = pathlib.Path(sys.argv[1])
out = {}
for rep in sorted(root.glob('*/quint/REPORT.md')):
    mod = rep.parent.parent.name
    lines = rep.read_text().splitlines()
    cmds = []
    for i, l in enumerate(lines):
        s = l.strip()
        m = re.match(r'^(?:-\s*)?(?:Command:\s*)?`?(quint (?:typecheck|run|test|verify)\b[^`]*)`?\s*$', s)
        if not m:
            continue
        cmd = m.group(1).strip()
        exp = None
        for j in range(i+1, min(i+6, len(lines))):
            t = lines[j]
            if re.match(r'^\s*(?:-\s*Command:|quint )', t.strip()):
                break
            mm = re.search(r'[Ee]xit(?: code)?[:`\s]*`?\s*([0-9])', t)
            if mm:
                exp = int(mm.group(1)); break
        cmds.append({'cmd': cmd, 'exp': exp, 'line': i+1})
    out[mod] = cmds
json.dump(out, sys.stdout, indent=1)
