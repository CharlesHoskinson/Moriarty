#!/usr/bin/env python3
"""Structural lint for an LLM wiki (llm-wiki/). No LLM calls.

From the workflowskills wiki-llm skill. Raw sources may live anywhere under the
home folder, so every source path is written as `~/...` and expanded here.

Usage:
  python3 llm-wiki/tools/lint.py                 check the wiki; exit 1 on any failure
  python3 llm-wiki/tools/lint.py --write-index   rebuild wiki/index.md first, then check
  python3 llm-wiki/tools/lint.py --json          print the report as JSON

Checks (failures): a page missing from index.md, an index entry with no file,
a broken [[wikilink]] or relative .md link, a page without complete front matter,
a source page whose source_file does not exist, a SOURCES.tsv row whose path
does not exist (columns path, type, date, note), an em or en dash (or a spaced
hyphen used as a dash), any email address, a source page with no log entry, an
empty page, a log entry that breaks the rolling log format (header, 1 to 4
bullets, time order), an absolute home path or a Windows drive mount on any page
or in CLAUDE.md (write `~/` instead), and a status page (WorkstreamStatus,
overview) whose last_updated is older than the newest log entry. Orphans (pages
nothing links to) are reported as warnings.
"""
import json, os, re, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))   # llm-wiki/
REPO = os.path.dirname(ROOT)                                          # repository root
WIKI = os.path.join(ROOT, "wiki")
SOURCES_TSV = os.path.join(ROOT, "raw", "SOURCES.tsv")
SECTIONS = ("sources", "entities", "concepts", "syntheses")
REQUIRED = ("title", "type", "tags", "sources", "last_updated")
SOURCE_REQUIRED = ("date", "source_file", "source_url")
WIKILINK = re.compile(r"\[\[([^\]|#]+)(?:[|#][^\]]*)?\]\]")
MDLINK = re.compile(r"\]\(([^)#\s]+\.md)(?:#[^)]*)?\)")
DASHES = re.compile("[—–]")
SPACED_HYPHEN = re.compile(r"\S (-{1,2}) \S")
TABLE_RULE = re.compile(r"^\s*\|?\s*:?-{3,}")
EMAIL = re.compile(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}")
EMAIL_ALLOW = set()   # no email address belongs in this wiki
LOG_HEADER = re.compile(r"^## \[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] ([a-z-]+) \| (\S.*)$")
LOG_OPS = {"commit", "push", "lane-start", "lane-done", "audit", "qa", "decision", "issue",
           "incident", "plan", "build", "queue", "note", "ingest", "lint"}
# Absolute home paths and Windows mounts; pages write `~/...` instead. The log is exempt (append-only history).
STALE_PATH = re.compile(r"/home/[a-z_][a-z0-9_-]*/|/mnt/c/")
TSV_TYPES = {"report", "brief", "lane-report", "take", "design", "open-questions", "ideas", "plan",
             "notes", "index", "issue-draft", "readme", "pins", "log", "ruling", "profile", "spec",
             "graph", "roadmap", "memory"}


def resolve(path):
    """Source paths are written from the home folder (`~/...`); a bare path is taken from the wiki's repository."""
    path = path.strip()
    return os.path.expanduser(path) if path.startswith("~") else os.path.join(REPO, path)
# Pages that must be at least as new as the newest log entry (concepts/WikiUpkeep.md).
STATUS_PAGES = {"WorkstreamStatus": "syntheses/WorkstreamStatus.md", "overview": "overview.md"}
# Prose files outside wiki/ that the dash and email rules also cover.
EXTRA_PROSE = [os.path.join(ROOT, "CLAUDE.md")]


def pages():
    out = {}
    for d in SECTIONS:
        p = os.path.join(WIKI, d)
        for f in sorted(os.listdir(p)) if os.path.isdir(p) else []:
            if f.endswith(".md"):
                out[f[:-3]] = f"{d}/{f}"
    return out


def front_matter(text):
    m = re.match(r"^---\n(.*?)\n---\n", text, re.S)
    if not m:
        return None, text
    fm = {}
    for line in m.group(1).splitlines():
        if ":" in line and not line.startswith(" "):
            k, v = line.split(":", 1)
            fm[k.strip()] = v.strip().strip('"')
    return fm, text[m.end():]


def summary(body):
    for para in re.split(r"\n\s*\n", body):
        para = "\n".join(l for l in para.strip().splitlines() if not l.startswith("#")).strip()
        if not para or para.startswith(("#", "|", ">", "```")) or re.match(r"\d+\.", para):
            continue
        para = re.sub(r"^- ", "", para)
        para = WIKILINK.sub(lambda m: m.group(1), para).replace("**", "")
        para = " ".join(para.split())
        s = re.split(r"(?<=[.!?])\s", para)[0]
        return s if len(s) <= 180 else s[:177].rsplit(" ", 1)[0] + "..."
    return ""


def write_index(allp):
    titles = {"sources": "Sources", "entities": "Entities", "concepts": "Concepts", "syntheses": "Syntheses"}
    lines = ["# Wiki Index", "",
             "Every page in the wiki. Rebuild with `python3 llm-wiki/tools/lint.py --write-index` after each ingest.", "",
             "## Overview", "",
             "- [Overview](overview.md): where the program is, in ten lines, with links",
             "- [Log](log.md): append-only rolling log of every finished workstream step", ""]
    for d in SECTIONS:
        group = [(n, r) for n, r in allp.items() if r.startswith(d + "/")]
        lines += [f"## {titles[d]} ({len(group)})", ""]
        for name, rel in sorted(group, key=lambda x: x[0].lower()):
            fm, body = front_matter(open(os.path.join(WIKI, rel), encoding="utf-8").read())
            title = (fm or {}).get("title", name)
            s = summary(body)
            lines.append(f"- [{title}]({rel})" + (f": {s}" if s else ""))
        lines.append("")
    open(os.path.join(WIKI, "index.md"), "w", encoding="utf-8").write("\n".join(lines))


def prose_hits(path, label):
    hits, in_code = [], False
    for i, line in enumerate(open(path, encoding="utf-8"), 1):
        for addr in EMAIL.findall(line):
            if addr.lower() not in EMAIL_ALLOW:
                hits.append(("email", f"{label}:{i}: email address {addr[:3]}...(redacted)"))
        if line.lstrip().startswith("```"):
            in_code = not in_code
            continue
        if in_code:
            continue
        if DASHES.search(line):
            hits.append(("dashes", f"{label}:{i}: em or en dash"))
        elif not TABLE_RULE.match(line):
            prose = re.sub(r"`[^`]*`", "", line)
            if SPACED_HYPHEN.search(prose):
                hits.append(("dashes", f"{label}:{i}: spaced hyphen used as a dash"))
    return hits


def stale_hits(path, label):
    return [f"{label}:{i}: absolute home path or drive mount (write ~/ instead)" for i, line in enumerate(open(path, encoding="utf-8"), 1)
            if STALE_PATH.search(line)]


def check_log(text):
    """The rolling log rule: exact header, known op, 1 to 4 bullets, time order."""
    problems, entries = [], []
    lines = text.splitlines()
    starts = [i for i, l in enumerate(lines) if l.startswith("## ")]
    for n, i in enumerate(starts):
        head = lines[i]
        m = LOG_HEADER.match(head)
        if not m:
            problems.append(f"log.md:{i + 1}: bad header: {head[:70]}")
            continue
        stamp, op, _ = m.groups()
        if op not in LOG_OPS:
            problems.append(f"log.md:{i + 1}: unknown op '{op}'")
        end = starts[n + 1] if n + 1 < len(starts) else len(lines)
        body = [l for l in lines[i + 1:end] if l.strip()]
        bullets = [l for l in body if l.startswith("- ")]
        if len(bullets) != len(body):
            problems.append(f"log.md:{i + 1}: entry body must be bullets only")
        if not 1 <= len(bullets) <= 4:
            problems.append(f"log.md:{i + 1}: {len(bullets)} bullets (need 1 to 4)")
        if entries and stamp < entries[-1] and op != "note":
            problems.append(f"log.md:{i + 1}: time {stamp} is earlier than the entry before ({entries[-1]})")
        entries.append(max(stamp, entries[-1]) if entries else stamp)
    return problems, len(entries), (entries[-1] if entries else "")


def main():
    allp = pages()
    if "--write-index" in sys.argv:
        write_index(allp)
    report = {k: [] for k in ("not_in_index", "index_missing_file", "broken_links", "front_matter",
                              "missing_source_files", "sources_tsv", "dashes", "email",
                              "sources_without_log", "log_format", "stale_paths", "stale_status",
                              "empty", "orphans")}
    index = open(os.path.join(WIKI, "index.md"), encoding="utf-8").read()
    log = open(os.path.join(WIKI, "log.md"), encoding="utf-8").read()
    valid = set(allp) | {"overview", "index", "log"}
    inbound = {k: 0 for k in allp}
    files = dict(allp, overview="overview.md", index="index.md", log="log.md")
    for name, rel in files.items():
        path = os.path.join(WIKI, rel)
        text = open(path, encoding="utf-8").read()
        if name in allp or name == "overview":
            fm, body = front_matter(text)
            req = REQUIRED + (SOURCE_REQUIRED if rel.startswith("sources/") else ())
            missing = [k for k in req if not fm or not fm.get(k)]
            if missing:
                report["front_matter"].append(f"{rel}: missing {', '.join(missing)}")
            if len(body.strip()) < 80:
                report["empty"].append(rel)
            if rel.startswith("sources/") and fm and fm.get("source_file"):
                for sf in (s.strip() for s in fm["source_file"].split(";")):
                    if sf and not os.path.exists(resolve(sf)):
                        report["missing_source_files"].append(f"{rel}: {sf}")
        if name in allp and f"]({rel})" not in index:
            report["not_in_index"].append(rel)
        for tgt in (m.strip() for m in WIKILINK.findall(text)):
            if tgt not in valid:
                report["broken_links"].append(f"{rel} -> [[{tgt}]]")
            elif tgt in inbound and tgt != name:
                inbound[tgt] += 1
        for tgt in MDLINK.findall(text):
            if "://" in tgt:
                continue
            dest = os.path.normpath(os.path.join(os.path.dirname(path), tgt))
            if not os.path.exists(dest):
                report["broken_links"].append(f"{rel} -> ({tgt})")
            else:
                n = os.path.basename(dest)[:-3]
                if n in inbound and n != name and rel != "index.md":
                    inbound[n] += 1
        for kind, hit in prose_hits(path, f"wiki/{rel}"):
            report[kind].append(hit)
        if rel != "log.md":
            report["stale_paths"] += stale_hits(path, f"wiki/{rel}")
        if rel.startswith("sources/") and f"`{name}`" not in log:
            report["sources_without_log"].append(rel)
    for m in re.findall(r"\]\(((?:%s)/[^)]+\.md)\)" % "|".join(SECTIONS), index):
        if not os.path.exists(os.path.join(WIKI, m)):
            report["index_missing_file"].append(m)
    for extra in EXTRA_PROSE:
        if os.path.exists(extra):
            for kind, hit in prose_hits(extra, os.path.relpath(extra, REPO)):
                report[kind].append(hit)
            report["stale_paths"] += stale_hits(extra, os.path.relpath(extra, REPO))
    rows = 0
    with open(SOURCES_TSV, encoding="utf-8") as fh:
        head = fh.readline().rstrip("\n").split("\t")
        if head != ["path", "type", "date", "note"]:
            report["sources_tsv"].append(f"header is {head}, expected path, type, date, note")
        for i, line in enumerate(fh, 2):
            if not line.strip():
                continue
            cols = line.rstrip("\n").split("\t")
            rows += 1
            if len(cols) != 4 or not all(c.strip() for c in cols):
                report["sources_tsv"].append(f"row {i}: needs 4 non-empty columns")
            elif not os.path.exists(resolve(cols[0])):
                report["sources_tsv"].append(f"row {i}: {cols[0]} does not exist")
            elif cols[1] not in TSV_TYPES:
                report["sources_tsv"].append(f"row {i}: type '{cols[1]}' is not one of {sorted(TSV_TYPES)}")
            elif not re.match(r"^\d{4}-\d{2}-\d{2}$", cols[2]):
                report["sources_tsv"].append(f"row {i}: date '{cols[2]}' is not YYYY-MM-DD")
    log_problems, log_entries, newest = check_log(log)
    report["log_format"] = log_problems
    for name, rel in STATUS_PAGES.items():
        fm, _ = front_matter(open(os.path.join(WIKI, rel), encoding="utf-8").read())
        updated = (fm or {}).get("last_updated", "")
        if newest and updated < newest[:10]:
            report["stale_status"].append(f"{rel}: last_updated {updated or 'missing'} is older than the newest log entry ({newest})")
    report["orphans"] = [allp[k] for k, n in inbound.items() if n == 0]
    failures = [k for k in report if k != "orphans" and report[k]]
    if "--json" in sys.argv:
        print(json.dumps(report, indent=2))
    else:
        counts = {d: sum(1 for r in allp.values() if r.startswith(d + "/")) for d in SECTIONS}
        print(f"# Wiki lint: {len(allp)} pages " + ", ".join(f"{d} {n}" for d, n in counts.items())
              + f"; log entries {log_entries}; SOURCES.tsv rows {rows}")
        for k, v in report.items():
            tag = "warn" if k == "orphans" else ("FAIL" if v else "ok")
            print(f"\n## {k} ({len(v)}) {tag}")
            for x in v:
                print(f"- {x}")
        print("\nRESULT:", "FAIL " + ", ".join(failures) if failures else "PASS")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
