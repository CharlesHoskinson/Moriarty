#!/usr/bin/env python3
"""Read-only public-constant arithmetic checks; no Rust, services or proof operations."""
import hashlib, json, re, sys, tarfile
from pathlib import Path
root = Path(sys.argv[1])
cache = Path(sys.argv[2])
def sha(b): return hashlib.sha256(b).hexdigest()
def modulus(s): return int(re.search(r"const MODULUS: &\'static str =\s*\"(0x[0-9a-f]+)\"", s).group(1), 16)
base = root / 'repos/midnightntwrk/midnight-zk'
fp = (base/'curves/src/bls12_381/fp.rs').read_bytes()
fq = (base/'curves/src/bls12_381/fq.rs').read_bytes()
p, r = modulus(fp.decode()), modulus(fq.decode())
archive = cache/'midnight-curves-0.3.1.crate'
assert sha(archive.read_bytes()) == 'efaa236cfabccb367c0a89e58a6e8f0b4ba70fb81efd07acfe1d3a0ec3bed081'
with tarfile.open(archive) as t:
    assert modulus(t.extractfile('midnight-curves-0.3.1/src/bls12_381/fp.rs').read().decode()) == p
    assert modulus(t.extractfile('midnight-curves-0.3.1/src/bls12_381/fq.rs').read().decode()) == r
params = (base/'circuits/src/field/foreign/params.rs').read_text()
scope = params.split('impl FieldEmulationParams<midnight_curves::Fq, midnight_curves::Fp>')[1].split(chr(10) + '}')[0]
w = int(re.search(r'LOG2_BASE: u32 = (\d+)', scope).group(1))
n = int(re.search(r'NB_LIMBS: u32 = (\d+)', scope).group(1))
assert (w,n)==(56,7) and p.bit_length()==381 and r.bit_length()==255
b=1<<w
assert b**n >= p and b < r and r < p
vectors=[0,1,r-1,r,r+1,p-1,b-1,b,b+1]
for x in vectors:
    shifted=(x-1)%p
    limbs=[(shifted>>(w*i))&(b-1) for i in range(n)]
    assert all(0<=v<b for v in limbs)
    assert (1+sum(v*b**i for i,v in enumerate(limbs)))%p==x
# These are two valid BASE-field encodings, not two claimed valid curve points.
assert 0 != r and 0%r == r%r and 0<=r<p
# Exact arithmetic, not a circuit cost estimate or test of primality/pairing.
assert (p**12-1)%r==0
print(json.dumps({'status':'PASS_PUBLIC_CONSTANT_ARITHMETIC_ONLY','baseModulusHex':hex(p),'scalarModulusHex':hex(r),'baseBits':p.bit_length(),'scalarBits':r.bit_length(),'limbBits':w,'limbs':n,'shiftedEncodingBoundaryVectors':len(vectors),'noninjectiveSingleScalarCast':{'canonicalBaseInputs':['0',str(r)],'scalarResidues':['0','0']},'finalExponentIntegerBits':((p**12-1)//r).bit_length(),'limbCapacityBits':w*n,'nativeFpSha256':sha(fp),'nativeFqSha256':sha(fq),'outerCurvesArchiveSha256':sha(archive.read_bytes()),'scope':'Arithmetic on public constants only; no native API, curve/subgroup validation, constraint satisfaction, pairing, rows, fit, proving or ledger acceptance established.'},indent=2))
