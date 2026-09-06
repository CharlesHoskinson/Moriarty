#!/usr/bin/env python3
"""K Framework documentation graph. Run the graphify pipeline with agy (Antigravity CLI, gemini-3.8-flash-high) as the
semantic extractor and community labeller instead of Claude subagents."""
import json, subprocess, sys, time, re, glob, os
from pathlib import Path
ROOT = Path('/home/charl/Moriarty/graphs/k-framework'); os.chdir(ROOT)
INPUT = ROOT / 'corpus'; OUT = ROOT / 'graphify-out'
SPEC = Path('/home/charl/.claude/skills/graphify/references/extraction-spec.md')
MODEL = 'gemini-3.8-flash-high'; BUDGET = 90_000
LOG = open(OUT / 'agy-run.log', 'a')
def log(*a):
    s = time.strftime('%H:%M:%S ') + ' '.join(str(x) for x in a); print(s, flush=True); LOG.write(s + '\n'); LOG.flush()

SCHEMA = {"type":"object","properties":{
 "nodes":{"type":"array","items":{"type":"object","properties":{"id":{"type":"string"},"label":{"type":"string"},"file_type":{"type":"string","enum":["code","document","paper","image","rationale","concept"]},"source_file":{"type":"string"},"rationale":{"type":"string"}},"required":["id","label","file_type","source_file"]}},
 "edges":{"type":"array","items":{"type":"object","properties":{"source":{"type":"string"},"target":{"type":"string"},"relation":{"type":"string"},"confidence":{"type":"string","enum":["EXTRACTED","INFERRED","AMBIGUOUS"]},"confidence_score":{"type":"number"},"source_file":{"type":"string"}},"required":["source","target","relation","confidence","confidence_score","source_file"]}},
 "hyperedges":{"type":"array","items":{"type":"object","properties":{"id":{"type":"string"},"label":{"type":"string"},"nodes":{"type":"array","items":{"type":"string"}},"relation":{"type":"string"},"confidence":{"type":"string"},"confidence_score":{"type":"number"},"source_file":{"type":"string"}},"required":["id","label","nodes","relation","confidence","confidence_score","source_file"]}}},
 "required":["nodes","edges","hyperedges"]}
(OUT / 'agy-extract.schema.json').write_text(json.dumps(SCHEMA))
LABEL_SCHEMA = {"type":"object","properties":{"labels":{"type":"array","items":{"type":"object","properties":{"community":{"type":"integer"},"label":{"type":"string"}},"required":["community","label"]}}},"required":["labels"]}
(OUT / 'agy-label.schema.json').write_text(json.dumps(LABEL_SCHEMA))

def agy(prompt, schema_path, tries=3, timeout=900):
    dec = json.JSONDecoder()
    for t in range(tries):
        r = None
        try:
            r = subprocess.run(['agy','--model',MODEL,'--output-format','json','--json-schema',str(schema_path),'--print',prompt],
                               capture_output=True, text=True, timeout=timeout, stdin=subprocess.DEVNULL)
            raw = r.stdout
            env = None
            i = 0
            while i < len(raw):
                j = raw.find('{', i)
                if j < 0: break
                try:
                    obj, end = dec.raw_decode(raw, j)
                    if isinstance(obj, dict) and 'status' in obj and 'response' in obj: env = obj
                    i = end
                except ValueError:
                    i = j + 1
            if env is None: raise ValueError('no agy envelope in stdout')
            if env.get('status') != 'SUCCESS': raise ValueError('agy status ' + str(env.get('status')) + ' ' + str(env.get('response'))[:200])
            body = env.get('structured_output')
            if not isinstance(body, dict):
                resp = env.get('response', '')
                body = json.loads(resp) if isinstance(resp, str) else resp
            usage = env.get('usage', {})
            return body, usage.get('input_tokens', 0), usage.get('output_tokens', 0)
        except Exception as e:
            log(f'  agy attempt {t+1} failed: {type(e).__name__}: {str(e)[:200]} | stdout tail: {(r.stdout[-300:] if r else "")} | stderr: {(r.stderr[-300:] if r else "")}')
            time.sleep(15)
    return None, 0, 0

# ---- extraction prompt (from extraction-spec.md, verbatim block) ----
spec_text = SPEC.read_text(encoding='utf-8')
block = spec_text.split('```')[1]
def extraction_prompt(files_with_content, n, total):
    file_list = '\n'.join(p for p,_ in files_with_content)
    head = block.replace('CHUNK_NUM', str(n)).replace('TOTAL_CHUNKS', str(total)).replace('FILE_LIST', file_list).replace('DEEP_MODE', 'false')
    head = head.split('Then write the JSON to disk')[0]
    body = '\n\n'.join(f'===== FILE: {p} =====\n{c}' for p,c in files_with_content)
    return (head + f"\nThe build root for node-ID stems is {INPUT} (so {INPUT}/docs/user_manual.md has stem docs_user_manual). "
            "The file contents are provided below; do not attempt to read files or use tools. Output only the JSON object.\n\n" + body)

detect = json.loads((OUT/'.graphify_detect.json').read_text(encoding='utf-8'))
# Part A: AST for code files
from graphify.extract import collect_files, extract
code_files = []
for f in detect['files'].get('code', []):
    code_files.extend(collect_files(Path(f)) if Path(f).is_dir() else [Path(f)])
if code_files:
    ast = extract(code_files, cache_root=INPUT)
else:
    ast = {'nodes':[],'edges':[],'input_tokens':0,'output_tokens':0}
(OUT/'.graphify_ast.json').write_text(json.dumps(ast, indent=2, ensure_ascii=False), encoding='utf-8')
log(f"AST: {len(ast['nodes'])} nodes, {len(ast['edges'])} edges from {len(code_files)} code files")

# B0 cache
from graphify.cache import check_semantic_cache, save_semantic_cache
all_files = [f for cat in ('document','paper','image') for f in detect['files'].get(cat, [])]
cn, ce, ch, uncached = check_semantic_cache(all_files, root=str(INPUT), prompt_file=str(SPEC))
log(f'Cache: {len(all_files)-len(uncached)} hit, {len(uncached)} to extract')
cached = {'nodes':cn,'edges':ce,'hyperedges':ch}

# chunk by byte budget, splitting oversized files into segments
units = []  # (path, content_segment)
for f in uncached:
    txt = Path(f).read_text(encoding='utf-8', errors='replace')
    if len(txt.encode()) <= BUDGET:
        units.append((f, txt))
    else:
        parts, cur = [], ''
        for para in txt.split('\n\n'):
            if len((cur+para).encode()) > BUDGET - 2000 and cur:
                parts.append(cur); cur = ''
            cur += para + '\n\n'
        if cur: parts.append(cur)
        for i,p in enumerate(parts):
            units.append((f, f'[segment {i+1} of {len(parts)} of this file]\n' + p))
chunks, cur, size = [], [], 0
for u in units:
    b = len(u[1].encode())
    if cur and (size + b > BUDGET or len(cur) >= 12):
        chunks.append(cur); cur, size = [], 0
    cur.append(u); size += b
if cur: chunks.append(cur)
log(f'{len(units)} units -> {len(chunks)} chunks')

for old in glob.glob(str(OUT/'.graphify_chunk_*.json')): os.remove(old)
ok = 0
for i, ch_ in enumerate(chunks, 1):
    log(f'chunk {i}/{len(chunks)}: {len(ch_)} units, {sum(len(c.encode()) for _,c in ch_)//1024} KB: ' + ', '.join(sorted({Path(p).name for p,_ in ch_}))[:200])
    body, tin, tout = agy(extraction_prompt(ch_, i, len(chunks)), OUT/'agy-extract.schema.json')
    if body is None:
        log(f'  chunk {i} FAILED after retries'); continue
    body.setdefault('hyperedges', []); body['input_tokens']=tin; body['output_tokens']=tout
    for n in body['nodes']:
        for k in ('source_location','source_url','captured_at','author','contributor'): n.setdefault(k, None)
    for e in body['edges']:
        e.setdefault('source_location', None); e.setdefault('weight', 1.0)
    (OUT/f'.graphify_chunk_{i:02d}.json').write_text(json.dumps(body, indent=1, ensure_ascii=False), encoding='utf-8')
    ok += 1
    log(f'  chunk {i} ok: {len(body["nodes"])} nodes, {len(body["edges"])} edges, {len(body["hyperedges"])} hyperedges, tokens {tin}/{tout}')
log(f'extraction: {ok}/{len(chunks)} chunks succeeded')
if ok < len(chunks)/2:
    log('ABORT: more than half the chunks failed'); sys.exit(1)

# B3 merge chunks, cache, merge with cached
new = {'nodes':[],'edges':[],'hyperedges':[],'input_tokens':0,'output_tokens':0}
for c in sorted(glob.glob(str(OUT/'.graphify_chunk_*.json'))):
    d = json.loads(Path(c).read_text(encoding='utf-8'))
    for k in ('nodes','edges','hyperedges'): new[k] += d.get(k, [])
    new['input_tokens'] += d.get('input_tokens',0); new['output_tokens'] += d.get('output_tokens',0)
saved = save_semantic_cache(new['nodes'], new['edges'], new['hyperedges'], root=str(INPUT), allowed_source_files=uncached, prompt_file=str(SPEC))
seen, ded = set(), []
for n in cached['nodes'] + new['nodes']:
    if n['id'] not in seen: seen.add(n['id']); ded.append(n)
sem = {'nodes':ded,'edges':cached['edges']+new['edges'],'hyperedges':cached['hyperedges']+new['hyperedges'],'input_tokens':new['input_tokens'],'output_tokens':new['output_tokens']}
(OUT/'.graphify_semantic.json').write_text(json.dumps(sem, indent=2, ensure_ascii=False), encoding='utf-8')
log(f'semantic: {len(ded)} nodes, {len(sem["edges"])} edges, cached {saved} files')

# Part C merge AST + semantic
seen = {n['id'] for n in ast['nodes']}; merged_nodes = list(ast['nodes'])
for n in sem['nodes']:
    if n['id'] not in seen: merged_nodes.append(n); seen.add(n['id'])
extraction = {'nodes':merged_nodes,'edges':ast['edges']+sem['edges'],'hyperedges':sem['hyperedges'],'input_tokens':sem['input_tokens'],'output_tokens':sem['output_tokens']}
(OUT/'.graphify_extract.json').write_text(json.dumps(extraction, indent=2, ensure_ascii=False), encoding='utf-8')
log(f'merged: {len(merged_nodes)} nodes, {len(extraction["edges"])} edges')

# Step 4 build
from graphify.build import build_from_json
from graphify.cluster import cluster, score_all
from graphify.analyze import god_nodes, surprising_connections, suggest_questions
from graphify.report import generate
from graphify.export import to_json
G = build_from_json(extraction, root=str(INPUT), directed=False)
if G.number_of_nodes() == 0: log('ERROR: empty graph'); sys.exit(1)
communities = cluster(G); cohesion = score_all(G, communities)
tokens = {'input':extraction['input_tokens'],'output':extraction['output_tokens']}
gods = god_nodes(G); surprises = surprising_connections(G, communities)
labels = {cid: 'Community '+str(cid) for cid in communities}
questions = suggest_questions(G, communities, labels)
if not to_json(G, communities, str(OUT/'graph.json')): log('ERROR: shrink guard refused'); sys.exit(1)
(OUT/'GRAPH_REPORT.md').write_text(generate(G, communities, cohesion, labels, gods, surprises, detect, tokens, str(INPUT), suggested_questions=questions), encoding='utf-8')
analysis = {'communities':{str(k):v for k,v in communities.items()},'cohesion':{str(k):v for k,v in cohesion.items()},'gods':gods,'surprises':surprises,'questions':questions}
(OUT/'.graphify_analysis.json').write_text(json.dumps(analysis, indent=2, ensure_ascii=False), encoding='utf-8')
log(f'Graph: {G.number_of_nodes()} nodes, {G.number_of_edges()} edges, {len(communities)} communities')

# Step 4.5 health
from graphify.diagnostics import diagnose_extraction, format_diagnostic_report
summary = diagnose_extraction(extraction, directed=False, root=str(INPUT))
flags = [f'{summary[k]} {lab}' for k,lab in (('dangling_endpoint_edges','dangling-endpoint edges'),('missing_endpoint_edges','missing-endpoint edges'),('self_loop_edges','self-loop edges'),('directed_same_endpoint_collapsed_edges','collapsed (directed) edges'),('undirected_same_endpoint_collapsed_edges','collapsed (undirected) edges')) if summary.get(k,0)]
health = ('GRAPH HEALTH WARNING: ' + '; '.join(flags) + ' - graph may be incomplete/corrupt.') if flags else 'Graph health: OK (no dangling/missing/collapsed edges).'
log(health); (OUT/'health.txt').write_text(format_diagnostic_report(summary) + '\n' + health + '\n')

# Step 5 labels via agy
lab_in = []
for cid, members in communities.items():
    labs = [G.nodes[m].get('label', m) for m in list(members)[:40]]
    lab_in.append(f'community {cid} ({len(members)} nodes): ' + '; '.join(labs))
prompt = ("You are labelling communities of a knowledge graph built from the K Framework documentation. For each community give a 2 to 5 word plain-language name describing what its nodes have in common (e.g. 'Rule Attributes', 'Haskell Backend Proving'). Output JSON {\"labels\":[{\"community\":<id>,\"label\":\"...\"}]} covering every community id listed, nothing else.\n\n" + '\n'.join(lab_in))
body, tin, tout = agy(prompt, OUT/'agy-label.schema.json')
if body and 'labels' in body:
    labels = {int(x['community']): x['label'] for x in body['labels'] if int(x['community']) in communities}
    for cid in communities: labels.setdefault(cid, 'Community '+str(cid))
    log(f'labels from agy: {len(labels)} (tokens {tin}/{tout})')
else:
    log('label step failed; keeping placeholder labels')
questions = suggest_questions(G, communities, labels)
(OUT/'GRAPH_REPORT.md').write_text(generate(G, communities, cohesion, labels, gods, surprises, detect, tokens, str(INPUT), suggested_questions=questions), encoding='utf-8')
(OUT/'.graphify_labels.json').write_text(json.dumps({str(k):v for k,v in labels.items()}, ensure_ascii=False), encoding='utf-8')
to_json(G, communities, str(OUT/'graph.json'), community_labels=labels)
(OUT/'labels.json').write_text(json.dumps({str(k):v for k,v in labels.items()}, indent=1, ensure_ascii=False))

# Step 6 html
r = subprocess.run(['graphify','export','html'], capture_output=True, text=True, cwd=ROOT); log('export html:', (r.stdout+r.stderr).strip()[-300:])

# Step 9 manifest + cost
from datetime import datetime, timezone
from graphify.detect import save_manifest
from graphify.cli import _stamped_manifest_files
corpus = detect.get('all_files') or detect['files']
mf = _stamped_manifest_files(corpus, extraction, INPUT)
sem_types = ('document','paper','image')
dispatched = {f for t,fl in detect['files'].items() if t in sem_types for f in fl}
stamped = {f for fl in mf.values() for f in fl}
scan = {f for fl in corpus.values() for f in fl}
save_manifest(mf, root=str(INPUT), scan_corpus=scan, clear_semantic=(dispatched-stamped) or None)
cost = {'runs':[],'total_input_tokens':0,'total_output_tokens':0}
if (OUT/'cost.json').exists(): cost = json.loads((OUT/'cost.json').read_text())
cost['runs'].append({'date':datetime.now(timezone.utc).isoformat(),'input_tokens':tokens['input'],'output_tokens':tokens['output'],'files':detect.get('total_files',0),'backend':'agy '+MODEL})
cost['total_input_tokens'] += tokens['input']; cost['total_output_tokens'] += tokens['output']
(OUT/'cost.json').write_text(json.dumps(cost, indent=2))
# keep analysis sidecar for the wiki page; remove the rest as the skill does
for f in ('.graphify_extract.json','.graphify_ast.json','.graphify_semantic.json'): (OUT/f).unlink(missing_ok=True)
for c in glob.glob(str(OUT/'.graphify_chunk_*.json')): os.remove(c)
log('DONE')
