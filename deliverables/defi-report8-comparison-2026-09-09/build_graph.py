#!/usr/bin/env python3
"""Build the report/source/Moriarty graph, preserving typed parallel evidence."""
import collections
import json
import re
from pathlib import Path

import networkx as nx
from graphify.cluster import cluster, score_all
from graphify.analyze import god_nodes, surprising_connections
from graphify.export import to_html, to_json

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
PREFIX = 'deliverables_defi_report8_comparison_2026_09_09_'


def read(path):
    return json.loads(path.read_text())


def main():
    datasets = [
        read(ROOT/'deliverables/modern-defi-taxonomy-2026-09-08/relationships.json'),
        read(ROOT/'deliverables/defi-taxonomy-papers-2026-09-08/extraction.json'),
        read(HERE/'report-extraction.json'),
    ]
    nodes, edges = {}, []
    for data in datasets:
        for n in data['nodes']:
            nodes.setdefault(n['id'], n)
        edges.extend(data['edges'])
    hyperedges = [h for d in datasets for h in d.get('hyperedges', [])]

    def node(key, label, source, kind='document', **fields):
        nodes[key] = {'id': key, 'label': label, 'file_type': kind,
                      'source_file': source, **fields}

    def edge(a, b, relation, source, confidence='EXTRACTED', **fields):
        edges.append({'source': a, 'target': b, 'relation': relation,
                      'source_file': source, 'confidence': confidence,
                      'confidence_score': 1.0 if confidence=='EXTRACTED' else 0.85,
                      'weight': 1.0, **fields})

    report = '.raw/captured/02e57f7b7616b730a7be59cb7cdc63e3753b007c2d0806393d63a35a9a6c1045.md'
    report_id = PREFIX+'report8'
    node(report_id, 'Report 8: financial taxonomy and standards atlas', report)
    manifest = read(HERE/'sources/standards/manifest.json')
    standards = {r['number']: r for r in manifest['records']}
    for num, row in standards.items():
        key = PREFIX+'standard_'+str(num)
        src = str((HERE/'sources/standards'/row['text']).relative_to(ROOT))
        node(key, row['title'], src, standard_number=num, status=row['formal_status'],
             source_url=row['canonical_url'], captured_at=row['retrieved_at'],
             source_location='title, metadata and specification', scope='current captured specification')
        edge(report_id, key, 'names_standard_identity', report,
             qualification='Identity only: opaque report citation does not pin this captured revision.')
        old = [n['id'] for n in datasets[0]['nodes']
               if n.get('standard_id') in ('ERC-'+str(num), 'EIP-'+str(num))]
        for prior in old:
            edge(key, prior, 'same_standard_as_prior_capture', src,
                 qualification='Same identifier, not an assertion of byte or behavioral equivalence.')
    stubs = set()
    for num, row in standards.items():
        for required in row['requires']:
            target = PREFIX+'standard_'+str(required)
            if target not in nodes:
                stubs.add(required)
                node(target, f'Standard {required} (dependency stub)',
                     str((HERE/'sources/standards'/row['text']).relative_to(ROOT)),
                     inspection_status='dependency-only; not captured in this intake')
            edge(PREFIX+'standard_'+str(num), target, 'normatively_requires',
                 str((HERE/'sources/standards'/row['text']).relative_to(ROOT)),
                 source_location='Requires metadata')
    for n in datasets[2]['nodes']:
        edge(report_id, n['id'], 'contains_extracted_concept', report)
        for num in set(map(int, re.findall(r'(?:ERC|EIP)[- ](\d+)', n['label']))):
            if num in standards:
                edge(n['id'], PREFIX+'standard_'+str(num), 'same_standard_identity', report,
                     qualification='Report description is not verification of current conformance.')
    for n in datasets[1]['nodes']:
        if n.get('source_id') in ('SRC-0100','SRC-0101','SRC-0102','SRC-0103') and n['file_type']=='paper':
            edge(report_id, n['id'], 'uses_paper_foundation', report)
    for i in range(1,13):
        sprint = f'SP{i:02}'
        node(PREFIX+sprint.lower(), sprint+' Moriarty roadmap', 'ROADMAP.md',
             status='full sprint open; scoped foundations may be complete')
    relevance = read(HERE/'relevance.json')
    for row in relevance['records']:
        for sprint in sorted(set(re.findall(r'SP\d{2}', json.dumps(row.get('owning_tasks',row.get('sprints',[])))))):
            edge(PREFIX+'standard_'+str(row['number']), PREFIX+sprint.lower(),
                 'informs_existing_task', str((HERE/'relevance.json').relative_to(ROOT)),
                 'INFERRED', qualification='Research relevance; no implementation or acceptance claim.')
    # Relativize every source to the shared repo root without changing historical IDs.
    for item in [*nodes.values(), *edges, *hyperedges]:
        p = Path(item.get('source_file') or '.')
        if p.is_absolute() and p.is_relative_to(ROOT):
            item['source_file'] = str(p.relative_to(ROOT))
    dangling = [e for e in edges if e['source'] not in nodes or e['target'] not in nodes]
    if dangling:
        raise SystemExit(f'Dangling relationships: {len(dangling)}')
    dataset = {'schema': 'moriarty.report8-linked-graph.v1', 'nodes': list(nodes.values()),
               'edges': edges, 'hyperedges': hyperedges, 'token_usage': None,
               'token_usage_note': 'Provider telemetry unavailable; not measured as zero.',
               'scope': 'Report extraction, all40 named standards, four-paper graph, prior atlas and task relevance.'}
    (HERE/'relationships.json').write_text(json.dumps(dataset,indent=2)+'\n')
    graph = nx.DiGraph()
    for key, n in nodes.items():
        graph.add_node(key, **n)
    grouped = collections.defaultdict(list)
    for e in edges:
        grouped[e['source'],e['target']].append(e)
    for (a,b), group in grouped.items():
        rels = sorted(set(e['relation'] for e in group))
        graph.add_edge(a,b,relation='; '.join(rels),relations=rels,evidence=group,
                       weight=len(group),confidence='EXTRACTED' if all(e['confidence']=='EXTRACTED' for e in group) else 'INFERRED')
    communities = cluster(graph)
    scores = score_all(graph,communities)
    labels = {}
    for cid, members in communities.items():
        ranks=sorted(members,key=lambda n: graph.degree(n),reverse=True)
        labels[cid] = nodes[ranks[0]]['label'][:65]
    assert to_json(graph,communities,str(HERE/'graph.json'))
    assert to_html(graph,communities,str(HERE/'graph.html'),community_labels=labels)
    stats = {'nodes':len(nodes),'typed_relationships':len(edges),'directed_pairs':len(grouped),
             'communities':len(communities),'dangling_edges':len(dangling),
             'parallel_relationships_retained_in_evidence':len(edges)-len(grouped),
             'dependency_stubs':sorted(stubs),'captured_named_standards':len(standards),
             'papers':4,'paper_pages':80,'cohesion':scores,'token_usage':None,
             'god_nodes':god_nodes(graph),'surprising_connections':surprising_connections(graph,communities)}
    (HERE/'graph-stats.json').write_text(json.dumps(stats,indent=2)+'\n')
    lines=['# Linked report, standards, papers and Moriarty graph','',
           f'{len(nodes)} nodes; {len(edges)} typed relationships; {len(grouped)} directed pairs; {len(communities)} communities.','',
           'All40 report-named ERC/EIP pages are captured. Four exact prior PDF captures (80pages) and their existing analyses are reused. Prior atlas nodes remain version-scoped. Dependency-only stubs are explicitly uninspected.','',
           'Parallel relationship types are retained in relationships.json and each graph edge evidence array; the navigation projection groups only endpoint pairs. Zero dangling endpoints. Extraction confidence is not protocol conformance. Token use is unmeasured.','',
           '## Useful connections','',
           '- Vault accounting → asynchronous entitlement → controller authority → SP08/SP10.',
           '- Series fees → non-fungible cohort accounting → bounded consolidation → SP03/SP11.',
           '- Interface mimicry in the attacks paper → token callback behavior → negative integration controls.',
           '- Prior FIN categories ↔ report FF categories: retain capital-raising versus treasury-management boundary.','',
           '## Questions to use this graph for','',
           '- Which normative interfaces constrain request rights without guaranteeing liquidity?',
           '- Which ERC lessons are portable to Midnight, and which remain EVM-specific?',
           '- Which claims are report assertions, captured normative rules, or proposed Moriarty requirements?','',
           '## Community cohesion (raw values)','',json.dumps(scores,indent=2)]
    lines += ['', '## High-degree nodes', '']
    lines += ['- '+item['label']+' — degree '+str(item['degree']) for item in stats['god_nodes'][:8]]
    lines += ['', 'High degree reflects corpus structure, not financial safety or implementation priority.']
    (HERE/'GRAPH_REPORT.md').write_text('\n'.join(lines)+'\n')
    print(json.dumps({k:stats[k] for k in ('nodes','typed_relationships','directed_pairs','communities','dangling_edges')}))


if __name__=='__main__':
    main()
