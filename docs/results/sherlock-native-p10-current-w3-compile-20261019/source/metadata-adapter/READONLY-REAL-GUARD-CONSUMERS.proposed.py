"""One-off Root-selected read-only calls to unchanged real guard consumers.
No helper load, main/run/worker/launch, marker, staging, cgroup mutation or SDK operation.
Reads/hashes/stat calls can affect access times; no content/namespace writes.
"""
import hashlib,importlib.util,json,sys
from pathlib import Path
assert sys.dont_write_bytecode, 'Use python3 -I -B -S'
assert len(sys.argv)==3
config=Path(sys.argv[1]);anchor=sys.argv[2]
assert len(anchor)==64 and hashlib.sha256(config.read_bytes()).hexdigest()==anchor
GUARD=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-owner-source-candidate-20261019/revision03-test-only/afterimages/bounded_sdk_compile.py')
assert hashlib.sha256(GUARD.read_bytes()).hexdigest()=='30077e57f829a111b08228e30f05fb3360e9b7534d7818dedf48d5ee367a0edc'
spec=importlib.util.spec_from_file_location('p10_readonly_real_guard',GUARD)
assert spec is not None and spec.loader is not None
g=importlib.util.module_from_spec(spec);spec.loader.exec_module(g)
c=g.validate(g.jload(config))
g.need(str(config.resolve())==c['paths']['config'],'config path binding')
g.executing_identity(c)
g.source_binding(c)
g.consent(c)
print(json.dumps({'status':'REAL_GUARD_VALIDATE_SOURCE_BINDING_CONSENT_PASS','config_sha256':anchor,'guard_sha256':'30077e57f829a111b08228e30f05fb3360e9b7534d7818dedf48d5ee367a0edc','scope':'Read-only selected real consumers; no helper/main/supervisor/worker/SDK/marker/launch/operation authorization.'}))
