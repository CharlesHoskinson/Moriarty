"""Offline public packet citation/receipt checks; executes no subject code."""
from pathlib import Path
import hashlib
import json
import re

R = Path('/home/charl/moriarty-c2-opus55-restored-20261017')
P = R / 'shared-check-main79-dual-role-review-preparation-20261018'
O = Path(__file__).resolve().parent
review = R / 'shared-check-main79-dual-role-terminal-review-20261018/source-job/TERMINAL-REVIEW.json'
sha = lambda b: hashlib.sha256(b).hexdigest()
raw = review.read_bytes()
assert sha(raw) == '515b69e0a31cb5de998e10041d9dac16615002c9aaffdfb9691643ff0763ae3c'
x = json.loads(raw)
(O / 'TERMINAL-REVIEW.original.json').write_bytes(raw)
def resolve(path):
    return P / path if (P / path).is_file() else P / 'head' / path
def dump(name, obj):
    (O / name).write_text(json.dumps(obj, indent=2) + '\n')
coverage = []
for path, entry in x['coverage'].items():
    c = dict(entry, path=path)
    p = resolve(c['path']); b = p.read_bytes()
    coverage.append({'path': c['path'], 'sha256': sha(b), 'bytes': len(b), 'matches': sha(b) == c['sha256']})
assert len(coverage) == 21 and all(c['matches'] for c in coverage)
quotes = []
corrections = []
owners = [('finding/' + c['id'], c) for c in x['findings']] + [('sourceCheck/' + k, c) for k,c in x['sourceChecks'].items()]
for owner,c in owners:
    for i,q in enumerate(c['quotes']):
        b = resolve(q['path']).read_bytes(); t = b.decode(); literal = q['exactQuote']
        a = {'owner': owner, 'index': i, 'path': q['path'], 'sha256': sha(b), 'hashMatches': sha(b) == q['sha256']}
        if 'lineStart' in q:
            at = [m.start() for m in re.finditer(re.escape(literal),t)]
            assert len(at) == 1, a
            start = t[:at[0]].count('\n') + 1
            end = start + literal.count('\n')
            a.update(originalLines=[q['lineStart'],q['lineEnd']], exactLines=[start,end], exactQuoteUnique=True)
            a['locatorContainsQuote'] = literal in '\n'.join(t.splitlines()[q['lineStart']-1:q['lineEnd']])
            if not a['locatorContainsQuote']: corrections.append(a.copy())
        else:
            val = json.loads(t)
            for token in q['jsonPointer'].lstrip('/').split('/'):
                token = token.replace('~1','/').replace('~0','~')
                val = val[int(token)] if isinstance(val,list) else val[token]
            a['jsonPointer'] = q['jsonPointer']
            if isinstance(val,str):
                a['selectedStringMatches'] = val == literal
                assert a['selectedStringMatches']
            else:
                assert literal.startswith('[') and json.loads(literal[1:]) == val
                a.update(selectedObjectMatchesAfterRemovingOneArrayPrefix=True, exactQuoteNeedsCorrection=True,
                         correction='Keep /0; remove the first [ and label the remainder compact JSON serialization of the selected object (not a literal byte substring).',
                         correctedCompactJson=literal[1:])
                corrections.append(a.copy())
        quotes.append(a)
assert len(quotes) == 27 and all(q['hashMatches'] for q in quotes)
assert len(corrections) == 9
receipt_counts = {}
final = json.loads((P/'current-receipts/FINAL-RESULT.json').read_text())
for group,key,n in [('public-tests','tests',10),('core-tests','coreTests',4)]:
    b = (P/f'current-receipts/{group}/STDOUT.log').read_bytes()
    totals = [list(map(int,m)) for m in re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out',b.decode())]
    assert sum(v[0] for v in totals) == n and all(not any(v[1:]) for v in totals)
    assert sha(b) == final[key]['stdoutSha256'] and final[key]['exitCode'] == 0
    receipt_counts[group] = {'passed':n,'summaries':totals,'stdoutSha256':sha(b),'receiptWallSeconds':final[key]['wallSeconds']}
cases = json.loads((P/'head/crates/fixtures/check-source/cases.json').read_text())
direct=[]
for c in cases:
    folder = P/'current-receipts/direct-cases'/c['id']
    baseline = P/'baseline-receipts/cases'/c['id']
    b = (folder/'STDOUT.log').read_bytes()
    expected = (P/'head/crates/fixtures/check-source'/c['expectedStdout']).read_bytes()
    res = json.loads((folder/'RESULT.json').read_text())
    assert b == expected and (folder/'STDERR.log').read_bytes() == b'' and res['exitCode'] == c['expectedExit']
    row={'case':c['id'],'exitCode':res['exitCode'],'bytes':len(b),'stdoutSha256':sha(b),'expectedBytesEqual':True,'stderrEmpty':True}
    if (baseline/'STDOUT.log').is_file():
        assert b == (baseline/'STDOUT.log').read_bytes()
        assert json.loads((baseline/'RESULT.json').read_text())['exitCode'] == res['exitCode']
        assert (baseline/'STDERR.log').read_bytes() == b''
        row['baselineBytesEqual']=True
        row['baselineExitAndEmptyStderrEqual']=True
    direct.append(row)
assert len(cases)==8 and all('--quiet' in c['humanArgv'] for c in cases)
dump('MECHANICAL-CHECKS.json',{'reviewSha256':sha(raw),'coverage':coverage,'quotes':quotes,'receiptCounts':receipt_counts,'directCases':direct,'sourceChecksCount':len(x['sourceChecks']),'materialContradictionFound':False,'sourceVoteSupplied':False,'reviewAdopted':False,'actualBackendIdentityAttested':False,'parentClosure':False,'execution':'Offline JSON/hash/source-text operations only; no subject runtime/provider execution.'})
dump('MINIMAL-CORRECTIONS.json',{'wholeOpinionUnchanged':True,'reviewSha256':sha(raw),'corrections':corrections})
print(json.dumps({'coverage':len(coverage),'quoteHashes':len(quotes),'shiftedRanges':8,'jsonQuoteFormattingCorrections':1,'receiptCounts':receipt_counts,'directCases':direct,'corrections':corrections},indent=2))
