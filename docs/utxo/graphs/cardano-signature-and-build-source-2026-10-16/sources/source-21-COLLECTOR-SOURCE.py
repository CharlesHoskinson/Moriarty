"""Two exact public Scrapling reads; no keys, transactions or old-batch retry."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,math,resource,time

OUT=Path('/home/charl/moriarty-eutxo-validation-2026-10-12/cardano-preprod-cli-params-and-era-public-source-data-20261016')
BASE='https://preprod.koios.rest/api/v1'
ENDPOINTS=('cli_protocol_params','era_summaries')
CAP=2*1024*1024
now=lambda:datetime.now(timezone.utc).isoformat()
sha=lambda b:hashlib.sha256(b).hexdigest()

def strict_json(raw):
    def pairs(xs):
        d={}
        for k,v in xs:
            if k in d:raise ValueError('duplicate_json_key')
            d[k]=v
        return d
    def finite(s):
        v=float(s)
        if not math.isfinite(v):raise ValueError('nonfinite_json_number')
        return v
    def bad(s):raise ValueError('nonfinite_json_constant')
    return json.loads(raw.decode('utf-8'),object_pairs_hook=pairs,parse_float=finite,parse_constant=bad)

def main():
    if OUT.exists():raise RuntimeError('refuse_existing_output_root_no_retry')
    OUT.mkdir()
    source=Path(__file__).read_bytes();(OUT/'COLLECTOR-SOURCE.py').write_bytes(source)
    spec=Path('/home/charl/moriarty-eutxo-validation-2026-10-12/cardano-preprod-public-observation-primary-sources-sol-20261016/captures/koiosapi-preprod.yaml')
    result={'format':'cardano-preprod-cli-params-and-era-public-source-data/1','startedAt':now(),'status':'STARTED','queries':[],
      'scope':'New two-endpoint source-data acquisition under existing user research/Preprod instruction; NOT a rerun or extension of spent five-query observation authority. No tip/address/funding query, wallet/key/private access, signing, SDK/proof/native VM or ledger mutation.',
      'collectorSha256':sha(source),'sourceContract':{'path':str(spec),'sha256':sha(spec.read_bytes())},
      'transport':{'scrapling':'0.4.15','timeoutEachSeconds':15,'attemptsPerEndpoint':1,'retries':0,'redirects':False,'cookiePersistence':False,'auth':False,'cleanEnvironmentRequired':True},
      'limits':{'maxPersistedBodyBytesEach':CAP,'wholeBodyBufferedBeforeSizeCheck':True,'fileSizeCapBytes':8*1024*1024,'addressSpaceCapBytes':1024*1024*1024,'cpuSoftHardSeconds':[20,25]},
      'ledgerAuthenticated':False,'cliCompatibilityEstablished':False,'completeEraHistoryEstablished':False,'atomicSnapshot':False}
    def save():(OUT/'RESULT.json').write_text(json.dumps(result,indent=2,allow_nan=False)+'\n')
    save()
    resource.setrlimit(resource.RLIMIT_CORE,(0,0));resource.setrlimit(resource.RLIMIT_FSIZE,(8*1024*1024,8*1024*1024))
    resource.setrlimit(resource.RLIMIT_AS,(1024*1024*1024,1024*1024*1024));resource.setrlimit(resource.RLIMIT_CPU,(20,25))
    t=time.monotonic()
    try:
        from scrapling.fetchers.requests import FetcherSession
        with FetcherSession(impersonate=None,stealthy_headers=False,retries=1,retry_delay=0,follow_redirects=False,max_redirects=0,verify=True,timeout=15,headers={'Accept':'application/json'}) as session:
            for name in ENDPOINTS:
                row={'endpoint':name,'url':BASE+'/'+name,'method':'GET','at':now(),'attempted':True,'returned':False,'validated':False}
                result['queries'].append(row);save()
                response=session.get(row['url'],timeout=15,retries=1,retry_delay=0,follow_redirects=False,max_redirects=0,verify=True,stealthy_headers=False,impersonate=None,discard_cookies=True,headers={'Accept':'application/json'})
                raw=bytes(response.body);headers={str(k).lower():str(v) for k,v in dict(response.headers).items()}
                row.update(returned=True,status=int(response.status),responseUrl=str(response.url),bytes=len(raw),sha256=sha(raw),headers={k:headers[k] for k in ['content-type','content-range'] if k in headers});save()
                if len(raw)>CAP:raise ValueError('body_exceeds_postbuffer_persist_bound')
                rel=name+'.json';(OUT/rel).write_bytes(raw);row['rawFile']=rel;save()
                if row['status']!=200 or row['responseUrl']!=row['url'] or getattr(response,'history',None):raise ValueError('unexpected_status_url_or_redirect')
                if headers.get('content-type','').split(';')[0].strip().lower()!='application/json':raise ValueError('unexpected_content_type')
                data=strict_json(raw)
                if not isinstance(data,(dict,list)):raise ValueError('unexpected_top_level_json_shape')
                row.update(validated=True,topLevelType=type(data).__name__,rowCount=len(data) if isinstance(data,list) else None)
                save()
        result['status']='CAPTURED_PUBLIC_SOURCE_DATA_REQUIRES_CONSUMER_VALIDATION'
    except Exception as e:
        result.update(status='FAILED_PRESERVED_NO_RETRY',errorType=type(e).__name__)
    result.update(completedAt=now(),wallSeconds=time.monotonic()-t);save()
    print(json.dumps({'status':result['status'],'queries':result['queries'],'outputRoot':str(OUT)},allow_nan=False))
    return 0 if result['status'].startswith('CAPTURED_') else 1

if __name__=='__main__':raise SystemExit(main())
