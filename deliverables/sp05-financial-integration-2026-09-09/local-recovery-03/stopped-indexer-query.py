"""Read only public transaction/block/action rows from the stopped indexer copy."""
import json,sqlite3,hashlib,time
from pathlib import Path
root=Path(__file__).parent;copy=json.loads((root/'stopped-indexer-copy.json').read_text());db=Path(copy['scratchPath']);assert hashlib.sha256(db.read_bytes()).hexdigest()==copy['sha256']
c=sqlite3.connect(db.as_uri()+'?mode=ro&immutable=1',uri=True);c.row_factory=sqlite3.Row
hash='1f64634de2761fc0f140dbe7784a0cbf7005f226799e94d9878932378bc731e6';address='ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713'
queries={
 'transaction':'''SELECT t.id, lower(hex(t.hash)) AS hash, t.protocol_version, length(t.raw) AS rawBytes, r.transaction_result, lower(hex(r.paid_fees)) AS paidFeesRaw, lower(hex(r.estimated_fees)) AS estimatedFeesRaw, b.height, lower(hex(b.hash)) AS blockHash, b.timestamp FROM transactions t JOIN regular_transactions r ON r.id=t.id JOIN blocks b ON b.id=t.block_id WHERE t.hash=?''',
 'identifiers':'SELECT lower(hex(identifier)) AS identifier FROM transaction_identifiers WHERE transaction_id=? ORDER BY id',
 'action':'''SELECT c.id,c.variant,c.attributes,length(c.state) AS stateBytes,lower(hex(c.address)) AS address,b.height,lower(hex(b.hash)) AS blockHash,t.id AS transaction_id,lower(hex(t.hash)) AS txHash FROM contract_actions c JOIN transactions t ON t.id=c.transaction_id JOIN blocks b ON b.id=t.block_id WHERE c.address=? ORDER BY c.id''',
 'latestBlock':'SELECT height,lower(hex(hash)) AS hash,timestamp FROM blocks ORDER BY height DESC LIMIT 1',
 'balances':'SELECT lower(hex(token_type)) AS tokenTypeRaw,lower(hex(amount)) AS amountRaw FROM contract_balances WHERE contract_action_id=? ORDER BY id'
}
rows=[dict(r) for r in c.execute(queries['transaction'],(bytes.fromhex(hash),))];actions=[dict(r) for r in c.execute(queries['action'],(bytes.fromhex(address),))]
for row in rows:row['identifiers']=[r['identifier'] for r in c.execute(queries['identifiers'],(row['id'],))]
for action in actions:
 action['balances']=[dict(r) for r in c.execute(queries['balances'],(action['id'],))]
 if action['txHash']==hash:
  state=c.execute('SELECT state FROM contract_actions WHERE id=?',(action['id'],)).fetchone()[0]
  path=root/'indexed-initialize-state.bin';path.write_bytes(state);action['stateSha256']=hashlib.sha256(state).hexdigest();action['stateArtifact']=path.name
result={'schema':'moriarty.stopped-indexer-public-observation/1','dbSha256':copy['sha256'],'transaction':rows,'contractActions':actions,'latestIndexedBlock':[dict(r) for r in c.execute(queries['latestBlock'])],'scope':'Actual stopped indexer DB observation; not independent node canonical finality, financial comparison, or proof acceptance'}
c.close();assert hashlib.sha256(db.read_bytes()).hexdigest()==copy['sha256']
(root/'stopped-indexer-queries.json').write_text(json.dumps(queries,indent=2)+'\n');(root/'stopped-indexer-result.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
