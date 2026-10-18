"""Additive JSON graph curation API. No product imports or canonical writes."""
import copy
import json


def relation_key(edge):
    return (edge['source'], edge['target'], edge.get('relation', edge.get('type')))


def append_delta(base, delta, edge_field):
    """Return a deep copy; preserve all old values/order; exact replay is a no-op.

    Conflicting node IDs or incoming relation triples abort. Existing parallel
    relation membership, including duplicate historical triples, is preserved.
    """
    result = copy.deepcopy(base)
    old_nodes = copy.deepcopy(base['nodes'])
    old_edges = copy.deepcopy(base[edge_field])
    by_id = {n['id']: n for n in old_nodes}
    assert len(by_id) == len(old_nodes), 'duplicate base node ID'
    edge_keys = {}
    for edge in old_edges:
        edge_keys.setdefault(relation_key(edge), []).append(edge)
    added_nodes = added_edges = 0
    for node in delta['nodes']:
        if node['id'] in by_id:
            assert node == by_id[node['id']], ('node collision', node['id'])
        else:
            result['nodes'].append(copy.deepcopy(node))
            by_id[node['id']] = node
            added_nodes += 1
    for edge in delta[edge_field]:
        assert edge['source'] in by_id and edge['target'] in by_id, ('dangling incoming edge', edge)
        key = relation_key(edge)
        if key in edge_keys:
            assert edge in edge_keys[key], ('relation collision', key)
        else:
            result[edge_field].append(copy.deepcopy(edge))
            edge_keys[key] = [edge]
            added_edges += 1
    assert result['nodes'][:len(old_nodes)] == old_nodes
    assert result[edge_field][:len(old_edges)] == old_edges
    for key in base:
        if key not in ['nodes', edge_field]:
            assert result[key] == base[key], ('metadata changed', key)
    return result, {'nodesAdded': added_nodes, 'relationsAdded': added_edges,
                    'nodesBefore': len(old_nodes), 'relationsBefore': len(old_edges),
                    'nodesAfter': len(result['nodes']), 'relationsAfter': len(result[edge_field]),
                    'oldNodesAndRelationsExactOrderedPrefix': True,
                    'oldMetadataUnchanged': True}


def predicate_holds(raw, predicate):
    op = predicate['operation']
    if op == 'text_contains':
        return predicate['expected'] in raw.decode()
    if op == 'text_contains_all':
        return all(s in raw.decode() for s in predicate['expected'])
    if op == 'text_count_equals':
        return raw.decode().count(predicate['text']) == predicate['expected']
    if op == 'json_pointer_equals':
        value = json.loads(raw)
        for part in predicate['jsonPointer'].lstrip('/').split('/'):
            part = part.replace('~1', '/').replace('~0', '~')
            value = value[int(part)] if isinstance(value, list) else value[part]
        return value == predicate['expected']
    raise AssertionError(('unsupported predicate', op))
