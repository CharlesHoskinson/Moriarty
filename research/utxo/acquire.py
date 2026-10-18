#!/usr/bin/env python3
"""Public research acquisition; never executes downloaded repository code."""
import argparse, datetime, hashlib, json, pathlib, subprocess, time
from urllib.parse import urlsplit
from urllib.robotparser import RobotFileParser
from scrapling.fetchers import Fetcher

def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def save_json(path, value): path.write_text(json.dumps(value, indent=2)+"\n")

def main():
    p=argparse.ArgumentParser(); p.add_argument('--dir', required=True)
    sub=p.add_subparsers(dest='command', required=True)
    f=sub.add_parser('fetch'); f.add_argument('url'); f.add_argument('name')
    g=sub.add_parser('repo'); g.add_argument('url'); g.add_argument('name')
    args=p.parse_args(); root=pathlib.Path(args.dir); root.mkdir(parents=True,exist_ok=True)
    receipt=root/(args.name+'.receipt.json')
    if receipt.exists(): raise SystemExit('Existing receipt preserved; choose another name')
    r={'requested_url':args.url,'retrieved_at':now(),'method':'scrapling.Fetcher' if args.command=='fetch' else 'git shallow clone','review_state':'unreviewed','authority':'primary','independence_key':urlsplit(args.url).netloc,'limitations':[]}
    try:
        if args.command=='repo':
            dest=root/args.name
            cmd=['git','-c','credential.helper=','clone','--depth','1','--no-recurse-submodules',args.url,str(dest)]
            result=subprocess.run(cmd,capture_output=True,text=True,timeout=240)
            r.update(exit_code=result.returncode,log=result.stderr[-1600:])
            if result.returncode: raise RuntimeError('clone failed')
            r['commit']=subprocess.check_output(['git','-C',str(dest),'rev-parse','HEAD'],text=True).strip()
            r['branch']=subprocess.check_output(['git','-C',str(dest),'branch','--show-current'],text=True).strip()
            r['remote']=subprocess.check_output(['git','-C',str(dest),'remote','get-url','origin'],text=True).strip()
            r['payload_path']=str(dest); r['status']='downloaded'; r['limitations'].append('Shallow snapshot; submodules, LFS payloads and full history not downloaded; code not executed.')
            r['sha256']=hashlib.sha256(r['commit'].encode()).hexdigest(); r['hash_kind']='commit-string-sha256'
        else:
            origin=urlsplit(args.url); robots_url=f'{origin.scheme}://{origin.netloc}/robots.txt'
            rob=Fetcher.get(robots_url,timeout=20); body=bytes(rob.body)
            (root/(args.name+'.robots.txt')).write_bytes(body)
            r['robots_status']=rob.status; r['robots_url']=robots_url
            if rob.status==200:
                parser=RobotFileParser();parser.parse(body.decode('utf-8','replace').splitlines())
                if not parser.can_fetch('MoriartyResearch',args.url):
                    r['status']='robots-disallowed';save_json(receipt,r);print(json.dumps(r));return
                time.sleep(max(1,min(parser.crawl_delay('MoriartyResearch') or parser.crawl_delay('*') or 1,20)))
            else: r['limitations'].append('Robots rules unavailable; individual public source retrieval only.')
            page=Fetcher.get(args.url,timeout=45); data=bytes(page.body)
            if len(data)>60_000_000: raise RuntimeError('payload exceeds 60MB collection limit')
            ct=str(page.headers.get('content-type','')); suffix='.pdf' if data.startswith(b'%PDF-') else '.html' if 'html' in ct else '.txt'
            dest=root/(args.name+suffix); dest.write_bytes(data)
            r.update(canonical_url=str(page.url),http_status=page.status,content_type=ct,bytes=len(data),sha256=hashlib.sha256(data).hexdigest(),payload_path=str(dest),status='downloaded' if page.status==200 else 'http-error')
            if suffix=='.pdf':
                out=root/(args.name+'.extract.txt'); proc=subprocess.run(['pdftotext','-layout',str(dest),str(out)],capture_output=True,text=True)
                r['extract_exit_code']=proc.returncode
                if out.exists():r['extract_path']=str(out)
            elif suffix=='.html':
                elements=page.css('main,article')
                text='\n\n'.join(e.get_all_text(separator='\n',strip=True) for e in elements) if elements else '\n'.join(e.get_all_text(separator='\n',strip=True) for e in page.css('h1,h2,h3,p,pre,li'))
                out=root/(args.name+'.extract.txt');out.write_text(text);r['extract_path']=str(out);r['extract_characters']=len(text)
                if len(text)<200:r['limitations'].append('Very short extract; may need alternate public source.')
    except Exception as exc:r.update(status='failed',error=str(exc)[:700])
    r['finished_at']=now();save_json(receipt,r)
    print(json.dumps({k:v for k,v in r.items() if k not in ['log']}))

if __name__=='__main__':main()
