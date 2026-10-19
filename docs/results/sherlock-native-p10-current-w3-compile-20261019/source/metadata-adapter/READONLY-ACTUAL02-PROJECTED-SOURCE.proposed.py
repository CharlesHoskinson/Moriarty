"""One-off Root-selected READ-ONLY actual02 DAG and prospective source-shape check.
No issuer/helper/main/supervisor/worker/SDK/Cargo/marker/staging/cgroup mutation.
Consent applies ONLY to unchanged actual02; projected source_binding is separate.
"""
import copy,hashlib,importlib.util,json,sys
from pathlib import Path
assert sys.dont_write_bytecode and len(sys.argv)==1
GUARD=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-owner-source-candidate-20261019/revision03-test-only/afterimages/bounded_sdk_compile.py');CONFIG=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-integration-metadata-plan-20261019/future-actual-02/config.final.json');MANIFEST=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p10-existing-issuer-materialization-20261019/revision04-pin-schema-projection/CHECKOUT-SOURCE-MANIFEST.pin-schema.json')
for p,h in [(GUARD,'30077e57f829a111b08228e30f05fb3360e9b7534d7818dedf48d5ee367a0edc'),(CONFIG,'5b946ecc36096dc86156f6d0cd741bf073aaab337c5e0dd7b3412ee63a7e76ab'),(MANIFEST,'1e042df3b0115993900bdc81dff1891863533e023d056d4d83e05e6092aeae59')]:assert hashlib.sha256(p.read_bytes()).hexdigest()==h
spec=importlib.util.spec_from_file_location('p10_readonly_actual02_guard',GUARD)
assert spec is not None and spec.loader is not None
g=importlib.util.module_from_spec(spec);spec.loader.exec_module(g)
original=g.validate(g.jload(CONFIG))
g.executing_identity(original)
g.consent(original)
projected=copy.deepcopy(original)
projected['source_manifest']={'path':str(MANIFEST),'sha256':'1e042df3b0115993900bdc81dff1891863533e023d056d4d83e05e6092aeae59'}
g.validate(projected)
g.source_binding(projected)
print(json.dumps({'status':'ACTUAL02_CONSENT_AND_PROJECTED_SOURCE_BINDING_PASS','original_config_sha256':'5b946ecc36096dc86156f6d0cd741bf073aaab337c5e0dd7b3412ee63a7e76ab','consumed_projection_sha256':'1e042df3b0115993900bdc81dff1891863533e023d056d4d83e05e6092aeae59','scope':'Actual02 consent observed separately from in-memory projected source_binding; no claim of future03 consent/issuer/native eligibility.'}))
