"""Adopted profile carrier only. Raw captured atoms are never rewritten.
No engine or source evaluation; inverse compares exact labels independently.
"""
from pathlib import Path
import re
from normal_compare import read, ptr, differences

P = Path(__file__).resolve().parent


def compare_profiles(expected, rows, numeric_tokens):
    table = read(P/'PROFILE-ID-TRANSPORT.json')
    checks, issues = [], []
    for i, row in enumerate(rows):
        scopes = [('/observation/admission/identity/profile_id',
                   '/observations/'+str(i)+'/admission/identity/profile_id', '/identity/profileId'),
                  ('/observation/image/run_id/identity/profile_id',
                   '/observations/'+str(i)+'/image/run_id/identity/profile_id', '/events/0/stepId/runId/identity/profileId')]
        if row['kind'] == 'event':
            scopes.append(('/event/step_id/run_id/identity/profile_id',
                           '/events/'+str(i-1)+'/step_id/run_id/identity/profile_id',
                           '/events/'+str(i-1)+'/stepId/runId/identity/profileId'))
        for wire, normalized, source in scopes:
            check = {'recordIndex': i, 'recordKind': row['kind'], 'actualWirePath': wire,
                     'actualRecordPath': '/records/'+str(i)+wire, 'normalizedConsumerPath': normalized,
                     'expectedPath': source, 'authoritySHA256': table['authoritySHA256']}
            try:
                value = ptr(row, wire)
            except (KeyError, IndexError, TypeError):
                try:
                    enclosing = ptr(row, wire.rsplit('/', 1)[0])
                except (KeyError, IndexError, TypeError):
                    enclosing = None
                check['status'] = ('PROFILE-ID-MISSING' if type(enclosing) is dict and 'profile_id' not in enclosing
                                   else 'PROFILE-ID-UNCOMPARED-ABSENT')
                checks.append(check)
                issues.append({'path': normalized, 'kind': check['status'], 'actualRecordPath': check['actualRecordPath']})
                continue
            check['rawValue'] = value
            token = numeric_tokens[i].get(wire)
            if token is not None:
                check['rawNumericToken'] = token['token']
                check['rawNumericKind'] = token['kind']
            try:
                label = ptr(expected, source)
            except (KeyError, IndexError, TypeError):
                check['status'] = 'PROFILE-ID-UNCOMPARED-EXPECTED-ABSENT'
            else:
                check['expectedLabel'] = label
                if type(label) is not str or label not in table['forward']:
                    check['status'] = 'PROFILE-ID-UNKNOWN-LABEL'
                elif type(value) is not int or token is None or token['kind'] != 'integer':
                    check['status'] = 'PROFILE-ID-INVALID-TOKEN'
                elif re.fullmatch(r'(0|[1-9][0-9]*)', token['token']) is None:
                    check['status'] = 'LEXICAL-NONCANONICAL-INTEGER'
                elif value > 255:
                    check['status'] = 'PROFILE-ID-NO-U8-CARRIER'
                elif str(value) not in table['inverse']:
                    check['status'] = 'PROFILE-ID-UNKNOWN-CODE'
                else:
                    inverse = table['inverse'][str(value)]
                    check['inverseLabel'] = inverse
                    label_issues = differences(label, inverse, normalized)
                    # Retain exact label differences in provenance; emit one status issue below.
                    check['labelDifferences'] = label_issues
                    if value != table['currentBCode']:
                        check['status'] = 'PROFILE-ID-FOREIGN-CURRENT-B-MISMATCH'
                    elif label_issues:
                        check['status'] = 'PROFILE-ID-LABEL-MISMATCH'
                    else:
                        check['status'] = 'ADOPTED-CURRENT-B-PROFILE-LABEL-MATCH'
            if check['status'] != 'ADOPTED-CURRENT-B-PROFILE-LABEL-MATCH':
                issues.append({'path': normalized, 'kind': check['status'],
                               'actualRecordPath': check['actualRecordPath'],
                               'rawValue': value, 'rawNumericToken': check.get('rawNumericToken'),
                               'expectedLabel': check.get('expectedLabel'), 'inverseLabel': check.get('inverseLabel')})
            checks.append(check)
    return checks, issues
