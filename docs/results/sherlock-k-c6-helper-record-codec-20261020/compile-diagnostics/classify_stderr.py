"""Data-only complete stderr partition/classification. No K import/build/run."""
from pathlib import Path
from hashlib import sha256
import re,json,collections,datetime
R=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017')
K=R/'k-C6-helper-control-result-codec-20261019'
D=R/'k-C6-helper-first-compile-diagnostic-classification-20261020'
p=K/'actual/00-compile/stderr';raw=p.read_bytes()
assert len(raw)==152635 and sha256(raw).hexdigest()=='b027eb3abf3c6eb1d0524d76da75e20c520d2b5426d3c8b94341e45093e4f95b'
(D/'stderr.original').write_bytes(raw)
def pin(p):
 b=Path(p).read_bytes();return dict(path=str(p),bytes=len(b),sha256=sha256(b).hexdigest())
def save(p,x):Path(p).write_text(json.dumps(x,indent=2,ensure_ascii=False)+'\n')
starts=list(re.finditer(rb'^\[(Warning|Error|Critical|Info)\] ([^:\n]+):',raw,re.M))
assert starts and starts[0].start()==0
rows=[];segments=[];sources={};template_messages={};alllocations=set()
for i,m in enumerate(starts):
 start=m.start();end=starts[i+1].start() if i+1<len(starts) else len(raw);block=raw[start:end];text=block.decode('utf-8');segments.append(block)
 src=re.search(r'\tSource\(([^\n]+)\)',text);loc=re.search(r'\tLocation\((\d+),(\d+),(\d+),(\d+)\)',text);assert src and loc
 source=Path(src.group(1));location=list(map(int,loc.groups()));message=text[:src.start()].strip();flat=' '.join(message.split())
 dep=re.search(r'Either remove\s+`klabel\(([^)]+)\)`',message);unused=re.search(r"Variable '([^']+)' defined but not used",message)
 if dep:
  category='deprecated_klabel';symbol=dep.group(1);var=None
  expected=f'[Warning] Compiler: Attribute `klabel(_)` is deprecated. Either remove `klabel({symbol})`, or replace it by `symbol({symbol})`.'
 elif unused:
  category='unused_variable';symbol=None;var=unused.group(1)
  expected=f"[Warning] Compiler: Variable '{var}' defined but not used. Prefix variable name with underscore if this is intentional."
 else:raise AssertionError('Unclassified diagnostic message at '+str(i+1))
 assert flat==expected,(i+1,flat)
 if source not in sources:sources[source]=source.read_text().splitlines(keepends=True)
 ls=sources[source];assert 0<location[0]<=location[2]<=len(ls)
 actual_lines=''.join(ls[location[0]-1:location[2]])
 rendered=text[loc.end():]
 caret_marker=re.search(r'\n\t +\.\t',rendered);assert caret_marker
 rendered_code=re.sub(r'^\n\t\d+ \|\t','',rendered[:caret_marker.start()])
 assert ''.join(rendered_code.split())==''.join(actual_lines.split())
 assert re.fullmatch(r'[\s\^~]*',rendered[caret_marker.end():]) is not None
 if symbol:assert 'klabel('+symbol+')' in actual_lines,(source,location,symbol)
 else:assert re.search(r'\b'+re.escape(var)+r'\b',actual_lines)
 key=(str(source),*location);assert key not in alllocations;alllocations.add(key)
 row=dict(ordinal=i+1,severity=m.group(1).decode(),emitter=m.group(2).decode(),category=category,
  byte_start=start,byte_end_exclusive=end,bytes=len(block),raw_sha256=sha256(block).hexdigest(),
  stderr_first_line=raw[:start].count(b'\n')+1,stderr_last_line=raw[:end].count(b'\n')+(0 if block.endswith(b'\n') else 1),
  message_exact=message,message_whitespace_normalized=flat,source=str(source),location=location,
  symbol=symbol,unused_variable=var,diagnostic_rendered_excerpt_exact=text[loc.end():],actual_source_lines_exact=actual_lines)
 rows.append(row)
assert b''.join(segments)==raw and rows[-1]['byte_end_exclusive']==len(raw)
sev=collections.Counter(x['severity'] for x in rows);cat=collections.Counter(x['category'] for x in rows);byfile=collections.defaultdict(collections.Counter)
for x in rows:byfile[Path(x['source']).name][x['category']]+=1
unusedcounts=collections.Counter(x['unused_variable'] for x in rows if x['unused_variable'])
labels=collections.Counter(x['symbol'] for x in rows if x['symbol'])
summary={'schema':'k-complete-first-compile-stderr-category-summary/1','status':'COMPLETE_BYTE_PARTITION_ALL_DIAGNOSTICS_CLASSIFIED','original':pin(p),'retained_copy':pin(D/'stderr.original'),'stderr_lines':len(raw.splitlines()),'diagnostics':len(rows),'severity_counts':dict(sev),'emitter_counts':dict(collections.Counter(x['emitter'] for x in rows)),'categories':dict(cat),'by_source':{k:dict(v) for k,v in byfile.items()},'unused_variable_counts':dict(unusedcounts),'distinct_deprecated_label_strings':len(labels),'duplicate_deprecated_label_strings':{k:v for k,v in labels.items() if v>1},'distinct_whitespace_normalized_messages':len({x['message_whitespace_normalized'] for x in rows}),'distinct_exact_message_prefixes':len({x['message_exact'] for x in rows}),'unique_full_diagnostic_blocks':len({x['raw_sha256'] for x in rows}),'unclassified_diagnostics':0,'uncovered_bytes':0,'all_source_location_rows_unique':True,'all_rendered_code_matches_pinned_source_ignoring_renderer_whitespace':True,'all_remaining_renderer_tails_are_caret_only':True,'diagnostic_sources':[pin(p) for p in sources],'coverage':'All raw bytes partitioned, all message prefixes match exactly one of two templates after whitespace-only normalization, every Source/Location retained, all raw and current source excerpt bytes retained; no payload/golden/model execution.','scope':'Diagnostic classification only. Warning-free false; compile result/runtime/equality/security/semantics are separate evidence.'}
save(D/'CATEGORY-SUMMARY.json',summary)
save(D/'DIAGNOSTIC-MEMBERSHIP.json',{'schema':'k-complete-stderr-diagnostic-membership/1','original':pin(p),'rows':rows,'raw_coverage':'Ordered half-open byte spans exhaust original exactly; raw stderr is retained byte-exact for any original renderer/caret reconstruction.'})
# Human-readable complete dynamic diagnostic membership (no 152k repeated headers).
md=['# Complete diagnostic content catalog\n\nAll330 diagnostics are included below. Two common message templates are shown once; every dynamic label/variable, source location and exact indicated source line is retained. This is a readable factoring of messages, not a semantic model transformation. Complete raw renderer/caret text and wrapping remain in stderr.original and DIAGNOSTIC-MEMBERSHIP.json.\n',
'## Templates\n\nD: `[Warning] Compiler: Attribute klabel(_) is deprecated. Either remove klabel(LABEL), or replace it by symbol(LABEL).`\n\nU: `[Warning] Compiler: Variable VARIABLE defined but not used. Prefix variable name with underscore if this is intentional.`\n\nOriginal backticks, quote punctuation and wrapping are preserved per record in the JSON membership.\n',
'## Source aliases\n']
alias={p:f'S{i+1}' for i,p in enumerate(sources)}
for p,a in alias.items():md.append(f'\n- {a}: `{p}`; SHA256 `{pin(p)["sha256"]}`.\n')
md.append('\n## All deprecation sites\n\n| Diagnostic | Source | Location (startLine,startCol,endLine,endCol) | Label |\n|---:|---|---|---|\n')
for x in rows:
 if x['category']=='deprecated_klabel':md.append('| '+str(x['ordinal'])+' | '+alias[Path(x['source'])]+' | '+','.join(map(str,x['location']))+' | `'+x['symbol']+'` |\n')
md.append('\n## All unused-variable sites and indicated rules\n')
for x in rows:
 if x['category']=='unused_variable':md.append(f'\nDiagnostic {x["ordinal"]}: U, variable `{x["unused_variable"]}`, {alias[Path(x["source"])]} Location({",".join(map(str,x["location"]))}).\n\n```k\n'+x['actual_source_lines_exact']+'```\n')
md.append('\n## Every indicated declaration source line (complete)\n\nThe following is exact current pinned source text for every D site. Diagnostic ordinals distinguish repeated bare label spellings in different roles.\n')
for x in rows:
 if x['category']=='deprecated_klabel':md.append(f'\nD{x["ordinal"]} {alias[Path(x["source"])]} Location({",".join(map(str,x["location"]))})\n\n```k\n'+x['actual_source_lines_exact']+'```\n')
(D/'ALL-DISTINCT-DIAGNOSTIC-CONTENT.md').write_text(''.join(md))
# Specific source context inspected to classify the fourteen ignored fields/payloads.
p=K/'source/c6-helper-codec.k';ls=p.read_text().splitlines(keepends=True)
(D/'SOURCE-CONTEXT-244-284.k.txt').write_text(''.join(ls[243:284]))
save(D/'SOURCE-CONTEXT-PROVENANCE.json',{'source':pin(p),'first_line':244,'last_line':284,'exact_excerpt':pin(D/'SOURCE-CONTEXT-244-284.k.txt'),'purpose':'Only inspect selected tag/common projections and their paired payload functions for the unused-variable warnings; no runtime/semantic oracle derivation.'})
save(D/'COMPLETE-BYTE-COVERAGE-PROOF.json',{'status':'PASS_LOSSLESS_COMPLETE_PARTITION','original':summary['original'],'retained':summary['retained_copy'],'ordered_segments':len(segments),'sum_segment_bytes':sum(map(len,segments)),'concatenation_sha256':sha256(b''.join(segments)).hexdigest(),'exact_bytes_equal':True,'uncovered_bytes':0,'all_message_templates_matched':True,'all_actual_source_sites_found':True,'all_rendered_code_matches_pinned_source_ignoring_renderer_whitespace':True,'all_noncode_renderer_tail_is_whitespace_or_caret':True,'warning_error_summary_is_not_semantic_approval':True})
print(json.dumps({k:summary[k] for k in ['diagnostics','severity_counts','categories','by_source','unused_variable_counts','distinct_deprecated_label_strings','duplicate_deprecated_label_strings','distinct_whitespace_normalized_messages','uncovered_bytes']},indent=2))
