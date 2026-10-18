"""Source-only packet freeze; no driver/product/provider import or invocation."""
from pathlib import Path
import json,hashlib,ast
P=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def write(n,v):(P/n).write_text(json.dumps(v,indent=2)+'\n')
contract=json.loads((P/'BOUND-CANDIDATE.json').read_text());subject=sha(P/'BOUND-CANDIDATE.json')
descriptor=json.loads((P/'HOST-READ-SCOPE.json').read_text())
descriptor['fullSourcePinInventory']['sha256']=sha(P/'PINNED-SOURCES.json');write('HOST-READ-SCOPE.json',descriptor)
files=descriptor['mandatoryHostWholeFiles']
for label,perspective in [('A','Focus on the single wholebatch budget, owned process/deadline/cleanup controls and the consumed prior failures; independently assess every host resource predicate.'),('B','Focus on preserving exact phase feature selections, fresh-target withinbatch reuse, distinct financial/Source6 binary custody, oracle/pin scope and manual single allocation; independently assess every host resource predicate.')]:
 schema=json.loads((P/('VOTE-'+label+'-SCHEMA.json')).read_text());schema['properties']['reviewedContractSha256']={'const':subject};write('VOTE-'+label+'-SCHEMA.json',schema)
 chunks=['Fresh independent WHOLE ordinary host-resource vote. Requested gemini-4-argon-high HIGH; CLI model label is not canonical backend attestation. No source/semantic/security vote. No execution. '+perspective,'Read every full mandatory host file once below; no duplicated alias rereads needed. Report real scope/limitations. PINNED-SOURCES full982inventory and all originals remain physically available by frozen hash, with its purpose in descriptor. Opaque product bodies not mandatory for this host vote; later fullSOURCE+RESULT is required. Schema path is supplied by the caller; output whole JSON only, concise substantive evidence/reasons and preserve all dissent.\nHOST-READ-SCOPE.json\n'+(P/'HOST-READ-SCOPE.json').read_text()]
 for n in files:chunks.append('\nFULL FILE '+n+' SHA256 '+sha(P/n)+'\n'+(P/n).read_text())
 prompt='\n'.join(chunks);assert len(prompt.encode())<125000;(P/('VOTE-'+label+'-PROMPT.md')).write_text(prompt)
source=(P/'run_batch.py').read_text();tree=ast.parse(source)
worker=source.split('def worker(')[1].split('def run():')[0]
validation=json.loads((P/'STATIC-VALIDATION.json').read_text());validation.update({'hostMandatoryFiles':len(files),'primaryBytes':(P/'PRIMARY.md').stat().st_size,'votePromptBytes':{n:(P/('VOTE-'+n+'-PROMPT.md')).stat().st_size for n in ['A','B']},'workerNoStartNewSessionOrPerPhaseTimeout':'start_new_session' not in worker and 'timeout=' not in worker,'newEnvironmentContractExactly13':len(json.loads((P/'ENVIRONMENT-CONTRACT.json').read_text())['baseEnvironmentExactly13'])==13,'beforeAfterEachPhaseIncludesConfigAbsence':worker.count("verify_configs(plan['absentAncestorConfigs'])")==2,'boundWorkerCommandListComparedToContract':"assert plan['steps'] == contract['steps']"in worker,'pythonAstParses':True})
assert validation['workerNoStartNewSessionOrPerPhaseTimeout'] and validation['beforeAfterEachPhaseIncludesConfigAbsence']
write('STATIC-VALIDATION.json',validation)
# No self/task circular pins: task pins manifest; manifest pins every source/prompt/schema input.
inputs=[p for p in sorted(P.rglob('*'))if p.is_file() and p.name!='MANIFEST.json' and not p.name.endswith('-TASK.PREPARED.json') and p.name!='FREEZE-SUMMARY.json']
write('MANIFEST.json',{'kind':'SOURCE_ONLY_FINITE_ORDINARY_RESOURCE_PROPOSAL_NOT_ADOPTED','files':[{'path':str(p.relative_to(P)),'bytes':p.stat().st_size,'sha256':sha(p)}for p in inputs]})
for label in ['A','B']:
 task={'id':'combined-financial-quantity-finite600-resource-vote-'+label.lower()+'-20261018-a1','model':'gemini-4-argon-high','packet':str(P),'reviewToken':'combined-financial-quantity-finite600-vote-'+label.lower()+'-20261018-v1','wallSeconds':420,'promptFile':str(P/('VOTE-'+label+'-PROMPT.md')),'promptSha256':sha(P/('VOTE-'+label+'-PROMPT.md')),'schemaFile':str(P/('VOTE-'+label+'-SCHEMA.json')),'schemaSha256':sha(P/('VOTE-'+label+'-SCHEMA.json')),'manifestSha256':sha(P/'MANIFEST.json'),'scope':'Fresh independent WHOLE host-only ordinary single600+5batch resource amendment. Not source/result/security/SDK/native approval. Root sole adopter; no provider/product execution by preparer.'};write('VOTE-'+label+'-TASK.PREPARED.json',task)
for row in json.loads((P/'MANIFEST.json').read_text())['files']:assert sha(P/row['path'])==row['sha256']
summary={'packet':str(P),'manifestSha256':sha(P/'MANIFEST.json'),'contractSha256':subject,'driverSha256':sha(P/'run_batch.py'),'primaryBytes':(P/'PRIMARY.md').stat().st_size,'promptBytes':validation['votePromptBytes'],'taskHashes':{label:sha(P/('VOTE-'+label+'-TASK.PREPARED.json'))for label in ['A','B']},'counts':contract['selectionCounts'],'inputFiles':len(inputs),'totalInputBytes':sum(p.stat().st_size for p in inputs),'providerProductExecutionByPreparer':False}
write('FREEZE-SUMMARY.json',summary);print(json.dumps(summary,indent=2))
