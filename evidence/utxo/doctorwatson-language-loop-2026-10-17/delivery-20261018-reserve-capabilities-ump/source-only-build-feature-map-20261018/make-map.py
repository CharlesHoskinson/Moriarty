"""Finite manifest/lock source inventory, not a Cargo resolver or unit graph."""
from pathlib import Path
import json,tomllib,hashlib,re,shutil

R=Path(__file__).parent
BASE=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
sha=lambda b:hashlib.sha256(b).hexdigest()
def dump(path,value): (R/path).write_text(json.dumps(value,indent=2,ensure_ascii=False)+'\n')
def identity(p):return {k:p[k] for k in ('name','version','source','checksum') if k in p}
all_data={}
for label in ['main-e4','draft-pr220']:
 root=R/'source'/label
 workspace=tomllib.loads((root/'Cargo.toml').read_text())
 lock=tomllib.loads((root/'Cargo.lock').read_text())['package']
 manifests={tomllib.loads(p.read_text())['package']['name']:(p,tomllib.loads(p.read_text())) for p in (root/'crates').glob('*/Cargo.toml')}
 facts=[]
 for name,(path,manifest) in manifests.items():
  for kind in ['dependencies','dev-dependencies','build-dependencies']:
   for alias,raw in manifest.get(kind,{}).items():
    spec={'version':raw} if isinstance(raw,str) else dict(raw)
    inherited=workspace.get('workspace',{}).get('dependencies',{}).get(alias)
    base=({'version':inherited} if isinstance(inherited,str) else dict(inherited or {})) if spec.get('workspace') else {}
    merged=base|spec
    merged['features']=sorted(set(base.get('features',[])+spec.get('features',[])))
    package=merged.get('package',alias)
    facts.append({'from':name,'manifest':str(path.relative_to(R)),'kind':kind,'alias':alias,'package':package,'literalDeclaration':raw,'workspaceDeclaration':inherited if spec.get('workspace') else None,'effectiveLiteralFields':merged,'optional':merged.get('optional',False),'localPackage':package if package in manifests else None,'sameNameLockIdentities':[identity(p) for p in lock if p['name']==package]})
 all_data[label]={'workspaceResolver':workspace['workspace']['resolver'],'packageFeatures':{name:m.get('features',{}) for name,(p,m) in manifests.items()},'dependencyDeclarationFacts':facts,'repositoryBuildRsPaths':[],'claim':'Exact manifest fields and lock identity references only; no Cargo resolution or selected compilation units.'}
 # Follow only unconditional LOCAL NORMAL declarations; optional additions below are explicit scenario premises.
 local_edges={name:[f['localPackage'] for f in facts if f['from']==name and f['kind']=='dependencies' and not f['optional'] and f['localPackage']] for name in manifests}
 def local_walk(start):
  seen=set();queue=list(start)
  while queue:
   p=queue.pop()
   if p in seen:continue
   seen.add(p);queue.extend(local_edges.get(p,[]))
  return sorted(seen)
 configs={
  'core-normal':{'localRoots':['mori-core'],'rootDev':[],'features':[]},
  'core-tests':{'localRoots':['mori-core'],'rootDev':['mori-core'],'features':[]},
  'cli-normal-no-default':{'localRoots':['mori'],'rootDev':[],'features':[]},
  'cli-tests-no-default':{'localRoots':['mori'],'rootDev':['mori'],'features':[]},
  'cli-tests-no-default-core2-disjoint':{'localRoots':['mori'],'rootDev':['mori'],'features':['mori/core2-disjoint-draft','mori-core/core2-disjoint-draft']},
  'cli-tests-no-default-source6':{'localRoots':['mori','mori-source6'],'rootDev':['mori'],'features':['mori/source6-expansion','mori-source6/source6-transfer-slice']},
 }
 config_rows=[]
 for config,premise in configs.items():
  local=local_walk(premise['localRoots'])
  decl=[f for f in facts if (f['from'] in local and f['kind']=='dependencies' and not f['optional'] and not f['localPackage']) or (f['from'] in premise['rootDev'] and f['kind']=='dev-dependencies' and not f['localPackage'])]
  direct=[];unknown=[]
  for d in decl:
   candidates=d['sameNameLockIdentities']
   literal=d['effectiveLiteralFields'].get('version','').lstrip('=^')
   exact=[p for p in candidates if p['version']==literal]
   # Equality to the written version is an identity-reference shortcut, not a semver/Cargo solution.
   if len(exact)==1:chosen=exact
   elif len(candidates)==1:chosen=candidates
   else:
    chosen=candidates;unknown.append({'declaration':d,'reason':'Multiple or absent same-name lock identities; no resolver choice made.'})
   direct.extend(chosen)
  keys=lambda p:(p['name'],p['version'],p.get('source',''))
  direct=list({keys(p):p for p in direct}.values())
  seen={};queue=list(direct);edge_unknown=[]
  while queue:
   node=queue.pop();key=keys(node)
   if key in seen:continue
   seen[key]=node
   original=next(p for p in lock if keys(p)==key)
   for ref in original.get('dependencies',[]):
    pieces=ref.split(' ');name=pieces[0]
    version=pieces[1] if len(pieces)>1 and not pieces[1].startswith('(') else None
    source=next((p[1:-1] for p in pieces[1:] if p.startswith('(')),None)
    targets=[p for p in lock if p['name']==name and (version is None or p['version']==version) and (source is None or p.get('source')==source)]
    if len(targets)!=1:edge_unknown.append({'from':node,'literalLockDependency':ref,'candidateIdentities':[identity(p) for p in targets]})
    queue.extend(identity(p) for p in targets)
  config_rows.append({'scenario':config,'premises':premise,'localNormalDeclarationReachability':local,'directExternalDeclarationFacts':decl,'directLockIdentityReferences':sorted(direct,key=keys),'lockEdgeReachabilityEnvelope_NOT_SELECTED_UNITS':sorted(seen.values(),key=keys),'unresolvedDirectIdentityChoices':unknown,'unresolvedLockReferenceNamespaces':edge_unknown,'limits':'The lock omits feature, target and dependency-kind distinctions. This envelope can include inactive optional/platform/dev edges and is not an active Cargo graph.'})
 dump(label+'-SCENARIO-DECLARATIONS.json',config_rows)
 tests=json.loads((R/(label+'-TEST-DECLARATIONS.json')).read_text())
 target_rows=[]
 for pkg in ['mori','mori-core']:
  manifest=manifests[pkg][1]
  explicit={t['name']:t for t in manifest.get('test',[])}
  for test in tests:
   path=test['gitPath']
   if not path.startswith('crates/'+pkg+'/tests/') or path.count('/')!=3:continue
   name=Path(path).stem;spec=explicit.get(name,{})
   headers=[x for x in test['declarationLines'] if x['text'].startswith('#![')]
   inner=[x for x in test['declarationLines'] if '#[cfg' in x['text'] and not x['text'].startswith('#![')]
   target_rows.append({'package':pkg,'target':name,'gitPath':path,'requiredFeatures':spec.get('required-features',[]),'fileCfgHeaders':headers,'innerCfgDeclarations':inner,'fullFileSha256':test['fullFileSha256'],'scope':'Source target/module availability only, no test behavior/security/resource evaluation.'})
 dump(label+'-TEST-TARGET-AVAILABILITY.json',target_rows)
 # Only normal Layout A unit tests; no hostile/security tests are inspected for judgments.
 unit=[]
 for path in sorted((root/'crates/mori-core/src/sherlock/layout_a').glob('*normal_tests.rs')):
  text=path.read_text();names=[{'name':m.group(1),'line':text[:m.start()].count('\n')+1} for m in re.finditer(r'(?m)^fn (\w+)\(',text) if '#[test]' in text[max(0,m.start()-80):m.start()]]
  unit.append({'path':str(path.relative_to(R)),'sha256':sha(path.read_bytes()),'tests':names,'containingModulePath':'mori_core::sherlock::layout_a','outerCfg':'sherlock/mod.rs:73 #[cfg(test)] gates layout_a; each normal_tests module is also cfg(test); no native feature gate.'})
 dump(label+'-NORMAL-ROUNDTRIP-UNIT-DECLARATIONS.json',unit)
dump('WORKSPACE-MANIFEST-FACTS.json',all_data)

# Reuse only existing finite public metadata/owner records; do not touch cache or tool files.
inputs=[('compiler-toolchain-metadata/TOOL-METADATA.json','native-tool-metadata.json'),('compiler-one-check-candidate/TOOL-CONFIG-METADATA.json','historical-tool-config-metadata.json'),('compiler-one-check-candidate/full-inputs/doctorwatson-core-actual-result-20261018-packet/attempt-1/PRE-ATTEMPT-TOOL-METADATA.json','rust-tool-pre-attempt-metadata.json'),('compiler-observed-build-inventory/BUILD-PROC-MACRO-CANDIDATES.json','historical-backend-declaration-candidates.json'),('compiler-build-root-owner-packet/PER-FILE-PROVENANCE.json','build-root-source-provenance.json')]
provenance=[]
for original,name in inputs:
 b=(BASE/original).read_bytes();dest=R/'existing-public-receipts'/name;dest.parent.mkdir(exist_ok=True);dest.write_bytes(b)
 provenance.append({'originalPath':str(BASE/original),'copiedPath':str(dest.relative_to(R)),'bytes':len(b),'sha256':sha(b),'kind':'Historical public source/tool metadata; not a fresh observation or source-only CLI Cargo selection.'})
dump('EXISTING-RECEIPT-PINS.json',provenance)
old=json.loads((BASE/'compiler-observed-build-inventory/BUILD-PROC-MACRO-CANDIDATES.json').read_text())
owner=json.loads((BASE/'compiler-build-root-owner-packet/PER-FILE-PROVENANCE.json').read_text())
surface={}
for label in all_data:
 configs=json.loads((R/(label+'-SCENARIO-DECLARATIONS.json')).read_text())
 surface[label]=[]
 for c in configs:
  nodes={(p['name'],p['version'],p.get('source'),p.get('checksum')) for p in c['lockEdgeReachabilityEnvelope_NOT_SELECTED_UNITS']}
  candidates=[]
  for p in old['packages']:
   x=p['identity'];key=(x['name'],x['version'],x['full_source_identity'],x['lock_checksum'])
   if key not in nodes:continue
   sources=[f for f in owner['copied_files'] if f['identity']['metadata_package_id']==x['metadata_package_id']]
   candidates.append({'identity':x,'historicalCustomBuildDeclarations':p['custom_build_targets_as_declared'],'historicalProcMacroDeclarations':p['proc_macro_targets_as_declared'],'preservedBuildSourceReferences':[{'path':str(BASE/'compiler-build-root-owner-packet'/f['output_path']),'sha256':f['sha256'],'category':f['category']} for f in sources],'qualification':'Exact identity intersection with a lock-edge envelope. Not selected or executed for this CLI/Core selector; historical backend feature strings are not reused.'})
  surface[label].append({'scenario':c['scenario'],'boundedHistoricalDeclarationCandidates':candidates,'unknowns':'Historical packet covers only previously observed backend identities; absent packages may still declare build/proc macro targets. No exhaustive selected surface claimed.'})
dump('BUILD-PROC-MACRO-REFERENCE-SURFACE.json',surface)
print(json.dumps({'localPackages':len(manifests),'scenarioCount':len(config_rows),'sourceRepoBuildRs':0,'normalRoundtripModules':len(unit),'historicalReceipts':len(provenance)}))
