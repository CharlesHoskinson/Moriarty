"""Reviewer stdlib byte checks. Reads only allowlisted paths (phase 1/2) and the parent candidate
files listed by its SOURCE-MANIFEST once that manifest's hash equals the pinned a6e17efd.
Imports no candidate/operator/reviewer code. Writes one JSON result (exclusive create)."""
import hashlib, json, sys, datetime

OUT = '/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-evidence-repair-independent-source-review-20261015/'
B = '/home/charl/moriarty-eutxo-validation-2026-10-12/'
A = B + 'current-core-groth16-evidence-repair-resume-source-20261015/'
SR = B + 'current-core-groth16-compiler-independent-source-review-20261015/'
RR = B + 'current-core-groth16-compiler-repair-independent-review-20261015/'
EX = B + 'current-core-groth16-env-corrected-offline-execution-20261015/'
RV = B + 'current-core-groth16-independent-result-review-20261015/'
P = B + 'current-core-groth16-compiler-review-repair-source-20261015/'
NEVER = {'PROMPT.md', 'PROCESS.json', 'FINAL-RESPONSE.md', 'PREPARATION.json', 'run.py',
         'provider-result.private.json', 'process.stderr', 'operator.stderr'}


def sha(path):
    name = path.rsplit('/', 1)[-1]
    if name in NEVER or name.endswith('.stderr'):
        raise SystemExit('refused to open ' + path)
    with open(path, 'rb') as f:
        b = f.read()
    return hashlib.sha256(b).hexdigest(), len(b)


def canon(o):
    return json.dumps(o, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode()


res = {'at_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'checks': {}}
C = res['checks']

# 1. author output manifest
om_path = A + 'OUTPUT-MANIFEST.json'
C['author_output_manifest_sha'] = sha(om_path)[0]
om = json.load(open(om_path))
skip = {'inherited/old-partial/FINAL-RESPONSE.md', 'inherited/old-partial/PROCESS.json'}
mism = []
for f in om['files']:
    if f['path'] in skip:
        continue
    h, n = sha(A + f['path'])
    if (h, n) != (f['sha256'], f['bytes']):
        mism.append(f['path'])
C['author_files_checked'] = len(om['files']) - len(skip)
C['author_mismatches'] = mism
C['author_aggregate_recomputed'] = hashlib.sha256(canon(om['files'])).hexdigest()
C['author_aggregate_declared'] = om['aggregate_sha256']

# 2. prompt pins
pins = {
    A + 'REPORT.md': 'e72f1aa3de7805a68c8eecfb9897f4ba02c2e3270d6956f79e88c227b43a1fda',
    A + 'OUTPUT-MANIFEST.json': '99589a043fca136e5fd630e463441845b2872dbdf1816ad1bc1e2b115355c1c9',
    B + 'current-core-groth16-evidence-repair-root-verification-20261015/VERIFICATION.json':
        '77064c8bd603b088d54fea192fd3a0c05de0ea3daa5adee35668a4cb89ef4d81',
    A + 'candidate/SOURCE-MANIFEST.json': 'd3bdd5009e42e13110559142a7d62ba2823f0723690ae1094033e7684b09005c',
    A + 'candidate/BUILD-MANIFEST.json': 'edbe3193c8eabc51a1d5595757c69c0bbe2caeb89e53cc3dc25564cead44b41a',
    A + 'review-overlay/R3-MANIFEST.json': '640678296f54ec9eca5dedcdcf2f931546e4608131f14bb6d6b60d4c7b5576aa',
    A + 'EXECUTION-PROPOSAL.json': '2d803e50585c6d8f0d5065e90f6c8a0260357fc40a2cbcdb2e4af52dfafa51d6',
    RV + 'REPORT.md': '06e316bcf575c9258c292c29c84712ab2be7ff21be735add8c236f0be47d6ece',
    RR + 'plumbing-r2/R2-MANIFEST.json': '5a256c766908ff521b6a6cf483d1b109d17aa2d931a63598ab39b183cc2b580b',
    SR + 'REVIEW-MANIFEST.json': '757ed0bfc5804eec8bac3f1db063b1239ead0cdbeccbc5fa3ec3160a1a6ab151',
    B + 'upstream-sherlock-4d969-20261015/MANIFEST.json':
        '186c4907f7439952d60f6d024203d612a262578bacaebca39ba8ff264f459037',
    P + 'SOURCE-MANIFEST.json': 'a6e17efd0a47194c70ac6e4467efce2d47f9254dc47a8e596f6add2b92aef97e',
    P + 'BUILD-MANIFEST.json': '4e3ba2e2796b425598b9b2abc022c4d6782bf032044aa53e124af4946c543b47',
    EX + 'REPORT.md': '18afb040ff7b961543f5b8dc478faa42044a4e4129f4f6dfed58349d1ac84cc1',
    EX + 'OUTPUT-MANIFEST.json': '386d5cde38c62940ab37bfbfd9aab4f4f2866663255c7081463fcc1c4b4a1dc0',
    RV + 'proposals/FIX-PROPOSALS.md': 'd7d3e40278a6e4a3a721003d0f6afd072f41ffa11d4bd254161f6e0a5fa2f4b7',
}
C['prompt_pins'] = {p.replace(B, ''): (sha(p)[0] == h) for p, h in pins.items()}

# 3. lineage manifests
def check_list(root, files):
    bad = []
    for f in files:
        h, n = sha(root + f['path'])
        if (h, n) != (f['sha256'], f['bytes']):
            bad.append(f['path'])
    return bad

C['source_review_manifest_mismatch'] = check_list(SR, json.load(open(SR + 'REVIEW-MANIFEST.json'))['files'])
C['r2_manifest_mismatch'] = check_list(RR + 'plumbing-r2/', json.load(open(RR + 'plumbing-r2/R2-MANIFEST.json'))['files'])
exm = json.load(open(EX + 'OUTPUT-MANIFEST.json'))
C['old_run_manifest_mismatch'] = check_list(
    EX, [dict(path=k, **v) for k, v in exm['files'].items()
         if not k.endswith('.stderr') and 'listing' not in k])
r3 = json.load(open(A + 'review-overlay/R3-MANIFEST.json'))
C['r3_manifest_mismatch'] = check_list(A + 'review-overlay/', r3['files'])
C['r3_count'] = len(r3['files'])

# 4. candidate SOURCE/BUILD manifests
sm = json.load(open(A + 'candidate/SOURCE-MANIFEST.json'))
C['candidate_source_entries'] = len(sm['files'])
C['candidate_source_mismatch'] = check_list(A + 'candidate/', sm['files'])
C['candidate_sidecar'] = open(A + 'candidate/SOURCE-MANIFEST.json.sha256').read().strip()
bm = json.load(open(A + 'candidate/BUILD-MANIFEST.json'))
C['candidate_build_entries'] = len(bm['files'])
C['candidate_build_mismatch'] = check_list(A + 'candidate/', bm['files'])
C['candidate_source_names_build'] = sm.get('build_manifest_sha256')

# 5. parent comparison
psm = json.load(open(P + 'SOURCE-MANIFEST.json'))
C['parent_source_entries'] = len(psm['files'])
C['parent_source_mismatch'] = check_list(P, psm['files'])
pmap = {f['path']: f['sha256'] for f in psm['files']}
cmap = {f['path']: f['sha256'] for f in sm['files']}
C['delta_vs_parent'] = {
    'unchanged': sum(1 for k in cmap if pmap.get(k) == cmap[k]),
    'changed': sorted(k for k in cmap if k in pmap and pmap[k] != cmap[k]),
    'added': sorted(k for k in cmap if k not in pmap),
    'removed': sorted(k for k in pmap if k not in cmap),
}
pbm = json.load(open(P + 'BUILD-MANIFEST.json'))
pb = {f['path']: f['sha256'] for f in pbm['files']}
cb = {f['path']: f['sha256'] for f in bm['files']}
C['build_delta_vs_parent'] = sorted(k for k in set(pb) | set(cb) if pb.get(k) != cb.get(k))
C['build_format_keys_equal'] = (pbm['format'] == bm['format'],
                                sorted(pbm) == sorted(bm),
                                sorted(pbm['files'][0]) == sorted(bm['files'][0]))
tests = sorted(k for k in cmap if k.startswith('source/mori-r1cs/tests/'))
C['test_files'] = {k: (cmap[k] == pmap.get(k)) for k in tests}
srcs = sorted(k for k in cmap if k.startswith('source/mori-r1cs/src/'))
C['src_files_equal_parent'] = {k: (cmap[k] == pmap.get(k)) for k in srcs}
# history copy of origin-r2 vs actual origin bytes named by the candidate history
# 6. compiler digest predictions
core = 'f47318a26a4df0477e89962a9de0e9b48f12d01d0e9b4a8f5e74603beaa235ed'
def digest(bmsha):
    return hashlib.sha256(canon({'compiler_source_manifest_sha256': bmsha,
                                 'core_closure_sha256': core})).hexdigest()
C['digest_control_parent'] = digest('4e3ba2e2796b425598b9b2abc022c4d6782bf032044aa53e124af4946c543b47')
C['digest_predicted_candidate'] = digest(sha(A + 'candidate/BUILD-MANIFEST.json')[0])

with open(OUT + 'work/VERIFY-RESULT.json', 'x') as f:
    json.dump(res, f, indent=1, sort_keys=True)
print(json.dumps(res, indent=1, sort_keys=True))
