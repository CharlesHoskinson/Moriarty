"""Finite public source custody only. Never invoke package tools or source hooks."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib, json, sys, tarfile, io, shutil, tomllib, subprocess
from scrapling.fetchers import Fetcher

ROOT = Path(__file__).parent
def sha(data): return hashlib.sha256(data).hexdigest()
def dump(path, obj):
    (ROOT / path).write_text(json.dumps(obj, indent=2, ensure_ascii=False) + '\n')
def fetch(url, path):
    began = datetime.now(timezone.utc).isoformat()
    response = Fetcher.get(url, timeout=30)
    body = response.body
    if not isinstance(body, bytes): body = bytes(body)
    dest = ROOT / path
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_bytes(body)
    record = {'requestedUrl': url, 'canonicalUrl': str(response.url), 'status': response.status,
              'requestStartedAt': began, 'retrievedAt': datetime.now(timezone.utc).isoformat(),
              'path': path, 'bytes': len(body), 'sha256': sha(body), 'transport': 'Scrapling Fetcher.get',
              'credentialsSupplied': False}
    receipts = json.loads((ROOT / 'RETRIEVALS.json').read_text()) if (ROOT / 'RETRIEVALS.json').exists() else []
    receipts.append(record)
    dump('RETRIEVALS.json', receipts)
    if response.status != 200: raise RuntimeError(f'HTTP {response.status}: {url}')
    return body

if __name__ == '__main__' and sys.argv[1] == 'index':
    entries = {}
    for name, suffix in [('lsp-server', 'ls/p-/lsp-server'), ('gen-lsp-types', 'ge/n-/gen-lsp-types'), ('crossbeam-channel', 'cr/os/crossbeam-channel')]:
        body = fetch('https://index.crates.io/' + suffix, 'index/' + name + '.jsonl')
        rows = [json.loads(line) for line in body.splitlines() if line.strip()]
        if name == 'crossbeam-channel':
            selected = [r for r in rows if len(r['vers'].split('.')) == 3 and r['vers'].startswith('0.5.') and r['vers'].split('.')[-1].isdigit() and int(r['vers'].split('.')[-1]) >= 15]
        else:
            version = {'lsp-server':'0.10.0', 'gen-lsp-types':'0.11.0'}[name]
            selected = [r for r in rows if r['vers'] == version]
        entries[name] = selected
    dump('INDEX-VERSION-ENTRIES.json', entries)
    print(json.dumps({n:[{'version':r['vers'], 'yanked':r['yanked'], 'checksum':r['cksum']} for r in rows] for n, rows in entries.items()}, indent=2))

if __name__ == '__main__' and sys.argv[1] == 'packages':
    old = Path('/home/charl/moriarty-c2-opus55-restored-20261017/rust-lsp-first-capability-source-proposal/public-dependencies')
    (ROOT / 'original-provenance').mkdir()
    for filename in ['PUBLIC-RETRIEVALS.json', 'PROPOSED-SOURCE-PINS.json']:
        shutil.copyfile(old / filename, ROOT / 'original-provenance' / filename)
    original = json.loads((old / 'PROPOSED-SOURCE-PINS.json').read_text())
    indices = json.loads((ROOT / 'INDEX-VERSION-ENTRIES.json').read_text())
    records, files, declarations, omitted = [], [], [], []
    total = 0
    for name, entries in indices.items():
        fetch(f'https://crates.io/api/v1/crates/{name}', f'registry/{name}.json')
        for entry in entries:
            if entry['yanked']: continue
            version = entry['vers']; identity = name + '-' + version
            meta = json.loads(fetch(f'https://crates.io/api/v1/crates/{name}/{version}', f'registry/{identity}.json'))['version']
            deps = json.loads(fetch(f'https://crates.io/api/v1/crates/{name}/{version}/dependencies', f'registry/{identity}-dependencies.json'))
            assert meta['num'] == version and meta['crate'] == name
            assert meta['checksum'] == entry['cksum'] and meta['yanked'] == entry['yanked']
            archive_path = 'archives/' + identity + '.crate'
            archive_dest = ROOT / archive_path; archive_dest.parent.mkdir(exist_ok=True)
            reused = name in ('lsp-server', 'gen-lsp-types')
            if reused:
                prior = next(p for p in original['packages'] if p['name'] == name and p['version'] == version)
                data = (old / (identity + '.crate')).read_bytes()
                assert sha(data) == prior['checksum'] == entry['cksum']
                archive_dest.write_bytes(data)
                archive_origin = {'kind':'byte-reused prior public retrieval', 'originalArchivePath':str(old / (identity + '.crate')), 'originalRetrievedAt':prior['retrievedAt'], 'requestedUrl':prior['url'], 'canonicalUrl':None, 'status':None, 'statusBasis':'Original source pin does not record HTTP status or redirect URL; these are not reconstructed.', 'priorPinPath':'original-provenance/PROPOSED-SOURCE-PINS.json'}
            else:
                url = f'https://static.crates.io/crates/{name}/{identity}.crate'
                data = fetch(url, archive_path)
                archive_origin = {'kind':'fresh public retrieval', 'receipt':'RETRIEVALS.json', 'requestedUrl':url}
            assert sha(data) == meta['checksum'] == entry['cksum']
            text_files = []
            with tarfile.open(fileobj=io.BytesIO(data), mode='r:gz') as archive:
                seen = set()
                for member in archive.getmembers():
                    parts = member.name.split('/')
                    if member.name.startswith('/') or any(p in ('..', '') for p in parts) or parts[0] != identity or '\\' in member.name:
                        raise RuntimeError('unsafe member path: ' + member.name)
                    if member.name in seen: raise RuntimeError('duplicate archive path: ' + member.name)
                    seen.add(member.name)
                    if member.isdir(): continue
                    if not member.isfile():
                        omitted.append({'package':identity,'member':member.name,'reason':'not a regular file; never followed'})
                        continue
                    if member.size > 30_000_000: raise RuntimeError('member exceeds source cap')
                    body = archive.extractfile(member).read()
                    assert len(body) == member.size
                    try: body.decode('utf-8')
                    except UnicodeDecodeError:
                        omitted.append({'package':identity,'member':member.name,'bytes':len(body),'sha256':sha(body),'reason':'non UTF-8 text'})
                        continue
                    if b'\0' in body:
                        omitted.append({'package':identity,'member':member.name,'bytes':len(body),'sha256':sha(body),'reason':'NUL-containing binary candidate'})
                        continue
                    if total + len(body) > 30_000_000: raise RuntimeError('30MB source cap exceeded')
                    text_files.append((member.name, body))
            for member_name, body in text_files:
                path = 'source/' + member_name
                dest = ROOT / path; dest.parent.mkdir(parents=True,exist_ok=True); dest.write_bytes(body)
                total += len(body)
                files.append({'path':path, 'bytes':len(body),'sha256':sha(body),'package':identity,'archivePath':archive_path,'archiveSha256':sha(data),'archiveMember':member_name,'indexEntryChecksum':entry['cksum'],'registryChecksum':meta['checksum'],'archiveOriginKind':archive_origin['kind']})
            manifest = tomllib.loads((ROOT / 'source' / identity / 'Cargo.toml').read_text())
            declarations.append({'package':identity,'manifestPath':'source/' + identity + '/Cargo.toml','packageDeclarations':manifest.get('package',{}),'libDeclarations':manifest.get('lib',{}),'features':manifest.get('features',{}),'dependencies':manifest.get('dependencies',{}),'devDependencies':manifest.get('dev-dependencies',{}),'buildDependencies':manifest.get('build-dependencies',{}),'targetDeclarations':manifest.get('target',{}),'buildRsMembers':[m for m,b in text_files if m.endswith('/build.rs')], 'claim':'Literal declarations only; no feature/target/unit graph resolution or source execution.'})
            records.append({'name':name,'version':version,'sourceNamespace':'registry+https://github.com/rust-lang/crates.io-index','indexEntry':entry,'registryVersion':meta,'registryDependencies':deps,'archive':{'path':archive_path,'bytes':len(data),'sha256':sha(data),'indexAndRegistryChecksumMatch':True,'origin':archive_origin},'sourceFiles':len(text_files),'sourceBytes':sum(len(b) for _,b in text_files),'resolutionStatus':'unresolved source candidate; not selected by Cargo'})
    dump('PACKAGES.json',records)
    dump('PUBLIC-FILE-PROVENANCE.json',files)
    dump('MANIFEST-DECLARATIONS.json',declarations)
    dump('OMISSIONS.json',omitted)
    dump('SOURCE-COUNTS.json',{'packages':len(records),'sourceFiles':len(files),'sourceBytes':total,'sourceCapBytes':30_000_000,'omissions':len(omitted)})
    print(json.dumps({'packages':[(p['name'],p['version'],p['sourceFiles'],p['sourceBytes']) for p in records],'sourceFiles':len(files),'sourceBytes':total,'omissions':len(omitted)},indent=2))
