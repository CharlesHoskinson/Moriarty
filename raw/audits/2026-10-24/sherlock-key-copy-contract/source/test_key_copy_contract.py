"""Public synthetic regression: SDK06 staged decider-key copies versus Root-adopted keygen02 originals.
Executes only the receiver's top-level definitions; all descriptors are synthetic DATA and no descriptor path is opened.
KEY_COPY_RECEIVER=<path> selects the receiver source under test (old baseline: RED; overlay: GREEN). Stdlib unittest only.
"""
import copy,os,unittest
from pathlib import Path
SRC=Path(os.environ.get('KEY_COPY_RECEIVER',str(Path(__file__).resolve().parent/'actual_wallet_receiving.py')))
R={'__name__':'actual_call_receiving','__file__':str(SRC)};exec(compile(SRC.read_bytes(),str(SRC),'exec'),R)
VK=(2189,'bd9e9d89ad0a5570571160b29dc8b2d89e11e3c6065b46638a2e978f9218776e');REL=(2806,'08b7f6db1db1d7143bfa7660dbb29c8070683cfca5757d690857f2e0a657785f')
ORIG='/synthetic/keygen02/key-output01/decider-key';PREP='/synthetic/sdk06/revision06';RUN=PREP+'/private-runtime01';KEYS=RUN+'/decider-key';BUILD='/synthetic/build9'
CONTROL=PREP+'/EXACT-CONTROL-AND-OPERATIONS.HELD.json';ROLES=[('rawDeciderVK','decider.vk'),('deciderRelation','decider.relation')]
def row(p,v):return dict(path=p,bytes=v[0],sha256=v[1])
def fixture():
 adoption=row('/synthetic/root-keygen02/ADOPTION.json',(2370,'1'*64));products=[row(ORIG+'/decider.vk',VK),row(ORIG+'/decider.relation',REL)]
 build=[row(BUILD+'/keys/acceptValidity.verifier',(1353,'f2d10277b0e853e37e448e1a57c97ab6d3edcf6f5d15cab8673c8708136796f4')),row(BUILD+'/keys/settleValidity.verifier',(1353,'5'*64))]+[row(BUILD+'/other%02d'%i,(i+1,'%064x'%(i+1))) for i in range(11)]
 control=dict(buildDirectory=BUILD,buildMountFiles=copy.deepcopy(build),preparationRoot=PREP,privateRuntimeRoot=RUN,stagedKeyDir=KEYS,deciderConfigurationPath=RUN+'/decider-config.json',materialConfiguration=row('/synthetic/material02/EXACT-CONTROL-AND-OPERATIONS.HELD.json',(3,'3'*64)))
 return dict(
  cfg=dict(buildDirectory=BUILD,buildMountFiles=build,actualKeygenAdoption=adoption),
  kad=dict(products=products),
  mcfg=dict(keyProducts=copy.deepcopy(products),actualKeygenAdoption=dict(adoption)),
  control=control,
  freeze=dict(files=[row(PREP+'/deploy_wallet_receiving.py',(18704,'7'*64)),row(CONTROL,(268702,R['SDK06_CONTROL']))]),
  ad=dict(configuration_sha256=R['SDK06_CONTROL'],sourceFreeze=row(PREP+'/SOURCE-FREEZE.json',(6933,'4'*64)),deciderConfiguration=row(RUN+'/decider-config.json',(2839,'6'*64))),
  rc=dict(key_dir=KEYS,settle_verifier_sha256='5'*64),
  b=dict(rawDeciderVK=row(KEYS+'/decider.vk',VK),deciderRelation=row(KEYS+'/decider.relation',REL)))
def pinned(f):
 # Caller-pin stand-in: exact descriptor -> synthetic DATA; any other descriptor refuses.
 table={f['ad']['sourceFreeze']['path']:(f['ad']['sourceFreeze'],f['freeze']),CONTROL:(f['freeze']['files'][1],f['control']),f['control']['materialConfiguration']['path']:(f['control']['materialConfiguration'],f['mcfg'])}
 def pinned_json(r):
  d,v=table[r['path']];assert r==d;return v
 return pinned_json
def chain(f):
 # Same order as binding(): adopted_build's authenticated SDK06 control, then the typed key-copy check.
 p=pinned(f);control=R['adopted_build'](f['cfg'],f['ad'],f['rc'],p)
 return R['adopted_key_copies'](f['b'],f['kad'],control,p({k:control['materialConfiguration'][k] for k in ['path','bytes','sha256']}),f['ad'],f['rc'],f['cfg'])
def at(k,**v):return lambda f:f['b'][k].update(v)
def moved(f):
 f['control'].update(stagedKeyDir='/synthetic/elsewhere/decider-key');f['rc'].update(key_dir='/synthetic/elsewhere/decider-key')
 for k,n in ROLES:f['b'][k].update(path='/synthetic/elsewhere/decider-key/'+n)
REFUSE=[
 ('copy at arbitrary path',at('rawDeciderVK',path='/synthetic/tmp/decider.vk')),
 ('copy is the keygen02 origin row',lambda f:f['b'].update(rawDeciderVK=dict(f['kad']['products'][0]))),
 ('copy at the other role name',at('rawDeciderVK',path=KEYS+'/decider.relation')),
 ('key roles swapped',lambda f:f['b'].update(rawDeciderVK=row(KEYS+'/decider.vk',REL))),
 ('wrong copy sha',at('deciderRelation',sha256='0'*64)),
 ('wrong copy bytes',at('rawDeciderVK',bytes=2188)),
 ('untyped copy bytes',at('rawDeciderVK',bytes=2189.0)),
 ('extra copy field',at('rawDeciderVK',mode=0o444)),
 ('ambiguous duplicate origin',lambda f:f['kad']['products'].append(row('/synthetic/keygen03/decider.vk',VK))),
 ('missing origin role',lambda f:f['kad']['products'].pop()),
 ('material02 froze another origin',lambda f:f['mcfg']['keyProducts'][0].update(path='/synthetic/keygen01/decider.vk')),
 ('material02 under another keygen adoption',lambda f:f['mcfg']['actualKeygenAdoption'].update(sha256='2'*64)),
 ('origin differs from material02',lambda f:f['kad']['products'][1].update(sha256='0'*64)),
 ('adopted rc key_dir elsewhere',lambda f:f['rc'].update(key_dir=PREP+'/decider-key')),
 ('staged dir outside SDK06 private runtime',moved),
 ('adopted decider config is not the SDK06 staged one',lambda f:f['ad']['deciderConfiguration'].update(path=PREP+'/decider-config.json')),
 ('control preparation root not the source-freeze root',lambda f:f['control'].update(preparationRoot='/synthetic/sdk05')),
 ('unauthenticated SDK06 control',lambda f:f['ad'].update(configuration_sha256='0'*64)),
]
class KeyCopyContract(unittest.TestCase):
 def test_adopted_build_returns_authenticated_sdk06_control(self):
  f=fixture();self.assertIs(R['adopted_build'](f['cfg'],f['ad'],f['rc'],pinned(f)),f['control'])
 def test_legitimate_staged_copy_different_path_exact_origin_identity(self):
  f=fixture();key=chain(f);self.assertEqual(key,{'decider.vk':f['kad']['products'][0],'decider.relation':f['kad']['products'][1]})
  for k,n in ROLES:
   # The former full-row predicate is false for this legitimate copy; bytes/sha identity is exact.
   self.assertNotEqual(f['b'][k],key[n]);self.assertEqual((f['b'][k]['bytes'],f['b'][k]['sha256']),(key[n]['bytes'],key[n]['sha256']))
 def test_refusals(self):
  for name,mutate in REFUSE:
   with self.subTest(name):
    f=fixture();mutate(f);self.assertRaises(AssertionError,chain,f)
if __name__=='__main__':unittest.main()
