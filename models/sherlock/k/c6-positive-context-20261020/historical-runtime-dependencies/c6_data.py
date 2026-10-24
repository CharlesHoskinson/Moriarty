"""Data-only renderer/whole-tree consumer; no K transition or oracle derivation."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import sys
sys.path.insert(0, str(Path(__file__).resolve().parent))
import nominal_trace_codec as codec

ROOT = Path(__file__).resolve().parents[1]


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result: raise ValueError('duplicate JSON key: ' + key)
        result[key] = value
    return result


def loads(text):
    return json.loads(text, object_pairs_hook=unique_object,
                      parse_constant=lambda x: (_ for _ in ()).throw(ValueError(x)))


def load(path): return loads(Path(path).read_bytes())


def configure():
    # Existing parser's concrete grammar registry; no evaluator imports.
    syntax = (ROOT / 'source/nominal-product-syntax.k').read_text() + '\n' + (ROOT / 'source/c6-logical-syntax.k').read_text()
    codec.REG = {}
    syntax = re.sub(r'//[^\n]*', '', syntax)
    for sort, body in re.findall(r'syntax\s+(\w+)\s*::=\s*(.*?)(?=\n\s*syntax|\nendmodule)', syntax, re.S):
        for name, args in re.findall(r'(\w+)\(([^()]*)\)', body):
            sorts = tuple(x.strip() for x in args.split(',') if x.strip())
            if name in codec.REG: raise ValueError('duplicate constructor declaration')
            codec.REG[name] = (sort, sorts)


configure()


def encode(value, sort):
    if sort in ('Int', 'Bool', 'String'):
        required = {'Int': int, 'Bool': bool, 'String': str}[sort]
        if type(value) is not required: raise ValueError('wrong scalar type for ' + sort)
        return codec.atom(sort, value)
    if type(value) is not dict or set(value) != {'constructor', 'arguments'}:
        raise ValueError('exact constructor/arguments input required')
    name, args = value['constructor'], value['arguments']
    if type(name) is not str or name not in codec.REG or type(args) is not list:
        raise ValueError('unknown constructor or argument carrier')
    actual_sort, child_sorts = codec.REG[name]
    if actual_sort != sort or len(args) != len(child_sorts):
        raise ValueError('constructor sort or arity mismatch')
    return codec.C(name, *(encode(v, s) for v, s in zip(args, child_sorts)))


def tree(term):
    if term.name in ('@Int', '@Bool', '@String'):
        return {'sort': term.sort, 'value': term.args[0]}
    if term.name not in codec.REG: raise ValueError('unhandled constructor')
    return {'sort': term.sort, 'constructor': term.name,
            'arguments': [tree(x) for x in term.args]}


def same(a, b):
    if type(a) is not type(b): return False
    if type(a) is dict: return a.keys() == b.keys() and all(same(a[k], b[k]) for k in a)
    if type(a) is list: return len(a) == len(b) and all(same(x,y) for x,y in zip(a,b))
    return a == b


def complete_term(text):
    # New module has exactly one <k> cell. No search, skipped cell or truncation.
    text = text.strip()
    if text.startswith('<generatedTop>') and text.endswith('</generatedTop>'):
        text = text[len('<generatedTop>'):-len('</generatedTop>')].strip()
    if text.startswith('<k>') and text.endswith('</k>'):
        text = text[len('<k>'):-len('</k>')].strip()
        if text.endswith('~> .K'): text = text[:-len('~> .K')].rstrip()
    return text


def pin(path):
    path = Path(path); raw = path.read_bytes()
    return {'path': str(path.resolve()), 'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest()}


def checked(row):
    if type(row) is not dict or set(row) != {'path','bytes','sha256'} or pin(row['path']) != row:
        raise ValueError('exact file pin differs')
    return Path(row['path']).read_bytes()


def write(path, raw):
    with Path(path).open('xb') as stream: stream.write(raw)
    return pin(path)


def queries(document):
    if set(document) != {'schema','profile','admission','cases'} or document['schema'] != 'sherlock-c6-selected-effects-input/1' or document['profile'] != 'logical-C6-selected-effects/1':
        raise ValueError('source input schema/profile differs')
    names = set(); result = codec.C('c6NoQueries')
    for row in reversed(document['cases']):
        if set(row) != {'case_id','query'} or type(row['case_id']) is not str or row['case_id'] in names:
            raise ValueError('case identity differs or duplicates')
        names.add(row['case_id'])
        result = codec.C('c6QueryRow', codec.atom('String', row['case_id']), encode(row['query'],'C6Query'), result)
    return result


def decode_complete(text, source):
    parsed = codec.parse_printed(complete_term(text), 'C6Answers')
    names = []; at = parsed
    while at.name == 'c6AnswerRow':
        names.append(at.args[0].args[0]); at = at.args[2]
    if at.name != 'c6NoAnswers' or names != [x['case_id'] for x in source['cases']]:
        raise ValueError('complete answer count/order/identity differs')
    return tree(parsed)


def validate_barrier(barrier, actual_pin):
    if (type(barrier) is not dict or type(barrier.get('expected_body_reads')) is not int
            or barrier['expected_body_reads'] != 0 or type(barrier.get('members')) is not list
            or actual_pin not in barrier['members']):
        raise ValueError('actual-before-expected barrier membership missing')


def compare_documents(actual, expected):
    return {'status':'PASS_WHOLE_TYPED_SELECTED_EFFECTS' if same(actual,expected) else 'FAIL',
            'whole_typed_equal':same(actual,expected),'full_interpreter_or_native_claim':False}


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    sub = cli.add_subparsers(dest='action',required=True)
    r=sub.add_parser('render'); r.add_argument('input'); r.add_argument('output')
    d=sub.add_parser('decode'); d.add_argument('input'); d.add_argument('printed'); d.add_argument('output')
    c=sub.add_parser('compare'); c.add_argument('manifest')
    a=cli.parse_args()
    if a.action == 'render':
        result=write(a.output,('c6Run('+codec.render(queries(load(a.input)))+')\n').encode())
    elif a.action == 'decode':
        source=load(a.input); queries(source)
        result=write(a.output,(json.dumps(decode_complete(Path(a.printed).read_text(),source),indent=2,allow_nan=False)+'\n').encode())
    else:
        m=load(a.manifest)
        if set(m) != {'actual','expected','durable_freeze'}: raise ValueError('exact comparison manifest required')
        barrier=loads(checked(m['durable_freeze']))
        validate_barrier(barrier, m['actual'])
        actual=loads(checked(m['actual']))
        # First expected semantic read occurs only after all preceding checks.
        expected=loads(checked(m['expected']))
        result=compare_documents(actual,expected)
        result.update(actual=m['actual'],expected=m['expected'],expected_body_reads=1)
        print(json.dumps(result)); raise SystemExit(0 if result['whole_typed_equal'] else 1)
    print(json.dumps({'status':'DATA_PERSISTED','product':result,'expected_body_reads':0}))


if __name__ == '__main__': main()
