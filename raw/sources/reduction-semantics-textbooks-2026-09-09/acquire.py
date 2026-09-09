"""Bounded public acquisition; no execution of downloaded source or browser bypass."""
from datetime import datetime, timezone
from hashlib import sha256
from pathlib import Path
from urllib.request import Request, urlopen
from urllib.robotparser import RobotFileParser
from urllib.parse import urlsplit
import json, time
from scrapling.fetchers import Fetcher

ROOT = Path(__file__).resolve().parent
RECEIPTS = ROOT / 'receipts.jsonl'

def record(url, canonical, status, body, name, method, **extra):
    target = ROOT / name
    target.parent.mkdir(parents=True, exist_ok=True)
    if target.exists():
        raise RuntimeError(f'Immutable capture already exists: {name}')
    target.write_bytes(body)
    receipt = dict(requested_url=url, canonical_url=canonical, retrieved_at=datetime.now(timezone.utc).isoformat(),
                   http_status=status, path=name, sha256=sha256(body).hexdigest(), bytes=len(body),
                   acquisition_method=method, source_class='primary', **extra)
    with RECEIPTS.open('a') as f:
        f.write(json.dumps(receipt, sort_keys=True)+'\n')
    return receipt

def web(url, name, *, robots=False):
    r = Fetcher.get(url, timeout=40)
    body = bytes(r.body)
    receipt = record(url, str(r.url), r.status, body, name, 'Scrapling 0.4.15 Fetcher.get',
                     last_modified=r.headers.get('last-modified'), coverage='robots response' if robots else 'complete response body')
    if r.status == 200 and not robots and '.html' in name:
        nodes = r.css('main, article, .maincolumn, #main')
        if not nodes: nodes = r.css('body')
        text = '\n'.join(n.get_all_text(separator='\n', strip=True) for n in nodes)
        (ROOT / (name + '.txt')).write_text(text)
    time.sleep(.5)
    return r

def github(repo, commit, name):
    base='https://api.github.com/repos/'+repo
    for suffix, filename in [('',name+'-repo.json'),('/commits/'+commit,name+'-commit.json')]:
        url=base+suffix
        with urlopen(Request(url, headers={'User-Agent':'Moriarty-public-source-research'}), timeout=40) as r:
            record(url,r.url,r.status,r.read(),filename,'GitHub structured REST API')
    url=base+'/tarball/'+commit
    with urlopen(Request(url, headers={'User-Agent':'Moriarty-public-source-research'}), timeout=90) as r:
        record(url,r.url,r.status,r.read(),name+'-source.tar.gz','GitHub structured REST API pinned tarball',
               remote='https://github.com/'+repo+'.git', commit=commit, checked_out_branch=None,
               submodule_state='archive; external submodule contents not acquired',dirty_state='not a checkout; immutable archive',
               coverage='entire public repository archive at the pinned commit')

if __name__ == '__main__':
    for host in ['plfa.github.io','softwarefoundations.cis.upenn.edu','download.racket-lang.org']:
        r=web('https://'+host+'/robots.txt',host+'-robots.txt',robots=True)
        if r.status not in (200,404): raise RuntimeError('Robots unavailable: '+host)
        if r.status == 200:
            parser=RobotFileParser(); parser.parse(bytes(r.body).decode().splitlines())
            if not parser.can_fetch('*','https://'+host+'/'): raise RuntimeError('Robots disallow: '+host)
    web('https://plfa.github.io/','plfa-index.html')
    web('https://softwarefoundations.cis.upenn.edu/plf-current/index.html','sf-plf-index.html')
    web('https://softwarefoundations.cis.upenn.edu/plf-current/plf.tgz','sf-plf-complete.tgz')
    github('plfa/plfa.github.io','1c39b1e82caa4283274e64ba58c3bc3c9ab781eb','plfa')
    github('racket/redex','649ad23438ffc0bb7144b54c8e39eb1c0ffdf43e','redex')
    web('https://download.racket-lang.org/releases/9.0/pdf-doc/redex.pdf','redex-9.0-complete.pdf')
