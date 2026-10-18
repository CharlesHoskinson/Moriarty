import json,tempfile,unittest
from pathlib import Path
from publication_policy import extraction
class ExtractionRepairs(unittest.TestCase):
 def test_reproduced_shapes_in_public_text(self):
  with tempfile.TemporaryDirectory() as d:
   p=Path(d)/'events.log'
   for value in [{'private_key':'a'*64},{'client_secret':'b'*40},{'mnemonic':'abandon '*11+'about'},{'cborHex':'5820'+'f'*64}]:
    for event in [{'type':'text','data':json.dumps(value)},{'payload_type':'run.output.delta','payload':{'text':json.dumps(value)}}]:
     p.write_text(json.dumps(event)+'\n');body,receipt=extraction(p);self.assertEqual(body,b'');self.assertEqual(receipt['status'],'withheld-unsafe-public-text')
   p.write_text(json.dumps({'type':'text','data':'Public result: checks passed.'})+'\n');self.assertEqual(extraction(p)[0],b'Public result: checks passed.')
if __name__=='__main__':unittest.main(verbosity=2)
