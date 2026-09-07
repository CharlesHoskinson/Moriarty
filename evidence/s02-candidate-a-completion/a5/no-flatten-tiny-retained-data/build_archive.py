import sys
if sys.flags.optimize or not sys.dont_write_bytecode:
    raise RuntimeError('builder requires optimization zero and -B')
import hashlib, io, json, re, tarfile
from pathlib import Path, PurePosixPath
HERE=Path(__file__).resolve().parent
ROOT=Path('/home/charl/Moriarty/.worktrees/s01-audit-start')
S=ROOT/'.superpowers/sdd'
Q=Path('/home/charl/.npm-global/lib/node_modules/@informalsystems/quint')
PROPOSAL=S/'a5-no-flatten-tiny-package-proposal-20260906.md'
ORIGINAL_INDEX=S/'a5-no-flatten-tiny-independent-index-20260906.json'
ADMISSION=S/'a5-no-flatten-diagnostic-20260906/root-admission-tiny-retained-type-application.json'
def need(ok,message):
    if not ok:raise ValueError(message)
def digest(b):return hashlib.sha256(b).hexdigest()
def checked(path,expected=None):
    p=Path(path);need(p.is_file() and not p.is_symlink(),'regular original')
    b=p.read_bytes();row={'path':str(p),'bytes':len(b),'sha256':digest(b)}
    if expected is not None:need(row==expected,'changed original: '+str(p))
    return row,b
def read(path,sha):
    row,b=checked(path);need(row['sha256']==sha,'control pin');return json.loads(b)
def expand(path):
    need(path[:2] in ('S/','Q/'),'finite extra prefix')
    return str((S if path[:2]=='S/' else Q)/path[2:])
def archive_name(path):
    p=PurePosixPath(path)
    need(p.is_absolute() and str(p)==path and '..' not in p.parts,'normalized original absolute path')
    name=path[1:];need(name and not PurePosixPath(name).is_absolute(),'relative archive path');return name
support=['README.md','build_archive.py','audit.py','adopted-proposal-sha256.txt']
need({p.name for p in HERE.iterdir()}==set(support),'exclusive initial package files')
adopted=(HERE/'adopted-proposal-sha256.txt').read_text()
need(re.fullmatch('[0-9a-f]{64}\n',adopted) is not None,'actual adopted proposal digest')
proposal_row,proposal_bytes=checked(PROPOSAL)
need(proposal_row['sha256']==adopted.strip(),'exact adopted proposal bytes')
base=read(ORIGINAL_INDEX,'a0310464e1382cfe0786012af951bb37c86e0a17d5825d21d72cfb4eff89d69e')['originals']
admission=read(ADMISSION,'09eeff84e1f750b4ffae73dc44173aae058a5343f3bf64801be4480ab848e0ac')
need(len(base)==746 and len(admission['files'])==284,'closed admitted counts')
wanted={}
def add(row):
    p=row['path'];need(set(row)=={'path','bytes','sha256'},'exact pin schema')
    archive_name(p);need(p not in wanted or wanted[p]==row,'conflicting pin');wanted[p]=row
for path,row in base.items():need(path==row['path'],'original index path');add(row)
for row in admission['files']:add(row)
need(len(wanted)==758,'exact admitted union')
section=proposal_bytes.decode().split('<!-- tiny-package-extras -->\n```json\n',1)[1].split('\n```',1)[0]
extras=json.loads(section);need(len(extras)==25,'exact additional originals')
for name,size,h in extras:
    p=expand(name);need(p not in wanted,'extra already present');add({'path':p,'bytes':size,'sha256':h})
need(len(wanted)==783 and sum(r['bytes'] for r in wanted.values())==35860565,'fixed existing closure')
need(str(PROPOSAL) not in wanted,'proposal is separate last member');add(proposal_row)
runtime_archive=str(S/'a5-factoring-receipts/tool-store/runtime.tar.gz')
python_archive=str(S/'a4-checker-task1-receipts/python-environment.tar.gz')
need(runtime_archive not in wanted and python_archive not in wanted,'external archives excluded')
for name in wanted:
    need('/a5-no-flatten-full-supplement' not in name and '/a5-no-flatten-diagnostic-20260906/full-' not in name and
         '/a5-no-flatten-diagnostic-20260906/root-dispatch-full.json' not in name and
         '/a5-no-flatten-diagnostic-20260906/transport/full/' not in name and
         '/a4-producer-receipts/a5-noflat-full-20260906/' not in name,'no future full originals')
archive=HERE/'original-evidence.tar.gz';members=[]
with tarfile.open(archive,'x:gz') as tar:
    for path,row in sorted(wanted.items()):
        _,b=checked(path,row);need(len(b)<=16*1024*1024,'bounded existing member')
        name=archive_name(path);m=tarfile.TarInfo(name);m.size=len(b);m.mode=0o644
        tar.addfile(m,io.BytesIO(b));members.append({'originalPath':path,'archivePath':name,'bytes':len(b),'sha256':digest(b)})
for row in wanted.values():checked(row['path'],row)
need(len(members)==784,'exact archive count')
archive_row,_=checked(archive);archive_row['path']=archive.name
supports=[]
for name in support:
    row,_=checked(HERE/name);row['path']=name;supports.append(row)
index={'schema':'moriarty.a5-no-flatten-tiny-retained-data/v1','proposalSha256':adopted.strip(),
       'archive':archive_row,'members':members,'packageSupport':supports,
       'externalRuntimeArchives':[
         {'path':runtime_archive,'sha256':'f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c'},
         {'path':python_archive,'sha256':'7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac'}],
       'limits':{'existingOriginals':783,'archiveMembers':784,'existingOriginalBytes':35860565},
       'scope':'lossless original tiny failed gate plus separately admitted retained-data correction; no full result',
       'externalRuntimeAudit':'recorded bindings only; archives and installed trees not read',
       'rootToolAvailability':'583fc6 and b31f8d remain transcript-only references; admission files preserved; no reconstruction'}
with (HERE/'index.json').open('x') as stream:json.dump(index,stream,indent=2,sort_keys=True);stream.write('\n')
print(json.dumps({'ok':True,'archive':archive_row,'members':len(members),'indexSha256':digest((HERE/'index.json').read_bytes()),
                  'originalFailedGateRetained':True,'correctedTinyCapabilityOnly':True,'nativeInvocations':0}))
