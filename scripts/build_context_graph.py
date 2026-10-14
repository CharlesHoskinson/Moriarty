#!/usr/bin/env python3
"""Build a deterministic, provenance-preserving repository context graph.

Run with the Python interpreter that has Graphify installed, for example:
  /home/charl/.local/share/uv/tools/graphifyy/bin/python \\
    scripts/build_context_graph.py --repo . --out /tmp/moriarty-graph

This producer reads only Git-tracked, non-excluded files. It does not install
dependencies or call an LLM/provider.
"""
from __future__ import annotations
import argparse, ast, hashlib, json, os, platform, re, subprocess, sys
from collections import Counter, defaultdict
from importlib.metadata import PackageNotFoundError, version as distribution_version
from pathlib import Path, PurePosixPath

def parse_args():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', required=True, type=Path, help='Git worktree to inventory and parse')
    parser.add_argument('--out', required=True, type=Path, help='Output directory; may be an ignored cache inside the repo')
    return parser.parse_args()

ARGS = parse_args()
ROOT = ARGS.repo.expanduser().resolve(strict=True)
OUT_REQUESTED = ARGS.out.expanduser()
OUT_ABSOLUTE = OUT_REQUESTED.absolute()
for component in (OUT_ABSOLUTE, *OUT_ABSOLUTE.parents):
    if component.exists() and component.is_symlink():
        raise SystemExit('--out path cannot contain symlinks')
OUT = OUT_REQUESTED.resolve(strict=False)
if OUT == ROOT:
    raise SystemExit('--out must not equal --repo')
if any(part in {'.git', '.worktrees', 'target', '.moriarty-dev'} for part in OUT.relative_to(ROOT).parts) if OUT.is_relative_to(ROOT) else False:
    raise SystemExit('--out cannot be inside a protected repository directory')
if OUT.is_relative_to(ROOT):
    relative_out = OUT.relative_to(ROOT).as_posix().rstrip('/') + '/'
    # An in-repository cache is allowed only when ignored and absent from the
    # tracked index. All graph inputs come from git ls-files, never os.walk.
    ignored = subprocess.run(['git', 'check-ignore', '-q', '--', relative_out.rstrip('/')], cwd=ROOT, check=False).returncode == 0
    if not ignored:
        raise SystemExit('--out inside --repo must be Git-ignored')
    if subprocess.run(['git', 'ls-files', '--error-unmatch', '--', relative_out.rstrip('/')], cwd=ROOT, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode == 0:
        raise SystemExit('--out cannot overlap tracked repository paths')
OUT.mkdir(parents=True, exist_ok=True)
if OUT.is_symlink():
    raise SystemExit('--out must not resolve to a symlink')
from graphify.extract import extract as graphify_extract
from graphify.build import build_from_json
from graphify.cluster import cluster, score_all
from graphify.analyze import god_nodes, surprising_connections, suggest_questions
from graphify.report import generate
from graphify.diagnostics import diagnose_extraction, format_diagnostic_report
from graphify.export import to_json

try:
    GRAPHIFY_DISTRIBUTION_VERSION=distribution_version('graphifyy')
except PackageNotFoundError as exc:
    raise SystemExit('Graphify is importable but its graphifyy distribution version is unavailable') from exc
PYTHON_VERSION=platform.python_version()
RUNTIME_VERSIONS={'python':PYTHON_VERSION,'python_implementation':platform.python_implementation(),'graphify_distribution':'graphifyy','graphify_version':GRAPHIFY_DISTRIBUTION_VERSION}

def atomic_text(path: Path, content: str):
    tmp=path.with_name(path.name+'.tmp')
    tmp.write_text(content,encoding='utf-8')
    os.replace(tmp,path)

def atomic_json(path: Path, value, *, indent=2):
    atomic_text(path,json.dumps(value,ensure_ascii=False,indent=indent))

atomic_json(OUT/'graph-ready.json',{'ready':False,'reason':'graph build in progress'})
commit = subprocess.check_output(['git','rev-parse','HEAD'], cwd=ROOT, text=True).strip()
def verify_clean_tracked_tree():
    for args in (['diff','--quiet','HEAD','--'], ['diff','--cached','--quiet','HEAD','--']):
        result=subprocess.run(['git',*args],cwd=ROOT,check=False)
        if result.returncode != 0:
            raise SystemExit('refusing to graph a dirty tracked-source tree (git diff against HEAD is non-empty)')

verify_clean_tracked_tree()
# Git metadata yields a complete tracked-file inventory and blob object IDs without opening contents.
raw = subprocess.check_output(['git','ls-files','-s','-z'], cwd=ROOT)
tracked = []
for rec in raw.decode('utf-8','surrogateescape').split('\0'):
    if not rec: continue
    meta, path = rec.split('\t',1)
    mode, oid, stage = meta.split()
    tracked.append({'path':path, 'mode':mode, 'blob_oid':oid, 'stage':int(stage)})
if any(rec['stage'] != 0 for rec in tracked):
    raise SystemExit('refusing index with unresolved merge stages')
if OUT.is_relative_to(ROOT):
    output_prefix=OUT.relative_to(ROOT).as_posix().rstrip('/')+'/'
    if any(rec['path']==output_prefix.rstrip('/') or rec['path'].startswith(output_prefix) for rec in tracked):
        raise SystemExit('--out overlaps tracked repository files')

# Any path in these groups gets inventory metadata only. These hold historical captures,
# evidence bundles, generated proof material or protected key/witness data.
archive_roots = ('raw/','.raw/','evidence/','deliverables/','wiki-llm/','experiments/','corpus/','wiki/attachments/historical-evidence/')
never_walk_root = ('.moriarty-dev/',)
protected_re = re.compile(r'(^|[/_.-])(credential|credentials|secret|private[-_ ]?(key|witness)|witness|keyset|prover[-_ ]?key|srs|proving[-_ ]?cache|native[-_ ]?provider)([/_.-]|$)', re.I)
binary_exts = {'.bin','.jpg','.jpeg','.png','.gif','.webp','.pdf','.wasm','.so','.dylib','.dll','.exe','.class','.o','.a','.rlib','.zkey','.pk','.vk','.ptau','.srs','.snap','.raw','.preimage'}

def is_archive(path): return path.startswith(archive_roots)
def is_never_walk(path): return path.startswith(never_walk_root)
def is_protected(path): return bool(protected_re.search(path))
def is_binary_path(path): return Path(path).suffix.lower() in binary_exts

def fid(path): return 'file:' + path

def did(path): return 'dir:' + path

def addnode(nodes, seen, n):
    if n['id'] not in seen:
        seen.add(n['id']); nodes.append(n)

nodes=[]; edges=[]; seen=set()
# All repository directories plus all tracked files. The only source data used here is Git metadata.
dirs=set()
for rec in tracked:
    p=PurePosixPath(rec['path'])
    acc=[]
    for part in p.parts[:-1]:
        acc.append(part); dirs.add('/'.join(acc))
for d in sorted(dirs, key=lambda x:(x.count('/'),x)):
    addnode(nodes,seen,{'id':did(d),'label':PurePosixPath(d).name,'node_type':'directory','source_file':d,'source_location':'directory entry','_origin':'inventory','confidence':'EXTRACTED'})
file_meta={}
for rec in tracked:
    p=rec['path']; cat=('protected_excluded' if is_protected(p) else 'never_walk_excluded' if is_never_walk(p) else 'archival_inventory_only' if is_archive(p) else 'binary_inventory_only' if is_binary_path(p) or rec['mode']=='120000' else 'candidate_content')
    rec['coverage_class']=cat
    # Object ID is exact commit content identity; no protected/binary content is opened.
    node={'id':fid(p),'label':PurePosixPath(p).name,'node_type':'file','file_type':'binary' if is_binary_path(p) else 'document' if Path(p).suffix.lower() in {'.md','.rst','.txt'} else 'code' if Path(p).suffix.lower() in {'.rs','.py','.ts','.tsx','.js','.jsx','.mjs','.mori','.qnt','.rkt'} else 'other','source_file':p,'source_location':'tracked file','git_blob_oid':rec['blob_oid'],'git_mode':rec['mode'],'coverage_class':cat,'confidence':'EXTRACTED','_origin':'inventory'}
    addnode(nodes,seen,node); file_meta[p]=rec
    parent=PurePosixPath(p).parent.as_posix()
    if parent!='.': edges.append({'source':fid(p),'target':did(parent),'relation':'contained_in','confidence':'EXTRACTED','source_file':p,'source_location':'tracked path hierarchy','_origin':'inventory'})

# Deterministic source scope: live implementation, tests, examples, scripts and site; never private/config state.
code_exts={'.py','.rs','.ts','.tsx','.js','.jsx','.mjs'}
code_roots=('crates/','packages/','plugins/','scripts/','tests/','examples/','site/','toolchains/')
code_paths=[Path(r['path']) for r in tracked if r['path'].startswith(code_roots) and Path(r['path']).suffix.lower() in code_exts and r['mode'] != '120000' and not is_protected(r['path']) and not is_archive(r['path']) and not is_never_walk(r['path']) and not is_binary_path(r['path'])]
code_result={'nodes':[],'edges':[],'input_tokens':0,'output_tokens':0}
if code_paths:
    try:
        # Pass absolute paths: Graphify interprets relative input paths against
        # process cwd, which may differ from --repo when refreshing another
        # clean worktree. `root` still makes every exported source_file relative.
        code_result=graphify_extract([ROOT/path for path in code_paths], root=ROOT, cache_root=OUT, parallel=True)
    except (ImportError, RuntimeError) as exc:
        # Keep a deterministic stdlib AST pass for Python when the installed
        # Graphify parser extra is unavailable. Do not silently claim other
        # languages were structurally parsed.
        code_result={'nodes':[],'edges':[],'input_tokens':0,'output_tokens':0,
                     'parser_error':f'{type(exc).__name__}: {exc}'}
        for path in code_paths:
            if path.suffix.lower() != '.py':
                continue
            try:
                source=(ROOT/path).read_text(encoding='utf-8')
                tree=ast.parse(source, filename=str(path))
            except (OSError, UnicodeError, SyntaxError):
                continue
            base=path.as_posix().replace('/','_').replace('.','_').replace('-','_')
            code_result['nodes'].append({'id':base,'label':path.name,'file_type':'code','node_type':'file','source_file':path.as_posix(),'source_location':'L1','confidence':'EXTRACTED','_origin':'python-stdlib-ast'})
            for item in ast.walk(tree):
                if not isinstance(item,(ast.FunctionDef,ast.AsyncFunctionDef,ast.ClassDef)):
                    continue
                nid=base+'_'+item.name
                code_result['nodes'].append({'id':nid,'label':item.name,'file_type':'code','node_type':'class' if isinstance(item,ast.ClassDef) else 'function','source_file':path.as_posix(),'source_location':f'L{item.lineno}','confidence':'EXTRACTED','_origin':'python-stdlib-ast'})
                code_result['edges'].append({'source':nid,'target':base,'relation':'declared_in','confidence':'EXTRACTED','source_file':path.as_posix(),'source_location':f'L{item.lineno}','_origin':'python-stdlib-ast'})

# Graphify can expose absolute source paths in its extraction bookkeeping when
# it receives absolute inputs. Normalize every exported source path back to the
# frozen repository root, and fail closed if an extractor reports outside it.
def portable_path(value):
    if not isinstance(value,str) or not value:
        return value
    path=Path(value)
    if path.is_absolute():
        try: return path.resolve().relative_to(ROOT).as_posix()
        except (ValueError,OSError,RuntimeError):
            raise SystemExit(f'extractor emitted source outside --repo: {value}')
    return value
for key in ('extracted_sources','failed_sources','skipped_sources'):
    if isinstance(code_result.get(key),list):
        code_result[key]=[portable_path(x) for x in code_result[key]]
for collection in ('nodes','edges'):
    for item in code_result.get(collection,[]):
        if 'source_file' in item:
            item['source_file']=portable_path(item['source_file'])

# Keep short fixture/utility files structurally visible when Graphify elects
# not to emit symbols. Python uses the stdlib AST. Rust fallback is lexical and
# is labeled as such; it never asserts that a referenced symbol is defined.
graphify_file_roots={}
for node in code_result.get('nodes',[]):
    sf=node.get('source_file')
    if sf and node.get('file_type')=='code' and node.get('source_location')=='L1' and node.get('label')==Path(sf).name:
        graphify_file_roots.setdefault(sf,node['id'])
graphify_symbol_sources={n.get('source_file') for n in code_result.get('nodes',[]) if n.get('source_file') and graphify_file_roots.get(n.get('source_file')) != n.get('id')}
graphify_no_symbol_sources=set(graphify_file_roots)-graphify_symbol_sources
tracked_paths={r['path'] for r in tracked}
fallback_nodes=[]; fallback_edges=[]; fallback_summary=Counter()
rust_decl=re.compile(r'^\s*(?:(?:pub(?:\([^)]*\))?)\s+)?(?:(?:async|unsafe)\s+)*(fn|struct|enum|trait|type|mod|const|static)\s+([A-Za-z_][A-Za-z0-9_]*)')
rust_use=re.compile(r'^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);')
rust_include=re.compile(r'include_(?:str|bytes)!\s*\(\s*"([^"]+)"\s*\)')
for path in code_paths:
    sf=path.as_posix()
    if sf in graphify_symbol_sources:
        continue
    try:
        source=(ROOT/path).read_text(encoding='utf-8')
    except (OSError,UnicodeError):
        continue
    if path.suffix.lower()=='.py':
        try: tree=ast.parse(source,filename=sf)
        except SyntaxError: continue
        base='fallback_'+sf.replace('/','_').replace('.','_').replace('-','_')
        fallback_nodes.append({'id':base,'label':path.name,'file_type':'code','node_type':'file','source_file':sf,'source_location':'L1','confidence':'EXTRACTED','_origin':'python-stdlib-ast','extraction_source':'Python stdlib AST fallback'})
        for item in ast.walk(tree):
            if isinstance(item,(ast.FunctionDef,ast.AsyncFunctionDef,ast.ClassDef)):
                nid=base+'_'+item.name
                fallback_nodes.append({'id':nid,'label':item.name,'file_type':'code','node_type':'class' if isinstance(item,ast.ClassDef) else 'function','source_file':sf,'source_location':f'L{item.lineno}','confidence':'EXTRACTED','_origin':'python-stdlib-ast','extraction_source':'Python stdlib AST fallback'})
                fallback_edges.append({'source':nid,'target':base,'relation':'declared_in','confidence':'EXTRACTED','source_file':sf,'source_location':f'L{item.lineno}','_origin':'python-stdlib-ast'})
        fallback_edges.append({'source':fid(sf),'target':base,'relation':'parsed_as','confidence':'EXTRACTED','source_file':sf,'source_location':'file-level stdlib AST root','_origin':'python-stdlib-ast'})
        fallback_summary['python_stdlib_ast']+=1
    elif path.suffix.lower()=='.rs':
        base='fallback_'+sf.replace('/','_').replace('.','_').replace('-','_')
        fallback_nodes.append({'id':base,'label':path.name,'file_type':'code','node_type':'file','source_file':sf,'source_location':'L1','confidence':'EXTRACTED','_origin':'rust-lexical-fallback','extraction_source':'Rust declaration/reference lexical fallback; no parse claim'})
        fallback_edges.append({'source':fid(sf),'target':base,'relation':'lexically_scanned_as','confidence':'EXTRACTED','source_file':sf,'source_location':'file-level lexical scan','_origin':'rust-lexical-fallback'})
        for line_no,line in enumerate(source.splitlines(),1):
            decl=rust_decl.match(line)
            if decl:
                kind,name=decl.groups(); nid=f'lexical:{sf}#L{line_no}:{kind}:{name}'
                fallback_nodes.append({'id':nid,'label':name,'node_type':'lexical_declaration','source_file':sf,'source_location':f'L{line_no}','confidence':'EXTRACTED','_origin':'rust-lexical-fallback','extraction_source':'line-oriented Rust declaration regex; syntax not validated'})
                fallback_edges.append({'source':nid,'target':base,'relation':'lexically_declares','confidence':'EXTRACTED','source_file':sf,'source_location':f'L{line_no}','_origin':'rust-lexical-fallback'})
                if kind=='mod':
                    candidates=[(path.parent/(name+'.rs')).as_posix(),(path.parent/name/'mod.rs').as_posix()]
                    target=next((x for x in candidates if x in tracked_paths),None)
                    if target:
                        fallback_edges.append({'source':fid(sf),'target':fid(target),'relation':'lexical_module_reference','confidence':'EXTRACTED','source_file':sf,'source_location':f'L{line_no}','_origin':'rust-lexical-fallback'})
            use=rust_use.match(line)
            if use:
                token=use.group(1).strip(); uid=f'unresolved:rust-use:{sf}#L{line_no}:{token}'
                fallback_nodes.append({'id':uid,'label':token,'node_type':'unresolved_symbol','resolution_status':'unresolved_or_external','source_file':sf,'source_location':f'L{line_no}','confidence':'AMBIGUOUS','_origin':'rust-lexical-fallback','extraction_source':'Rust use path; lexical only; not resolved'})
                fallback_edges.append({'source':fid(sf),'target':uid,'relation':'lexically_references','confidence':'EXTRACTED','source_file':sf,'source_location':f'L{line_no}','_origin':'rust-lexical-fallback'})
            for inc in rust_include.finditer(line):
                target=(path.parent/inc.group(1)).as_posix()
                # Exact path resolution is performed against Git inventory.
                fallback_edges.append({'source':fid(sf),'target':fid(target) if target in tracked_paths else 'unresolved:rust-include:'+sf+'#'+inc.group(1),'relation':'lexical_include_reference','confidence':'EXTRACTED','source_file':sf,'source_location':f'L{line_no}','context':inc.group(1),'_origin':'rust-lexical-fallback'})
                if target not in tracked_paths:
                    fallback_nodes.append({'id':'unresolved:rust-include:'+sf+'#'+inc.group(1),'label':inc.group(1),'node_type':'unresolved_symbol','resolution_status':'unresolved_or_external','source_file':sf,'source_location':f'L{line_no}','confidence':'AMBIGUOUS','_origin':'rust-lexical-fallback'})
        fallback_summary['rust_lexical']+=1
fallback_sources={n.get('source_file') for n in fallback_nodes if n.get('source_file')}
unsupported_no_symbol_sources=sorted(graphify_no_symbol_sources-fallback_sources)
unsupported_no_symbol_extensions=sorted({Path(p).suffix.lower() or '(none)' for p in unsupported_no_symbol_sources})
for n in code_result.get('nodes',[]):
    n.setdefault('confidence','EXTRACTED' if n.get('_origin')=='ast' else 'INFERRED')
    n.setdefault('extraction_source','graphify AST')
    addnode(nodes,seen,n)
for e in code_result.get('edges',[]):
    edges.append(e)
for n in fallback_nodes:
    addnode(nodes,seen,n)
for e in fallback_edges:
    edges.append(e)
ast_file_nodes={}
for n in code_result.get('nodes',[]):
    sf=n.get('source_file')
    if sf and n.get('file_type')=='code' and n.get('source_location')=='L1' and n.get('label')==Path(sf).name:
        ast_file_nodes.setdefault(sf,n['id'])
for sf,nid in ast_file_nodes.items():
    edges.append({'source':fid(sf),'target':nid,'relation':'parsed_as','confidence':'EXTRACTED','source_file':sf,'source_location':'file-level AST root','_origin':'source-file-bridge'})

# Read/structure only current authored docs and operational source docs. Historical evidence,
# captures, logs, generated result bundles, binaries and protected names stay metadata-only.
prose_roots=('concepts/','crates/','packages/','plugins/','scripts/','tests/','examples/','site/','openspec/','docs/','wiki/','.superpowers/')
root_docs={'AGENTS.md','CLAUDE.md','GEMINI.md','README.md','ROADMAP.md','RUST_GUIDELINES.md','WIKI_SCHEMA.md','LICENSE'}
prose_exts={'.md','.rst','.txt'}
exclude_prose_roots=('docs/evidence/','wiki/attachments/historical-evidence/','.superpowers/sdd/')
prose_paths=[]
for r in tracked:
    p=r['path']
    if (p in root_docs or (p.startswith(prose_roots) and Path(p).suffix.lower() in prose_exts)) and r['mode'] != '120000' and not p.startswith(exclude_prose_roots) and not is_protected(p) and not is_archive(p) and not is_never_walk(p) and not is_binary_path(p):
        prose_paths.append(p)
# Heading extraction and explicit Markdown/Obsidian local references; no claims are generated from archives.
mdlink=re.compile(r'!?\[[^\]]*\]\(([^)\s]+)(?:\s+[^)]*)?\)')
wiki_link=re.compile(r'!?\[\[([^\]]+)\]\]')
head_re=re.compile(r'^(#{1,6})\s+(.+?)\s*#*\s*$')
path_mention=re.compile(r'(?<![A-Za-z0-9_.-])((?:crates|packages|plugins|scripts|tests|examples|site|openspec|docs|evidence|deliverables)/[A-Za-z0-9_./-]+)')
path_set=set(r['path'] for r in tracked)
for p in prose_paths:
    try: text=(ROOT/p).read_text(encoding='utf-8', errors='replace')
    except (OSError, UnicodeError): continue
    docid=fid(p); headings=[]
    # Preserve source-grounded prose anchors: one first prose paragraph per
    # Markdown section. These are quoted source excerpts, not model summaries.
    active_target=docid; paragraph=[]; paragraph_line=None; in_fence=False; section_has_excerpt=[False]
    def flush_excerpt():
        if not paragraph or section_has_excerpt[0]: return
        excerpt=' '.join(x.strip() for x in paragraph).strip()
        if len(excerpt) >= 45:
            excerpt=re.sub(r'\s+',' ',excerpt)
            if len(excerpt)>360: excerpt=excerpt[:357].rsplit(' ',1)[0]+'…'
            claim_id=f'excerpt:{p}#L{paragraph_line}'
            addnode(nodes,seen,{'id':claim_id,'label':excerpt,'node_type':'document_excerpt','source_file':p,'source_location':f'L{paragraph_line}','confidence':'EXTRACTED','_origin':'source-excerpt','extraction_source':'first prose paragraph in section'})
            edges.append({'source':claim_id,'target':active_target,'relation':'states','confidence':'EXTRACTED','source_file':p,'source_location':f'L{paragraph_line}','_origin':'source-excerpt'})
            section_has_excerpt[0]=True
    for line_no,line in enumerate(text.splitlines(),1):
        if line.strip().startswith('```') or line.strip().startswith('~~~'):
            flush_excerpt(); paragraph=[]; paragraph_line=None; in_fence=not in_fence; continue
        if in_fence: continue
        hm=head_re.match(line)
        if hm:
            flush_excerpt(); paragraph=[]; paragraph_line=None
            active_target=f'heading:{p}#L{line_no}'; active_line=line_no
            section_has_excerpt[0]=False
        elif not line.strip() or line.lstrip().startswith('|') or line.lstrip().startswith('<!--'):
            flush_excerpt(); paragraph=[]; paragraph_line=None
        else:
            if paragraph_line is None: paragraph_line=line_no
            paragraph.append(line.strip())
    flush_excerpt()
    for line_no,line in enumerate(text.splitlines(),1):
        m=head_re.match(line)
        if m:
            title=m.group(2).strip()
            h_id=f'heading:{p}#L{line_no}'
            addnode(nodes,seen,{'id':h_id,'label':title,'node_type':'heading','source_file':p,'source_location':f'L{line_no}','confidence':'EXTRACTED','_origin':'document-structure','extraction_source':'Markdown heading'})
            edges.append({'source':h_id,'target':docid,'relation':'section_of','confidence':'EXTRACTED','source_file':p,'source_location':f'L{line_no}','_origin':'document-structure'})
            headings.append((line_no,title,h_id))
        for rawref in mdlink.findall(line):
            dest=rawref.split('#',1)[0]
            if not dest or re.match(r'^(https?|mailto|tel):',dest,re.I): continue
            target=(PurePosixPath(p).parent / dest).as_posix()
            target=str(PurePosixPath(target))
            if target in path_set:
                edges.append({'source':docid,'target':fid(target),'relation':'declares_reference','confidence':'EXTRACTED','source_file':p,'source_location':f'L{line_no}','context':rawref,'_origin':'document-link'})
        for rawref in wiki_link.findall(line):
            dest=rawref.split('|',1)[0].split('#',1)[0].strip()
            if not dest: continue
            candidates=[dest+'.md',dest]
            target=next((x for x in candidates if x in path_set),None)
            if target:
                edges.append({'source':docid,'target':fid(target),'relation':'declares_wikilink','confidence':'EXTRACTED','source_file':p,'source_location':f'L{line_no}','context':rawref,'_origin':'document-link'})
        for rawpath in path_mention.findall(line):
            target=rawpath.rstrip('.,;:)`')
            if target in path_set:
                edges.append({'source':docid,'target':fid(target),'relation':'mentions_repository_path','confidence':'EXTRACTED','source_file':p,'source_location':f'L{line_no}','context':target,'_origin':'document-path-reference'})
# Remove duplicate identical edges without collapsing distinct source locations.
edge_seen=set(); dedup_edges=[]
for e in edges:
    k=tuple((f,e.get(f)) for f in ('source','target','relation','source_file','source_location','context'))
    if k not in edge_seen: edge_seen.add(k); dedup_edges.append(e)
edges=dedup_edges

# Preserve unresolved AST/reference endpoints as explicitly unresolved nodes.
# This removes dangling links from the navigable graph without pretending that
# an external or unparsed symbol has a known definition.
raw_node_ids={n['id'] for n in nodes}
unresolved=sorted({endpoint for e in edges for endpoint in (e.get('source'),e.get('target')) if endpoint and endpoint not in raw_node_ids})
raw_diagnostic=diagnose_extraction({'nodes':nodes,'edges':edges,'hyperedges':[]},directed=True,root=ROOT)
for endpoint in unresolved:
    addnode(nodes,seen,{'id':endpoint,'label':endpoint,'node_type':'unresolved_symbol','resolution_status':'unresolved_or_external','source_file':'','source_location':'not resolved in selected source scope','confidence':'AMBIGUOUS','_origin':'unresolved-endpoint'})

# Exact extraction input retained for repeatable build and diagnostics.
extraction={'nodes':nodes,'edges':edges,'hyperedges':[],'input_tokens':0,'output_tokens':0}
atomic_json(OUT/'extraction.json',extraction,indent=None)
atomic_json(OUT/'code-ast.json',{**code_result,'fallback_nodes':fallback_nodes,'fallback_edges':fallback_edges,'fallback_summary':dict(fallback_summary)})
atomic_json(OUT/'inventory.json',{'commit':commit,'tracked_files':tracked})
atomic_text(OUT/'edge-provenance.jsonl',''.join(json.dumps(e,ensure_ascii=False)+'\n' for e in edges))
atomic_json(OUT/'unresolved-endpoints.json',{'count':len(unresolved),'items':[{'id':x,'resolution_status':'unresolved_or_external'} for x in unresolved]})

# Diagnostics on the exact graphify extraction before clustering.
diagnostic=diagnose_extraction(extraction,directed=True,root=ROOT)
diagnostic_bundle={'raw_extraction':raw_diagnostic,'final_extraction':diagnostic,'unresolved_endpoints_materialized':len(unresolved),'edge_provenance_sidecar':'edge-provenance.jsonl','graphify_export_multigraph':False,'collapse_limitation':'Graphify build/export uses a simple DiGraph; distinct raw relations/source locations sharing a directed endpoint pair collapse in graph.json and remain fully available in extraction.json and edge-provenance.jsonl.'}

G=build_from_json(extraction,root=ROOT,directed=True)
if G.number_of_nodes()==0: raise SystemExit('empty graph, refusing export')
# Graphify's semantic ghost cleanup can coalesce tracked-file inventory nodes
# into its AST file-self nodes. Reinsert any removed inventory nodes verbatim so
# graph.json retains all 29,833 tracked-file records as well as the AST graph.
inventory_file_nodes=[n for n in extraction['nodes'] if n.get('id','').startswith('file:')]
materialized_inventory_nodes=0
for n in inventory_file_nodes:
    if n['id'] in G: continue
    attrs={k:v for k,v in n.items() if k!='id'}
    G.add_node(n['id'],**attrs)
    materialized_inventory_nodes+=1
    sf=n.get('source_file','')
    parent=PurePosixPath(sf).parent.as_posix()
    if parent!='.' and did(parent) in G:
        G.add_edge(n['id'],did(parent),relation='contained_in',confidence='EXTRACTED',source_file=sf,source_location='tracked path hierarchy',_origin='inventory')
    if sf in ast_file_nodes and ast_file_nodes[sf] in G:
        G.add_edge(n['id'],ast_file_nodes[sf],relation='parsed_as',confidence='EXTRACTED',source_file=sf,source_location='file-level AST root',_origin='source-file-bridge')
diagnostic_bundle['inventory_nodes_reinserted_after_graphify_normalization']=materialized_inventory_nodes
diagnostic_bundle['graph_export_node_count_after_inventory_reinsertion']=G.number_of_nodes()
diagnostic_bundle['graph_export_edge_count_after_inventory_reinsertion']=G.number_of_edges()
atomic_json(OUT/'graph-diagnostics.json',diagnostic_bundle)
comms=cluster(G)
cohesion=score_all(G,comms)
labels={cid:f'Community {cid}' for cid in comms}
gods=god_nodes(G)
surprises=surprising_connections(G,comms)
questions=suggest_questions(G,comms,labels)
wrote=to_json(G,comms,str(OUT/'graph.json.tmp'),built_at_commit=commit)
if not wrote: raise SystemExit('graphify export shrink guard refused graph.json')
os.replace(OUT/'graph.json.tmp',OUT/'graph.json')
# Curate plain labels based on top node labels by community.
for cid,members in comms.items():
    c=Counter()
    for nid in members:
        nd=G.nodes[nid]
        c[nd.get('label',nid)]+=1
    top=' '.join(x for x,_ in c.most_common(3))
    if any(x in top.lower() for x in ('loan','swap','repay','ledger','actus','defi')): name='Financial Language and Ledger'
    elif any(x in top.lower() for x in ('proof','native','recursive','verifier','prover')): name='Proof and Native Backend'
    elif any(x in top.lower() for x in ('spec','sprint','mc01','mc02','mc03','mc04','mc05','mc06','mc07','mc08')): name='Roadmap and Acceptance Specs'
    elif any(x in top.lower() for x in ('wiki','research','source','claim')): name='Research and Wiki'
    elif any(x in top.lower() for x in ('site','guide','tutorial','editor')): name='Developer Experience'
    else: name='Repository Structure'
    labels[cid]=name
atomic_json(OUT/'community-labels.json',{str(k):v for k,v in labels.items()})
# Reexport with labels as a separate overwrite from the same extraction.
to_json(G,comms,str(OUT/'graph.json.tmp'),community_labels=labels,built_at_commit=commit)
os.replace(OUT/'graph.json.tmp',OUT/'graph.json')
report=generate(G,comms,cohesion,labels,gods,surprises,
                {'total_files':len(tracked),'total_words':0,'warning':'inventory spans archived and non-text artifacts; word count was not computed'},
                {'input':0,'output':0},'.',suggested_questions=questions)
parsed_source_files=graphify_symbol_sources
graphify_no_symbol_sources=set(graphify_file_roots)-parsed_source_files
content_read_paths=set(str(p) for p in code_paths)|set(prose_paths)
excerpt_count=sum(1 for n in nodes if n.get('node_type')=='document_excerpt')
ext_counts=Counter(Path(r['path']).suffix.lower() or '(none)' for r in tracked)
# Add provenance and coverage header, preserving graphify report's analysis sections.
atomic_text(OUT/'GRAPH_REPORT.md',
    '# Moriarty repository graph\n\n'
    f'- Commit: `{commit}`\n- Tracked files inventoried: {len(tracked):,}\n'
    f'- Graph: {G.number_of_nodes():,} nodes; {G.number_of_edges():,} edges; {len(comms):,} communities\n'
    f'- Structural code files passed to Graphify AST: {len(code_paths):,}\n'
    f'- Graphify emitted declaration/reference symbols for {len(parsed_source_files):,} code files; {len(graphify_no_symbol_sources):,} files had no symbol nodes.\n'
    f'- Explicit source fallbacks apply only to no-symbol `.py`/`.rs` files: {dict(fallback_summary)}; {len(unsupported_no_symbol_sources)} other no-symbol files ({", ".join(unsupported_no_symbol_extensions) or "none"}) remain without fallback. Rust fallback is lexical, not a parse claim.\n'
    f'- Runtime versions: Python {PYTHON_VERSION}; graphifyy {GRAPHIFY_DISTRIBUTION_VERSION}.\n'
    f'- Current authored prose files read for Markdown headings/references: {len(prose_paths):,}\n'
    f'- Source excerpts: {excerpt_count:,}; unresolved/external symbols represented explicitly: {len(unresolved):,}\n'
    f'- `.mori` language files: {ext_counts.get(".mori",0):,}; `.qnt` formal models: {ext_counts.get(".qnt",0):,} (inventory-only)\n'
    '- Prose anchors are source excerpts (first paragraph per Markdown section), with headings and declared links; no model-generated paraphrases. Archived evidence/capture trees are inventory-only.\n'
    '- Provider activity: deterministic pass-0 extraction made no LLM/provider calls; Luna author usage is unmeasured here.\n\n'
    + report)
# Coverage manifest with exact content-read sets and explicit boundaries.
classes=Counter(r['coverage_class'] for r in tracked)
coverage={'commit':commit,'tracked_file_count':len(tracked),'coverage_classes':dict(classes),'inventory_complete':True,'runtime_versions':RUNTIME_VERSIONS,
 'inventory_source':'git ls-files -s at exact commit; each entry includes Git blob object ID (content digest identity), mode and stage',
 'content_reads':{'graphify_ast_attempted':len(code_paths),'graphify_ast_source_files_with_output_nodes':len(graphify_file_roots),'graphify_ast_source_files_with_symbols':len(parsed_source_files),'graphify_ast_no_symbol_files':sorted(graphify_no_symbol_sources),'markdown_structure_and_excerpts':len(prose_paths),'source_excerpt_nodes':excerpt_count,'inventory_only':len(tracked)-len(content_read_paths),
 'archive_metadata_only':classes.get('archival_inventory_only',0),'protected_excluded':classes.get('protected_excluded',0),'never_walk_excluded':classes.get('never_walk_excluded',0),'binary_metadata_only':classes.get('binary_inventory_only',0)},
 'code_scope':f'tracked live code under crates, packages, plugins, scripts, tests, examples, site, toolchains with .py/.rs/.ts/.tsx/.js/.jsx/.mjs extensions; Graphify {GRAPHIFY_DISTRIBUTION_VERSION} extraction ran under Python {PYTHON_VERSION}. In the no-symbol set, fallback applies only to .py (stdlib AST) and .rs (lexical declaration/reference scan). {len(unsupported_no_symbol_sources)} other no-symbol source files ({", ".join(unsupported_no_symbol_extensions) or "none"}) remain without fallback and make no symbol-extraction claim. .mori and .qnt are inventoried but not AST-parsed.',
 'language_source_inventory':{'.mori':ext_counts.get('.mori',0),'.qnt':ext_counts.get('.qnt',0),'ast_parsed':False},
 'prose_scope':'current authored docs under root, concepts, crates, packages, plugins, scripts, tests, examples, site, openspec, docs, wiki, .superpowers; historical and evidence-only subtrees excluded from content reads',
 'excluded_patterns':['raw/**','.raw/**','evidence/**','deliverables/**','wiki-llm/**','experiments/**','corpus/**','wiki/attachments/historical-evidence/**','.moriarty-dev/**','path names matching credential/private key/witness/keyset/prover key/SRS/proving cache/native provider','binary extensions and symlink targets'],
 'per_file_digest':'Git blob OID records exact committed blob identity; no excluded content was opened for a second digest.',
 'semantic_depth':'All repository files have file-level metadata nodes. Current prose files were content-read and yield heading nodes, first-paragraph source excerpts per section, local Markdown/Obsidian links, and direct repository-path mentions. These are extracted text anchors, not model-generated semantic summaries. Archived material is inventory-only. AST/function/type/call extraction is limited to supported live code files.',
 'provider_activity':{'llm_provider_calls':0,'luna_author_usage':'unmeasured_here','semantic_model_paraphrases':0},
 'source_fallbacks':dict(fallback_summary),'no_symbol_files_without_fallback':unsupported_no_symbol_sources,
 'limitations':['No paragraph-level semantic pass across archived trees.','Graphify diagnostics report dangling endpoints, self-loops and same-endpoint edge collapse; consult graph-diagnostics.json.','No HTML export due graph size >5,000 nodes.']}
atomic_json(OUT/'coverage.json',coverage)
# machine summary
summary={'commit':commit,'tracked':len(tracked),'nodes':G.number_of_nodes(),'edges':G.number_of_edges(),'communities':len(comms),'inventory_file_nodes':len(inventory_file_nodes),'inventory_nodes_reinserted':materialized_inventory_nodes,'code_selected':len(code_paths),'code_ast_source_files':len(parsed_source_files),'code_ast_source_files_with_output_nodes':len(graphify_file_roots),'code_ast_no_symbol_sources':sorted(graphify_no_symbol_sources),'code_ast_nodes':len(code_result.get('nodes',[])),'code_ast_edges':len(code_result.get('edges',[])),'source_fallbacks':dict(fallback_summary),'no_symbol_files_without_fallback':unsupported_no_symbol_sources,'runtime_versions':RUNTIME_VERSIONS,'prose_read':len(prose_paths),'source_excerpt_nodes':excerpt_count,'language_source_inventory':{'.mori':ext_counts.get('.mori',0),'.qnt':ext_counts.get('.qnt',0),'ast_parsed':False},'coverage_classes':dict(classes),'diagnostics':diagnostic}
atomic_json(OUT/'build-summary.json',summary)

# Bounded query examples are generated from edges actually present in this
# extraction. They do not assume a pre-existing sample file or invent paths.
sample_edges=[]
for edge in edges:
    if edge.get('relation') not in {'declares_reference','declares_wikilink','mentions_repository_path'}:
        continue
    source=edge.get('source_file','')
    if edge.get('relation')=='mentions_repository_path' and (source=='ROADMAP.md' or source.startswith(('docs/','openspec/','concepts/'))):
        sample_edges.append(edge)
sample_edges.sort(key=lambda e:(e.get('source_file',''),e.get('source_location',''),e.get('target',''),e.get('relation','')))
priority_sources=('ROADMAP.md','docs/superpowers/plans/2026-10-11-bound-replay.md','docs/NATIVE-CORE.md')
prioritized=[e for source in priority_sources for e in sample_edges if e.get('source_file')==source]
selected_direct=[]; selected_keys=set()
for edge in prioritized+sample_edges:
    key=(edge.get('source'),edge.get('target'),edge.get('relation'),edge.get('source_location'))
    if key in selected_keys: continue
    selected_keys.add(key);selected_direct.append(edge)
    if len(selected_direct)>=24: break
samples=[]
for edge in selected_direct:
    samples.append({'source':'file:'+edge['source_file'],'target':edge['target'],'relation':edge['relation'],
                    'source_file':edge['source_file'],'source_location':edge.get('source_location'),
                    'context':edge.get('context'),'confidence':'EXTRACTED','query_kind':'direct_source_reference'})
code_refs=[e for e in edges if e.get('relation')=='mentions_repository_path' and
           e.get('target','').startswith('file:') and
           e.get('target','')[5:].startswith(('crates/','packages/','plugins/','scripts/','tests/','examples/','site/'))]
code_refs.sort(key=lambda e:(e.get('source_file',''),e.get('source_location',''),e.get('target','')))
for edge in code_refs[:24]:
    samples.append({'source':'file:'+edge['source_file'],'target':edge['target'],'relation':edge['relation'],
                    'source_file':edge['source_file'],'source_location':edge.get('source_location'),
                    'context':edge.get('context'),'confidence':'EXTRACTED','query_kind':'roadmap_or_doc_to_code_or_test'})
sample_query={'commit':commit,'generation':'deterministic selection from exact extracted edge provenance; bounded to 48 records',
              'queries':samples[:48],
              'limitations':'Each record is a direct extracted edge, not a claimed transitive path. An absent edge is not evidence of semantic independence.'}
atomic_json(OUT/'sample-queries.json',sample_query)

# Ensure the checked-out commit, index entries, and tracked worktree remain
# exactly the snapshot used for extraction before publishing ready=true.
verify_clean_tracked_tree()
commit_after=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
raw_after=subprocess.check_output(['git','ls-files','-s','-z'],cwd=ROOT)
if commit_after != commit or raw_after != raw:
    raise SystemExit('source commit/index changed during graph build; refusing ready=true')
ready_files=['graph.json','GRAPH_REPORT.md','coverage.json','graph-diagnostics.json','inventory.json','extraction.json','edge-provenance.jsonl','unresolved-endpoints.json','code-ast.json','community-labels.json','build-summary.json','sample-queries.json']
ready={'commit':commit,'ready':True,'files':{name:{'sha256':hashlib.sha256((OUT/name).read_bytes()).hexdigest(),'bytes':(OUT/name).stat().st_size} for name in ready_files}}
atomic_json(OUT/'graph-ready.json',ready)
print(json.dumps(summary,ensure_ascii=False,indent=2))
print(format_diagnostic_report(diagnostic))
