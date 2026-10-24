"""Complete META printed-wire consumer using the existing parser unchanged.

Only public actual capture and declared source input are read. No expected operand,
comparison, runner, security policy or model arithmetic is implemented here.
Root's existing reviewed composition supplies pinned operands and durable barrier.
"""
import argparse
import json
from pathlib import Path
import sys

SOURCE_DIR = Path(__file__).resolve().parent
BASE = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-logical-constructor-origin-counterpart-20261019')
sys.path.insert(0, str(BASE / 'source'))
sys.path.insert(0, str(SOURCE_DIR))
import c6_data as base
import meta_ports


def consume_complete(printed, registry, source):
    """Whole existing parser -> full typed tree -> full six-field common projection."""
    data = {row['name']: (row['sort'], tuple(row['argument_sorts']))
            for row in registry['constructors']}
    saved = base.codec.REG
    try:
        base.codec.REG = data
        parsed = base.codec.parse_printed(base.complete_term(printed), 'CMObservations')
        internal = base.tree(parsed)
    finally:
        base.codec.REG = saved
    wire = meta_ports.to_wire(internal)
    return {'common': meta_ports.observations_document(internal, source),
            'internal': internal, 'wire': wire}


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument('source_input')
    cli.add_argument('registry')
    cli.add_argument('printed')
    cli.add_argument('common_output')
    cli.add_argument('internal_output')
    cli.add_argument('wire_output')
    args = cli.parse_args()
    source = base.load(args.source_input)
    registry = base.load(args.registry)
    result = consume_complete(Path(args.printed).read_text(), registry, source)
    outputs = {}
    for kind, path in [('common', args.common_output), ('internal', args.internal_output),
                       ('wire', args.wire_output)]:
        outputs[kind] = base.write(path, (json.dumps(result[kind], indent=2,
                                      allow_nan=False) + '\n').encode())
    print(json.dumps({'status': 'COMPLETE_C6_META_PUBLIC_CAPTURE_PROJECTED_NOT_YET_COMPARED',
                      'products': outputs, 'expected_body_reads': 0}))


if __name__ == '__main__':
    main()
