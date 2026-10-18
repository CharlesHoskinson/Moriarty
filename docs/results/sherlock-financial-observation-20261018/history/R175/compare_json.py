#!/usr/bin/env python3
"""Exact comparison of selected already-serializable public DTO JSON only.
No production imports or execution. Typed borrowed InvocationObservation has no
new wire schema here. Root selects a fixture and supplies the exact extracted
serializable value, without filling missing fields from expected data.
"""
import argparse
import json
from pathlib import Path
P=Path(__file__).resolve().parent
FIXTURES={
 'program':'EXPECTED-PROGRAM.json','plan':'EXPECTED-PLAN.json',
 **{name.lower()+'-'+kind.lower():'EXPECTED-'+name+'-'+kind+'.json'
 for name in ['START','FALSE-AWAIT','TRUE-RESUME']
 for kind in ['RESULT','ACCOUNTING','TENTATIVE-PUBLICATION']}}
class Difference(ValueError):pass

def exact(expected,actual,path=''):
 if type(expected) is not type(actual):
  raise Difference(f'{path or "/"}: type {type(actual).__name__} != {type(expected).__name__}')
 if isinstance(expected,dict):
  if expected.keys()!=actual.keys():
   raise Difference(f'{path or "/"}: missing={sorted(expected.keys()-actual.keys())}, extra={sorted(actual.keys()-expected.keys())}')
  for key,value in expected.items():
   exact(value,actual[key],path+'/'+key.replace('~','~0').replace('/','~1'))
 elif isinstance(expected,list):
  if len(expected)!=len(actual):raise Difference(f'{path or "/"}: different cardinality')
  for i,(a,b) in enumerate(zip(expected,actual)):exact(a,b,path+'/'+str(i))
 elif expected!=actual:raise Difference(f'{path or "/"}: {actual!r} != {expected!r}')

def compare(selection,actual):
 exact(json.loads((P/FIXTURES[selection]).read_text()),actual)

def main():
 parser=argparse.ArgumentParser(description=__doc__)
 parser.add_argument('selection',choices=FIXTURES);parser.add_argument('actual',type=Path)
 a=parser.parse_args()
 try:compare(a.selection,json.loads(a.actual.read_text()))
 except (ValueError,OSError) as e:parser.exit(1,str(e)+'\n')
 print('PASS: exact selected public DTO JSON equals sealed independent fixture')
if __name__=='__main__':main()
