"""Build local Graphify views from frozen extracted data. No extraction or network."""
from pathlib import Path
import collections
import copy
import hashlib
import html
import importlib.metadata
import json
import re

from graphify.build import build_from_json
from graphify.cluster import cluster, score_all
from graphify.analyze import god_nodes, surprising_connections, suggest_questions
from graphify.report import generate
from graphify.export import to_json
from graphify.exporters.html import to_html
from graphify.diagnostics import diagnose_extraction

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'graph'


def read(p):
    return json.loads(p.read_text())


def write(p, value):
    p.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n')


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


class Unknown:
    def __format__(self, spec):
        return 'UNKNOWN'


def main():
    assert importlib.metadata.version('graphifyy') == '0.9.91'
    parent_path = OUT / 'frozen/parent/graph-v12.json'
    assert sha(parent_path) == '68b8af13690e30642e88aadde1743280b70abfc0dbddf4d14bf1fa4f2895beff'
    parent = read(parent_path)
    delta_path = OUT / 'frozen/revision-3/semantic-v13-luna.json'
    assert sha(delta_path) == '56f6af3caeeed1d02298fe434a248dbe40e64c2723a0c9342d73f786292b1b5e'
    delta = read(delta_path)
    assert len(delta['nodes']) == 76 and len(delta['edges']) == 39
    # R3's legacy links alias contains only 11 entries and one stale citation.
    # Preserve that raw file, but integrate only the complete corrected edges.
    alias_by_id = {e['id']: e for e in delta.get('links', [])}
    alias_differences = [dict(id=e['id'], corrected=e['source_location'],
                              legacy=alias_by_id[e['id']].get('source_location'))
                         for e in delta['edges'] if e['id'] in alias_by_id and e != alias_by_id[e['id']]]
    parent_ids = {n['id'] for n in parent['nodes']}
    new_ids = [n['id'] for n in delta['nodes']]
    assert len(new_ids) == len(set(new_ids)) and not parent_ids.intersection(new_ids)
    raw = copy.deepcopy(parent)
    raw['nodes'] += copy.deepcopy(delta['nodes'])
    raw['links'] += copy.deepcopy(delta['edges'])
    raw['hyperedges'] += copy.deepcopy(delta['hyperedges'])
    all_ids = {n['id'] for n in raw['nodes']}
    assert all(e['source'] in all_ids and e['target'] in all_ids for e in raw['links'])
    assert all(n in all_ids for h in raw['hyperedges'] for n in h.get('nodes', []))
    source_map = {r['source_file']: r for r in read(OUT / 'SOURCE-PATH-MAP.json')['files']}
    bindings = lines = pointers = 0
    citation_rows = []
    for item in delta['nodes'] + delta['edges']:
        row = source_map[item['source_file']]
        body = ROOT / row['portable']
        assert sha(body) == row['sha256'] == item['source_sha256']
        bindings += 1
        for start, end in re.findall(r'L(\d+)(?:-L?(\d+))?', item['source_location']):
            assert 1 <= int(start) <= int(end or start) <= len(body.read_text().splitlines())
            lines += 1
        for pointer in re.findall(r'\$(/[^;\s]+)', item['source_location']):
            value = read(body)
            for part in pointer.split('/')[1:]:
                part = part.replace('~1', '/').replace('~0', '~')
                value = value[int(part)] if isinstance(value, list) else value[part]
            pointers += 1
        citation_rows.append(dict(id=item['id'], kind='edge' if 'source' in item else 'node',
                                  source_file=item['source_file'], portable=row['portable'],
                                  source_sha256=item['source_sha256'], source_location=item['source_location']))
    write(OUT / 'CITATION-INDEX.json', citation_rows)

    # This archival file preserves list order and every payload byte value.
    write(OUT / 'graph-raw.json', raw)
    extraction = dict(nodes=copy.deepcopy(raw['nodes']), edges=copy.deepcopy(raw['links']),
                      hyperedges=copy.deepcopy(raw['hyperedges']))
    diagnostics = diagnose_extraction(copy.deepcopy(extraction), directed=False, root=str(ROOT))
    diagnostics['rawSchemaFields'] = dict(
        nodesWithoutSourceFile=sum(not n.get('source_file') for n in raw['nodes']),
        edgesWithoutConfidence=sum('confidence' not in e for e in raw['links']),
        qualification='Graphify emits missing-field warnings. The archival graph keeps the original records unchanged.')
    write(OUT / 'GRAPH-HEALTH.json', diagnostics)
    G = build_from_json(copy.deepcopy(extraction), root=str(ROOT), directed=False)
    removed_analysis_ids = sorted(all_ids - set(G.nodes))
    assert not set(G.nodes) - all_ids, 'Unexpected Graphify-generated IDs'
    assert set(new_ids).issubset(G.nodes), 'New delta nodes cannot disappear'
    write(OUT / 'ANALYSIS-NORMALIZATION.json', dict(
        rawNodes=len(all_ids), analysisNodes=len(G), removedHistoricalIds=removed_analysis_ids,
        addedIds=[], qualification='Graphify legacy alias normalization affects only the derived analysis view. The archival graph retains every node and raw link.'))
    communities = cluster(G)
    cohesion = score_all(G, communities)
    assert set(n for members in communities.values() for n in members) == set(G.nodes)
    # These labels describe the highest-degree member, without new semantic edges.
    labels = {}
    for cid, members in communities.items():
        anchor = sorted(members, key=lambda n: (-G.degree(n), n))[0]
        words = str(G.nodes[anchor].get('label', anchor)).split()
        labels[cid] = ' '.join(words[:5])
    gods = god_nodes(G)
    surprises = surprising_connections(G, communities)
    questions = suggest_questions(G, communities, labels)
    analysis = dict(communities=communities, cohesion=cohesion, labels=labels,
                    god_nodes=gods, surprising_connections=surprises, suggested_questions=questions)
    write(OUT / 'ANALYSIS.json', analysis)
    assert to_json(G, communities, str(OUT / 'graph-analysis.json'), community_labels=labels,
                   built_at_commit='3602c4c0e46a1b969d4e74fa331114f9f1b8c770')
    node_community = {n: cid for cid, members in communities.items() for n in members}
    changes = []
    for node in raw['nodes']:
        if node['id'] not in node_community:
            continue
        cid = node_community[node['id']]
        before = {k: node[k] for k in ('community', 'community_name') if k in node}
        after = dict(community=cid, community_name=labels[cid])
        if before != after:
            changes.append(dict(id=node['id'], before=before, after=after))
    # Archival graph.json retains original community metadata. Derived assignments
    # belong only to graph-analysis.json, ANALYSIS.json and COMMUNITY-CHANGES.json.
    write(OUT / 'graph.json', raw)
    write(OUT / 'COMMUNITY-CHANGES.json', changes)
    detection = dict(total_files=40, total_words=sum(len((ROOT / r['portable']).read_text().split()) for r in source_map.values()),
                     files={}, qualification='Frozen V13 cited-source corpus only. Parent corpus was not detected or extracted.')
    report = generate(G, communities, cohesion, labels, gods, surprises, detection,
                      dict(input=Unknown(), output=Unknown()), 'frozen-parent-plus-Luna-V13',
                      suggested_questions=questions, built_at_commit='3602c4c0e46a1b969d4e74fa331114f9f1b8c770')
    prefix = ('# Publication qualification\n\n'
              'This report describes a derived undirected analysis view. All semantic claims remain source-attributed.\n'
              'The map supplies no security, proof, deployment or ledger acceptance.\n\n'
              f'Raw graph: {len(raw["nodes"])} nodes, {len(raw["links"])} links, {len(raw["hyperedges"])} hyperedge.\n'
              f'Analysis view: {G.number_of_nodes()} nodes, {G.number_of_edges()} edges, {len(communities)} communities.\n'
              'Graphify can collapse parallel edges and omit import self-loops. Raw records retain all historical facts.\n'
              f'Graphify also merged {len(removed_analysis_ids)} historical alias nodes. ANALYSIS-NORMALIZATION.json lists their IDs.\n'
              'The archival graph retains parent community metadata. Community changes apply only to the analysis view.\n'
              f'Raw schema warnings: {diagnostics["rawSchemaFields"]["nodesWithoutSourceFile"]} nodes lack source_file and '
              f'{diagnostics["rawSchemaFields"]["edgesWithoutConfidence"]} edges lack confidence. Original records remain unchanged.\n'
              'The parent corpus exceeds 500 files and 5,000 nodes. No parent source detection or extraction occurred.\n'
              'The HTML uses the bounded community view. Its vis-network dependency uses a CDN.\n'
              'Requested semantic author: Luna/high. Actual provider, engine, effort, tokens and cost: UNKNOWN.\n'
              'No new extraction or model call occurred. Historical usage remains UNKNOWN.\n'
              'Luna FULL read labels are author claims. JSON pointer resolution does not verify semantic support.\n'
              'Revision 1 partial reads and revision 2 invalid citations remain historical failures. Only revision 3 is integrated.\n'
              'The frozen parent publication is 52f5584bd71b57be8b6b23d5dd98d454022a7cb6.\n\n')
    (OUT / 'GRAPH_REPORT.md').write_text(prefix + report)
    assert to_html(G, communities, str(OUT / 'graph.html'), community_labels=labels, node_limit=5000)
    # A separate local index makes every new source citation usable after clone.
    rows = []
    for item in delta['nodes'] + delta['edges']:
        row = source_map[item['source_file']]
        href = str(Path(row['portable']).relative_to('graph'))
        label = item.get('label', item.get('relation', item['id']))
        rows.append('<tr><td>' + html.escape(item['id']) + '</td><td>' + html.escape(label) +
                    '</td><td><a href="' + html.escape(href, quote=True) + '">' +
                    html.escape(Path(href).name) + '</a></td><td>' + html.escape(item['source_location']) + '</td></tr>')
    (OUT / 'SOURCE-INDEX.html').write_text('<!doctype html><html lang="en"><meta charset="utf-8">'
        '<title>V13 source citations</title><h1>V13 source citations</h1>'
        '<p>This source index gives locations and attribution. It gives no acceptance or certification.</p>'
        '<p><a href="graph.html">Community graph</a> | <a href="SOURCE-PATH-MAP.json">Original path map</a></p>'
        '<table><thead><tr><th>ID</th><th>Label</th><th>Copied source</th><th>Original location</th></tr></thead><tbody>' +
        '\n'.join(rows) + '</tbody></table></html>')
    pairs = collections.Counter(tuple(sorted((e['source'], e['target']))) for e in raw['links'])
    exact = collections.Counter(json.dumps(e, sort_keys=True) for e in raw['links'])
    receipt = dict(format='graphify-local-publication-build/1', graphifyVersion='0.9.91',
        parent=dict(nodes=len(parent['nodes']), rawLinks=len(parent['links']), hyperedges=len(parent['hyperedges']),
                    sha256=sha(parent_path), originalCommunities=len(set(n.get('community') for n in parent['nodes']))),
        delta=dict(nodes=76, edges=39, revision=3, sha256=sha(delta_path)),
        legacyLinksAlias=dict(count=len(delta.get('links', [])), integrated=False, differences=alias_differences),
        predicted=dict(nodes=6177, rawLinks=16856, hyperedges=1),
        measured=dict(nodes=len(raw['nodes']), rawLinks=len(raw['links']), hyperedges=len(raw['hyperedges']),
            analysisNodes=G.number_of_nodes(), analysisEdges=G.number_of_edges(), communities=len(communities),
            duplicateRawObjects=sum(n-1 for n in exact.values()), repeatedUndirectedPairs=sum(n-1 for n in pairs.values()),
            rawSelfLoops=sum(e['source']==e['target'] for e in raw['links']),
            sourceBindings=bindings, lineRanges=lines, jsonPointers=pointers, communityChanges=len(changes)),
        analysisNormalization=dict(removedHistoricalNodes=len(removed_analysis_ids), newDeltaNodesRetained=len(new_ids)),
        pipeline=['build_from_json', 'cluster', 'score_all', 'god_nodes', 'surprising_connections', 'suggest_questions', 'generate', 'to_json', 'to_html'],
        semanticUsage=dict(requestedModel='gpt-6-luna', requestedEffort='high', actualModel='UNKNOWN',
                           actualProvider='UNKNOWN', actualEffort='UNKNOWN', tokens='UNKNOWN', cost='UNKNOWN'),
        extraction='NOT_INVOKED', network='NOT_REQUESTED', installation='NOT_INVOKED',
        authorityCreated='NONE', securityReview='NOT_PERFORMED_BY_PUBLICATION_AUTHOR',
        sourceSemanticSupport='PENDING_FRESH_NONAUTHOR_AUDIT',
        historicalParentSources='ARCHIVAL_ONLY_NOT_RESOLVED_OR_REREAD',
        htmlRuntime='UNRUN_IN_BROWSER_CDN_DEPENDENCY_NOT_BUNDLED')
    write(OUT / 'BUILD-RECEIPT.json', receipt)
    print(json.dumps(receipt['measured']))


if __name__ == '__main__':
    main()
