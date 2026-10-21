"""Synthetic public fixtures. These tests never execute SDK/native/proof/wallet code."""
import copy
from decimal import Decimal
import hashlib
import json
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import metrics as m

H = 'a' * 64

def sample(name='s1', value=2):
    receipt_hash=hashlib.sha256(name.encode()).hexdigest()
    return {'schema': 'sherlock-benchmark-sample/1', 'sample_id': name,
            'workload_id': 'synthetic-test', 'workload_digest_sha256': H,
            'evidence_class': 'synthetic', 'status': 'success',
            'identity': {'source_sha256': H, 'compiler_sha256': None,
                         'ledger_sha256': None, 'network': None,
                         'circuit_sha256': None, 'profile': 'test-subprocess/1',
                         'environment_sha256': H, 'toolchain_sha256': H},
            'cache_state': 'not_applicable', 'metrics': {
                'prove.wall_seconds': {'value': value, 'unit': 'seconds', 'status': 'measured',
                    'source': {'receipt_sha256': receipt_hash, 'pointer': '/duration_seconds',
                               'scope':'subprocess','timer_resolution_seconds':0.001}, 'reason': None}},
            'layers': {}, 'receipts': [{'path': 'public-test.json', 'sha256': receipt_hash, 'schema': 'test/1'}],
            'effects': {'expected_sha256': None, 'observed_sha256': None, 'match': None},
            'events': {}}

class MetricsTests(unittest.TestCase):
    def test_sample_copy_and_units(self):
        s = sample(); checked = m.validate_sample(s)
        checked['identity']['profile'] = 'changed'
        self.assertEqual(s['identity']['profile'], 'test-subprocess/1')
        s['metrics']['prove.wall_seconds']['unit'] = 'milliseconds'
        with self.assertRaisesRegex(ValueError, 'unit'): m.validate_sample(s)

    def test_bad_numeric_values_and_counts(self):
        for value in (True, -1, float('nan'), float('inf'), '1'):
            with self.subTest(value=value), self.assertRaises(ValueError): m.validate_sample(sample(value=value))
        s = sample(); s['metrics'] = {'circuit.used_rows': dict(s['metrics']['prove.wall_seconds'], value=1.5, unit='rows')}
        with self.assertRaises(ValueError): m.validate_sample(s)

    def test_missing_unavailable_censored_and_zero_are_distinct(self):
        s = sample(value=0)
        self.assertTrue(m.metric_coverage(s, ['prove.wall_seconds'])['complete'])
        self.assertEqual(m.metric_coverage(s, ['proof.bytes'])['missing'], ['proof.bytes'])
        s['metrics']['prove.wall_seconds'].update(value=None, status='unavailable', source=None, reason='not exposed')
        self.assertEqual(m.metric_coverage(s, ['prove.wall_seconds'])['unavailable'], ['prove.wall_seconds'])
        s['metrics']['prove.wall_seconds'].update(value=30, status='censored', source={'receipt_sha256': s['receipts'][0]['sha256'], 'pointer': '/timeout'}, reason='timeout')
        self.assertFalse(m.metric_coverage(s, ['prove.wall_seconds'])['complete'])

    def test_required_metrics_do_not_accept_failed_samples(self):
        for state in ('failure','censored','not_run'):
            s = sample(); s['status'] = state
            self.assertFalse(m.metric_coverage(s, ['prove.wall_seconds'])['complete'])

    def test_empty_requirements_do_not_manufacture_pass(self):
        with self.assertRaises(ValueError): m.metric_coverage(sample(), [])

    def test_lineage_must_reference_retained_receipt(self):
        s = sample(); s['receipts'] = []
        with self.assertRaisesRegex(ValueError, 'receipt'): m.validate_sample(s)

    def test_layer_dimensions_are_observed_not_inferred(self):
        s = sample(); item = s['metrics']['prove.wall_seconds']
        s['layers'] = {'child': {'circuit.k': dict(item, value=10, unit='exponent')}}
        checked = m.validate_sample(s)
        self.assertNotIn('circuit.used_rows', checked['layers']['child'])
        self.assertEqual(m.metric_coverage(s, ['layer:child:circuit.used_rows'])['missing'], ['layer:child:circuit.used_rows'])
        s['layers']['child'].update({'circuit.domain_capacity_rows': dict(item, value=8, unit='rows'),
                                   'circuit.used_rows': dict(item, value=9, unit='rows')})
        with self.assertRaisesRegex(ValueError, 'capacity'): m.validate_sample(s)

    def test_events_require_utc_chronology_and_keep_local_verify_separate(self):
        s=sample(); s['events']={'proof_ready':'2026-10-21T00:00:00Z','submission':'2026-10-21T00:00:02Z','readback':'2026-10-21T00:00:05Z'}
        self.assertEqual(m.event_latencies(s), {'network.proof_ready_to_submission_seconds':2.0,'network.submission_to_readback_seconds':3.0})
        self.assertNotIn('local_verify.wall_seconds', m.event_latencies(s))
        s['events']['ack']='2026-10-20T00:00:01Z'
        with self.assertRaisesRegex(ValueError, 'chronolog'): m.validate_sample(s)

    def test_comparable_profile_separates_cache_identity_and_evidence(self):
        a=sample(); b=copy.deepcopy(a); b['evidence_class']='host'
        self.assertNotEqual(m.comparable_profile(a),m.comparable_profile(b))
        b=copy.deepcopy(a); b['identity']['environment_sha256']='b'*64
        self.assertNotEqual(m.comparable_profile(a),m.comparable_profile(b))
        a['cache_state']='warm'
        with self.assertRaisesRegex(ValueError,'cache_identity'): m.validate_sample(a)
        a['cache_identity_sha256']=H
        self.assertTrue(m.validate_sample(a))

    def test_summaries_retain_failure_censoring_missing_and_raw(self):
        a=sample('a',1); b=sample('b',3); c=sample('c',99); c['status']='failure'
        d=sample('d',30); d['status']='censored'; d['metrics']['prove.wall_seconds'].update(status='censored',reason='timeout')
        e=sample('e'); e['metrics']={}
        r=m.summarize([a,b,c,d,e]); g=r['groups'][0]; st=g['metrics']['prove.wall_seconds']
        self.assertEqual(g['counts'],{'total':5,'success':3,'failure':1,'censored':1,'not_run':0})
        self.assertEqual((st['n'],st['mean'],st['median'],st['p95']),(2,2.0,2.0,3))
        self.assertEqual((st['missing'],st['censored'],st['excluded_terminal']),(1,1,1))
        self.assertEqual(len(r['samples']),5)
        self.assertIn('Baseline only',m.render_markdown(r))
        self.assertEqual(m.summarize([a])['groups'][0]['metrics']['prove.wall_seconds']['sample_stdev'],None)

    def test_duplicates_not_counted_as_repetitions(self):
        with self.assertRaisesRegex(ValueError,'duplicate'): m.summarize([sample(),sample()])

    def test_strict_json_rejects_duplicate_keys_and_nonfinite(self):
        for raw in ('{"a":1,"a":2}', '{"a":NaN}'):
            with self.assertRaises(ValueError): m.loads_strict(raw)

    def test_explicit_receipt_pointer_collection_and_scaling(self):
        with tempfile.TemporaryDirectory() as tmp:
            p=Path(tmp)/'receipt.json'; raw=b'{"format":"synthetic/1","resources":{"wall_ms":1250,"rss_kib":4},"terminal":"ok"}'
            p.write_bytes(raw); sha=hashlib.sha256(raw).hexdigest()
            bindings=[{'path':'receipt.json','sha256':sha,'terminal':{'pointer':'/terminal','success_values':['ok'],'censored_values':[]},'fields':[
                {'metric':'prove.wall_seconds','pointer':'/resources/wall_ms','unit':'seconds','scale':.001},
                {'metric':'prove.peak_rss_bytes','pointer':'/resources/rss_kib','unit':'bytes','scale':1024},
                {'metric':'proof.bytes','pointer':'/not-exposed','unit':'bytes'}]}]
            s=sample(); s['metrics']={}; s['receipts']=[]
            r=m.collect_sample(s,bindings,Path(tmp))
            self.assertEqual(r['metrics']['prove.wall_seconds']['value'],1.25)
            self.assertEqual(r['metrics']['prove.peak_rss_bytes']['value'],4096)
            self.assertEqual(r['metrics']['proof.bytes']['status'],'unavailable')
            self.assertEqual(r['receipts'][0]['sha256'],sha)
            self.assertNotIn('transaction.call.bytes',r['metrics'])
            bindings[0]['sha256']=H
            with self.assertRaisesRegex(ValueError,'digest'): m.collect_sample(s,bindings,Path(tmp))

    def test_collection_rejects_path_escape_and_duplicate_metric(self):
        with tempfile.TemporaryDirectory() as tmp:
            s=sample(); s['metrics']={}; s['receipts']=[]
            with self.assertRaises(ValueError):m.collect_sample(s,[{'path':'../secret','sha256':H,'fields':[]}],Path(tmp))
            raw=b'{"v":1}'; p=Path(tmp)/'r.json';p.write_bytes(raw)
            f={'metric':'prove.wall_seconds','pointer':'/v','unit':'seconds'}
            with self.assertRaisesRegex(ValueError,'duplicate'):m.collect_sample(s,[{'path':'r.json','sha256':hashlib.sha256(raw).hexdigest(),'terminal':{'pointer':'/v','success_values':[1],'censored_values':[]},'fields':[f,f]}],Path(tmp))

    def test_real_host_receipt_shape_cannot_be_elevated_to_native(self):
        with tempfile.TemporaryDirectory() as tmp:
            raw=json.dumps({'format':'sherlock-normal-host-run/1','terminal_state':'timeout:run-producer','steps':[{'name':'run-producer','state':'timeout','child':{'duration_seconds':30,'timed_out':True,'exit_code':-9}}]}).encode()
            Path(tmp,'r.json').write_bytes(raw); sha=hashlib.sha256(raw).hexdigest()
            s=sample();s['metrics']={};s['receipts']=[];s['evidence_class']='host'
            binding={'path':'r.json','sha256':sha,'fields':[{'metric':'witness.wall_seconds','pointer':'/steps/0/child/duration_seconds','unit':'seconds'}]}
            r=m.collect_sample(s,[binding],Path(tmp))
            self.assertEqual(r['status'],'censored')
            self.assertFalse(m.metric_coverage(r,['witness.wall_seconds'])['complete'])
            s['evidence_class']='native'
            with self.assertRaisesRegex(ValueError,'host'):m.collect_sample(s,[binding],Path(tmp))

    def test_json_pointer_escaping(self):
        self.assertEqual(m.resolve_pointer({'a/b':{'~x':[0,7]}},'/a~1b/~0x/1'),7)
        with self.assertRaises(ValueError):m.resolve_pointer({},'/bad~2')

    def test_semantic_verifier_false_is_failure_even_with_zero_exit(self):
        with tempfile.TemporaryDirectory() as tmp:
            raw=b'{"exit_code":0,"verifies":false,"wall":.2}'.replace(b':.2',b':0.2')
            Path(tmp,'v.json').write_bytes(raw)
            binding={'path':'v.json','sha256':hashlib.sha256(raw).hexdigest(),'fields':[],
                     'terminal':{'pointer':'/verifies','success_values':[True],'censored_values':[]}}
            s=sample();s['receipts']=[];s['metrics']={}
            self.assertEqual(m.collect_sample(s,[binding],Path(tmp))['status'],'failure')
            binding['terminal']['pointer']='/exit_code'
            self.assertEqual(m.collect_sample(s,[binding],Path(tmp))['status'],'failure')

    def test_measurement_scope_and_resolution_are_preserved(self):
        s=sample();src=s['metrics']['prove.wall_seconds']['source']
        src.update(scope='subprocess',timer_resolution_seconds=0.001)
        self.assertEqual(m.validate_sample(s)['metrics']['prove.wall_seconds']['source'],src)
        src['timer_resolution_seconds']=-1
        with self.assertRaises(ValueError):m.validate_sample(s)

    def test_explicit_regression_tolerance_requires_same_profile_and_count(self):
        a=m.summarize([sample('a',1),sample('b',3)])
        b=m.summarize([sample('c',3),sample('d',5)])
        limits={'prove.wall_seconds':{'statistic':'median','relative_increase':0.1,'absolute_increase':0,'min_samples':2}}
        self.assertEqual(m.compare_baseline(a,b,limits)['status'],'regression')
        limits['prove.wall_seconds']['absolute_increase']=2
        self.assertEqual(m.compare_baseline(a,b,limits)['status'],'within_declared_tolerance')
        limits['prove.wall_seconds']['min_samples']=3
        self.assertEqual(m.compare_baseline(a,b,limits)['status'],'incomplete')
        c=sample('different');c['identity']['profile']='other'
        with self.assertRaisesRegex(ValueError,'comparable'):m.compare_baseline(a,m.summarize([c]),limits)

    def test_different_resource_scopes_never_pool(self):
        a=sample('a');b=sample('b')
        a['metrics']['prove.wall_seconds']['source']['scope']='subprocess'
        b['metrics']['prove.wall_seconds']['source']['scope']='single_process'
        with self.assertRaisesRegex(ValueError,'scope'):m.summarize([a,b])

    def test_byte_and_proof_metrics_stay_distinct(self):
        s=sample();source=s['metrics']['prove.wall_seconds']['source']
        s['metrics']={'proof.bytes':{'value':256,'unit':'bytes','status':'measured','source':source,'reason':None}}
        self.assertEqual(m.metric_coverage(s,['transaction.call.bytes'])['missing'],['transaction.call.bytes'])

    def test_revision_comparison_requires_explicit_subject_change_and_fixed_context(self):
        a=sample('a');a['identity']['compiler_sha256']=H
        b=sample('b');b['identity']['compiler_sha256']='b'*64
        limits={'prove.wall_seconds':{'statistic':'median','relative_increase':0.1,'absolute_increase':0,'min_samples':1}}
        ar,br=m.summarize([a]),m.summarize([b])
        with self.assertRaisesRegex(ValueError,'comparable'):m.compare_baseline(ar,br,limits)
        result=m.compare_baseline(ar,br,limits,allowed_changed_subject_fields=['compiler_sha256'])
        self.assertEqual(result['status'],'within_declared_tolerance')
        self.assertEqual(result['changed_subject_fields'],['compiler_sha256'])
        self.assertEqual(result['baseline_identity'],a['identity'])
        self.assertEqual(result['current_identity'],b['identity'])
        for field,value in [('network','other-network'),('environment_sha256','c'*64),('toolchain_sha256','d'*64)]:
            c=copy.deepcopy(b);c['identity'][field]=value
            with self.subTest(field=field), self.assertRaisesRegex(ValueError,'comparable'):
                m.compare_baseline(ar,m.summarize([c]),limits,allowed_changed_subject_fields=['compiler_sha256'])
        for field,value in [('workload_digest_sha256','c'*64),('cache_state','cold')]:
            c=copy.deepcopy(b);c[field]=value
            with self.subTest(field=field), self.assertRaisesRegex(ValueError,'comparable'):
                m.compare_baseline(ar,m.summarize([c]),limits,allowed_changed_subject_fields=['compiler_sha256'])
        with self.assertRaises(ValueError):m.compare_baseline(ar,br,limits,allowed_changed_subject_fields=['environment_sha256'])

    def test_schema_registry_matches_runtime_units(self):
        schema=json.loads(Path(m.__file__).with_name('sample.schema.json').read_text())
        definitions=schema['$defs']['metrics']['properties']
        self.assertEqual(set(definitions),set(m.METRIC_UNITS))
        for name,unit in m.METRIC_UNITS.items():
            self.assertEqual(definitions[name]['$ref'],'#/$defs/metric_'+unit)

    def test_cli_collect_report_and_compare_only_read_selected_files(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp); raw=b'{"duration":2}'
            root.joinpath('r.json').write_bytes(raw)
            sha=hashlib.sha256(raw).hexdigest()
            s=sample();s['metrics']={};s['receipts']=[]
            root.joinpath('template.json').write_text(json.dumps(s))
            binding=[{'path':'r.json','sha256':sha,'terminal':{'pointer':'/duration','success_values':[2],'censored_values':[]},'fields':[{'metric':'host.handler.wall_seconds','pointer':'/duration','unit':'seconds','scope':'subprocess','timer_resolution_seconds':0.001}]}]
            root.joinpath('bindings.json').write_text(json.dumps(binding))
            self.assertEqual(m.main(['collect','--template',str(root/'template.json'),'--bindings',str(root/'bindings.json'),'--base-dir',tmp,'--output',str(root/'sample.json')]),0)
            self.assertEqual(m.main(['report','--samples',str(root/'sample.json'),'--output',str(root/'report.json'),'--markdown',str(root/'report.md')]),0)
            limits={'host.handler.wall_seconds':{'statistic':'median','relative_increase':0,'absolute_increase':0,'min_samples':1}}
            root.joinpath('limits.json').write_text(json.dumps(limits))
            s['sample_id']='s2';root.joinpath('template2.json').write_text(json.dumps(s))
            raw2=b'{"duration":2,"invocation":"second-synthetic-fixture"}';root.joinpath('r2.json').write_bytes(raw2)
            binding[0].update(path='r2.json',sha256=hashlib.sha256(raw2).hexdigest())
            root.joinpath('bindings2.json').write_text(json.dumps(binding))
            self.assertEqual(m.main(['collect','--template',str(root/'template2.json'),'--bindings',str(root/'bindings2.json'),'--base-dir',tmp,'--output',str(root/'sample2.json')]),0)
            self.assertEqual(m.main(['report','--samples',str(root/'sample2.json'),'--output',str(root/'report2.json')]),0)
            self.assertEqual(m.main(['compare','--baseline',str(root/'report.json'),'--current',str(root/'report2.json'),'--tolerances',str(root/'limits.json'),'--output',str(root/'comparison.json')]),0)
            self.assertEqual(json.loads(root.joinpath('comparison.json').read_text())['status'],'within_declared_tolerance')
            self.assertEqual(root.joinpath('r.json').read_bytes(),raw)

    def test_relabeling_one_receipt_does_not_create_more_observations(self):
        a=sample('a');b=copy.deepcopy(a);b['sample_id']='b'
        r=m.summarize([a,b]);stats=r['groups'][0]['metrics']['prove.wall_seconds']
        self.assertEqual(stats['n'],1)
        self.assertEqual(stats['duplicate_observations'],1)
        b['metrics']['prove.wall_seconds']['value']=3
        with self.assertRaisesRegex(ValueError,'same receipt'):m.summarize([a,b])

    def test_unknown_receipt_needs_explicit_terminal_and_semantic_false_dominates_exit(self):
        with tempfile.TemporaryDirectory() as tmp:
            raw=b'{"exit_code":0,"verifies":false,"wall":1}'
            Path(tmp,'r.json').write_bytes(raw)
            b={'path':'r.json','sha256':hashlib.sha256(raw).hexdigest(),'fields':[]}
            s=sample();s['metrics']={};s['receipts']=[]
            with self.assertRaisesRegex(ValueError,'terminal'):m.collect_sample(s,[b],Path(tmp))
            b['terminal']={'pointer':'/exit_code','success_values':[0],'censored_values':[]}
            self.assertEqual(m.collect_sample(s,[b],Path(tmp))['status'],'failure')

    def test_not_run_template_becomes_actual_success_from_positive_receipt(self):
        with tempfile.TemporaryDirectory() as tmp:
            raw=b'{"ok":true,"wall":1.5}'
            Path(tmp,'r.json').write_bytes(raw)
            b={'path':'r.json','sha256':hashlib.sha256(raw).hexdigest(),'terminal':{'pointer':'/ok','success_values':[True],'censored_values':[]},
               'fields':[{'metric':'prove.wall_seconds','pointer':'/wall','unit':'seconds'}]}
            s=sample();s['status']='not_run';s['metrics']={};s['receipts']=[]
            checked=m.collect_sample(s,[b],Path(tmp))
            self.assertEqual(checked['status'],'success')
            self.assertTrue(m.metric_coverage(checked,['prove.wall_seconds'])['complete'])

    def test_censored_receipt_censors_its_values_but_preserves_completed_other_phase(self):
        with tempfile.TemporaryDirectory() as tmp:
            ok=b'{"ok":true,"wall":1.5}'
            timeout=b'{"terminal":"timeout","wall":30,"cpu":28,"rss":4096}'
            for name,raw in [('ok',ok),('timeout',timeout)]:Path(tmp,name+'.json').write_bytes(raw)
            bindings=[{'path':'ok.json','sha256':hashlib.sha256(ok).hexdigest(),'terminal':{'pointer':'/ok','success_values':[True],'censored_values':[]},
                       'fields':[{'metric':'keygen.wall_seconds','pointer':'/wall','unit':'seconds'}]},
                      {'path':'timeout.json','sha256':hashlib.sha256(timeout).hexdigest(),'terminal':{'pointer':'/terminal','success_values':['ok'],'censored_values':['timeout']},
                       'fields':[{'metric':'prove.wall_seconds','pointer':'/wall','unit':'seconds'},
                                 {'metric':'prove.cpu_seconds','pointer':'/cpu','unit':'seconds'},
                                 {'metric':'prove.peak_rss_bytes','pointer':'/rss','unit':'bytes'},
                                 {'metric':'proof.bytes','pointer':'/absent','unit':'bytes'}]}]
            s=sample();s['metrics']={};s['receipts']=[]
            checked=m.collect_sample(s,bindings,Path(tmp))
            self.assertEqual(checked['status'],'censored')
            self.assertEqual(checked['metrics']['keygen.wall_seconds']['status'],'measured')
            for name in ('prove.wall_seconds','prove.cpu_seconds','prove.peak_rss_bytes'):
                self.assertEqual(checked['metrics'][name]['status'],'censored')
                self.assertTrue(checked['metrics'][name]['reason'])
            self.assertEqual(checked['metrics']['proof.bytes']['status'],'unavailable')
            st=m.summarize([checked])['groups'][0]['metrics']['prove.wall_seconds']
            self.assertEqual((st['n'],st['censored'],st['excluded_terminal']),(0,1,0))

    def test_circuit_observed_capacity_matches_k_without_arbitrary_degree_cap(self):
        s=sample();source=s['metrics']['prove.wall_seconds']['source']
        def put(name,value,unit):s['layers'].setdefault('child',{})[name]={'value':value,'unit':unit,'status':'measured','source':source,'reason':None}
        put('circuit.k',10,'exponent');put('circuit.domain_capacity_rows',262144,'rows')
        with self.assertRaisesRegex(ValueError,'capacity'):m.validate_sample(s)
        put('circuit.k',40,'exponent');put('circuit.domain_capacity_rows',2**40,'rows')
        self.assertTrue(m.validate_sample(s))
        put('circuit.domain_capacity_rows',1023,'rows')
        with self.assertRaisesRegex(ValueError,'capacity'):m.validate_sample(s)
        put('circuit.k',10**30,'exponent');s['layers']['child'].pop('circuit.domain_capacity_rows')
        self.assertTrue(m.validate_sample(s))
        self.assertNotIn('circuit.used_rows',m.validate_sample(s)['layers']['child'])

    def test_used_and_lookup_observed_rows_are_bounded_with_or_without_capacity(self):
        for name in ('circuit.used_rows','circuit.lookup_rows'):
            s=sample();source=s['metrics']['prove.wall_seconds']['source']
            s['metrics']={'circuit.k':{'value':10,'unit':'exponent','status':'measured','source':source,'reason':None},
                          name:{'value':1025,'unit':'rows','status':'measured','source':source,'reason':None}}
            with self.subTest(metric=name),self.assertRaisesRegex(ValueError,'rows'):m.validate_sample(s)
            s['metrics'][name]['value']=1024
            self.assertTrue(m.validate_sample(s))
            s['metrics'].pop('circuit.k')
            s['metrics']['circuit.domain_capacity_rows']={'value':512,'unit':'rows','status':'measured','source':source,'reason':None}
            with self.assertRaisesRegex(ValueError,'capacity'):m.validate_sample(s)

    def test_network_metric_requires_its_actual_event_duration(self):
        s=sample();s['events']={'submission':'2026-10-21T00:00:00Z','readback':'2026-10-21T00:01:40Z'}
        source=s['metrics']['prove.wall_seconds']['source']
        s['metrics']={'network.submission_to_readback_seconds':{'value':0.001,'unit':'seconds','status':'measured','source':source,'reason':None}}
        with self.assertRaisesRegex(ValueError,'event'):m.validate_sample(s)
        s['metrics']['network.submission_to_readback_seconds']['value']=100
        self.assertTrue(m.validate_sample(s))
        s['events'].pop('readback')
        with self.assertRaisesRegex(ValueError,'event'):m.validate_sample(s)

    def test_shared_exact_decimal_normalization_and_receipt_token_precision(self):
        self.assertEqual(m.normalize_number(3,0.1,unit='seconds'),0.3)
        self.assertEqual(m.receipt_metric_value(b'{"duration":3}','/duration',0.1,unit='seconds'),0.3)
        self.assertEqual(m.normalize_number(3.0,1024,unit='bytes'),3072)
        with self.assertRaises(ValueError):m.normalize_number(3,0.1,unit='bytes')
        with self.assertRaises(ValueError):m.normalize_number(True,1,unit='seconds')
        with self.assertRaises(ValueError):m.normalize_number(3,0,unit='seconds')
        with self.assertRaisesRegex(ValueError,'precision'):m.receipt_metric_value(b'{"duration":0.123456789123456789}','/duration',1,unit='seconds')
        with self.assertRaises(ValueError):m.receipt_metric_value(b'{"duration":1e-1000}','/duration',1,unit='seconds')
        with self.assertRaises(ValueError):m.receipt_metric_value(b'{"duration":1e-10000000}','/duration',1,unit='seconds')
        self.assertEqual(m.normalize_number(Decimal('0.1'),3,unit='seconds'),0.3)
        with self.assertRaises(ValueError):m.receipt_metric_value(b'{"duration":3,"duration":4}','/duration',1,unit='seconds')

    def test_collector_uses_same_exact_decimal_result_as_acceptance(self):
        with tempfile.TemporaryDirectory() as tmp:
            raw=b'{"ok":true,"wall":3}'
            Path(tmp,'r.json').write_bytes(raw)
            b={'path':'r.json','sha256':hashlib.sha256(raw).hexdigest(),'terminal':{'pointer':'/ok','success_values':[True],'censored_values':[]},
               'fields':[{'metric':'prove.wall_seconds','pointer':'/wall','unit':'seconds','scale':0.1}]}
            s=sample();s['receipts']=[];s['metrics']={}
            checked=m.collect_sample(s,[b],Path(tmp))
            self.assertEqual(checked['metrics']['prove.wall_seconds']['value'],0.3)
            self.assertEqual(checked['metrics']['prove.wall_seconds']['value'],m.receipt_metric_value(raw,'/wall',0.1,unit='seconds'))

    def test_required_metric_case_contract_never_double_prefixes_catalog_ids(self):
        ids=['prove.wall_seconds','layer:child:circuit.k']
        self.assertEqual(m.required_metric_ids({'required_metrics':ids,'required_layers':['child']}),ids)
        self.assertEqual(m.required_metric_ids({'required_metrics':['proof.bytes'],'required_layers':['child'],'required_layer_metrics':['circuit.k']}),['proof.bytes','layer:child:circuit.k'])
        self.assertEqual(m.required_metric_ids({'required_metrics':['host.handler.wall_seconds'],'required_layers':[],'required_layer_metrics':[]}),['host.handler.wall_seconds'])
        with self.assertRaises(ValueError):m.required_metric_ids({'required_metrics':ids,'required_layers':['child'],'required_layer_metrics':['layer:child:circuit.k']})
        with self.assertRaises(ValueError):m.required_metric_ids({'required_metrics':ids+ids})

    def test_shared_comparison_retains_failures_without_losing_successful_baseline(self):
        a=sample('old-success',0);f=sample('old-failed',50);f['status']='failure'
        b=sample('new-success',0.1);c=sample('new-censored',30);c['status']='censored'
        c['metrics']['prove.wall_seconds'].update(status='censored',reason='timeout')
        rule={'prove.wall_seconds':{'statistic':'median','relative_increase':0.1,'absolute_increase':0.1,'min_samples':1}}
        result=m.compare_baseline(m.summarize([a,f]),m.summarize([b,c]),rule)
        self.assertEqual(result['status'],'within_declared_tolerance')
        self.assertEqual(result['baseline_counts']['failure'],1)
        self.assertEqual(result['current_counts']['censored'],1)
        self.assertEqual(result['metrics']['prove.wall_seconds']['limit'],0.1)
        rule['prove.wall_seconds']['min_samples']=2
        self.assertEqual(m.compare_baseline(m.summarize([a,f]),m.summarize([b,c]),rule)['status'],'incomplete')

    def test_failure_scope_unknown_does_not_override_success_distribution_scope(self):
        a=sample('a');b=sample('b');b['status']='failure';b['metrics']['prove.wall_seconds']['source'].pop('scope')
        a['metrics']['prove.wall_seconds']['source']['scope']='subprocess'
        report=m.summarize([a,b]);metric=report['groups'][0]['metrics']['prove.wall_seconds']
        self.assertEqual(metric['scope'],'subprocess')
        self.assertEqual(metric['observed_scopes'],['subprocess','unknown'])
        self.assertEqual((metric['n'],metric['excluded_terminal']),(1,1))

    def test_duplicate_time_int_and_float_spelling_still_one_observation(self):
        a=sample('a',1);b=copy.deepcopy(a);b['sample_id']='b';b['metrics']['prove.wall_seconds']['value']=1.0
        result=m.summarize([a,b])['groups'][0]['metrics']['prove.wall_seconds']
        self.assertEqual((result['n'],result['duplicate_observations']),(1,1))

    def test_nonobject_acceptance_extension_is_rejected_structurally(self):
        s=sample();s['extensions']={'acceptance':'success'}
        with self.assertRaisesRegex(ValueError,'acceptance'):m.validate_sample(s)

    def test_comparison_threshold_and_median_use_exact_decimal_arithmetic(self):
        rule={'prove.wall_seconds':{'statistic':'median','relative_increase':2,'absolute_increase':0,'min_samples':1}}
        r=m.compare_baseline(m.summarize([sample('a',0.1)]),m.summarize([sample('b',0.30000000000000004)]),rule)
        self.assertEqual(r['status'],'regression')
        self.assertEqual(r['metrics']['prove.wall_seconds']['limit_decimal'],'0.3')
        rule['prove.wall_seconds']['relative_increase']=0
        r=m.compare_baseline(m.summarize([sample('a',0.1),sample('b',0.2)]),m.summarize([sample('c',0.15000000000000002)]),rule)
        self.assertEqual(r['status'],'regression')
        self.assertEqual(r['metrics']['prove.wall_seconds']['baseline_statistic_decimal'],'0.15')

    def test_missing_terminal_predicate_is_retained_failure_not_missing_success(self):
        with tempfile.TemporaryDirectory() as tmp:
            raw=b'{"exit_code":137,"wall":2}'
            Path(tmp,'r.json').write_bytes(raw)
            b={'path':'r.json','sha256':hashlib.sha256(raw).hexdigest(),'terminal':{'pointer':'/verifier/verifies','success_values':[True],'censored_values':[]},
               'fields':[{'metric':'prove.wall_seconds','pointer':'/wall','unit':'seconds'}]}
            s=sample();s['receipts']=[];s['metrics']={}
            checked=m.collect_sample(s,[b],Path(tmp))
            self.assertEqual(checked['status'],'failure')
            self.assertTrue(checked['extensions']['collection_diagnostics'])
            self.assertEqual(checked['metrics']['prove.wall_seconds']['value'],2)

    def test_same_receipt_alias_fields_or_unit_selectors_do_not_supply_repetitions(self):
        a=sample('a',2);b=copy.deepcopy(a);b['sample_id']='b'
        b['metrics']['prove.wall_seconds']['source'].update(pointer='/wall_ms',scale=0.001)
        stat=m.summarize([a,b])['groups'][0]['metrics']['prove.wall_seconds']
        self.assertEqual((stat['n'],stat['duplicate_observations']),(1,1))
        b['metrics']['prove.wall_seconds']['value']=2.1
        with self.assertRaisesRegex(ValueError,'same receipt'):m.summarize([a,b])

    def test_baseline_comparison_cannot_compare_same_observation_or_unknown_timing(self):
        a=sample('a');b=sample('b')
        rule={'prove.wall_seconds':{'statistic':'median','relative_increase':0,'absolute_increase':0,'min_samples':1}}
        ar=m.summarize([a])
        with self.assertRaisesRegex(ValueError,'sample_id'):m.compare_baseline(ar,ar,rule)
        b['metrics']['prove.wall_seconds']['source']=copy.deepcopy(a['metrics']['prove.wall_seconds']['source'])
        b['receipts']=copy.deepcopy(a['receipts'])
        b['metrics']['prove.wall_seconds']['source']['pointer']='/aliased_wall'
        with self.assertRaisesRegex(ValueError,'observation'):m.compare_baseline(ar,m.summarize([b]),rule)
        a=sample('a');b=sample('b')
        for name in ('scope','timer_resolution_seconds'):
            for s in (a,b):s['metrics']['prove.wall_seconds']['source'].pop(name,None)
            with self.assertRaisesRegex(ValueError,'scope|resolution'):m.compare_baseline(m.summarize([a]),m.summarize([b]),rule)
            for s in (a,b):s['metrics']['prove.wall_seconds']['source'].update(scope='subprocess',timer_resolution_seconds=0.001)

    def test_receipt_parser_and_cli_inputs_share_bounded_json_reading(self):
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as tmp:
            p=Path(tmp,'oversize.json');p.write_bytes(b'{"padding":"'+b'x'*64+b'"}')
            with patch.object(m,'MAX_RECEIPT_BYTES',32):
                with self.assertRaisesRegex(ValueError,'size|limit|bound'):m.receipt_metric_value(p.read_bytes(),'/padding',unit='seconds')
                with self.assertRaisesRegex(ValueError,'size|limit|bound'):m.read_json_bytes(p)

    def test_effect_match_claim_must_agree_with_digests(self):
        s=sample(); s['effects']={'expected_sha256':H,'observed_sha256':'b'*64,'match':True}
        with self.assertRaises(ValueError):m.validate_sample(s)

if __name__=='__main__':unittest.main()
