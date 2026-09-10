"""Extract and execute literal source SQL against an in-memory synthetic schema."""
import hashlib,json,re,sqlite3,time
from pathlib import Path
start=time.monotonic();root=Path(__file__).parent;source=(root/'source-contract-action.rs').read_text()
names=['get_latest_contract_action_by_address','get_contract_action_by_address_and_block_hash','get_contract_action_by_address_and_block_height']
queries={}
for name in names:
 body=source.split('async fn '+name+'(',1)[1].split('async fn ',1)[0]
 queries[name]=re.search(r'let query = indoc! \{"(.*?)"\};',body,re.S).group(1)
(root/'extracted-queries.json').write_text(json.dumps(queries,indent=2)+'\n')
db=sqlite3.connect(':memory:')
db.executescript('CREATE TABLE blocks(id INTEGER PRIMARY KEY,hash TEXT,height INTEGER);CREATE TABLE transactions(id INTEGER PRIMARY KEY,block_id INTEGER);CREATE TABLE contract_actions(id INTEGER PRIMARY KEY,address TEXT,state TEXT,attributes TEXT,zswap_state TEXT,transaction_id INTEGER);')
db.executemany('INSERT INTO blocks VALUES(?,?,?)',[(1,'deployment-hash',20313),(2,'later-empty-hash',20323)])
db.execute('INSERT INTO transactions VALUES(1,1)');db.execute('INSERT INTO contract_actions VALUES(1,?,?,?,?,1)',('loan-address','original-constructor-state','deploy','synthetic-zswap'))
results={
 'historicalHash':db.execute(queries[names[1]],{'1':'loan-address','2':'deployment-hash'}).fetchone(),
 'currentEmptyHash':db.execute(queries[names[1]],{'1':'loan-address','2':'later-empty-hash'}).fetchone(),
 'currentEmptyHeight':db.execute(queries[names[2]],{'1':'loan-address','2':20323}).fetchone(),
 'latest':db.execute(queries[names[0]],{'1':'loan-address'}).fetchone(),
}
assert results['currentEmptyHash'] is None and results['currentEmptyHeight'] is None
assert results['latest']==results['historicalHash'] and results['latest'][2]=='original-constructor-state'
db.close()
print(json.dumps({'status':'REPRODUCED','sourceSha256':hashlib.sha256(source.encode()).hexdigest(),'sqliteVersion':sqlite3.sqlite_version,'elapsedMs':(time.monotonic()-start)*1000,'results':results,'scope':'Literal extracted indexer SQL over synthetic two-block SQLite fixture; not live indexer storage'},indent=2))
