"""Behavioral tests use temporary public files only; no product/native evidence."""
import copy
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
import delivery
import fixture_support


def encoded(value):
    return (json.dumps(value, sort_keys=True, separators=(',', ':'), allow_nan=False) + '\n').encode()


class DeliveryTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(dir=Path(__file__).parent)
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.serial = 0
        self.source = self.file('source.py', b'public source\n')
        self.tool = self.file('tool', b'public tool\n')
        self.config = self.file('config.json', b'{}\n')
        self.plan = {'schema': 'doctorwatson.delivery-plan/1', 'integrator': 'Root',
                     'tasks': [self.task('build')]}
        self.history = {'schema': 'doctorwatson.delivery-history/1', 'results': []}

    def file(self, name, raw):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(raw)
        return {'path': str(path), 'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest()}

    def artifact(self, role, value=None, raw=None):
        self.serial += 1
        ref = self.file(f'artifact-{self.serial}', raw if raw is not None else encoded(value))
        return {'role': role, **ref}

    def task(self, name, adapter='moriarty-sdk-build', dependencies=None, inputs=None):
        return {'id': name, 'owner': 'Sol', 'adapter': adapter, 'attempt': name + '-001',
                'disposition': 'active', 'launch_state': 'not-started',
                'identity': {'integration_base': 'current-source-cut', 'source': [self.source],
                             'tools': [self.tool], 'config': [self.config], 'oracle_sha256': 'a' * 64},
                'inputs': {} if inputs is None else inputs,
                'dependencies': dependencies or [], 'eligible_reviewers': ['Opus']}

    def save(self):
        p = self.root / 'plan.json'
        h = self.root / 'history.json'
        for path, value in ((p,self.plan),(h,self.history)):
            data=encoded(value)
            if not path.exists() or path.read_bytes()!=data:
                path.write_bytes(data)
        return p, h

    def fp(self, name='build'):
        p, _ = self.save()
        return delivery.fingerprint(delivery.load_plan(p), name)

    def success(self, task=None, payload=None, adopt=False):
        task = task or self.plan['tasks'][0]
        fp = self.fp(task['id'])
        adapter = task['adapter']
        header = {'schema': 'doctorwatson.' + adapter + '-observation/1',
                  'task': task['id'], 'attempt': task['attempt'], 'fingerprint': fp}
        defaults = {
            'source-integration': {'integrated': True},
            'moriarty-sdk-build': {'built': True, 'test_bodies_executed': False},
            'sdk-selected-tests': {'tests': [{'name': n, 'status': 'passed'} for n in task['inputs'].get('selectors', [])]},
            'compact-anchor-build9': {'built': True, 'skip_zk': False},
            'recl9-terminal-verify': {'verified': True, **task['inputs']},
            'sherlock-anchor-settle-readback': {'accepted': True, 'settled': True,
                'phase': 'SETTLED_FINALIZED_READBACK_OK', **task['inputs']},
            'sherlock-rollup-benchmark': {'settled': True, **task['inputs'], 'k': 19,
                'validator_cpu_ms': None, 'validator_cpu_status': 'unavailable'},
        }
        obs = self.artifact('observation', {**header, **(defaults.get(adapter, {}) if payload is None else payload)})
        roles = delivery.required_roles(adapter)
        artifacts = [obs] + [self.artifact(r, raw=b'public opaque bytes\n') for r in roles if r != 'observation']
        for ref in artifacts:
            if ref['role']=='resource-receipt':
                ref.update(self.file(Path(ref['path']).name,encoded({
                    'schema':'doctorwatson.cleanup-observation/1','task':task['id'],
                    'attempt':task['attempt'],'fingerprint':fp,
                    'cleanup':'complete','restoration':'complete'})))
        r = {'task': task['id'], 'attempt': task['attempt'], 'fingerprint': fp, 'adapter': adapter,
             'terminal': 'succeeded', 'exit_code': 0, 'signal': None, 'cleanup': 'complete',
             'submission': 'included' if adapter in ('sherlock-anchor-settle-readback', 'sherlock-rollup-benchmark') else 'not-applicable',
             'artifacts': artifacts, 'review': None, 'adoption': None}
        if adopt:
            manifest = delivery.artifact_manifest_digest(artifacts)
            common = {'task': task['id'], 'attempt': task['attempt'], 'fingerprint': fp,
                      'artifact_manifest_sha256': manifest}
            review = self.artifact('review', {'schema': 'doctorwatson.delivery-review/1', **common,
                         'reviewer': 'Opus', 'approved': True})
            r['review'] = review
            r['adoption'] = self.artifact('adoption', {'schema': 'doctorwatson.delivery-adoption/1',
                         **common, 'integrator': 'Root', 'review_sha256': review['sha256'], 'adopted': True})
        self.history['results'].append(r)
        return r

    def inspect(self):
        return delivery.inspect_delivery(*self.save())

    def row(self, name='build'):
        return next(t for t in self.inspect()['tasks'] if t['id'] == name)

    def rewrite_observation(self, result, update):
        ref = result['artifacts'][0]
        value = json.loads(Path(ref['path']).read_bytes()); value.update(update)
        ref.update(self.file(Path(ref['path']).name, encoded(value)))

    def invalid_plan(self, mutate):
        mutate(self.plan)
        p, _ = self.save()
        with self.assertRaises(delivery.SchemaError):
            delivery.load_plan(p)

    def test_ready_is_selection_only(self):
        out = self.inspect()
        self.assertEqual(out['ready_for_root_selection'], ['build'])
        self.assertIs(out['execution_authority'], False)
        self.assertEqual(out['producer_authenticity'], 'not-established')

    def test_unknown_plan_member(self):
        self.invalid_plan(lambda p: p.update(command=['sh', '-c', 'anything']))

    def test_arbitrary_command_in_task_refused(self):
        self.invalid_plan(lambda p: p['tasks'][0].update(argv=['anything']))

    def test_arbitrary_adapter_input_refused(self):
        self.invalid_plan(lambda p: p['tasks'][0]['inputs'].update(command='anything'))

    def test_duplicate_task(self):
        self.invalid_plan(lambda p: p['tasks'].append(copy.deepcopy(p['tasks'][0])))

    def test_unknown_dependency(self):
        self.invalid_plan(lambda p: p['tasks'][0]['dependencies'].append({'task': 'missing', 'stage': 'observed'}))

    def test_cycle(self):
        self.plan['tasks'].append(self.task('other', dependencies=[{'task': 'build', 'stage': 'observed'}]))
        self.invalid_plan(lambda p: p['tasks'][0]['dependencies'].append({'task': 'other', 'stage': 'accepted'}))

    def test_unknown_stage(self):
        self.invalid_plan(lambda p: p['tasks'][0]['dependencies'].append({'task': 'build', 'stage': 'passed'}))

    def test_unknown_adapter(self):
        self.invalid_plan(lambda p: p['tasks'][0].update(adapter='shell'))

    def test_schema_version(self):
        self.invalid_plan(lambda p: p.update(schema='doctorwatson.delivery-plan/2'))

    def test_boolean_is_not_byte_count(self):
        self.invalid_plan(lambda p: p['tasks'][0]['identity']['source'][0].update(bytes=True))

    def test_duplicate_json_members(self):
        p, _ = self.save(); p.write_text('{"schema": "a", "schema": "b"}')
        with self.assertRaises(delivery.SchemaError): delivery.load_plan(p)

    def test_nonfinite_json(self):
        for value in ['NaN', 'Infinity', '-Infinity', '1e999']:
            p, _ = self.save(); p.write_text('{"x": ' + value + '}')
            with self.subTest(value=value), self.assertRaises(delivery.SchemaError): delivery.load_plan(p)

    def test_success_requires_real_complete_artifacts(self):
        self.success()
        self.assertEqual(self.row()['state'], 'observed')
        self.assertEqual(self.inspect()['ready_for_root_selection'], [])

    def test_missing_artifact_cannot_pass(self):
        r = self.success(); Path(r['artifacts'][1]['path']).unlink()
        self.assertEqual(self.row()['state'], 'artifact-invalid')

    def test_tampered_artifact_cannot_pass(self):
        r = self.success(); Path(r['artifacts'][1]['path']).write_bytes(b'tampered')
        self.assertEqual(self.row()['state'], 'artifact-invalid')

    def test_missing_membership_cannot_pass(self):
        r = self.success(); r['artifacts'].pop()
        self.assertEqual(self.row()['state'], 'artifact-invalid')

    def test_unexpected_artifact_role_refused(self):
        r = self.success(); r['artifacts'].append(self.artifact('unknown', raw=b'x'))
        with self.assertRaises(delivery.SchemaError): self.inspect()

    def test_symlink_artifact_cannot_pass(self):
        r = self.success(); p = Path(r['artifacts'][1]['path']); q = p.with_suffix('.real')
        p.rename(q); p.symlink_to(q)
        self.assertEqual(self.row()['state'], 'artifact-invalid')

    def test_exit_zero_without_semantic_success(self):
        self.success(payload={'built': False, 'test_bodies_executed': False})
        self.assertEqual(self.row()['state'], 'predicate-failed')

    def test_build_does_not_assert_test_execution(self):
        self.success(payload={'built': True, 'test_bodies_executed': True})
        self.assertEqual(self.row()['state'], 'predicate-failed')

    def test_artifact_identity_is_checked(self):
        r = self.success(); self.rewrite_observation(r, {'attempt': 'late-old-attempt'})
        self.assertEqual(self.row()['state'], 'artifact-invalid')

    def test_artifact_schema_and_unknown_members_checked(self):
        for update in [{'schema': 'moriarty-core/1'}, {'verified': True}]:
            with self.subTest(update=update):
                self.history['results'].clear(); r = self.success(); self.rewrite_observation(r, update)
                self.assertEqual(self.row()['state'], 'artifact-invalid')

    def test_exact_boolean_verifier(self):
        self.plan['tasks'][0] = self.task('verify', 'recl9-terminal-verify', inputs={k: hashlib.sha256(b'public opaque bytes\n').hexdigest() for k in ['relation_sha256', 'key_sha256', 'proof_sha256', 'statement_sha256']})
        for value in [False, 1, 'true', None]:
            with self.subTest(value=value):
                self.history['results'].clear(); self.success(payload={'verified': value, **self.plan['tasks'][0]['inputs']})
                self.assertNotEqual(self.row('verify')['state'], 'observed')

    def handkey(self):
        pin = hashlib.sha256(b'public opaque bytes\n').hexdigest()
        self.plan['tasks'][0] = self.task('verify', 'handkey-verify-pis',
            inputs={k:pin for k in ['key_sha256','proof_sha256','statement_sha256']})
        try:
            r = self.success(payload={'verifies':True, **self.plan['tasks'][0]['inputs']})
        except delivery.SchemaError as exc:
            self.fail(f'current handkey JSON adapter must be supported: {exc}')
        stdout = next(a for a in r['artifacts'] if a['role']=='stdout')
        stdout.update(self.file(Path(stdout['path']).name,encoded({'verifies':True,'verify_s':0.25})))
        return r, stdout

    def test_actual_handkey_json_true_vector_verdict(self):
        self.handkey()
        self.assertEqual(self.row('verify')['state'],'observed')
        self.assertEqual(self.row('verify')['scope'],'supplied handkey public-vector verification; ledger acceptance not established')

    def test_actual_handkey_false_with_exit_zero_fails(self):
        r, stdout = self.handkey()
        stdout.update(self.file(Path(stdout['path']).name, encoded({'verifies':False,'verify_s':0.25})))
        self.assertEqual(self.row('verify')['state'],'predicate-failed')

    def test_actual_handkey_boolean_type_is_exact(self):
        r, stdout = self.handkey()
        stdout.update(self.file(Path(stdout['path']).name, encoded({'verifies':1,'verify_s':0.25})))
        self.assertEqual(self.row('verify')['state'],'artifact-invalid')

    def test_actual_handkey_stdout_requires_one_complete_json_object(self):
        _,stdout=self.handkey()
        raw=encoded({'verifies':True,'verify_s':0.25})*2
        stdout.update(self.file(Path(stdout['path']).name,raw))
        self.assertEqual(self.row('verify')['state'],'artifact-invalid')

    def test_actual_handkey_unknown_stdout_members_refused(self):
        r, stdout = self.handkey()
        stdout.update(self.file(Path(stdout['path']).name, encoded({'verifies':True,'verify_s':0.25,'invented':True})))
        self.assertEqual(self.row('verify')['state'],'artifact-invalid')

    def test_verifier_tuple_mismatch(self):
        self.plan['tasks'][0] = self.task('verify', 'recl9-terminal-verify', inputs={k: hashlib.sha256(b'public opaque bytes\n').hexdigest() for k in ['relation_sha256', 'key_sha256', 'proof_sha256', 'statement_sha256']})
        self.success(payload={'verified': True, **self.plan['tasks'][0]['inputs'], 'key_sha256': 'c'*64})
        self.assertEqual(self.row('verify')['state'], 'predicate-failed')

    def test_cleanup_debt_cannot_pass(self):
        r = self.success(); r['cleanup'] = 'owed'
        self.assertEqual(self.row()['state'], 'cleanup-debt')

    def test_result_cleanup_string_cannot_replace_actual_cleanup_artifact(self):
        r=self.success()
        ref=next(a for a in r['artifacts'] if a['role']=='resource-receipt')
        ref.update(self.file(Path(ref['path']).name,b'not a cleanup receipt'))
        self.assertEqual(self.row()['state'],'artifact-invalid')

    def test_cleanup_artifact_owed_blocks_even_complete_result_string(self):
        r=self.success()
        ref=next(a for a in r['artifacts'] if a['role']=='resource-receipt')
        value=json.loads(Path(ref['path']).read_bytes()); value['restoration']='owed'
        ref.update(self.file(Path(ref['path']).name,encoded(value)))
        self.assertEqual(self.row()['state'],'cleanup-debt')

    def test_historical_cleanup_debt_survives_attempt_and_source_changes(self):
        r=self.success(); r['cleanup']='owed'
        self.plan['tasks'][0]['attempt']='build-002'
        self.plan['tasks'][0]['identity']['oracle_sha256']='d'*64
        self.assertEqual(self.row()['state'],'cleanup-debt')
        self.assertEqual(self.inspect()['ready_for_root_selection'],[])

    def test_historical_unknown_submission_does_not_disappear(self):
        r=self.success(); r['submission']='unknown'
        self.plan['tasks'][0]['attempt']='build-002'
        self.assertEqual(self.row()['state'],'unknown-reconcile')
        self.assertEqual(self.inspect()['ready_for_root_selection'],[])

    def test_unknown_submission_cannot_pass(self):
        r = self.success(); r['submission'] = 'unknown'
        self.assertEqual(self.row()['state'], 'unknown-reconcile')

    def test_missing_receipt_after_possible_launch_is_unknown(self):
        self.plan['tasks'][0]['launch_state'] = 'possible-launch'
        self.assertEqual(self.row()['state'], 'unknown-reconcile')
        self.assertEqual(self.inspect()['ready_for_root_selection'], [])

    def test_missing_receipt_new_attempt_with_history_is_unknown(self):
        self.success(); self.plan['tasks'][0]['attempt'] = 'build-002'
        self.plan['tasks'][0]['launch_state'] = 'possible-launch'
        self.assertEqual(self.row()['state'], 'unknown-reconcile')
        self.assertEqual(self.row()['history'][0]['currentness'], 'stale-attempt')

    def test_late_old_attempt_never_passes_or_erases_history(self):
        self.success(); self.plan['tasks'][0]['attempt'] = 'build-002'
        row = self.row()
        self.assertEqual(row['state'], 'stale')
        self.assertEqual(len(row['history']), 1)
        self.assertEqual(row['history'][0]['currentness'], 'stale-attempt')

    def test_source_drift_is_stale_not_missing(self):
        self.success(); Path(self.source['path']).write_bytes(b'changed source\n')
        row = self.row()
        self.assertEqual(row['state'], 'stale')
        self.assertEqual(len(row['history']), 1)
        self.assertEqual(row['history'][0]['currentness'], 'stale-input-files')

    def test_updated_current_source_pin_invalidates_historical_p12(self):
        self.success(); self.plan['tasks'][0]['identity']['source'] = [self.file('new-current-source', b'new source')]
        self.assertEqual(self.row()['history'][0]['currentness'], 'stale-fingerprint')
        self.assertEqual(self.row()['state'], 'stale')

    def test_dependency_drift_transitively_stales_descendant_only(self):
        self.plan['tasks'] += [self.task('child', dependencies=[{'task':'build','stage':'observed'}]), self.task('unrelated')]
        for t in self.plan['tasks']: self.success(t)
        self.plan['tasks'][0]['identity']['oracle_sha256'] = 'c'*64
        self.assertEqual(self.row('child')['state'], 'stale')
        self.assertEqual(self.row('unrelated')['state'], 'observed')

    def test_source_integrated_does_not_satisfy_observed(self):
        self.plan['tasks'][0] = self.task('source', 'source-integration')
        self.plan['tasks'].append(self.task('child', dependencies=[{'task':'source','stage':'observed'}]))
        self.success(self.plan['tasks'][0])
        self.assertEqual(self.row('source')['state'], 'source-integrated')
        self.assertEqual(self.row('child')['state'], 'blocked-dependency')

    def test_source_edge_can_unlock_independent_source_work(self):
        self.plan['tasks'][0] = self.task('source', 'source-integration')
        self.plan['tasks'].append(self.task('child', dependencies=[{'task':'source','stage':'source-integrated'}]))
        self.success(self.plan['tasks'][0])
        self.assertEqual(self.inspect()['ready_for_root_selection'], ['child'])

    def test_observed_does_not_satisfy_accepted(self):
        self.plan['tasks'].append(self.task('child', dependencies=[{'task':'build','stage':'accepted'}]))
        self.success()
        self.assertEqual(self.row('child')['state'], 'blocked-dependency')

    def test_review_and_root_adoption_are_explicit_external_premises(self):
        self.plan['tasks'].append(self.task('child', dependencies=[{'task':'build','stage':'accepted'}]))
        self.success(adopt=True)
        row = self.row()
        self.assertIs(row['acceptance_recorded'], True)
        self.assertEqual(self.inspect()['ready_for_root_selection'], ['child'])
        self.assertEqual(row['producer_authenticity'], 'not-established')

    def test_author_cannot_be_applicable_reviewer(self):
        r = self.success(adopt=True)
        ref = r['review']; value = json.loads(Path(ref['path']).read_bytes()); value['reviewer'] = 'Sol'
        ref.update(self.file(Path(ref['path']).name, encoded(value)))
        self.assertFalse(self.row()['acceptance_recorded'])

    def test_changed_artifacts_invalidate_review_and_adoption(self):
        r = self.success(adopt=True)
        ref = r['artifacts'][1]; ref.update(self.file(Path(ref['path']).name, b'updated stdout'))
        self.assertFalse(self.row()['acceptance_recorded'])

    def test_terminal_failure_not_automatically_retried(self):
        r = self.success(); r.update(terminal='timed-out', exit_code=None, signal=9)
        self.assertEqual(self.row()['state'], 'timed-out')
        self.assertEqual(self.inspect()['ready_for_root_selection'], [])

    def test_new_attempt_id_cannot_retry_unchanged_timed_out_operation(self):
        r=self.success(); r.update(terminal='timed-out',exit_code=None,signal=9)
        self.plan['tasks'][0]['attempt']='build-002'
        self.assertEqual(self.inspect()['ready_for_root_selection'],[])
        self.assertIn('unchanged consumed operation requires changed inputs', self.row()['blockers'])

    def test_changed_operation_can_be_selected_without_erasing_failed_attempt(self):
        r=self.success(); r.update(terminal='timed-out',exit_code=None,signal=9)
        self.plan['tasks'][0]['attempt']='build-002'
        self.plan['tasks'][0]['identity']['config']=[self.file('changed-config.json',b'{"diagnostic":"phase-timing"}\n')]
        self.assertEqual(self.inspect()['ready_for_root_selection'],['build'])
        self.assertEqual(self.row()['history'][0]['terminal'],'timed-out')

    def test_cancelled_retains_history(self):
        self.success(); self.plan['tasks'][0]['disposition']='cancelled'
        self.assertEqual(self.row()['state'], 'cancelled')
        self.assertEqual(len(self.row()['history']), 1)

    def test_duplicate_attempt_receipt_is_ambiguous(self):
        r = self.success(); self.history['results'].append(copy.deepcopy(r))
        with self.assertRaises(delivery.SchemaError): self.inspect()

    def test_unknown_outcome_refused(self):
        r=self.success(); r['terminal']='maybe-success'
        with self.assertRaises(delivery.SchemaError): self.inspect()

    def test_failure_record_validity_is_not_task_success(self):
        r=self.success(); r.update(terminal='failed', exit_code=1)
        self.assertTrue(self.inspect()['records_valid'])
        self.assertEqual(self.row()['state'], 'failed')

    def test_compile_cannot_close_sdk_tests_or_compact_build9(self):
        self.plan['tasks'] += [self.task('sdk','sdk-selected-tests', inputs={'selectors':['a','b']}), self.task('compact','compact-anchor-build9')]
        self.success()
        self.assertEqual(self.inspect()['ready_for_root_selection'], ['sdk','compact'])

    def test_sdk_listing_or_partial_membership_not_success(self):
        self.plan['tasks'][0] = self.task('sdk','sdk-selected-tests', inputs={'selectors':['a','b']})
        self.success(payload={'tests':[{'name':'a','status':'passed'}]})
        self.assertEqual(self.row('sdk')['state'], 'predicate-failed')

    def test_compact_requires_all_nine_native_leaves(self):
        self.plan['tasks'][0] = self.task('compact','compact-anchor-build9')
        r=self.success(); r['artifacts']=[a for a in r['artifacts'] if a['role']!='zkir/settleValidity.bzkir']
        self.assertEqual(self.row('compact')['state'], 'artifact-invalid')

    def test_benchmark_16_does_not_close_96(self):
        self.plan['tasks'][0] = self.task('bench','sherlock-rollup-benchmark', inputs={'profile':'fenrir-history-v1','transactions':96,'batches':6,'seed':'0x5354524157'})
        self.success(payload={'profile':'fenrir-history-v1','transactions':16,'batches':1,'seed':'0x5354524157','settled':True,'k':19,'validator_cpu_ms':None,'validator_cpu_status':'unavailable'})
        self.assertEqual(self.row('bench')['state'], 'predicate-failed')

    def settlement_inputs(self):
        return {'network':'stagenet','anchor':'public-anchor',
                'deployment_sha256':'b'*64,'binding_sha256':'c'*64,
                'batch':1,'expected_root':'42'}

    def test_settlement_plan_precedes_output_without_retrospective_mutation(self):
        self.plan['tasks'][0]=self.task('settle','sherlock-anchor-settle-readback',
                                       inputs={**self.settlement_inputs(),
                                               'anchor':fixture_support.FIXTURE_ADDRESS})
        plan_path,history_path=self.save()
        before_bytes=plan_path.read_bytes()
        before_stat=plan_path.stat()
        self.assertNotIn('post_state_sha256',self.plan['tasks'][0]['inputs'])
        self.assertEqual(list(self.root.glob('artifact-*')),[])
        try:
            before_fp=delivery.fingerprint(delivery.load_plan(plan_path),'settle')
        except delivery.SchemaError as exc:
            self.fail(f'legal predispatch plan must not require future readback digest: {exc}')
        result=self.success(self.plan['tasks'][0])
        post=next(a for a in result['artifacts'] if a['role']=='post-state')
        # Actual harness-shaped serializer output (settleParent +
        # assertReadbackAction/assertSuccessfulSubmitReceipt returns),
        # bound to this predispatch plan; not opaque bytes.
        harness=fixture_support.settle_result(expected_root='42',batch_no=1)
        harness['post_state_sha256']=post['sha256']
        source=next(a for a in result['artifacts'] if a['role']=='source-result')
        source.update(self.file(Path(source['path']).name,
                      (json.dumps(harness,separators=(',',':'),allow_nan=False)+'\n').encode()))
        self.rewrite_observation(result,{'post_state_sha256':post['sha256']})
        _,history_path=self.save()
        report=delivery.inspect_delivery(plan_path,history_path)
        row=report['tasks'][0]
        self.assertEqual(row['state'],'observed')
        self.assertEqual(row['observation']['post_state_sha256'],post['sha256'])
        self.assertEqual(delivery.fingerprint(delivery.load_plan(plan_path),'settle'),before_fp)
        self.assertEqual(plan_path.read_bytes(),before_bytes)
        self.assertEqual(plan_path.stat().st_mtime_ns,before_stat.st_mtime_ns)
        Path(post['path']).write_bytes(b'changed observed readback')
        self.assertEqual(delivery.inspect_delivery(plan_path,history_path)['tasks'][0]['state'],'artifact-invalid')
        self.assertEqual(delivery.fingerprint(delivery.load_plan(plan_path),'settle'),before_fp)

    def test_observed_poststate_digest_must_match_checked_output(self):
        self.plan['tasks'][0]=self.task('settle','sherlock-anchor-settle-readback',
                                       inputs=self.settlement_inputs())
        try:
            result=self.success()
        except delivery.SchemaError as exc:
            self.fail(f'predispatch expected-root plan must be supported: {exc}')
        self.rewrite_observation(result,{'post_state_sha256':'d'*64})
        self.assertEqual(self.row('settle')['state'],'artifact-invalid')

    def test_source_integrated_edge_requires_source_integration_producer(self):
        self.plan['tasks'].append(self.task('child',
            dependencies=[{'task':'build','stage':'source-integrated'}]))
        path,_=self.save()
        with self.assertRaises(delivery.SchemaError):
            delivery.load_plan(path)

    def test_settlement_expectedroot_is_canonical_public_decimal(self):
        for root in [42,'042','-1','1.0','true']:
            with self.subTest(root=root):
                self.plan['tasks'][0]=self.task('settle','sherlock-anchor-settle-readback',
                    inputs={**self.settlement_inputs(),'expected_root':root})
                path,_=self.save()
                with self.assertRaises(delivery.SchemaError): delivery.load_plan(path)

    def test_readback_core1_is_not_anchor_settlement(self):
        inputs={'network':'stagenet','anchor':'public-anchor','deployment_sha256':'b'*64,'binding_sha256':'c'*64,'batch':1,'expected_root':'42'}
        self.plan['tasks'][0]=self.task('settle','sherlock-anchor-settle-readback',inputs=inputs)
        r=self.success(); self.rewrite_observation(r, {'schema':'moriarty-core/1'})
        self.assertEqual(self.row('settle')['state'], 'artifact-invalid')

    def test_unknown_validator_cpu_remains_null(self):
        self.plan['tasks'][0] = self.task('bench','sherlock-rollup-benchmark', inputs={'profile':'fenrir-history-v1','transactions':16,'batches':1,'seed':'0x5354524157'})
        self.success()
        self.assertIsNone(self.row('bench')['observation']['validator_cpu_ms'])


if __name__ == '__main__': unittest.main()
