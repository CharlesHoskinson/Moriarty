"""Freeze finite custody facts, without invoking Cargo or package sources."""
from acquire import ROOT, sha, dump, fetch
import json, subprocess, tomllib

packages = json.loads((ROOT / 'PACKAGES.json').read_text())
for package in packages:
    if package['name'] not in ('lsp-server', 'gen-lsp-types'): continue
    origin = package['archive']['origin']
    origin['status'] = None
    origin['canonicalUrl'] = None
    origin['statusBasis'] = 'Original successful archive source pin records requested URL, retrieval time and checksum, but not HTTP status or redirect URL. These fields are not reconstructed.'
    fresh = fetch(origin['requestedUrl'], 'archive-verification/' + package['name'] + '-' + package['version'] + '.crate')
    assert sha(fresh) == package['archive']['sha256']
    assert fresh == (ROOT / package['archive']['path']).read_bytes()
    package['archive']['freshPublicVerification'] = {'receipt':'RETRIEVALS.json','path':'archive-verification/' + package['name'] + '-' + package['version'] + '.crate','byteIdenticalToReusedArchive':True}
dump('PACKAGES.json', packages)

repo = '/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017'
commit = '74a909d69639df8823a527a4d608a7516ccd69a2'
repo_files = []
for path in ['Cargo.lock','Cargo.toml','crates/mori/Cargo.toml']:
    body = subprocess.run(['git','--no-optional-locks','show',commit + ':' + path],cwd=repo,check=True,stdout=subprocess.PIPE).stdout
    dest = ROOT / 'repo-source' / path; dest.parent.mkdir(parents=True,exist_ok=True); dest.write_bytes(body)
    repo_files.append({'path':'repo-source/' + path,'gitSource':commit + ':' + path,'sha256':sha(body),'bytes':len(body)})
tree = subprocess.run(['git','--no-optional-locks','rev-parse',commit + '^{tree}'],cwd=repo,check=True,stdout=subprocess.PIPE).stdout.decode().strip()
dump('REPO-SOURCE-PINS.json',{'commit':commit,'tree':tree,'basis':'Exact root-provided current main commit blobs; no branch alias or working tree source substitution.','files':repo_files})
lock = tomllib.loads((ROOT / 'repo-source/Cargo.lock').read_text())
normal_edges = []
conditional_edges = []
for package in packages:
    for dep in package['indexEntry']['deps']:
        edge = {'from':package['name'] + '@' + package['version'],'indexDependency':dep}
        if dep['kind'] != 'normal' or dep['optional'] or dep['target'] is not None:
            conditional_edges.append(edge); continue
        name = dep.get('package') or dep['name']
        edge['sameNameCurrentLockIdentities'] = [{k:p[k] for k in ('name','version','source','checksum') if k in p} for p in lock['package'] if p['name'] == name]
        edge['selectionClaim'] = 'None. Same-name lock presence and literal requirement only; not a Cargo solver result or an active unit graph.'
        normal_edges.append(edge)
names = ['lsp-server','gen-lsp-types','crossbeam-channel']
missing = [name for name in names if not any(p['name'] == name for p in lock['package'])]
dump('DECLARATION-AND-LOCK-COMPARISON.json',{'currentCommit':commit,'lockSha256':sha((ROOT / 'repo-source/Cargo.lock').read_bytes()),'proposedDependencies':[{'name':'lsp-server','requirement':'=0.10.0'},{'alias':'lsp-types','name':'gen-lsp-types','requirement':'=0.11.0','defaultFeatures':False,'features':[]}],'matchingNonYankedCrossbeamChannelAlternatives':['0.5.15','0.5.16','0.5.17'],'currentlyAbsentNames':missing,'normalUnconditionalIndexDeclarations':normal_edges,'conditionalOptionalOrDevDeclarations':conditional_edges,'unresolved':['Cargo selection, lock update and resolved transitive versions were not run.','Existing compatible lock identities can be compared manually, but their retention/unification is not established.','Optional URL implementations and dependency dev/test targets are declarations, not part of the adopted no-optional-feature proposal.','Only three new package names are identified by this bounded declaration comparison, not a completeness claim for a future resolved graph.']})

assert len(missing) == 3
for f in json.loads((ROOT / 'PUBLIC-FILE-PROVENANCE.json').read_text()):
    data = (ROOT / f['path']).read_bytes()
    assert sha(data) == f['sha256'] and len(data) == f['bytes']
for receipt in json.loads((ROOT / 'RETRIEVALS.json').read_text()):
    data = (ROOT / receipt['path']).read_bytes()
    assert sha(data) == receipt['sha256'] and len(data) == receipt['bytes'] and receipt['status'] == 200
print(json.dumps({'commit':commit,'tree':tree,'absentNames':missing,'verifiedSourceFiles':159,'sourceBytes':2565027,'freshArchiveComparisons':2},indent=2))
