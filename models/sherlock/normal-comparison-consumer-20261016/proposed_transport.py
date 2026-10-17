"""Qualified adopted normal presentation transport. No engine acceptance.
The three mappings use pinned format spellings and exact literal references.
"""
from pathlib import Path
from normal_compare import read, require, keys, differences
from host_transport import HostTransport as AuditedTransport

P = Path(__file__).resolve().parent


class ProposedTransport(AuditedTransport):
    def __init__(self):
        super().__init__()
        self.formats = read(P/'FORMAT-TRANSPORT.json')
        self.enums = read(P/'ENUM-SPELLING-TRANSPORT.json')
        require(self.formats['status'] == 'ROOT_QUALIFIED_PRESENTATION_ADOPTED', '/formatTransport/status', 'unexpected source status')
        require(self.enums['forward'] == self.enums['inverse'], '/enumTransport', 'identity spelling inverse differs')
        self.profile = read(P/'PROFILE-ID-TRANSPORT.json')
        require({str(v): k for k, v in self.profile['forward'].items()} == self.profile['inverse'], '/profileTransport', 'profile mapping is not bijective')
        self.references = []
        self.link_checks = []
        self.family = None
        self.format = None

    def value(self, value, source, target, field=None):
        if field == 'profileId':
            import re
            require(re.fullmatch(r'/(observations/[0-9]+/(admission/identity|image/run_id/identity)|events/[0-9]+/step_id/run_id/identity)/profile_id', target) is not None, source, 'PROFILE-ID-UNSCOPED: no other field mapping')
            require(type(value) is str and value in self.profile['forward'], source, 'PROFILE-ID-UNKNOWN-LABEL')
            return self.atom(value, source, target, 'adopted-profile-label-to-u8; inverse independently compared with wire provenance', self.profile['forward'][value])
        if type(value) is str and field in self.enums['forward']:
            mapping = self.enums['forward'][field]
            require(value in mapping, source, 'GAP-ENUM-SPELLING: unknown pinned field variant')
            return {'$variant': self.atom(value, source, target+'/$variant', 'pinned-unit-enum', mapping[value])}
        if type(value) is dict and value.get('tag') in ('Const', 'Data'):
            keys(value, ('tag', 'id'), source)
            require(type(value['id']) is int and value['id'] >= 0, source+'/id', 'GAP-REF-SPELLING: exact natural integer payload required')
            return {'$variant': self.atom(value['tag'], source+'/tag', target+'/$variant', 'pinned-Ref-discriminator'),
                    'id': self.atom(value['id'], source+'/id', target+'/id')}
        return super().value(value, source, target, field)

    def owner(self, owner, source, target):
        entry = self.format['ownerEntry']
        if entry == 'string' and owner == 'Entry':
            bit = self.atom(owner, source, target+'/owner_is_function', 'pinned-Entry-bit', False)
        elif entry == 'tagged' and type(owner) is dict and owner == {'tag': 'Entry'}:
            bit = self.atom(owner['tag'], source+'/tag', target+'/owner_is_function', 'pinned-Entry-bit', False)
        else:
            form = self.format['ownerFunction']
            require(form is not None, source, 'GAP-FUNCTION-OWNER-SPELLING: no fallback from another format')
            if form == 'keyed':
                keys(owner, ('Function',), source)
                index, index_path = owner['Function'], source+'/Function'
                self.mapping.append({'expectedPath': source, 'actualPath': target+'/owner_is_function', 'operation': 'pinned-Function-variant-key', 'structuralDiscriminator': True, 'sourceKind': 'sole-object-key discriminator, not a boolean payload leaf', 'indexLeafRecordedSeparately': source+'/Function'})
            else:
                keys(owner, ('tag', 'index'), source)
                require(owner['tag'] == 'Function', source+'/tag', 'unknown pinned owner variant')
                self.atom(owner['tag'], source+'/tag', target+'/owner_is_function', 'pinned-Function-bit', True)
                index, index_path = owner['index'], source+'/index'
            require(type(index) is int and index >= 0, index_path, 'natural Function index required; no clamping')
            return {'owner_is_function': True, 'function': self.atom(index, index_path, target+'/function')}
        # Source-fixed inactive COLUMN, not a zero substituted for missing actual data.
        # The complete actual function field must exist and equal this literal.
        self.mapping.append({'expectedPath': source, 'actualPath': target+'/function', 'operation': 'source-fixed-canonical-inactive-column', 'expectedColumn': 0,
                             'authority': self.formats['ownerDispositionSHA256']})
        return {'owner_is_function': bit, 'function': 0}

    def program(self, value, source, target):
        out = {}
        for key, rows in value.items():
            if key != 'occurrences':
                out.update(super().program({key: rows}, source, target))
                continue
            out[key] = []
            if not rows: self.atom(rows, source+'/occurrences', target+'/occurrences', 'empty-array')
            for i, row in enumerate(rows):
                s, t = source+'/occurrences/'+str(i), target+'/occurrences/'+str(i)
                keys(row, ('owner', 'path'), s)
                result = self.owner(row['owner'], s+'/owner', t)
                result['path'] = self.value(row['path'], s+'/path', t+'/path')
                out[key].append(result)
        return out

    def payload(self, event, source, target):
        tag, value = event['outcomeTag'], event['outcomePayload']
        require(type(tag) is str and tag in self.format['literalOutcomeTags'], source, 'GAP-ACTIVE-PAYLOAD-ENVELOPE: outcome tag has no pinned literal spelling for this format')
        out = {'$variant': tag}
        if tag == 'Running':
            spelling = self.format['runningInactive']
            matches = ((spelling == 'integer0' and type(value) is int and value == 0) or
                       (spelling == 'empty-object' and type(value) is dict and len(value) == 0) or
                       (spelling == 'null' and value is None))
            require(matches, source, 'Running inactive literal differs from this exact pinned format')
            self.atom(value, source, target+'/$variant', 'format-indexed-unit-Running', 'Running')
        else:
            field = {'Halted': 'value', 'Rejected': 'code', 'Exhausted': 'reason'}[tag]
            keys(value, (field,), source)
            if tag == 'Rejected':
                require(type(value[field]) is int and 0 <= value[field] <= 255, source+'/code', 'no u8 presentation carrier; not ErrorCode semantic validation')
            if tag == 'Exhausted':
                require(self.format['exhaustReason'] == 'name' and type(value[field]) is str and value[field] in self.format['literalExhaustReasons'], source+'/reason', 'GAP-EXHAUST-REASON-SPELLING: no pinned literal name for this format')
            out[field] = self.value(value[field], source+'/'+field, target+'/'+field, field)
            self.mapping.append({'expectedPath': source, 'actualPath': target+'/$variant', 'operation': 'explicit-separate-outcomeTag-discriminator-reference', 'expectedDiscriminator': tag})
        return out

    def occurrence(self, expected, index):
        anno = expected['eventAnnotations'][index]
        source = '/eventAnnotations/'+str(index)+'/sourceOccurrence'
        if 'sourceOccurrence' in anno:
            require(self.format['sourceReference'] == 'literalAnnotation', source, 'unexpected annotation form for pinned format')
            keys(anno['sourceOccurrence'], ('programSourceDigest', 'owner', 'astPath'), source)
            return anno['sourceOccurrence'], source
        require(self.format['sourceReference'] == 'sourceOfExpectedTables', source, 'GAP-SOURCE-REFERENCE-FORMAT')
        event, program = expected['events'][index], expected['programView']
        e = event['sourceExprId']
        require(type(e) is int and 0 <= e < len(program['occurrences']), '/events/'+str(index)+'/sourceExprId', 'literal occurrence row index unavailable')
        row = program['occurrences'][e]
        keys(row, ('owner', 'path'), '/programView/occurrences/'+str(e))
        keys(row['path'], ('start', 'len'), '/programView/occurrences/'+str(e)+'/path')
        start, length = row['path']['start'], row['path']['len']
        require(type(start) is int and type(length) is int and 0 <= start <= start+length <= len(program['occurrencePaths']) and (length != 0 or start == 0), source, 'exact literal path slice unavailable; no normalization/clamping')
        # Only expected row and path array cells are read. No expression is read.
        value = {'programSourceDigest': expected['identity']['programMembershipDigest'], 'owner': row['owner'],
                 'astPath': program['occurrencePaths'][start:start+length]}
        source = '/declaredSourceOf/events/'+str(index)
        self.references.append({'expectedReference': source, 'rowPath': '/programView/occurrences/'+str(e),
                                'pathArray': '/programView/occurrencePaths', 'slice': {'start': start, 'len': length},
                                'authority': self.format['sourceReferenceAuthority'], 'operation': 'literal-row-and-slice-only'})
        return value, source

    def prepare(self, expected):
        self.family = expected['family']
        require(self.family in self.formats['formats'], '/family', 'GAP FORMAT-UNPINNED')
        self.format = self.formats['formats'][self.family]
        require(expected.get('sourceFormat') == self.format['formatId'], '/sourceFormat', 'FORMAT-UNPINNED: exact literal source /format required, no family-only fallback')
        self.mapping.append({'expectedPath': '/sourceFormat', 'actualPath': None, 'operation': 'exact-adopted-expected-format-domain-selector; not an observed engine field', 'sourceFormat': expected['sourceFormat']})
        for i, event in enumerate(expected['events']):
            for field in ('helperPhaseBefore', 'helperPhaseAfter'):
                phase = event[field]
                require(phase is None or type(phase) is str and phase in self.enums['forward']['phase'],
                        '/events/'+str(i)+'/'+field, 'GAP-ENUM-SPELLING: unknown pinned helper phase')
        tree = super().prepare(expected)
        # Replace only the explicitly implemented three mapping gaps.
        self.gaps = [g for g in self.gaps if not (g['expectedPath'].startswith('/events/') and '/outcomePayload' in g['expectedPath'])
                     and not (g['expectedPath'].startswith('/eventAnnotations/') and '/sourceOccurrence' in g['expectedPath'])]
        for i, (event, row) in enumerate(zip(expected['events'], tree['events'])):
            row['outcome_payload'] = self.payload(event, '/events/'+str(i)+'/outcomePayload', '/events/'+str(i)+'/outcome_payload')
            occurrence, source = self.occurrence(expected, i)
            loc = {'$variant': 'Occurrence', **self.owner(occurrence['owner'], source+'/owner', '/events/'+str(i)+'/source_occurrence')}
            for j, position in enumerate(occurrence['astPath']):
                require(type(position) is int and position >= 0, source+'/astPath/'+str(j), 'exact natural path position required')
            loc['ast_path'] = self.value(occurrence['astPath'], source+'/astPath', '/events/'+str(i)+'/source_occurrence/ast_path')
            row['source_occurrence'] = loc
            digest_path = '/events/'+str(i)+'/step_id/run_id/identity/program_membership_digest'
            require(type(occurrence['programSourceDigest']) is dict and occurrence['programSourceDigest'] == {'tag': 'Unselected'},
                    source+'/programSourceDigest', 'GAP-SOURCE-DIGEST-FORMAT: no pinned Selected spelling; no digest is synthesized')
            digest = self.value(occurrence['programSourceDigest'], source+'/programSourceDigest', digest_path)
            self.references.append({'expectedPath': source+'/programSourceDigest', 'actualPath': digest_path,
                                    'expectedValue': digest, 'operation': 'declared-reference-comparison',
                                    'meaning': 'Unselected binds nothing; no digest synthesized from LocatorC'})
            anno = expected['eventAnnotations'][i]
            for key, target_index in [('preState', i), ('postState', i+1)]:
                if key in anno:
                    self.link_checks.append({'expectedPath': '/eventAnnotations/'+str(i)+'/'+key,
                                             'actualPath': '/observations/'+str(target_index),
                                             'expectedIndex': anno[key], 'actualSequenceIndex': target_index,
                                             'operation': 'ordered-array-position-reference'})
        return tree

    def compare(self, expected, actual):
        report = super().compare(expected, actual)
        from normal_compare import ptr
        for ref in self.references:
            if 'actualPath' not in ref: continue
            try: value = ptr(actual, ref['actualPath'])
            except (KeyError, IndexError, TypeError):
                report['differences'].append({'path': ref['actualPath'], 'kind': 'missing_declared_reference_carrier', 'expectedPath': ref['expectedPath']})
                continue
            for diff in differences(ref['expectedValue'], value, ref['actualPath']):
                diff['expectedReferencePath'] = ref['expectedPath'];report['differences'].append(diff)
        for link in self.link_checks:
            if type(link['expectedIndex']) is not int or link['expectedIndex'] != link['actualSequenceIndex']:
                report['differences'].append({'path': link['actualPath'], 'kind': 'ordered_literal_reference_mismatch', **link})
        # Keep both actual fields; neither payload tag is chosen as authoritative.
        for i, row in enumerate(actual.get('events', []) if type(actual) is dict else []):
            if type(row) is dict and type(row.get('outcome_tag')) is dict and type(row.get('outcome_payload')) is dict:
                if row['outcome_tag'].get('$variant') != row['outcome_payload'].get('$variant'):
                    report['differences'].append({'path': '/events/'+str(i), 'kind': 'ACTUAL-OUTCOME-TAG-PAYLOAD-INCONSISTENT', 'actualOutcomeTag': row['outcome_tag'], 'actualOutcomePayload': row['outcome_payload']})
        report['dataComparison'] = 'MISMATCH' if report['differences'] else 'NO_MAPPED_DIFFERENCES'
        report['sourceStatus'] = 'ROOT_QUALIFIED_PRESENTATION_ADOPTED'
        report['formatAuthoritySHA256'] = self.profile['authoritySHA256']
        report['unresolvedGoverningClauses'] = ['PC-1 producer contract unobserved; C7 fragment uninstalled', 'C9 lexical transport outside scoped profile IDs is not newly implemented or accepted by this successor']
        report['transportGaps'].append({'expectedPath': '/profilePresentation', 'reason': 'PROFILE-ID-UNCOMPARED-RAW-WIRE-PROVENANCE: direct aggregate transport has no singular record/token provenance; JSONL adapter adds independent inverse comparisons'})
        report['status'] = 'GAP'
        report['formatReferenceComparisons'] = self.references
        report['literalLinkComparisons'] = self.link_checks
        report['implementedThreeMaps'] = ['OCCURRENCE-OWNER', 'EVENT-OUTCOME-PAYLOAD', 'EVENT-SOURCE-OCCURRENCE']
        report['engineComparison'] = 'UNOBSERVED_WITHOUT_ROOT_PRODUCER_ORIGIN'
        report['bijectionStatus'] = 'THREE_QUALIFIED_ROOT_ADOPTED_FORMAT_MAPS_IMPLEMENTED; remaining named carrier/domain gaps preserved; no full-envelope or engine acceptance'
        return report
