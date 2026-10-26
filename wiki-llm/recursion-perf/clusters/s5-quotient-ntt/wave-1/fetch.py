import pathlib, subprocess, sys, urllib.request
url, slug, title = sys.argv[1:4]
pub = sys.argv[4] if len(sys.argv)>4 else 'Unknown'
p = pathlib.Path('raw/s5-quotient-ntt')/(slug+'.md')
r = subprocess.run(['scrapling','extract','get',url,str(p),'--timeout','40'],capture_output=True,text=True)
body = p.read_text() if p.exists() else 'FETCH FAILED\n'+r.stdout+r.stderr
if 'raw.githubusercontent.com/' in url:
    try:
        body='```text\n'+urllib.request.urlopen(url,timeout=40).read().decode()+'\n```\n'
    except Exception as e:
        body += '\nExact source fallback failed: '+str(e)
p.write_text(f'# {title}\n> Source: {url}\n> Collected: 2026-10-26\n> Published: {pub}\n\n'+body)
print(str(p), 'bytes',p.stat().st_size,'scrapling status',r.returncode)
