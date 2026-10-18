import subprocess,json,datetime,hashlib,concurrent.futures,base64
from pathlib import Path
D=Path(__file__).parent; C=D/"captures"; manifest=[]
def capture(name,args):
 start=datetime.datetime.now(datetime.timezone.utc).isoformat(); p=subprocess.run(["gh"]+args,capture_output=True); end=datetime.datetime.now(datetime.timezone.utc).isoformat()
 if p.returncode: raise RuntimeError(name+":"+p.stderr.decode())
 f=C/name; f.write_bytes(p.stdout); rec={"path":str(f.relative_to(D)),"command":["gh"]+args,"startedAt":start,"capturedAt":end,"sha256":hashlib.sha256(p.stdout).hexdigest()}; manifest.append(rec); return json.loads(p.stdout) if not name.endswith("jsonl") else None
repo="CharlesHoskinson/Moriarty"
with concurrent.futures.ThreadPoolExecutor(max_workers=5) as e:
 jobs={"main.json":["api",f"repos/{repo}/git/ref/heads/main"],"utxo.json":["api",f"repos/{repo}/git/ref/heads/utxo"],"open-prs.json":["pr","list","--repo",repo,"--state","open","--limit","1000","--json","number,title,body,url,state,updatedAt,isDraft,headRefName,headRefOid,baseRefName,baseRefOid,mergeStateStatus"],"open-issues.json":["issue","list","--repo",repo,"--state","open","--limit","1000","--json","number,title,body,url,state,updatedAt,labels"]}
 results=dict(zip(jobs,e.map(lambda z:capture(*z),jobs.items())))
nums=sorted(set([p["number"] for p in results["open-prs.json"]]+[227,229,233]))
jobs={}
for n in nums:
 jobs[f"pr-{n}.json"]=["pr","view",str(n),"--repo",repo,"--json","number,title,body,url,state,updatedAt,isDraft,headRefName,headRefOid,baseRefName,baseRefOid,mergeCommit,mergedAt,mergeable,mergeStateStatus,commits,reviews"]
 jobs[f"pr-{n}-files.jsonl"]=["api","--paginate",f"repos/{repo}/pulls/{n}/files?per_page=100","--jq",".[]|{filename,status,sha,changes,additions,deletions,previous_filename}"]
 jobs[f"pr-{n}-commits.jsonl"]=["api","--paginate",f"repos/{repo}/pulls/{n}/commits?per_page=100","--jq",".[]|{sha,tree:.commit.tree.sha,parents:[.parents[].sha],message:.commit.message}"]
 jobs[f"pr-{n}-rest.json"]=["api",f"repos/{repo}/pulls/{n}","--jq","{number,state,draft,changed_files,head:{sha:.head.sha},base:{sha:.base.sha}}"]
with concurrent.futures.ThreadPoolExecutor(max_workers=8) as e: list(e.map(lambda z:capture(*z),jobs.items()))
utxo=results["utxo.json"]["object"]["sha"]
x=capture("canonical-tracker-contents.json",["api",f"repos/{repo}/contents/docs/utxo/doctorwatson/github/work-items.json?ref={utxo}"])
(D/"work-items-before.json").write_bytes(base64.b64decode(x["content"]))
merge=json.loads((C/"pr-229.json").read_text())["mergeCommit"]["oid"]
capture("pr-229-merge-commit.json",["api",f"repos/{repo}/commits/{merge}","--jq","{sha,message:.commit.message,tree:.commit.tree.sha,parents:[.parents[].sha]}"])
merge227=json.loads((C/"pr-227.json").read_text())["mergeCommit"]["oid"]
capture("pr-227-merge-commit.json",["api",f"repos/{repo}/commits/{merge227}","--jq","{sha,message:.commit.message,tree:.commit.tree.sha,parents:[.parents[].sha]}"])
merge233=json.loads((C/"pr-233.json").read_text())["mergeCommit"]["oid"]
capture("pr-233-merge-commit.json",["api",f"repos/{repo}/commits/{merge233}","--jq","{sha,message:.commit.message,tree:.commit.tree.sha,parents:[.parents[].sha]}"])
(D/"CAPTURE-MANIFEST.json").write_text(json.dumps(sorted(manifest,key=lambda x:x["path"]),indent=2)+"\n")
print(json.dumps({"main":results["main.json"]["object"]["sha"],"utxo":utxo,"openPRs":[(p["number"],p["headRefOid"],p["isDraft"]) for p in results["open-prs.json"]],"issues":[p["number"] for p in results["open-issues.json"]],"trackerCount":json.loads((D/"work-items-before.json").read_text())["counts"]},indent=2))
