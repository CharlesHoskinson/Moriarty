"""Write operator-proposals/OPERATOR-RESULT-PARSER.diff: the parser-only revision of the
reviewed operator run_pathb.py (c9cd7237..., read-only) that makes RESULT.json use the same
libtest_rows.py parser as the analysis. Stdlib only; the operator file is read, never
executed or imported; the diff is created exclusively."""
import difflib, hashlib, os, resource

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

SRC = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-env-corrected-offline-execution-20261015/tools/run_pathb.py"
WANT = "c9cd72377f463b28cf2da791364b3f419018e5ff7ba560656983c207b28b28e1"
OUT = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-evidence-repair-resume-source-20261015/operator-proposals/OPERATOR-RESULT-PARSER.diff"

b = open(SRC, "rb").read()
assert hashlib.sha256(b).hexdigest() == WANT
old = b.decode()
new = old

def sub(a, c):
    global new
    assert new.count(a) == 1, a
    new = new.replace(a, c)

sub("import time\n", "import time\nimport types\n")
sub("RV = V / 'current-core-groth16-compiler-repair-independent-review-20261015'\n",
    "RV = V / 'current-core-groth16-compiler-repair-independent-review-20261015'\n"
    "# PROPOSED (resume P-4): one parser for RESULT.json and the analysis. A frozen copy of\n"
    "# operator-proposals/libtest_rows.py; its sha256 must be FREEZE.json\n"
    "# records['tools/libtest_rows.py']. The verified bytes themselves are executed (no\n"
    "# second read), so the hashed file is exactly the parser that runs.\n"
    "LIBTEST_ROWS = T / 'tools' / 'libtest_rows.py'\n"
    "LIBTEST = None\n")
sub("def tests_of(text):\n"
    "    res = {}\n"
    "    for line in text.splitlines():\n"
    "        if line.startswith('test ') and ' ... ' in line:\n"
    "            name, _, outcome = line[5:].partition(' ... ')\n"
    "            res.setdefault(name.strip(), []).append(outcome.strip())\n"
    "    return res\n",
    "def load_libtest_rows(want_sha256):\n"
    "    \"\"\"Executes the FREEZE-pinned parser bytes as a module; None on any mismatch.\"\"\"\n"
    "    b = LIBTEST_ROWS.read_bytes()\n"
    "    if sha_b(b) != want_sha256:\n"
    "        return None\n"
    "    mod = types.ModuleType('libtest_rows')\n"
    "    exec(compile(b, str(LIBTEST_ROWS), 'exec'), mod.__dict__)\n"
    "    return mod\n")
sub("    if sha_b(plan_b) != freeze['records']['OPTION-A-PLAN.json'] or sha(__file__) != freeze['records']['tools/run_pathb.py']:\n"
    "        result['stops'].append('operator or Option A plan differs from FREEZE.json')\n",
    "    if sha_b(plan_b) != freeze['records']['OPTION-A-PLAN.json'] or sha(__file__) != freeze['records']['tools/run_pathb.py']:\n"
    "        result['stops'].append('operator or Option A plan differs from FREEZE.json')\n"
    "    global LIBTEST\n"
    "    LIBTEST = load_libtest_rows(freeze['records'].get('tools/libtest_rows.py'))\n"
    "    if LIBTEST is None:\n"
    "        result['stops'].append('libtest_rows.py differs from FREEZE.json: no step runs')\n")
sub("        for sid in ('E4', 'E5-r2'):\n"
    "            p = RUN / f'{sid}.stdout'\n"
    "            if p.exists():\n"
    "                text = p.read_text(errors='replace')\n"
    "                result.setdefault('tests', {})[sid] = {'outcomes': tests_of(text),\n"
    "                                                       'summaries': [l for l in text.splitlines() if l.startswith('test result:')]}\n"
    "                if sid == 'E5-r2':\n"
    "                    result['e5_identity_lines'] = [l for l in text.splitlines() if l.startswith('identity ')]\n",
    "        for sid in ('E4', 'E5-r2'):\n"
    "            p = RUN / f'{sid}.stdout'\n"
    "            if p.exists() and LIBTEST is not None:\n"
    "                text = p.read_text(errors='replace')\n"
    "                # Same parse_stream as the analysis: header split first, outcome is the\n"
    "                # terminal token, identity lines include the inline first line.\n"
    "                result.setdefault('tests', {})[sid] = LIBTEST.result_tests(text)\n"
    "                if sid == 'E5-r2':\n"
    "                    result['e5_identity_lines'] = result['tests'][sid]['identity_lines']\n")

text = "".join(difflib.unified_diff(old.splitlines(keepends=True), new.splitlines(keepends=True),
                                    fromfile="run_pathb.py (reviewed, c9cd7237)", tofile="run_pathb.py (PROPOSED parser-only revision)", n=3))
header = ("# PROPOSED, UNREVIEWED, NEVER APPLIED. Parser-only revision of the reviewed operator.\n"
          "# Step names (E5-r2 -> E5-r3), paths and the r3 step list belong to EXECUTION-PROPOSAL.json\n"
          "# and to a separately reviewed operator; this diff changes only how RESULT.json parses\n"
          "# libtest output. The operator itself is never edited in place.\n")
fd = os.open(OUT, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
with os.fdopen(fd, "w") as f:
    f.write(header + text)
print(hashlib.sha256((header + text).encode()).hexdigest(), len(text.splitlines()))
