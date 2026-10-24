#!/usr/bin/env python3
"""Pure normal synthetic fixture checks; no subject/runtime test execution."""
import copy
import hashlib
import json
from pathlib import Path
import unittest
from compare_json import FIXTURES, Difference, compare, exact
P=Path(__file__).resolve().parent

def load(name):return json.loads((P/name).read_text())
class NormalLiteralData(unittest.TestCase):
 def test_all_selected_serializable_records_accept(self):
  for selection,filename in FIXTURES.items():
   with self.subTest(selection=selection):compare(selection,copy.deepcopy(load(filename)))
 def test_exact_counter_amount_type_and_missing_fields(self):
  gold=load('EXPECTED-START-ACCOUNTING.json')
  mutations=[]
  c=copy.deepcopy(gold);c['PublishedSegment']['accounts'][0]['gross_debit']={'Exact':'30'};mutations.append(c)
  c=copy.deepcopy(gold);c['PublishedSegment']['accounts'][0]['gross_debit']={'Exact':32};mutations.append(c)
  c=copy.deepcopy(gold);del c['PublishedSegment']['accounts'][0]['fee_debit'];mutations.append(c)
  c=copy.deepcopy(gold);c['PublishedSegment']['accounts'].reverse();mutations.append(c)
  c=copy.deepcopy(gold);c['PublishedSegment']['effects'][0],c['PublishedSegment']['effects'][1]=c['PublishedSegment']['effects'][1],c['PublishedSegment']['effects'][0];mutations.append(c)
  c=copy.deepcopy(gold);c['PublishedSegment']['effects'].pop();mutations.append(c)
  c=copy.deepcopy(gold);c['PublishedSegment']['accounts'].append(copy.deepcopy(c['PublishedSegment']['accounts'][1]));mutations.append(c)
  for i,candidate in enumerate(mutations):
   with self.subTest(i=i):
    with self.assertRaises(Difference):compare('start-accounting',candidate)
 def test_preserved_states_duties_work_and_none_are_selected(self):
  gold=load('EXPECTED-FALSE-AWAIT-RESULT.json')
  changes=[(('Ok','Waiting','advanced'),True),
   (('Ok','Waiting','segment'),{}),
   (('Ok','Waiting','attempted'),'0'),
   (('Ok','Waiting','continuation','logical','committed'),'9'),
   (('Ok','Waiting','continuation','boundary','state','marker','value'),'78'),
   (('Ok','Waiting','continuation','boundary','state','flag','value'),False),
   (('Ok','Waiting','continuation','boundary','balances',0,'amount'),'12'),
   (('Ok','Waiting','continuation','boundary','balances',5,'amount'),'100'),
   (('Ok','Waiting','continuation','boundary','liabilities',1,'amount'),'0'),
   (('Ok','Waiting','residual',0,'amount'),'0'),
   (('Ok','Waiting','continuation','logical','cap'),'7')]
  for path,value in changes:
   with self.subTest(path=path):
    c=copy.deepcopy(gold);parent=c
    for key in path[:-1]:parent=parent[key]
    parent[path[-1]]=value
    with self.assertRaises(Difference):compare('false-await-result',c)
 def test_present_zero_liability_retained_residual_filtered(self):
  result=load('EXPECTED-TRUE-RESUME-RESULT.json')['Ok']['Complete']
  liabilities=result['segment']['execution']['post']['liabilities']
  self.assertEqual([x['name'] for x in liabilities],['auditDuty','pay-beneficiary'])
  self.assertEqual(liabilities[1]['amount'],'0')
  self.assertEqual([x['name'] for x in result['residual']],['auditDuty'])
  candidate=load('EXPECTED-TRUE-RESUME-RESULT.json')
  candidate['Ok']['Complete']['segment']['execution']['post']['liabilities'].pop()
  with self.assertRaises(Difference):compare('true-resume-result',candidate)
 def test_source_work_and_published_accounting_scope(self):
  w=load('EXPECTED-WORK.json')
  self.assertEqual(2+2+2+1,int(w['start']['attempted']))
  self.assertEqual(1+1,int(w['false_await']['attempted']))
  self.assertEqual(2+2+2+2,int(w['true_resume']['attempted']))
  self.assertEqual(7+8,int(w['final_committed_total']))
  self.assertEqual(7+2+8,int(w['chosen_invocation_attempt_total']))
  self.assertEqual(load('EXPECTED-TRUE-RESUME-ACCOUNTING.json')['PublishedSegment']['accounts'][0]['gross_debit'],{'Exact':'11'})
  with self.assertRaises(Difference):compare('true-resume-accounting',load('EXPECTED-START-ACCOUNTING.json'))
  with self.assertRaises(Difference):compare('false-await-accounting',{'PublishedSegment':{'effects':[],'accounts':[]}})
 def test_full_literal_steps_match_independent_arithmetic(self):
  data=load('EXPECTED.json')
  self.assertEqual(data['initial'],{k:v for k,v in load('inputs/scenario.json').items() if k!='inputs'})
  start=data['observations']['start']['result']['Ok']['Waiting']
  true=data['observations']['true_resume']['result']['Ok']['Complete']
  self.assertEqual([s['index'] for s in start['segment']['execution']['steps']],[0,1,2])
  self.assertEqual([s['index'] for s in true['segment']['execution']['steps']],[3,4,5,6])
  self.assertEqual(start['continuation'],data['observations']['false_await']['result']['Ok']['Waiting']['continuation'])
  self.assertEqual(start['continuation']['boundary'],true['segment']['execution']['pre'])
  self.assertEqual(data['accounting_cross_checks']['first_reserve'],str(100-30-2))
  self.assertEqual(data['accounting_cross_checks']['first_liability'],str(40-30))
  self.assertEqual(data['accounting_cross_checks']['cumulative_gross_debit'],str(30+2+10+1))
 def test_hashes_of_independent_manual_canonical_literals(self):
  data=load('EXPECTED.json')['program_context']
  for which in ['program','plan']:
   raw=(P/('EXPECTED-'+which.upper()+'-CANONICAL.json')).read_bytes()
   self.assertEqual(json.loads(raw),data[which])
   self.assertEqual(hashlib.sha256(raw).hexdigest(),data[which+'_hash'])
 def test_unselected_failure_and_tentative_gaps_are_not_filled(self):
  f=load('EXPECTED.json')['local_failure_observation']
  self.assertEqual(f['status'],'PARAMETRIC_ONLY_NO_DIAGNOSTIC_INPUT_SELECTED')
  self.assertEqual(f['failed_tentative_snapshot']['status'],'unavailable')
  self.assertEqual(f['failed_tentative_effects']['status'],'unavailable')
  self.assertEqual(f['returned_continuation']['status'],'not-returned')
  self.assertEqual(f['input_committed_prefix'],load('EXPECTED.json')['committed_first_prefix'])
if __name__=='__main__':unittest.main(verbosity=2)
