from pathlib import Path
import json,hashlib,os
D=Path(__file__).parent;sha=lambda b:hashlib.sha256(b).hexdigest()
proof=json.loads((D/'STRICT-INVERSE-PROOF.json').read_text());native_path=Path(proof['nativeWindow']['path']);b=native_path.read_bytes();assert sha(b)==proof['nativeWindow']['sha256'];window=json.loads(b)
assert set(window)=={'before','after_cell','after_exit','events'}
public=proof['structuralPublicArities'];assert len(public['programViewFields'])==23 and len(public['imageFields'])==23 and len(public['eventDeclarationFields'])==28
for key in ['before','after_cell','after_exit']:
 host=window[key];assert set(host)=={'admission','image'} and set(host['admission'])=={'identity','bounds','view','layout'};assert set(host['admission']['view'])==set(public['programViewFields']);assert set(host['image'])==set(public['imageFields'])
assert type(window['events']) is list and len(window['events'])==2
for event in window['events']:assert set(event)==set(public['eventDeclarationFields'])
inverse=json.loads(Path(proof['strictInverseWindow']['path']).read_text());canonical=lambda x:json.dumps(x,ensure_ascii=False,sort_keys=True,separators=(',',':')).encode();assert sha(canonical(inverse))==proof['wholeSelectedTypedWindowCanonicalSha256']==proof['wholeInverseTypedWindowCanonicalSha256']
r={'status':'DECLARED_POSITIVE_WINDOW_ARITIES_AND_FULL_RECORD_FIELDS_PASS','scope':'Public declared structural arities checked on computed Quint projection only, no Rust/expected comparison or semantic vote.','threeWholeHostRecords':True,'eachWholeAdmissionAndAllProgramViewImageFields':True,'twoWholeOrderedEventsAndEveryDeclaredEventField':True,'entireSelectedITFInverseCanonicalEquality':True,'noRecordFieldOrEventSubsetComparison':True}
p=D/'FULL-FIELD-ARITY-RECEIPT.json';b=(json.dumps(r,indent=2)+'\n').encode();p.write_bytes(b);os.chmod(p,0o400)
print('Full declared record fields and positive structural arities PASS; no payload values displayed')
