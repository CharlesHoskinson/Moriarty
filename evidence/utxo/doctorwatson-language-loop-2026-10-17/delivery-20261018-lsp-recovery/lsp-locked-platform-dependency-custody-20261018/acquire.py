"""Finite lock-pinned public source acquisition; no package source execution."""
from pathlib import Path, PurePosixPath
from datetime import datetime, timezone
import hashlib, json, tomllib, tarfile, io, subprocess
from scrapling.fetchers import Fetcher

R = Path(__file__).parent
I = Path('/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017')
TARGETS = [('objc2','0.6.5'),('rustls-platform-verifier-android','0.2.0'),('wasm-bindgen-futures','0.4.79'),('web-sys','0.3.106'),('zerocopy-derive','0.8.61')]
CAP = 30_000_000
def sha(data): return hashlib.sha256(data).hexdigest()
def dump(path,value):
    dest=R/path;dest.parent.mkdir(parents=True,exist_ok=True)
    dest.write_text(json.dumps(value,indent=2,ensure_ascii=False)+'\n')
receipts=[]
def fetch(url,path):
    started=datetime.now(timezone.utc).isoformat()
    response=Fetcher.get(url,timeout=30)
    data=response.body
    dest=R/path;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(data)
    receipts.append({'requestedUrl':url,'canonicalUrl':str(response.url),'status':response.status,'requestStartedAt':started,'retrievedAt':datetime.now(timezone.utc).isoformat(),'path':path,'bytes':len(data),'sha256':sha(data),'transport':'Scrapling Fetcher.get','credentialsSupplied':False})
    dump('RETRIEVALS.json',receipts)
    if response.status!=200:raise RuntimeError(f'HTTP {response.status}: {url}')
    return data

lock_bytes=(I/'Cargo.lock').read_bytes()
(R/'INPUT-Cargo.lock').write_bytes(lock_bytes)
lock=tomllib.loads(lock_bytes.decode())
registry=[p for p in lock['package'] if p.get('source','').startswith('registry+')]
pins=[]
for name,version in TARGETS:
    candidates=[p for p in registry if p['name']==name and p['version']==version]
    assert len(candidates)==1,(name,version,candidates)
    pin=candidates[0]
    assert pin['source']=='registry+https://github.com/rust-lang/crates.io-index'
    assert len(pin['checksum'])==64
    pins.append(pin)
head=subprocess.run(['git','--no-optional-locks','rev-parse','HEAD'],cwd=I,check=True,stdout=subprocess.PIPE).stdout.decode().strip()
dump('LOCK-PINS.json',{'capturedAt':datetime.now(timezone.utc).isoformat(),'originalLockPath':str(I/'Cargo.lock'),'lockCopy':'INPUT-Cargo.lock','lockSha256':sha(lock_bytes),'lockBytes':len(lock_bytes),'observedHead':head,'registryPackageIdentities':len(registry),'targets':pins,'selection':'Exact identities supplied by root and confirmed against actual current lock; no version choice or dependency resolution.'})
packages=[];files=[];omissions=[];total=0
for pin in pins:
    name=pin['name'];version=pin['version'];identity=name+'-'+version
    archive_path='archives/'+identity+'.crate'
    archive=fetch(f'https://static.crates.io/crates/{name}/{identity}.crate',archive_path)
    if sha(archive)!=pin['checksum']:raise RuntimeError('Cargo.lock checksum mismatch: '+identity)
    version_path='registry/'+identity+'.json'
    meta=json.loads(fetch(f'https://crates.io/api/v1/crates/{name}/{version}',version_path))['version']
    assert meta['crate']==name and meta['num']==version and meta['checksum']==pin['checksum']
    fetch(f'https://crates.io/api/v1/crates/{name}/{version}/dependencies','registry/'+identity+'-dependencies.json')
    members=[];seen=set();package_files=[]
    with tarfile.open(fileobj=io.BytesIO(archive),mode='r:gz') as tar:
        for member in tar.getmembers():
            path=PurePosixPath(member.name)
            if member.name.startswith('/') or '\\' in member.name or '..' in path.parts or not path.parts or path.parts[0]!=identity:
                raise RuntimeError('Unsafe archive member path: '+member.name)
            if member.name in seen:raise RuntimeError('Duplicate member: '+member.name)
            seen.add(member.name)
            if member.isdir():continue
            if not member.isfile():
                omissions.append({'package':identity,'member':member.name,'reason':'nonregular member; not followed'})
                continue
            if member.size>CAP:raise RuntimeError('Source member exceeds cap: '+member.name)
            data=tar.extractfile(member).read();assert len(data)==member.size
            try:data.decode('utf-8')
            except UnicodeDecodeError:
                omissions.append({'package':identity,'member':member.name,'bytes':len(data),'sha256':sha(data),'reason':'non-UTF-8 archive input retained in full checksum-verified archive, not extracted as text'})
                continue
            if b'\0' in data:
                omissions.append({'package':identity,'member':member.name,'bytes':len(data),'sha256':sha(data),'reason':'NUL-containing archive input retained in full checksum-verified archive, not extracted as text'})
                continue
            if total+len(data)>CAP:raise RuntimeError('30MB source cap exceeded')
            output='source/'+member.name
            dest=R/output;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(data)
            file={'path':output,'bytes':len(data),'sha256':sha(data),'originalMember':member.name,'package':identity,'archivePath':archive_path,'archiveSha256':sha(archive),'cargoLockChecksum':pin['checksum'],'registryVersionChecksum':meta['checksum']}
            files.append(file);package_files.append(file);total+=len(data)
    manifest_path=R/'source'/identity/'Cargo.toml'
    manifest=tomllib.loads(manifest_path.read_text())
    packages.append({'identity':identity,'lockPin':pin,'archivePath':archive_path,'archiveBytes':len(archive),'archiveSha256':sha(archive),'lockAndRegistryChecksumMatch':True,'registryVersionPath':version_path,'registryVersion':meta,'sourceFiles':len(package_files),'sourceBytes':sum(f['bytes'] for f in package_files),'manifestDeclarations':{'package':manifest.get('package',{}),'lib':manifest.get('lib',{}),'features':manifest.get('features',{}),'buildDependencies':manifest.get('build-dependencies',{}),'target':manifest.get('target',{})},'buildRsFiles':[f['path'] for f in package_files if f['path'].endswith('/build.rs')],'selectionClaim':False})
    dump('PACKAGES.json',packages);dump('PUBLIC-FILE-PROVENANCE.json',files);dump('OMISSIONS.json',omissions)
assert sha((I/'Cargo.lock').read_bytes())==sha(lock_bytes)
dump('SOURCE-COUNTS.json',{'packages':len(packages),'sourceFiles':len(files),'sourceBytes':total,'sourceCapBytes':CAP,'omissions':len(omissions),'originalLockStillSame':True})
print(json.dumps({'packages':[(p['identity'],p['sourceFiles'],p['sourceBytes']) for p in packages],'sourceFiles':len(files),'sourceBytes':total,'omissions':omissions},indent=2))
