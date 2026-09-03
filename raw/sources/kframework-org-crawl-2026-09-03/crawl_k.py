import json, re, time
from collections import Counter, deque
from urllib.parse import urljoin, urlparse, urldefrag
from scrapling.fetchers import Fetcher

ROOT = "https://kframework.org/"
MAX_PAGES = 60
seen, queue = set(), deque([ROOT])
pages, gh_links = {}, Counter()
robots = Fetcher.get("https://kframework.org/robots.txt", impersonate="chrome", timeout=30)
print("robots:", robots.status, (robots.body or b"")[:300])

while queue and len(pages) < MAX_PAGES:
    url = queue.popleft()
    if url in seen: continue
    seen.add(url)
    try:
        page = Fetcher.get(url, impersonate="chrome", timeout=30)
    except Exception as e:
        print("ERR", url, e); continue
    if page.status != 200:
        print("STATUS", page.status, url); continue
    title = page.css("title::text").get() if page.css("title") else ""
    links = [urldefrag(urljoin(url, h)).url for h in page.css("a::attr(href)").getall()]
    pages[url] = {"title": (title or "").strip(), "n_links": len(links)}
    for l in links:
        p = urlparse(l)
        if "github.com" in p.netloc:
            gh_links[l] += 1
        elif p.netloc == "kframework.org" and l not in seen and not re.search(r"\.(pdf|png|jpg|svg|zip|tar|gz)$", l):
            queue.append(l)
    time.sleep(0.3)

json.dump({"pages": pages, "github": gh_links.most_common()}, open("k_crawl.json","w"), indent=1)
print("\nPAGES", len(pages))
for u,v in pages.items(): print(" ", u, "|", v["title"])
print("\nGITHUB LINKS")
for l,c in gh_links.most_common(): print(f"{c:3d}  {l}")
