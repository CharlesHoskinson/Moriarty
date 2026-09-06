"""pyk smoke test against the kompiled lesson 1.2 definition.

Checks the three pyk paths the ZKIR plan depends on (wiki/zkir-k-semantics-plan.md):
parsing a program text into KAST, converting KAST to KORE and back unchanged, and
running a program file through krun with the result read back as KAST.

Usage: python pyk_roundtrip.py <lesson-02-a-kompiled>
"""
import sys
from pathlib import Path

from pyk.kast.inner import KSort, KToken
from pyk.ktool.krun import KRun

definition_dir = Path(sys.argv[1]).resolve()
krun = KRun(definition_dir)

term = krun.parse_token(KToken('colorOf(Banana())', KSort('Color')))
pretty = krun.pretty_print(term)
print('parsed:', pretty)
assert 'colorOf' in pretty and 'Banana' in pretty, pretty

kore = krun.kast_to_kore(term, KSort('Color'))
back = krun.kore_to_kast(kore)
assert back == term, (back, term)
print('kast -> kore -> kast: unchanged')

rc, result = krun.krun(definition_dir.parent / 'banana.color')
out = krun.pretty_print(result)
print('krun rc:', rc)
print('krun result:', out.replace('\n', ' '))
assert rc == 0 and 'Yellow' in out, out
print('pyk round trip: PASS')
