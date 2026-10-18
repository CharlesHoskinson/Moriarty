import subprocess,json,datetime,hashlib,concurrent.futures,base64
from pathlib import Path
D=Path(__file__).parent;repo="CharlesHoskinson/Moriarty";manifest=[]
def cap(job):
 name,args=job;start=datetime.datetime.now(datetime.timezone.utc).isoformat();r=subprocess.run(["gh"]+args,capture_output=True,check=True);p=D/"captures"/name;p.write_bytes(r.stdout);manifest.append({"path":str(p.relative_to(D)),"command":["gh"]+args,"startedAt":start,"capturedAt":datetime.datetime.now(datetime.timezone.utc).isoformat(),"sha256":hashlib.sha256(r.stdout).hexdigest()});return name,json.loads(r.stdout) if not name.endswith("jsonl") else None
jobs=[("main.json",["api",f"repos/{repo}/git/ref/heads/main"]),("utxo.json",["api",f"repos/{repo}/git/ref/heads/utxo"]),("open-prs.json",["pr","list","--repo",repo,"--state","open","--limit","1000","--json","number,title,state,isDraft,headRefOid,headRefName,baseRefOid,baseRefName,url"])]
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as e:initial=dict(e.map(cap,jobs))
match=[x for x in initial["open-prs.json"] if x["headRefOid"]=="829f4456dfda8d4fc5a4ece84c47f2539f846d9f"]
nums=[249,250]+[x["number"] for x in match];jobs=[]
for n in nums+[248]:
 jobs.append((f"pr-{n}.json",["pr","view",str(n),"--repo",repo,"--json","number,title,body,url,state,updatedAt,isDraft,headRefName,headRefOid,baseRefName,baseRefOid,mergeCommit,mergedAt"]))
 if n!=248:
  jobs += [(f"pr-{n}-files.jsonl",["api","--paginate",f"repos/{repo}/pulls/{n}/files?per_page=100","--jq",".[]|{filename,status,sha,changes,additions,deletions,previous_filename}"]),(f"pr-{n}-commits.jsonl",["api","--paginate",f"repos/{repo}/pulls/{n}/commits?per_page=100","--jq",".[]|{sha,tree:.commit.tree.sha,parents:[.parents[].sha],message:.commit.message}"]),(f"pr-{n}-rest.json",["api",f"repos/{repo}/pulls/{n}","--jq","{number,state,draft,changed_files,head:{sha:.head.sha},base:{sha:.base.sha}}"])]
with concurrent.futures.ThreadPoolExecutor(max_workers=8) as e:results=dict(e.map(cap,jobs))
utxo=initial["utxo.json"]["object"]["sha"];_,canonical=cap(("canonical-tracker-contents.json",["api",f"repos/{repo}/contents/docs/utxo/doctorwatson/github/work-items.json?ref={utxo}"]));(D/"work-items-before-37.json").write_bytes(base64.b64decode(canonical["content"]));(D/"CAPTURE-MANIFEST.json").write_text(json.dumps(manifest,indent=2)+"\n");print(json.dumps({"main":initial["main.json"]["object"]["sha"],"utxo":utxo,"actualIntegrationMatches":match,"newPRs":nums,"canonicalCounts":json.loads((D/"work-items-before-37.json").read_text())["counts"]},indent=2))
