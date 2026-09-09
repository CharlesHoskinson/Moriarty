import json,time,hashlib,datetime,pathlib
from scrapling.fetchers import Fetcher
D=pathlib.Path('/home/charl/.local/state/moriarty/sp05-public-dust-fixture');start=time.monotonic()
url='https://indexer.preview.midnight.network/api/v4/graphql'
id='0007458033e8be29bd7ea582bce27b817b85f638c57169e8f27283399e12c36221'
query='query($offset: TransactionOffset!) { transactions(offset:$offset) { id protocolVersion raw hash block {height hash} unshieldedCreatedOutputs {owner intentHash tokenType value} unshieldedSpentOutputs {owner intentHash tokenType value} ... on RegularTransaction {identifiers fees {estimatedFees paidFees} transactionResult {status segments {id success}}} } }'
request={'query':query,'variables':{'offset':{'identifier':id}}}
(D/'request.json').write_text(json.dumps({'url':url,'body':request},indent=2)+'\n')
r={'requestedAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'transport':'Scrapling0.4.15 Fetcher.post','url':url,'timeoutSeconds':20,'retryPolicy':'one attempt,no retry','knownHistoricalIdentifier':id}
try:
 p=Fetcher.post(url,json=request,timeout=20,retries=0)
 raw=bytes(p.body);(D/'response.json').write_bytes(raw)
 r.update(status=p.status,finalUrl=p.url,bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest())
 parsed=json.loads(raw);r['graphqlErrors']=parsed.get('errors');r['transactionCount']=len(parsed.get('data',{}).get('transactions',[]))
except Exception as e:r.update(errorType=type(e).__name__,error=str(e))
r['elapsedSeconds']=time.monotonic()-start
(D/'retrieval.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r,indent=2))
