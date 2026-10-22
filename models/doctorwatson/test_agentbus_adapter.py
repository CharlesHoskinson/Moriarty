"""Offline adapter contract tests; no AgentBus installation or identity is needed."""
import copy
import importlib
import importlib.util
import json
from pathlib import Path
import socket
import unittest
from unittest.mock import patch


class TransportFailure(Exception):
    code = 5


class FakeTransport:
    run = 'shared-run'
    did = 'did:key:zParticipant'

    def __init__(self):
        self.calls = []
        self.failure = None
        self.fail_claim_at = None

    def send(self, to, subj, body='', refs=(), ttl=None):
        self.calls.append(('send', to, subj, body, refs, ttl))
        if self.failure:
            raise self.failure
        return 'mqtt-envelope-id'

    def claim(self, resource, lease_min=None, wait=None):
        self.calls.append(('claim', resource, lease_min, wait))
        if len(self.calls) == self.fail_claim_at:
            raise TransportFailure('private detail must not escape')
        return {'kind': 'grant', 'resource': 'file:/repo/' + resource[5:],
                'request': 'current-command', 'hold': 'original-acquisition',
                'lease_end': '2030-01-01T00:00:00Z'}


class AdapterTests(unittest.TestCase):
    def setUp(self):
        self.assertIsNotNone(importlib.util.find_spec('agentbus_adapter'),
                             'the optional adapter has not been implemented')
        self.a = importlib.import_module('agentbus_adapter')
        self.context = dict(run_id='shared-run', task_id='compile', attempt_id='a1',
                            task_fingerprint='a' * 64, integration_base='base-sha',
                            sender_did='did:key:zParticipant')
        self.bus = FakeTransport()

    def notice(self, **overrides):
        return self.a.make_notice(event='handoff', **(self.context | overrides))

    def envelope(self, notice=None):
        return {'run': 'shared-run', 'from': self.bus.did,
                'body': json.dumps(notice or self.notice())}

    def test_stable_event_identity_and_attempt_changes(self):
        notice = self.notice()
        self.assertEqual(notice, self.notice())
        self.assertNotEqual(notice['event_id'], self.notice(attempt_id='a2')['event_id'])
        self.assertNotEqual(notice['event_id'], self.notice(task_fingerprint='b'*64)['event_id'])

    def test_valid_notice_only_requests_canonical_inspection(self):
        result = self.a.classify_notice(self.envelope(), expected=self.context)
        self.assertEqual(result['status'], 'advisory')
        self.assertEqual(result['action'], 'inspect-canonical-state')

    def test_context_changes_are_stale_and_duplicates_have_no_action(self):
        for field, value in [('run_id', 'other'), ('task_id', 'other'),
                             ('attempt_id', 'a2'), ('task_fingerprint', 'b'*64),
                             ('integration_base', 'new-base'), ('sender_did', 'did:key:zOther')]:
            with self.subTest(field=field):
                result = self.a.classify_notice(self.envelope(), expected=self.context | {field:value})
                self.assertEqual(result['status'], 'stale')
                self.assertNotIn('action', result)
        result = self.a.classify_notice(self.envelope(), expected=self.context,
                                       seen_ids={self.notice()['event_id']})
        self.assertEqual(result['status'], 'duplicate')
        self.assertNotIn('action', result)

    def test_transport_attribution_and_event_hash_must_match(self):
        for envelope in [self.envelope() | {'run':'wrong'}, self.envelope() | {'from':'did:key:zOther'},
                         self.envelope(self.notice() | {'event_id':'0'*64})]:
            self.assertEqual(self.a.classify_notice(envelope, expected=self.context)['status'], 'invalid')

    def test_strict_json_and_unknown_fields_refuse(self):
        for body in ['{"schema":1,"schema":2}', 'NaN', '[]', '{', '"\\ud800"', ' '*17000]:
            self.assertEqual(self.a.classify_notice(self.envelope() | {'body':body},
                                                   expected=self.context)['status'], 'invalid')
        result = self.a.classify_notice(self.envelope(self.notice() | {'launch':True}), expected=self.context)
        self.assertEqual(result['status'], 'invalid')

    def test_notice_rejects_unknown_event_and_bad_source_refs(self):
        for ref in [{'path':'relative','bytes':1,'sha256':'a'*64},
                    {'path':'/repo/../secret','bytes':1,'sha256':'a'*64},
                    {'path':'/repo/source','bytes':True,'sha256':'a'*64},
                    {'path':'/repo/source','bytes':1,'sha256':'A'*64},
                    {'path':'/repo/source','bytes':1,'sha256':'a'*64,'extra':0}]:
            with self.assertRaises(self.a.AdapterError):
                self.notice(refs=[ref])
        with self.assertRaises(self.a.AdapterError):
            self.a.make_notice(event='dispatch', **self.context)

    def test_bounded_unique_refs_and_caller_data_not_mutated(self):
        ref = {'path':'/repo/source','bytes':12,'sha256':'a'*64}
        before = copy.deepcopy(ref)
        notice = self.notice(refs=[ref])
        ref['bytes'] = 3
        self.assertEqual(notice['refs'], [before])
        for refs in [[before,before], [before]*17]:
            with self.assertRaises(self.a.AdapterError):
                self.notice(refs=refs)

    def test_publish_retries_keep_application_id_and_report_only_broker_receipt(self):
        notice = self.notice()
        for _ in range(2):
            receipt = self.a.publish_notice(self.bus, 'reviewer', notice)
            self.assertEqual(receipt, {'status':'published','event_id':notice['event_id'],
                                       'message_id':'mqtt-envelope-id'})
        self.assertEqual(self.bus.calls[0][3], self.bus.calls[1][3])

    def test_disabled_and_failed_transport_do_not_mutate_notice(self):
        notice = self.notice()
        before = copy.deepcopy(notice)
        self.assertEqual(self.a.publish_notice(None, 'reviewer', notice)['status'], 'disabled')
        for failure in [TransportFailure('private'), OSError('private')]:
            self.bus.failure = failure
            receipt = self.a.publish_notice(self.bus, 'reviewer', notice)
            self.assertEqual(receipt['status'], 'unavailable')
            self.assertNotIn('private', json.dumps(receipt))
        self.assertEqual(notice,before)

    def test_publish_wrong_run_or_did_refuses_before_send(self):
        for field, value in [('run','other'), ('did','did:key:zOther')]:
            with patch.object(self.bus, field, value):
                with self.assertRaises(self.a.AdapterError):
                    self.a.publish_notice(self.bus, 'reviewer', self.notice())
        self.assertEqual(self.bus.calls, [])

    def test_owned_claims_validate_all_paths_before_side_effects(self):
        for files in [['owned.py','unowned.py'], ['../outside'], ['/absolute'], ['dir/'], ['x#L1-L2'], ['a/./b']]:
            with self.assertRaises(self.a.AdapterError):
                self.a.claim_owned_files(self.bus, owned_files=['owned.py'], files=files)
        self.assertEqual(self.bus.calls, [])

    def test_claim_receipts_keep_original_hold_and_partial_failure(self):
        self.bus.fail_claim_at = 2
        receipt = self.a.claim_owned_files(self.bus, owned_files=['a.py','b.py'], files=['a.py','b.py'])
        self.assertEqual(receipt['status'],'incomplete')
        self.assertEqual(receipt['grants'][0]['hold'],'original-acquisition')
        self.assertEqual(receipt['grants'][0]['resource'],'file:/repo/a.py')
        self.assertEqual(len(self.bus.calls),2)

    def test_missing_hold_never_becomes_current_request_token(self):
        with patch.object(self.bus,'claim',return_value={'kind':'grant','request':'new','resource':'file:/repo/a.py'}):
            receipt = self.a.claim_owned_files(self.bus,owned_files=['a.py'],files=['a.py'])
        self.assertEqual(receipt['status'],'incomplete')
        self.assertEqual(receipt['grants'],[])

    def test_successful_and_disabled_claims_are_explicit(self):
        receipt = self.a.claim_owned_files(self.bus,owned_files=['a.py'],files=['a.py'])
        self.assertEqual(receipt['status'],'advisory')
        self.assertEqual(self.a.claim_owned_files(None,owned_files=['a.py'],files=['a.py'])['status'],'disabled')

    def test_active_claim_adds_retention_but_never_inventory_completeness(self):
        snapshot = {'run_id':'shared-run','captured_at':1000,'complete':True,
                    'claims':[{'resource':'file:/repo/a.py','holder_did':self.bus.did,
                               'lease_end':'2030-01-01T00:00:00Z'}]}
        result = self.a.cleanup_retention(snapshot, run_id='shared-run',
                                          resource_worktrees={'file:/repo/a.py':'/repo'},now=1001)
        self.assertIs(result['leases_known'],False)
        self.assertEqual(result['lease_worktrees'],['/repo'])

    def test_cleanup_unknown_stale_expired_and_empty_never_establish_inactivity(self):
        for snapshot in [None, {}, {'run_id':'other','captured_at':1000,'claims':[]},
                         {'run_id':'shared-run','captured_at':1,'claims':[]},
                         {'run_id':'shared-run','captured_at':1002,'claims':[]},
                         {'run_id':'shared-run','captured_at':1000,'claims':[]},
                         {'run_id':'shared-run','captured_at':1000,'claims':[{'resource':'unknown','holder_did':self.bus.did,'lease_end':1}]}]:
            result = self.a.cleanup_retention(snapshot,run_id='shared-run',resource_worktrees={},now=1001)
            self.assertIs(result['leases_known'],False)
            self.assertNotIn('inactive',result)

    def test_stale_partial_inventory_preserves_positive_retention(self):
        snapshot = {'run_id':'shared-run','captured_at':0,'claims':[
            {'resource':'r','holder_did':self.bus.did,'lease_end':2000}, {'garbage':True}]}
        result = self.a.cleanup_retention(snapshot,run_id='shared-run',resource_worktrees={'r':'/repo'},now=1001)
        self.assertEqual(result['lease_worktrees'],['/repo'])
        self.assertIs(result['leases_known'],False)

    def test_import_does_not_require_agentbus_or_open_network(self):
        with patch.object(socket,'socket',side_effect=AssertionError('network forbidden')):
            importlib.reload(self.a)
            self.a.publish_notice(None,'reviewer',self.notice())


if __name__ == '__main__':
    unittest.main()
