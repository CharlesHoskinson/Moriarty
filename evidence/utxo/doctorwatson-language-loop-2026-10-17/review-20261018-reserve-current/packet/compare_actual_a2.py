from pathlib import Path
import json,hashlib,subprocess,datetime,importlib.util
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');C=R/'core2-quint-shared-projection-check-cycle/actual-check-a1';D=R/'reserve-itf-decoder-source-20261018';O=R/'reserve-quint-independent-oracle-20261018'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def write(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
for line in (O/'FREEZE.sha256').read_text().splitlines():
 h,n=line.split(maxsplit=1);assert sha(O/n.lstrip('*'))==h
code=(D/'decode_reserve_itf.py').read_bytes();decoder=C/'decode_reserve_itf.py';decoder.write_bytes(code)
write(C/'DECODER-CUSTODY.json',{'source':str(D/'decode_reserve_itf.py'),'sha256':sha(decoder),'rootReadWholeSource':True,'sourceGrounding':'Pinned Quint0.32.0 ITF serializer + original lifecycle/prefix type declarations. Author wrote decoder before actualtrace exposure, reported no subsequentcodechange.','runtime':'Python stdlib only, file parsing/comparison, no model/native/SDK execution.'})
assert json.loads((C/'DECODER-STDOUT.json').read_text())['status']=='matched'
spec=importlib.util.spec_from_file_location('reserve_decoder',decoder);mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
lines=(C/'rust-shared-projection/STDOUT.log').read_text().splitlines();prefix='RESERVE_QUINT_PROJECTION ';selected=[l.partition(prefix) for l in lines if prefix in l];assert len(selected)==1;assert selected[0][0]=='test reserve_existing_trace_matches_complete_shared_quint_projection ... ';selected=[selected[0][2]]
rust=json.loads(selected[0]);write(C/'rust-actual-common-projection.json',rust)
expected=json.loads((O/'reserve-common-projection.expected.json').read_text());quint=json.loads((C/'quint-decoded-observations.json').read_text())['observations']
comparisons={'rustEntireProjectionVsIndependentOracle':mod.first_difference(rust,expected),'quintCapturedFiveObservationsVsIndependentOracle':mod.first_difference(quint,expected['observations']),'quintCapturedFiveObservationsVsActualRust':mod.first_difference(quint,rust['observations'])}
assert all(v is None for v in comparisons.values()),comparisons
result={'recordedAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'status':'MATCHED','comparisons':comparisons,'independentFreezeSha256':sha(O/'FREEZE.sha256'),'independentExpectedSha256':sha(O/'reserve-common-projection.expected.json'),'rustStdoutSha256':sha(C/'rust-shared-projection/STDOUT.log'),'rustProjectionSha256':sha(C/'rust-actual-common-projection.json'),'quintRawCaptureSha256':sha(C/'reserve_reserveSharedProjectionTraceTest_0.itf.json'),'quintDecodedSha256':sha(C/'quint-decoded-observations.json'),'decoderSha256':sha(decoder),'capturedModelObservations':len(quint),'committedLogicalWork':[s['c']['committed'] for s in quint],'cumulativeAttemptedLogicalWork':[s['attemptedTotal'] for s in quint],'residualDutyFinal':quint[-1]['c']['dutyAmount'],'limitations':['One existing finite normal reserve trajectory; adapter domain0..142 and declared effect/record/map shapes.','Quint initial/prefix FinancialFrames are pure assertions, not raw emitted trace variables; Rust frames separately equal independent expected.','No exhaustive invariant proof, K/Lean/circuit/native/proof/ledger/target correspondence acceptance.','Independent custodian related prior exposure is disclosed; no global blindness claim.']}
write(C/'CAPTURED-COMPARISON.json',result);print(json.dumps(result))
