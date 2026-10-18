"""Read-only Git source/path validation; no product imports or execution."""
import argparse, hashlib, json, pathlib, re, subprocess
ap=argparse.ArgumentParser(); ap.add_argument("--repo",required=True); args=ap.parse_args()
r=pathlib.Path(__file__).resolve().parent; x=json.loads((r/"RESULT.json").read_text()); b=x["base_commit"]
def git(*parts): return subprocess.check_output(["git","-C",args.repo,*parts])
for pin in x["source_pins"]:
    assert hashlib.sha256(git("show",b+":"+pin["path"])).hexdigest()==pin["sha256"]
    assert git("rev-parse",b+":"+pin["path"]).decode().strip()==pin["git_blob"]
for link in x["local_link_results"]:
    git("cat-file","-e",b+":"+link["resolved"])
    if link["fragment"]:
        body=git("show",b+":"+link["resolved"]).decode()
        slugs=[re.sub(r"[^\w\- ]","",h.lower()).replace(" ","-") for h in re.findall(r"^#+\s+(.+)$",body,re.M)]
        assert link["fragment"] in slugs or ('id="'+link["fragment"]+'"') in body or ('name="'+link["fragment"]+'"') in body
fixture=git("show",b+":crates/mori/tests/source6_expand.rs").decode()
aliases={name:"crates/mori/tests/fixtures/source6-expansion/"+member for name,member in re.findall(r'root\.join\("([^"]+)"\),\s*fixture!\("([^"]+)"\)',fixture)}
assert aliases==x["fixture_aliases"]
for target in aliases.values(): git("cat-file","-e",b+":"+target)
for path in x["catalogue_source_paths_checked"]: git("cat-file","-e",b+":"+path)
print(json.dumps({"result":"NO_PATCH","validation":"PASS_OFFLINE_SOURCE_PATH_LITERAL","base":b,"pinned_files":len(x["source_pins"]),"local_links":len(x["local_link_results"]),"product_runtime":"UNRUN"},indent=2))
