"""Bounded public-state extraction from an immutable stopped indexer copy."""
import hashlib,json,sqlite3
from pathlib import Path
root=Path(__file__).parent;copy=json.loads((root/'stopped-indexer-copy.json').read_text());db=Path(copy['scratchPath']);sha=lambda b:hashlib.sha256(b).hexdigest()
if sha(db.read_bytes())!=copy['sha256']:raise RuntimeError('COPY_HASH')
address='8824d69c9058f322b4f6da7e7cd8d49f3235db5fbe3d6080d25f239243812261'
queries={
 'action':'''SELECT c.id,c.variant,c.attributes,c.state,t.raw,t.id AS transactionId,t.protocol_version,r.transaction_result,lower(hex(t.hash)) AS transactionHash,b.height,lower(hex(b.hash)) AS blockHash,b.timestamp FROM contract_actions c JOIN transactions t ON t.id=c.transaction_id JOIN regular_transactions r ON r.id=t.id JOIN blocks b ON b.id=t.block_id WHERE c.address=? AND t.hash=?''',
 'identifiers':'SELECT lower(hex(identifier)) FROM transaction_identifiers WHERE transaction_id=? ORDER BY id',
 'balances':'SELECT lower(hex(token_type)) AS tokenTypeRaw,lower(hex(amount)) AS amountRaw FROM contract_balances WHERE contract_action_id=? ORDER BY id',
 'latestBlock':'SELECT height,lower(hex(hash)) AS hash,timestamp FROM blocks ORDER BY height DESC LIMIT 1'
}
c=sqlite3.connect(db.as_uri()+'?mode=ro&immutable=1',uri=True);c.row_factory=sqlite3.Row;records=[]
try:
 for stage,hash,height in [('swap','86c44ccb8220a8b3ef78bbb38dc89dc3479de31290c16e6fc84c5160d6a36e58',20403),('close','7a1c8e6ef185b78f3634766a1f435d39d7958fdce260af3270fbdfa56c24f0f3',20407)]:
  rows=c.execute(queries['action'],(bytes.fromhex(address),bytes.fromhex(hash))).fetchmany(2)
  if len(rows)!=1:raise RuntimeError('EXACT_ACTION_REQUIRED')
  row=dict(rows[0]);state=row.pop('state');raw=row.pop('raw')
  if row['height']!=height or row['variant']!='Call' or json.loads(row['attributes'])!={'Call':{'entry_point':stage}}:raise RuntimeError('ACTION_BINDING')
  if not 0<len(state)<=16777216 or not 0<len(raw)<=16777216:raise RuntimeError('BYTE_BOUND')
  if sha(raw)!=hash:raise RuntimeError('NATIVE_HASH')
  for name,data in [('indexed-'+stage+'-state.bin',state),(stage+'-transaction.bin',raw)]:
   with (root/name).open('xb') as f:f.write(data)
  row.update(stage=stage,stateBytes=len(state),stateSha256=sha(state),stateArtifact='indexed-'+stage+'-state.bin',transactionArtifact=stage+'-transaction.bin',identifiers=[r[0] for r in c.execute(queries['identifiers'],(row['transactionId'],))],indexedBalances=[dict(r) for r in c.execute(queries['balances'],(row['id'],))]);records.append(row)
 latest=[dict(r) for r in c.execute(queries['latestBlock'])]
finally:c.close()
after=sha(db.read_bytes())
if after!=copy['sha256']:raise RuntimeError('COPY_CHANGED')
result={'schema':'moriarty.stopped-swap-close-state/1','contractAddress':address,'dbSha256':after,'records':records,'latestIndexedBlock':latest,'scope':'Actual stopped indexer transaction/state evidence; indexed Success and height are not independent canonical finality or proof acceptance'}
(root/'queries.json').write_text(json.dumps(queries,indent=2)+'\n');(root/'indexed-result.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
