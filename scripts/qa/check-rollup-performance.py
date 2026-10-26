#!/usr/bin/env python3
"""B8: preserve actual fee/finalization/readback scopes and never infer validator time."""
import argparse
from decimal import Decimal
import json
import math
from pathlib import Path


def metrics(receipt, entry):
    if receipt['network'] != 'stagenet' or receipt['status'] != 'SucceedEntirely' or not receipt['completeEffectsMatch']:
        raise ValueError('requires a finalized complete stagenet effect')
    fees = receipt['fees']
    for name in ('paidFees', 'estimatedFees'):
        value = Decimal(fees[name])
        if not value.is_finite() or value < 0:
            raise ValueError('invalid actual indexer fee')
    timing = receipt['timing']
    for name in ('submissionAckSeconds', 'finalizationObservedSeconds', 'readbackSeconds', 'submissionToReadbackSeconds'):
        if not math.isfinite(timing[name]) or timing[name] <= 0:
            raise ValueError('missing actual host timing: ' + name)
    if not timing['submissionAckSeconds'] <= timing['finalizationObservedSeconds'] <= timing['submissionToReadbackSeconds']:
        raise ValueError('inconsistent submission/finalization/readback chronology')
    if timing['validatorCryptographicVerificationSeconds'] is not None or timing['validatorTimingEvidence'] != 'unavailable':
        raise ValueError('validator timing cannot be inferred from local verification or inclusion')
    local = receipt.get('localCryptographicVerification')
    if entry == 'acceptValidity' and (local is None or not math.isfinite(local) or local <= 0):
        raise ValueError('missing isolated local stock D verification time')
    proven = receipt.get('provenFeesSpecks')
    if entry != 'deploy' and (not isinstance(proven, str) or not proven.isdigit()):
        raise ValueError('missing actual proven transaction cost in SPECKs')
    return {'tx_id': receipt['txId'], 'tx_hash': receipt['txHash'], 'block_height': receipt['blockHeight'],
            'fees_indexer_raw': fees, 'fees_unit_scope': 'SDK/indexer values retained verbatim without conversion',
            'proven_transaction_cost_specks': proven, 'timing': timing,
            'local_cryptographic_verify_seconds': local,
            'local_verify_scope': 'isolated host stock D verify for acceptance; unavailable for deployment/compiled settlement',
            'validator_cryptographic_verify_seconds': None}


def check(target=Path('target')):
    candidates = list((target / 'sherlock-b4/checks').glob('run-*/B4.json'))
    if not candidates:
        raise ValueError('no independently checked B4 stagenet result')
    source = max(candidates, key=lambda p: p.stat().st_mtime_ns)
    result = json.loads(source.read_bytes())
    if result['schema'] != 'sherlock-b4-results/1':
        raise ValueError('wrong B4 result schema')
    attempts = list((target / 'sherlock-b4').glob('stagenet-*/attempt.json'))
    latest = max(attempts, key=lambda p: json.loads(p.read_bytes())['started_at'])
    if latest.parent.resolve() != Path(result['run']).resolve():
        raise ValueError('B4 result predates the newest stagenet attempt')
    readback = result['readback']
    if not readback['completeEffectsMatch'] or not readback['nativeProofsVerified']:
        raise ValueError('incomplete stagenet proof/effect evidence')
    return {'schema': 'sherlock-b8-results/1', 'predicate': 'B8', 'b4_source': str(source.resolve()),
            'address': readback['address'], 'network': 'stagenet',
            'transactions': {name: metrics(readback['receipts'][name], name)
                             for name in ('deploy', 'acceptValidity', 'settleValidity')}}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.parse_args()
    try:
        result = check()
    except (OSError, ValueError, KeyError, ArithmeticError) as error:
        parser.exit(1, f'B8 FAIL: {error}\n')
    destination = Path(result['b4_source']).parent / 'B8.json'
    destination.write_text(json.dumps(result, indent=2, sort_keys=True) + '\n')
    print('PASS B8: actual fees, submission/finalization/readback and local verification in separate scopes; validator time unavailable')


if __name__ == '__main__':
    main()
