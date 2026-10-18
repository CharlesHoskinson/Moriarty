"""Lossless generic JSON DAG transport. No model, codec, subprocess or golden access."""
import json

def factor(documents):
    nodes, indices = [], {}
    def visit(value):
        if type(value) is dict:
            node = ['object', [[key, visit(child)] for key, child in value.items()]]
        elif type(value) is list:
            node = ['list', [visit(child) for child in value]]
        else:
            kind = {type(None): 'null', bool: 'bool', int: 'int', float: 'float', str: 'string'}.get(type(value))
            if kind is None:
                raise TypeError(type(value).__name__)
            node = ['scalar', kind, value]
        key = json.dumps(node, ensure_ascii=False, separators=(',', ':'), allow_nan=False)
        if key not in indices:
            indices[key] = len(nodes)
            nodes.append(node)
        return indices[key]
    roots = {name: visit(value) for name, value in documents.items()}
    return {'format': 'lossless-typed-json-dag/1', 'nodes': nodes, 'roots': roots}

def expand(payload):
    if payload['format'] != 'lossless-typed-json-dag/1':
        raise ValueError('format')
    values = []
    for node in payload['nodes']:
        if node[0] == 'object':
            pairs = node[1]
            if len({key for key, _ in pairs}) != len(pairs):
                raise ValueError('duplicate object key')
            values.append({key: values[ref] for key, ref in pairs})
        elif node[0] == 'list':
            values.append([values[ref] for ref in node[1]])
        elif node[0] == 'scalar':
            kind = {type(None): 'null', bool: 'bool', int: 'int', float: 'float', str: 'string'}.get(type(node[2]))
            if node[1] != kind:
                raise ValueError('scalar type')
            values.append(node[2])
        else:
            raise ValueError('tag')
    return {name: values[ref] for name, ref in payload['roots'].items()}

def exact_json(left, right):
    if type(left) is not type(right):
        return False
    if isinstance(left, dict):
        return list(left) == list(right) and all(exact_json(left[key], right[key]) for key in left)
    if isinstance(left, list):
        return len(left) == len(right) and all(exact_json(a, b) for a, b in zip(left, right))
    return left == right
