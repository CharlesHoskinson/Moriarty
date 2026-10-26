"""Network latency and local cryptographic work must not become validator timing."""
import importlib.util
from pathlib import Path
import unittest

SPEC = importlib.util.spec_from_file_location('performance', Path(__file__).parents[1] / 'check-rollup-performance.py')
performance = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(performance)


class PerformanceScopeTests(unittest.TestCase):
    def receipt(self):
        return {'network': 'stagenet', 'status': 'SucceedEntirely', 'completeEffectsMatch': True,
                'fees': {'paidFees': '100', 'estimatedFees': '120'}, 'provenFeesSpecks': '99',
                'txId': 'public-id', 'txHash': 'public-hash', 'blockHeight': 1,
                'localCryptographicVerification': 0.004,
                'timing': {'submissionAckSeconds': 0.1, 'finalizationObservedSeconds': 2.0,
                           'readbackSeconds': 0.3, 'submissionToReadbackSeconds': 2.4,
                           'validatorCryptographicVerificationSeconds': None,
                           'validatorTimingEvidence': 'unavailable'}}

    def test_separate_observed_scopes_and_unavailable_validator_time(self):
        result = performance.metrics(self.receipt(), 'acceptValidity')
        self.assertIsNone(result['validator_cryptographic_verify_seconds'])
        self.assertEqual(result['local_cryptographic_verify_seconds'], 0.004)
        self.assertEqual(result['timing']['finalizationObservedSeconds'], 2.0)

    def test_inferred_validator_time_and_invalid_fee_are_refused(self):
        receipt = self.receipt()
        receipt['timing']['validatorCryptographicVerificationSeconds'] = 0.004
        with self.assertRaisesRegex(ValueError, 'cannot be inferred'):
            performance.metrics(receipt, 'acceptValidity')
        receipt = self.receipt(); receipt['fees']['paidFees'] = 'NaN'
        with self.assertRaisesRegex(ValueError, 'actual indexer fee'):
            performance.metrics(receipt, 'acceptValidity')
