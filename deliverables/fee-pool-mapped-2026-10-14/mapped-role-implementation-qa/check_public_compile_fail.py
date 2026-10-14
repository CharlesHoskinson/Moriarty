from pathlib import Path
import subprocess,time,json,re,hashlib
out=Path(__file__).parent
deps=Path('/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target/debug/deps')
lib=max(deps.glob('libmori_ledger-*.rlib'),key=lambda p:p.stat().st_mtime)
cases=[('into-native-artifact','mori_ledger::relation::Artifact','E0277'),('into-native-seed','mori_ledger::local_deployment::AdmittedSeed','E0277'),('into-proving-deployment','mori_ledger::local_deployment::DerivedLocalDeployment','E0277')]
records=[]
for name,target,expected in cases+[('private-bundle-fields',None,'E0451')]:
 source=(f'pub fn promote(bundle: mori_ledger::static_compile::StaticBundle) -> {target} {{ bundle.into() }}\n' if target else '#[allow(unreachable_code)]\npub fn fabricate(output: mori_ledger::call_artifacts::ArtifactLeaves) -> mori_ledger::static_compile::StaticBundle { mori_ledger::static_compile::StaticBundle { output, marker: todo!(), total: 0 } }\n')
 f=out/(name+'.rs');f.write_text(source)
 a=['flock','/tmp/moriarty-language-wave1-build.lock','rustc','+1.99.0','--edition=2024','--crate-type=lib','--crate-name',name.replace('-','_'),'--emit=metadata',str(f),'--extern','mori_ledger='+str(lib),'-L','dependency='+str(deps),'-o',str(out/(name+'.rmeta'))]
 t=time.monotonic();p=subprocess.run(a,text=True,capture_output=True);codes=re.findall(r'error\[(E\d+)\]',p.stderr)
 (out/(name+'.stderr.log')).write_text(p.stderr);(out/(name+'.stdout.log')).write_text(p.stdout)
 record={'argv':a,'elapsed_seconds':time.monotonic()-t,'exit':p.returncode,'expected_error':expected,'observed_error_codes':codes,'expected_refusal_observed':p.returncode==1 and codes==[expected],'source_sha256':hashlib.sha256(f.read_bytes()).hexdigest()}
 (out/(name+'.json')).write_text(json.dumps(record,indent=2)+'\n');records.append(record);print(name,json.dumps(record),flush=True)
 if not record['expected_refusal_observed']: print(p.stderr,flush=True);break
assert len(records)==4 and all(r['expected_refusal_observed'] for r in records)
(out/'public-capability-compile-fail.json').write_text(json.dumps({'checks':records,'native_authority':0,'classification':'compile-only exact Rust privacy/trait errors; no native execution'},indent=2)+'\n')
