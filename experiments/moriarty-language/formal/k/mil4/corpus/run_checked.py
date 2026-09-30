#!/usr/bin/env python3
"""Strict output check for the expanded MIL/4 S0 K hostile corpus.

The three accepted results below are literal independent expectations. This
runner compares complete parsed result terms and requires all 37 named cases.
"""
import json
import re
import subprocess
from pathlib import Path

from run import DEFINITION, call, corpus

HERE = Path(__file__).resolve().parent

# Explicit expected pre-state, complete ordered effects, post-state, phase,
# duty, and remaining work. Values follow s0-discriminators.md and fixture
# setup; they are deliberately not computed from the request terms.
EXPECTED_ACCEPTED = {
    'T-10-1': '''accepted(
      state(balance("O","A",100),balance("R","A",0),balance("F","A",0),
            allowance("O","A",11,0),noObligation(),head("h0"),noReplays(),1,0),
      effect(debit("O","A",11),effect(credit("R","A",10),
        effect(credit("F","A",1),effect(useAllowance("O","A",11),
          effect(useReplay(replayKey("D","O","T-10-1")),
            effect(advanceHead(head("h0"),head("h1")),noEffects())))))),
      state(balance("O","A",89),balance("R","A",10),balance("F","A",1),
            allowance("O","A",0,11),noObligation(),head("h1"),
            used(replayKey("D","O","T-10-1"),noReplays()),0,1),
      terminalSuccess(),noDuty(),0)''',
    'R-30': '''accepted(
      state(balance("P","A",100),balance("C","A",0),noBalance(),
            allowance("P","A",100,0),
            obligation("L","P","C","A",1000,10,1010,"Outstanding"),
            head("h0"),noReplays(),1,0),
      effect(debit("P","A",30),effect(credit("C","A",30),
        effect(setObligation("L",980,0,980,"Outstanding"),
          effect(useAllowance("P","A",30),
            effect(useReplay(replayKey("D","P","R-30")),
              effect(advanceHead(head("h0"),head("h1")),noEffects())))))),
      state(balance("P","A",70),balance("C","A",30),noBalance(),
            allowance("P","A",70,30),
            obligation("L","P","C","A",980,0,980,"Outstanding"),
            head("h1"),used(replayKey("D","P","R-30"),noReplays()),0,1),
      terminalSuccess(),noDuty(),0)''',
    'R-near-bound': '''accepted(
      state(balance("P","A",1),
            balance("C","A",340282366920938463463374607431768211454),
            noBalance(),
            allowance("P","A",1,340282366920938463463374607431768211454),
            obligation("L","P","C","A",170141183460469231731687303715884105726,
                       1,170141183460469231731687303715884105727,"Outstanding"),
            head("h0"),noReplays(),1,0),
      effect(debit("P","A",1),effect(credit("C","A",1),
        effect(setObligation("L",170141183460469231731687303715884105726,
                             0,170141183460469231731687303715884105726,
                             "Outstanding"),
          effect(useAllowance("P","A",1),
            effect(useReplay(replayKey("D","P","R-near-bound")),
              effect(advanceHead(head("h0"),head("h1")),noEffects())))))),
      state(balance("P","A",0),
            balance("C","A",340282366920938463463374607431768211455),
            noBalance(),
            allowance("P","A",0,340282366920938463463374607431768211455),
            obligation("L","P","C","A",170141183460469231731687303715884105726,
                       0,170141183460469231731687303715884105726,"Outstanding"),
            head("h1"),used(replayKey("D","P","R-near-bound"),noReplays()),0,1),
      terminalSuccess(),noDuty(),0)''',
}

TOKEN = re.compile(r'\s*(?:"(?:\\.|[^"\\])*"|-?[0-9]+|[A-Za-z][A-Za-z0-9_-]*|[(),])')


def parse_term(source):
    """Parse one complete constructor term; reject gaps and trailing terms."""
    tokens = []
    cursor = 0
    while cursor < len(source):
        match = TOKEN.match(source, cursor)
        if match is None:
            if source[cursor:].isspace():
                break
            raise ValueError(f'unparsed text at offset {cursor}: {source[cursor:cursor+40]!r}')
        tokens.append(match.group().strip())
        cursor = match.end()
    index = 0

    def expression():
        nonlocal index
        if index >= len(tokens):
            raise ValueError('truncated term')
        word = tokens[index]
        index += 1
        if word.startswith('"'):
            return ('string', json.loads(word))
        if re.fullmatch(r'-?[0-9]+', word):
            return ('int', int(word))
        if not re.fullmatch(r'[A-Za-z][A-Za-z0-9_-]*', word):
            raise ValueError(f'unexpected token {word!r}')
        if index >= len(tokens) or tokens[index] != '(':
            raise ValueError(f'constructor {word!r} missing opening parenthesis')
        index += 1
        arguments = []
        if index < len(tokens) and tokens[index] != ')':
            while True:
                arguments.append(expression())
                if index >= len(tokens):
                    raise ValueError('truncated argument list')
                if tokens[index] == ')':
                    break
                if tokens[index] != ',':
                    raise ValueError(f'expected comma, got {tokens[index]!r}')
                index += 1
        if index >= len(tokens) or tokens[index] != ')':
            raise ValueError('missing closing parenthesis')
        index += 1
        return (word, tuple(arguments))

    result = expression()
    if index != len(tokens):
        raise ValueError(f'extra term tokens: {tokens[index:index+5]!r}')
    return result


def extract_out(stdout):
    matches = re.findall(r'<out>\s*(.*?)\s*</out>', stdout, re.DOTALL)
    if len(matches) != 1:
        raise ValueError(f'expected exactly one <out> cell; found {len(matches)}')
    if not re.search(r'<k>\s*\.K\s*</k>', stdout):
        raise ValueError('nonterminal <k> cell')
    if len(re.findall(r'<s0>', stdout)) != 1 or len(re.findall(r'</s0>', stdout)) != 1:
        raise ValueError('incomplete or extra <s0> configurations')
    return matches[0]



def check_case(case, expected_out):
    """Require a complete terminal result and the unchanged stipulated tuple.

    Rejection has no state/effect payload. The only persistent input cell is
    external; comparing its entire term pins every pre-state component. This
    models no state publication and tuple preservation, not ledger rollback.
    """
    command = ['krun', '--definition', str(DEFINITION), '-o', 'pretty', '/dev/stdin']
    program = call('withAuthenticated', case['request'], case['premise'])
    proc = subprocess.run(command, input=program, text=True,
                          capture_output=True, timeout=120)
    observed_out = None
    observed_external = None
    error = None
    preserved = False
    try:
        observed_out = extract_out(proc.stdout)
        config = re.fullmatch(
            r'\s*<s0>\s*<k>\s*\.K\s*</k>\s*<out>\s*(.*?)\s*</out>'
            r'\s*<external>\s*(.*?)\s*</external>\s*</s0>\s*',
            proc.stdout, re.DOTALL)
        if config is None:
            raise ValueError('unexpected cells or text in complete terminal configuration')
        observed_external = config.group(2)
        preserved = parse_term(observed_external) == parse_term(case['premise'])
        matched = (proc.returncode == 0 and proc.stderr == '' and preserved
                   and parse_term(observed_out) == parse_term(expected_out))
    except ValueError as exc:
        matched = False
        error = str(exc)
    return {'case': case['name'], 'matched': matched,
            'expectedOut': expected_out, 'observedOut': observed_out,
            'externalPreserved': preserved, 'observedExternal': observed_external,
            'preState': case.get('preState'), 'round': case.get('round'),
            'requestedOutcome': case.get('outcome'),
            'parseError': error, 'exitCode': proc.returncode,
            'command': command, 'request': case['request'],
            'premise': case['premise'], 'program': program,
            'stdout': proc.stdout, 'stderr': proc.stderr}

def main():
    cases = corpus()
    names = [case['name'] for case, _ in cases]
    accepted_names = {case['name'] for case, verdict in cases if verdict == 'accepted'}
    if len(cases) != 37 or len(names) != len(set(names)):
        raise ValueError('expected 37 unique corpus cases')
    if accepted_names != set(EXPECTED_ACCEPTED):
        raise ValueError('accepted cases differ from the independent complete expectations')
    results = []
    for case, expected_verdict in cases:
        expected_out = (EXPECTED_ACCEPTED[case['name']]
                        if expected_verdict == 'accepted'
                        else f'rejected({json.dumps(expected_verdict[0])},'
                             f'{json.dumps(expected_verdict[1])},1)')
        row = check_case(case, expected_out)
        results.append(row)
        matched = row['matched']
        print(f'{case["name"]}: {"MATCH" if matched else "MISMATCH"}', flush=True)
    (HERE / 's1b-regression-results.json').write_text(json.dumps(results, indent=2) + '\n')
    return 0 if all(row['matched'] for row in results) else 1


if __name__ == '__main__':
    raise SystemExit(main())
