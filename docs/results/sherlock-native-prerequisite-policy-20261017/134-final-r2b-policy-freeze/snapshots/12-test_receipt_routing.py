import builtins
import contextlib
import copy
import hashlib
import io
import json
import os
import pathlib
import unittest
from unittest import mock

import bounded_sdk_compile as guard
import test_bounded_sdk_compile_symlink_config as symlink_cfg


HERE = pathlib.Path(__file__).resolve().parent
CONFIG_SCHEMA_JSON = json.loads((HERE / 'config.schema.json').read_text(encoding='utf-8'))
DECISION_SCHEMA_JSON = json.loads((HERE / 'decision-records.schema.json').read_text(encoding='utf-8'))


def h64(ch):
    return ch * 64


def forbid_io():
    def _blocked(*_a, **_kw):
        raise AssertionError('unexpected filesystem I/O in synthetic receipt routing test')
    return contextlib.ExitStack(), [
        mock.patch.object(builtins, 'open', side_effect=_blocked),
        mock.patch.object(io, 'open', side_effect=_blocked),
        mock.patch.object(os, 'open', side_effect=_blocked),
        mock.patch.object(os, 'stat', side_effect=_blocked),
        mock.patch.object(os, 'lstat', side_effect=_blocked),
        mock.patch.object(os, 'readlink', side_effect=_blocked),
        mock.patch.object(os, 'scandir', side_effect=_blocked),
        mock.patch.object(os, 'listdir', side_effect=_blocked),
    ]


def make_routing_config(mode='tier1'):
    c = symlink_cfg.base_config()
    c['mode'] = mode
    c['bindings']['runner'] = {'path': '/abs/runner.py', 'sha256': h64('1')}
    c['bindings']['subject'] = {'path': '/abs/subject.json', 'sha256': h64('2')}
    c['bindings']['amendments'] = {
        'path': '/abs/amendments.json',
        'sha256': '3676da94b6ad533a2276271c55c3c47b8c4473170b1200cf0dcae2b362fbda5d',
    }
    c['source_manifest'] = {
        'path': '/abs/source-manifest.proposal.json',
        'sha256': '9f48d1f227c2714bab374828b7fe4dd930aab660ea63dd2685f91d6250bcafce',
    }
    c['review_receipts'] = {
        'security': {'path': '/abs/receipt-security.json', 'sha256': h64('a')},
        'nonsecurity': {'path': '/abs/receipt-nonsecurity.json', 'sha256': h64('b')},
        'root': {'path': '/abs/receipt-root.json', 'sha256': h64('c')},
        'tier1': {'path': '/abs/receipt-tier1.json', 'sha256': h64('d')},
    }
    if mode == 'tier1':
        c['external_tier_consent'] = {'tier1': True, 'tier2': False}
    else:
        c['review_receipts']['tier2'] = {'path': '/abs/receipt-tier2.json', 'sha256': h64('e')}
        c['external_tier_consent'] = {'tier1': True, 'tier2': True}
    projection = copy.deepcopy(c)
    projection.pop('review_receipts')
    projection['bindings'].pop('config_inputs')
    ci_sha = hashlib.sha256(guard.json_bytes(projection)).hexdigest()
    c['bindings']['config_inputs'] = {'path': '/abs/config_inputs.json', 'sha256': ci_sha}
    return c


def make_records(c, sec_routing=('claude-opus-5-5', 'claude-opus-5-5', 'claude-opus-5-5', 'high', 'primary'),
                 nonsec_routing=('claude-opus-5-5', 'gemini-4-argon-high', 'gemini-4-argon-high', 'high', 'standing_opus_quota_outage_fallback')):
    runner_sha = c['bindings']['runner']['sha256']
    subject_sha = c['bindings']['subject']['sha256']
    manifest_sha = c['source_manifest']['sha256']
    ci_sha = c['bindings']['config_inputs']['sha256']
    amend_sha = c['bindings']['amendments']['sha256']
    sec_r = {
        'schema_version': 1,
        'authority_granted_by_this_file_alone': False,
        'kind': 'security_source_review',
        'reviewer_role': 'designated_security_owner',
        'decision': 'APPROVED_SOURCE_WITH_LIMITS',
        'primary_model': sec_routing[0],
        'requested_model': sec_routing[1],
        'returned_identity': sec_routing[2],
        'effort': sec_routing[3],
        'fallback_status': sec_routing[4],
        'terminal_sha256': h64('7'),
        'report_sha256': h64('8'),
        'runner_sha256': runner_sha,
        'source_manifest_sha256': manifest_sha,
        'subject_manifest_sha256': subject_sha,
    }
    nonsec_r = {
        'schema_version': 1,
        'authority_granted_by_this_file_alone': False,
        'kind': 'nonsecurity_source_review',
        'reviewer_role': 'nonsecurity_source_reviewer',
        'decision': 'APPROVED_NORMAL_SOURCE',
        'primary_model': nonsec_routing[0],
        'requested_model': nonsec_routing[1],
        'returned_identity': nonsec_routing[2],
        'effort': nonsec_routing[3],
        'fallback_status': nonsec_routing[4],
        'terminal_sha256': h64('9'),
        'report_sha256': h64('0'),
        'runner_sha256': runner_sha,
        'source_manifest_sha256': manifest_sha,
        'subject_manifest_sha256': subject_sha,
    }
    root_r = {
        'schema_version': 1,
        'authority_granted_by_this_file_alone': False,
        'kind': 'root_admission',
        'decision': 'ADMITTED_CONFIGURATION',
        'admitted_mode': c['mode'],
        'runner_sha256': runner_sha,
        'subject_manifest_sha256': subject_sha,
        'config_inputs_sha256': ci_sha,
        'security_receipt_sha256': c['review_receipts']['security']['sha256'],
        'nonsecurity_receipt_sha256': c['review_receipts']['nonsecurity']['sha256'],
    }
    tier1_r = {
        'schema_version': 1,
        'authority_granted_by_this_file_alone': False,
        'kind': 'user_tier1_grant',
        'tier': 1,
        'decision': 'GRANTED_TIER1',
        'root_receipt_sha256': c['review_receipts']['root']['sha256'],
        'config_inputs_sha256': ci_sha,
        'amendments_sha256': amend_sha,
    }
    projection = copy.deepcopy(c)
    projection.pop('review_receipts')
    projection['bindings'].pop('config_inputs')
    by_path = {
        c['review_receipts']['security']['path']: sec_r,
        c['review_receipts']['nonsecurity']['path']: nonsec_r,
        c['review_receipts']['root']['path']: root_r,
        c['review_receipts']['tier1']['path']: tier1_r,
        c['bindings']['config_inputs']['path']: projection,
    }
    if c['mode'] == 'tier1-and-compile':
        tier2_r = {
            'schema_version': 1,
            'authority_granted_by_this_file_alone': False,
            'kind': 'user_tier2_grant',
            'tier': 2,
            'decision': 'GRANTED_TIER2',
            'root_receipt_sha256': c['review_receipts']['root']['sha256'],
            'config_inputs_sha256': ci_sha,
            'amendments_sha256': amend_sha,
            'tier1_receipt_sha256': c['review_receipts']['tier1']['sha256'],
        }
        by_path[c['review_receipts']['tier2']['path']] = tier2_r
    return by_path


def run_consent_with_mocks(c, by_path):
    checked_pins = []

    def fake_check_pin(p):
        checked_pins.append((p['path'], p['sha256']))

    def fake_jload(path):
        key = str(path)
        if key not in by_path:
            raise AssertionError(f'unexpected jload path: {key}')
        return copy.deepcopy(by_path[key])

    stack, patches = forbid_io()
    with stack:
        for p in patches:
            stack.enter_context(p)
        stack.enter_context(mock.patch.object(guard, 'check_pin', side_effect=fake_check_pin))
        stack.enter_context(mock.patch.object(guard, 'jload', side_effect=fake_jload))
        guard.consent(c)
    return checked_pins


class TestReceiptRouting(unittest.TestCase):
    def test_validate_tier1_and_tier2_receipt_key_sets_accepted(self):
        stack, patches = forbid_io()
        with stack:
            for p in patches:
                stack.enter_context(p)
            c1 = make_routing_config('tier1')
            guard.validate(c1)
            c2 = make_routing_config('tier1-and-compile')
            guard.validate(c2)

    def test_validate_legacy_muse_and_wrong_receipt_keys_refused(self):
        stack, patches = forbid_io()
        with stack:
            for p in patches:
                stack.enter_context(p)

            c_muse_replace = make_routing_config('tier1')
            c_muse_replace['review_receipts']['muse'] = c_muse_replace['review_receipts'].pop('nonsecurity')
            with self.assertRaisesRegex(guard.Refusal, 'schema keys: review receipts'):
                guard.validate(c_muse_replace)

            c_muse_extra = make_routing_config('tier1')
            c_muse_extra['review_receipts']['muse'] = {'path': '/abs/receipt-muse.json', 'sha256': h64('f')}
            with self.assertRaisesRegex(guard.Refusal, 'schema keys: review receipts'):
                guard.validate(c_muse_extra)

            for missing_key in ('security', 'nonsecurity', 'root', 'tier1'):
                c_missing = make_routing_config('tier1')
                c_missing['review_receipts'].pop(missing_key)
                with self.assertRaisesRegex(guard.Refusal, 'schema keys: review receipts'):
                    guard.validate(c_missing)

            c_t1_with_t2 = make_routing_config('tier1')
            c_t1_with_t2['review_receipts']['tier2'] = {'path': '/abs/receipt-tier2.json', 'sha256': h64('e')}
            with self.assertRaisesRegex(guard.Refusal, 'schema keys: review receipts'):
                guard.validate(c_t1_with_t2)

            c_t2_without_t2 = make_routing_config('tier1-and-compile')
            c_t2_without_t2['review_receipts'].pop('tier2')
            with self.assertRaisesRegex(guard.Refusal, 'schema keys: review receipts'):
                guard.validate(c_t2_without_t2)

            c_bad_pin = make_routing_config('tier1')
            c_bad_pin['review_receipts']['nonsecurity'] = {'path': 'relative/path.json', 'sha256': h64('b')}
            with self.assertRaisesRegex(guard.Refusal, 'normalized absolute path required'):
                guard.validate(c_bad_pin)

            c_bad_sha = make_routing_config('tier1')
            c_bad_sha['review_receipts']['nonsecurity'] = {'path': '/abs/receipt-nonsecurity.json', 'sha256': 'XYZ'}
            with self.assertRaisesRegex(guard.Refusal, 'full sha256 required'):
                guard.validate(c_bad_sha)

    def test_consent_accepts_primary_and_standing_argon_fallback_tuples_tier1_and_tier2(self):
        primary_tuple = ('claude-opus-5-5', 'claude-opus-5-5', 'claude-opus-5-5', 'high', 'primary')
        fallback_tuple = (
            'claude-opus-5-5',
            'gemini-4-argon-high',
            'gemini-4-argon-high',
            'high',
            'standing_opus_quota_outage_fallback',
        )
        for mode in ('tier1', 'tier1-and-compile'):
            for sec_t in (primary_tuple, fallback_tuple):
                for nonsec_t in (primary_tuple, fallback_tuple):
                    c = make_routing_config(mode)
                    by_path = make_records(c, sec_routing=sec_t, nonsec_routing=nonsec_t)
                    checked = run_consent_with_mocks(c, by_path)
                    expected_pin_count = 5 if mode == 'tier1' else 6
                    self.assertEqual(len(checked), expected_pin_count)

    def test_consent_refuses_counterfactual_or_mismatched_routing_tuples(self):
        bad_tuples = [
            ('claude-opus-5-5', 'claude-opus-5-5', 'gemini-4-argon-high', 'high', 'standing_opus_quota_outage_fallback'),
            ('claude-opus-5-5', 'gemini-4-argon-high', 'claude-opus-5-5', 'high', 'standing_opus_quota_outage_fallback'),
            ('gemini-4-argon-high', 'gemini-4-argon-high', 'gemini-4-argon-high', 'high', 'standing_opus_quota_outage_fallback'),
            ('gemini-4-argon-high', 'gemini-4-argon-high', 'gemini-4-argon-high', 'high', 'primary'),
            ('claude-opus-5-5', 'gemini-4-argon-high', 'gemini-4-argon-high', 'high', 'primary'),
            ('claude-opus-5-5', 'claude-opus-5-5', 'claude-opus-5-5', 'high', 'standing_opus_quota_outage_fallback'),
            ('claude-opus-5-5', 'claude-opus-5-5', 'claude-opus-5-5', 'medium', 'primary'),
            ('claude-opus-5-5', 'gemini-4-argon-high', 'gemini-4-argon-high', 'HIGH', 'standing_opus_quota_outage_fallback'),
            ('claude-opus-5-5', 'muse', 'muse', 'high', 'primary'),
            ('claude-opus-5-5', 'claude-opus-5-5', 'claude-opus-5-5', 'high', 'temporary_fallback'),
        ]
        for receipt_key in ('security', 'nonsecurity'):
            for bad in bad_tuples:
                c = make_routing_config('tier1')
                by_path = make_records(c)
                rec_path = c['review_receipts'][receipt_key]['path']
                rec = by_path[rec_path]
                rec['primary_model'], rec['requested_model'], rec['returned_identity'], rec['effort'], rec['fallback_status'] = bad
                with self.assertRaisesRegex(guard.Refusal, 'source review routing identity'):
                    run_consent_with_mocks(c, by_path)

            c_missing = make_routing_config('tier1')
            by_path_missing = make_records(c_missing)
            by_path_missing[c_missing['review_receipts'][receipt_key]['path']].pop('primary_model')
            with self.assertRaisesRegex(guard.Refusal, 'schema keys: source review receipt'):
                run_consent_with_mocks(c_missing, by_path_missing)

            c_extra = make_routing_config('tier1')
            by_path_extra = make_records(c_extra)
            by_path_extra[c_extra['review_receipts'][receipt_key]['path']]['extra_field'] = 'x'
            with self.assertRaisesRegex(guard.Refusal, 'schema keys: source review receipt'):
                run_consent_with_mocks(c_extra, by_path_extra)

    def test_consent_refuses_wrong_kind_role_decision_version_or_authority_claim(self):
        for receipt_key in ('security', 'nonsecurity'):
            rec_Mutations = [
                ('schema_version', 2, 'receipt version or authority claim'),
                ('authority_granted_by_this_file_alone', True, 'receipt version or authority claim'),
                ('kind', 'muse_normal_review', 'source review decision/role'),
                ('reviewer_role', 'muse_normal_reviewer', 'source review decision/role'),
                ('decision', 'REJECTED', 'source review decision/role'),
            ]
            for field, bad_val, expected_msg in rec_Mutations:
                c = make_routing_config('tier1')
                by_path = make_records(c)
                by_path[c['review_receipts'][receipt_key]['path']][field] = bad_val
                with self.assertRaisesRegex(guard.Refusal, expected_msg):
                    run_consent_with_mocks(c, by_path)

        c_swap = make_routing_config('tier1')
        by_path_swap = make_records(c_swap)
        sec_p = c_swap['review_receipts']['security']['path']
        nonsec_p = c_swap['review_receipts']['nonsecurity']['path']
        by_path_swap[sec_p], by_path_swap[nonsec_p] = by_path_swap[nonsec_p], by_path_swap[sec_p]
        with self.assertRaisesRegex(guard.Refusal, 'source review decision/role'):
            run_consent_with_mocks(c_swap, by_path_swap)

    def test_consent_refuses_malformed_terminal_report_or_source_manifest_hashes(self):
        for receipt_key in ('security', 'nonsecurity'):
            for hash_field, err_label in [
                ('terminal_sha256', 'review terminal/report hash'),
                ('report_sha256', 'review terminal/report hash'),
                ('source_manifest_sha256', 'review full-source manifest hash'),
            ]:
                for bad_hash in ('A' * 64, 'a' * 63, 'g' * 64, ''):
                    c = make_routing_config('tier1')
                    by_path = make_records(c)
                    by_path[c['review_receipts'][receipt_key]['path']][hash_field] = bad_hash
                    with self.assertRaisesRegex(guard.Refusal, err_label):
                        run_consent_with_mocks(c, by_path)

            c_runner = make_routing_config('tier1')
            by_path_runner = make_records(c_runner)
            by_path_runner[c_runner['review_receipts'][receipt_key]['path']]['runner_sha256'] = h64('f')
            with self.assertRaisesRegex(guard.Refusal, 'review subject identity'):
                run_consent_with_mocks(c_runner, by_path_runner)

            c_subj = make_routing_config('tier1')
            by_path_subj = make_records(c_subj)
            by_path_subj[c_subj['review_receipts'][receipt_key]['path']]['subject_manifest_sha256'] = h64('f')
            with self.assertRaisesRegex(guard.Refusal, 'review subject identity'):
                run_consent_with_mocks(c_subj, by_path_subj)

        c_disagree = make_routing_config('tier1')
        by_path_disagree = make_records(c_disagree)
        by_path_disagree[c_disagree['review_receipts']['nonsecurity']['path']]['source_manifest_sha256'] = h64('4')
        with self.assertRaisesRegex(guard.Refusal, 'review candidates differ'):
            run_consent_with_mocks(c_disagree, by_path_disagree)

    def test_consent_refuses_root_and_tier_binding_mismatches(self):
        c_legacy_root = make_routing_config('tier1')
        by_path_legacy_root = make_records(c_legacy_root)
        root_rec = by_path_legacy_root[c_legacy_root['review_receipts']['root']['path']]
        root_rec['muse_receipt_sha256'] = root_rec.pop('nonsecurity_receipt_sha256')
        with self.assertRaisesRegex(guard.Refusal, 'schema keys: Root admission'):
            run_consent_with_mocks(c_legacy_root, by_path_legacy_root)

        for root_field, bad_val, expected_msg in [
            ('admitted_mode', 'tier1-and-compile', 'Root mode/decision'),
            ('runner_sha256', h64('f'), 'Root candidate binding'),
            ('subject_manifest_sha256', h64('f'), 'Root candidate binding'),
            ('config_inputs_sha256', h64('f'), 'Root candidate binding'),
            ('security_receipt_sha256', h64('f'), 'Root review binding'),
            ('nonsecurity_receipt_sha256', h64('f'), 'Root review binding'),
        ]:
            c = make_routing_config('tier1')
            by_path = make_records(c)
            by_path[c['review_receipts']['root']['path']][root_field] = bad_val
            with self.assertRaisesRegex(guard.Refusal, expected_msg):
                run_consent_with_mocks(c, by_path)

        for t1_field, bad_val, expected_msg in [
            ('root_receipt_sha256', h64('f'), 'grant Root identity'),
            ('config_inputs_sha256', h64('f'), 'grant scope identity'),
            ('amendments_sha256', h64('f'), 'grant scope identity'),
        ]:
            c = make_routing_config('tier1')
            by_path = make_records(c)
            by_path[c['review_receipts']['tier1']['path']][t1_field] = bad_val
            with self.assertRaisesRegex(guard.Refusal, expected_msg):
                run_consent_with_mocks(c, by_path)

        for t2_field, bad_val, expected_msg in [
            ('root_receipt_sha256', h64('f'), 'grant Root identity'),
            ('config_inputs_sha256', h64('f'), 'grant scope identity'),
            ('amendments_sha256', h64('f'), 'grant scope identity'),
            ('tier1_receipt_sha256', h64('f'), 'Tier 2 predecessor grant'),
        ]:
            c = make_routing_config('tier1-and-compile')
            by_path = make_records(c)
            by_path[c['review_receipts']['tier2']['path']][t2_field] = bad_val
            with self.assertRaisesRegex(guard.Refusal, expected_msg):
                run_consent_with_mocks(c, by_path)

        c_ci_tamper = make_routing_config('tier1')
        by_path_ci_tamper = make_records(c_ci_tamper)
        by_path_ci_tamper[c_ci_tamper['bindings']['config_inputs']['path']]['cpu_arch'] = 'tampered'
        with self.assertRaisesRegex(guard.Refusal, 'configuration projection differs'):
            run_consent_with_mocks(c_ci_tamper, by_path_ci_tamper)

    def test_decision_records_schema_locks_routing_tuples_and_nonsecurity_defs(self):
        defs = DECISION_SCHEMA_JSON['$defs']
        self.assertEqual(set(defs.keys()), {'security', 'nonsecurity', 'root', 'tier1', 'tier2'})
        self.assertNotIn('muse', defs)
        expected_req = [
            'schema_version',
            'authority_granted_by_this_file_alone',
            'kind',
            'reviewer_role',
            'decision',
            'primary_model',
            'requested_model',
            'returned_identity',
            'effort',
            'fallback_status',
            'terminal_sha256',
            'report_sha256',
            'runner_sha256',
            'source_manifest_sha256',
            'subject_manifest_sha256',
        ]
        for key in ('security', 'nonsecurity'):
            d = defs[key]
            self.assertEqual(d['required'], expected_req)
            self.assertEqual(d['properties']['primary_model'], {'const': 'claude-opus-5-5'})
            self.assertEqual(
                d['properties']['requested_model'],
                {'enum': ['claude-opus-5-5', 'gemini-4-argon-high']},
            )
            self.assertEqual(
                d['properties']['returned_identity'],
                {'enum': ['claude-opus-5-5', 'gemini-4-argon-high']},
            )
            self.assertEqual(len(d['oneOf']), 2)
            b0 = d['oneOf'][0]['properties']
            b1 = d['oneOf'][1]['properties']
            self.assertEqual(
                (b0['primary_model']['const'], b0['requested_model']['const'], b0['returned_identity']['const'], b0['effort']['const'], b0['fallback_status']['const']),
                ('claude-opus-5-5', 'claude-opus-5-5', 'claude-opus-5-5', 'high', 'primary'),
            )
            self.assertEqual(
                (b1['primary_model']['const'], b1['requested_model']['const'], b1['returned_identity']['const'], b1['effort']['const'], b1['fallback_status']['const']),
                ('claude-opus-5-5', 'gemini-4-argon-high', 'gemini-4-argon-high', 'high', 'standing_opus_quota_outage_fallback'),
            )
        root_def = defs['root']
        self.assertIn('nonsecurity_receipt_sha256', root_def['required'])
        self.assertNotIn('muse_receipt_sha256', root_def['required'])
        rr_schema = CONFIG_SCHEMA_JSON['properties']['review_receipts']
        self.assertEqual(rr_schema['required'], ['security', 'nonsecurity', 'root', 'tier1'])
        self.assertNotIn('muse', rr_schema['properties'])


if __name__ == '__main__':
    unittest.main()
