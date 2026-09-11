#!/usr/bin/env python3
"""Pinned-source arithmetic/encoding discriminator; no Rust/native/circuit execution."""
from pathlib import Path
import hashlib,json,re
D=Path(__file__).resolve().parent
E=json.loads((D/'source-evidence.json').read_text())
for r in E['sources']:
 assert hashlib.sha256((D/r['retainedPath']).read_bytes()).hexdigest()==r['sha256']
p=int(re.search(r'0x([0-9a-f]{96})',(D/'sources/midnight-curves-0.3.1/src/bls12_381/fp.rs').read_text()).group(1),16)
# Exact decisive branches of BLST's compressed decoder. Other finite points
# deliberately remain undecided: do not pretend this is its full subgroup decoder.
def prefix_status(b):
 if len(b)!=48:return 'REJECT_LENGTH'
 if not b[0]&128:return 'REJECT_COMPRESSED_FLAG'
 if b[0]&64:return 'IDENTITY' if b[0]==192 and not any(b[1:]) else 'REJECT_INFINITY_ENCODING'
 x=int.from_bytes(bytes([b[0]&31])+b[1:],'big')
 if x>=p:return 'REJECT_NONCANONICAL_X'
 if x==0:return 'REJECT_BLST_X_ZERO'
 return 'UNDECIDED_CURVE_AND_SUBGROUP'
def packed(n):return [n%(1<<224),n>>224]
def host_identity_shortcut(fields):return bool(fields) and fields[-1]==1
canonical_identity_pi=packed(p-1)+packed(p-1)+[1]
def strict_identity_pi(fields):return fields==canonical_identity_pi
cases=[]
for name,b,want in [('identity',bytes([192])+bytes(47),'IDENTITY'),('identity-sign',bytes([224])+bytes(47),'REJECT_INFINITY_ENCODING'),('identity-payload',bytes([192])+bytes(46)+bytes([1]),'REJECT_INFINITY_ENCODING'),('missing-compressed',bytes([64])+bytes(47),'REJECT_COMPRESSED_FLAG'),('short',bytes([192])+bytes(46),'REJECT_LENGTH'),('x-modulus',bytes([128|(p>>376)])+(p&((1<<376)-1)).to_bytes(47,'big'),'REJECT_NONCANONICAL_X'),('order3-x0-sign0',bytes([128])+bytes(47),'REJECT_BLST_X_ZERO'),('order3-x0-sign1',bytes([160])+bytes(47),'REJECT_BLST_X_ZERO')]:
 got=prefix_status(b);assert got==want;cases.append({'id':name,'bytesHex':b.hex(),'observedReferenceResult':got})
for fields in [[1],[0,0,0,0,1],canonical_identity_pi+[1]]:
 assert host_identity_shortcut(fields) and not strict_identity_pi(fields)
assert host_identity_shortcut(canonical_identity_pi) and strict_identity_pi(canonical_identity_pi)
# Foreign field decoder reconstructs +1 modulo p, so range-valid packed words
# are not sufficient to assert a unique export: p-1 and 2p-1 both decode to0.
assert ((p-1)+1)%p==((2*p-1)+1)%p==0
assert packed(p-1)!=packed(2*p-1) and 2*p-1 < 1<<392
# Independently expand each category, then compare closed formulas. The values
# below are illustrative shapes, not an observed/frozen production circuit.
examples=[]
for a,L,P,d,T,H,chunks,C,R in [(7,2,8,5,1,4,[2,3],0,2),(0,0,0,3,0,1,[],1,0)]:
 perm=(P+d-3)//(d-2)
 old_parts=[a,2*L,L,perm,T,1,H,1,1]
 new_parts=[a,L,sum(chunks),L,perm,T,H,1,1]
 old=sum(old_parts);new=sum(new_parts)
 assert old==a+3*L+perm+T+H+3
 assert new==a+2*L+sum(chunks)+perm+T+H+2
 examples.append({'shape':{'advice':a,'lookups':L,'permutationColumns':P,'degree':d,'trash':T,'quotientPoints':H,'lookupChunks':chunks,'freshCommittedPoints':C,'freshCarriedPoints':R},'installedProofPoints':old,'nativeProofPoints':new,'installedConservativeMembershipChecks':old+C+R,'nativeConservativeMembershipChecks':new+C+R,'scope':'synthetic arithmetic shape only'})
print(json.dumps({'status':'PASS_SCOPED','sourcePins':len(E['sources']),'compressedBoundaryCases':cases,'identityPublicInput':{'canonical':list(map(str,canonical_identity_pi)),'threeNoncanonicalShortcutAliasesRejectedByReference':True},'foreignFieldAlias':{'canonicalZeroWords':list(map(str,packed(p-1))),'noncanonicalZeroWords':list(map(str,packed(2*p-1))),'bothHostReconstructionEquationsReturnZero':True},'countDiscriminators':examples,'scope':'Exact source-pin checks and a small Python model of decisive decoder branches/encoding arithmetic; not BLST/Rust execution, full finite-point decoder, malicious-witness circuit test, proof or finalizer implementation.'},indent=2))
