"""Independent reader for numeric data joins; no annotation/subject imports."""
import hashlib,json
from pathlib import Path
D=Path(__file__).resolve().parent
proof=[]
for p in sorted(D.glob('*-B0-clocks.json')):
    d=json.loads(p.read_text());inputs=d['normativeInputs']
    for m in inputs.values():
        raw=Path(m['path']).read_bytes();assert len(raw)==m['bytes'] and hashlib.sha256(raw).hexdigest()==m['sha256']
    a=json.loads(Path(inputs['independentA39Prefix']['path']).read_text())
    n=json.loads(Path(inputs['R4Rows']['path']).read_text())
    rows={x['id']:x for k in ['legacy_completion_instances','helper_events','bookkeeping_events','administrative_boundaries'] for x in n[k]}
    prefix=d['clockPrefixes'];assert prefix[0]==[None,None,0,0,0,0,4096,0]
    assert len(d['intervals'])==len(a['records'])
    work=helper=visits=cursor=0;position=1
    for i,interval in enumerate(d['intervals']):
        assert interval['A_record_index']==i and interval['A_rule_line']==a['records'][i]['rule_start_line']
        assert interval['first_B_record']==position-1
        for rule in interval['helper_rule_ids']:
            r=rows[rule];assert r['status']=='Enabled'
            assert r['deltas']['base'][:3]==[-1,1,1]
            work+=1;helper+=1;visits+=r['helperVisits']
            assert prefix[position]==[i,rule,work,helper,visits,cursor,4096-work,i];position+=1
        charge=a['records'][i]['charge'];work+=charge;cursor+=charge
        assert prefix[position]==[i,interval['legacy_rule_id'],work,helper,visits,cursor,4096-work,i+1];position+=1
        assert interval['B_records_exclusive']==position-1
        assert work-helper==cursor and 0<=work<=4096
    assert position==len(prefix) and cursor==a['charged_A_work']
    for row in prefix:assert row[2]-row[3]==row[5] and row[2]+row[6]==4096
    proof.append({'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'exactNormAndIndependentSourcePins':True,'completeIntervalAndEveryPrefixSums':True,'semanticSnapshotIndexJoins':True,'lastClock':prefix[-1][2:7]})
(D/'INDEPENDENT-NUMERIC-PROOF.json').write_text(json.dumps({'status':'PASS_DATA_ONLY','members':proof,'noSubjectOrSemanticApproval':True},indent=2)+'\n')
print(json.dumps({'status':'PASS_DATA_ONLY','documents':len(proof)}))
