# 站点抓取模式经验库

每次成功抓取新类型站点后，Agent 应提示用户是否将**去敏感化后的通用经验**追加到此文件。

> 公司内站点、私有 API、登录态细节、CSRF header、真实 cookie 字段或不可公开域名，请写入本地未提交的 `site-patterns.local.md`，不要提交到公共仓库。

---

## Discourse 论坛 (linux.do, meta.discourse.org 等)

**站点特征**: Cloudflare 保护 + Ember.js SPA + 登录态区分
**推荐 Fetcher**: StealthyFetcher
**关键参数**:
- `solve_cloudflare=True` — 必须
- `network_idle=True` — 等待 Ember 渲染完成
- `timeout=60000` — CF 验证耗时长，至少 60 秒（毫秒单位）
**登录 cookie 字段**: `_forum_session`, `_t`
**不需要**: `cf_clearance`（StealthyFetcher 自动获取）
**JSON API**: `/t/topic/{id}.json`（需过 CF 后才可用）
**选择器参考**:
- 帖子列表: `.topic-post`
- 作者: `[data-user-card]::attr(data-user-card)`
- 内容: `.cooked` → `.get_all_text(strip=True)`

---

## 静态博客/文档站 (GitHub Pages, Hugo, Jekyll)

**站点特征**: 纯静态 HTML，无 JS 渲染依赖，无反爬
**推荐 Fetcher**: Fetcher（最快）
**关键参数**: `impersonate='chrome'`, `timeout=30`
**选择器参考**: `article`, `.content`, `.post-body`

---

## SPA 应用 (React/Vue/Next.js)

**站点特征**: JS 渲染，内容不在初始 HTML 中
**推荐 Fetcher**: DynamicFetcher
**关键参数**:
- `network_idle=True` — 等待 API 请求完成
- `wait_selector='.content-loaded'` — 等待关键元素（按实际调整）
- `disable_resources=True` — 跳过字体/图片加速
**备注**: 优先检查是否有 API 端点可直接用 Fetcher 请求（更快更稳定）

---

## API 端点 (REST/GraphQL)

**站点特征**: 返回 JSON，无需解析 HTML
**推荐 Fetcher**: Fetcher
**关键参数**: `impersonate='chrome'`, 自定义 `headers`
**处理方式**: `page.text` 获取 JSON → `json.loads()` 解析
**Scrapling 0.4.14 注意**: `application/json` 响应上的 `Response.text` 可能是空的 `TextHandler`（它表示已解析 DOM 节点的直接文本，不是原始响应体）。优先使用 `page.json()` 解析 JSON；需要原始字节时使用 `page.body`。
**备注**: 如果 API 有反爬，可能需要带 Referer/Origin 等 header

---

## 企业 React SPA + CSRF + 懒加载工作台（泛化模式）

**站点特征**: React SPA + 企业登录态 + 分页懒加载（例如“展开更多”按钮）
**推荐方案**: Playwright 直接控制（非 scrapling Fetcher）
**原因**: DynamicFetcher 可渲染首屏但无法点击交互；直接调 API 可能因浏览器环境、CSRF 或 interceptor 校验返回空响应 / 500
**关键流程**:
1. Playwright + cookies 加载页面，`wait_until='networkidle'`
2. 循环点击"展开更多"按钮加载全部数据
3. `page.inner_text('body')` 提取纯文本，按行解析
**Cookie 格式**: `list[dict]`，必填 `name/value/domain/path`
**CSRF**: 如果站点要求 cookie 中的 CSRF token 同步到 header，优先让真实浏览器 / 页面 interceptor 自动添加
**已知限制**: 这类站点常需要用户授权、登录态和本地 site pattern；具体域名、API endpoint、cookie 字段写入 `site-patterns.local.md`
**数据结构**: 常见为文本按行排列，类型前缀 → 标题 → 状态 → 优先级 → ...

---

## Microsoft Learn 文档站 (learn.microsoft.com)

**站点特征**: 服务端渲染的技术文档，无反爬，内容在初始 HTML 中
**推荐方案**: CLI quick path `scrapling extract get URL out.md --ai-targeted`（无需 StealthyFetcher）
**备注**: 页面含大量导航/页脚噪音（"This browser is no longer supported"、Ask Learn 侧栏等），但正文表格/代码块保留完整，--ai-targeted 输出可直接用。URL 重定向常见（如 `/wsl2-faq` → `/faq`），留意实际落地 URL 作为引用来源。

---

## arXiv 摘要页 (arxiv.org/abs/*)

**站点特征**: 服务端渲染的公开学术元数据页，无登录或 JavaScript 依赖。
**推荐方案**: CLI quick path `scrapling extract get URL out.md --ai-targeted`。
**可可靠提取**: 标题、作者、摘要、学科、arXiv/DOI 标识、精确提交版本和 UTC 时间、PDF/HTML/TeX 链接。
**备注**: `--ai-targeted` 仍会保留部分导航和第三方工具链接；下游应按标题至 submission-history 区间限域。论文正文应从公开 `/pdf/<id>vN` 版本另行下载并记录 SHA-256，不要把摘要页当作定理陈述的最终证据。

---

## GitHub Issue/PR 正文 vs 评论（重要区分）

**站点特征**: Issue/PR 详情页由 React 前端渲染；纯 HTTP 抓取（Fetcher/scrapling CLI）只能拿到 **issue 描述本体**（服务端渲染），**评论列表不会出现**（客户端异步加载，且长线程会分页/懒加载）
**推荐方案**: 若只需 issue 正文/元数据（labels/state/closedAt）→ scrapling CLI 足够；若需要**评论内容**（尤其是长期 tracking thread，例如年代跨度大的 bug 讨论）→ 改用 `gh` CLI（`gh issue view <N> --repo <owner>/<repo> --comments`，或 `gh api repos/<owner>/<repo>/issues/<N>/comments --paginate` 取结构化 JSON 含时间戳）
**踩坑经验**: 曾对 microsoft/WSL 的长期 issue（数年、上百评论）用 scrapling 抓取，结果只拿到 issue 描述和 metadata 骨架，完全遗漏了数百条评论和关键的"已修复/关闭"更新——必须用 `gh` 才能拿到真实讨论历史和时间线
**备注**: `gh` 对 GitHub 是比通用网页抓取更可靠、更结构化的入口，公开仓库无需鉴权

---

## GitHub Raw 源码端点 (raw.githubusercontent.com)

**站点特征**: 公开静态源码，无登录、JavaScript 或反爬要求。
**推荐 Fetcher**: CLI quick path `scrapling extract get`。
**关键陷阱**: 对 C/C++ 等包含尖括号 include 或模板语法的源码，`.txt` 输出和
`--ai-targeted` 可能把源码误当 HTML 清洗并截断。
**保真方案**: 将输出文件扩展名设为 `.html` 以保存原始响应字节，不使用
`--ai-targeted`；随后用 `rg` 或其他窄范围本地抽取，只把必要声明和代码段交给模型。
**备注**: Markdown、MediaWiki 等普通文本仍可优先使用 `.md`/`.txt` 与
`--ai-targeted`。此模式只用于公开源码，不涉及 cookie。

---

## PubMed Central full text (pmc.ncbi.nlm.nih.gov/articles/PMC*)

**站点特征**: 服务端渲染的开放获取论文，正文在初始 HTML 中；偶发 reCAPTCHA 门页。
**推荐 Fetcher**: CLI `scrapling extract get`（本次任务用 `--css-selector "p, h1, h2, h3, li"` 可拿到完整论文正文）。
**备注**: `jamanetwork.com` 对同样论文返回 403，应改走 PMC 或作者机构 PDF。`--css-selector` 会把段落挤成超长行并保留导航噪音，但 Key Points / Results / tables 的数字仍完整。PDF 输出不要用 CSS selector（得到空文件），先 `curl` 再 `pdftotext -layout`。

---

## Justia case pages and embedded opinions (law.justia.com)

**站点特征**: Case pages are server-rendered, but their visible HTML can contain only metadata and a summary. The full opinion is commonly embedded as a PDF from `cases.justia.com`.
**推荐方案**: Use `Fetcher` for the case page. Extract the `Download PDF` link, fetch that PDF separately with `Fetcher`, and run `pdftotext -layout` for quotation review.
**关键陷阱**: A selector limited to headings, paragraphs, and list items can miss the opinion because the text is inside the PDF viewer iframe.
**验证要求**: Confirm the PDF response begins with `%PDF-`, preserve the raw PDF, and quote from the extracted opinion rather than an automated page summary.

---

## Angular institutional SPA (ich.org, iprp.global)

**站点特征**: Angular 应用，初始 HTML 只有 `<app-root></app-root>` 加 bundled JS；`scrapling extract get` 返回 200 但正文 0 字节。
**推荐 Fetcher**: `scrapling extract fetch` 或 `stealthy-fetch`，`--network-idle`，`--wait-selector "p, article, .content"`。
**保真方案（本次验证可用）**: 不要抓官网 SPA。改下官方 PDF：`database.ich.org/sites/default/files/`、`admin.ich.org/sites/default/files/`，`curl` + `pdftotext -layout`。ICH 工作计划、MC/Assembly 纪要、guideline 都在这些静态 PDF 上。
**备注**: `extract get` 对 `.md` 会报成功但写空文件，容易误判为“已抽取”。先 `wc -c` 再往下用。

---

## 模板：添加新站点模式

复制以下模板，替换具体内容后追加到此文件：

```markdown
## 站点名称/类型 (代表域名)

**站点特征**: 描述
**推荐 Fetcher**: Fetcher / StealthyFetcher / DynamicFetcher
**关键参数**:
- `参数名=值` — 说明
**选择器参考**: CSS 选择器示例
**备注**: 踩坑经验
```

## Justia SCOTUS opinion pages (supreme.justia.com)

**站点特征**: Server-rendered case pages with full opinion text in HTML (no PDF detour needed, unlike law.justia.com). WAF rejects some curl_cffi TLS fingerprints with HTTP 403 while accepting others from the same host seconds apart — measured 2026-08-29: `impersonate="chrome"` 403, `"firefox"` 403, `"safari"` 200 on the identical URL.
**推荐方案**: `Fetcher.get(url, timeout=60, impersonate=...)` trying profiles in order (`safari` first as of 2026-08-29), sleeping ~1.5s between attempts and between pages. No browser fetcher needed.
**关键陷阱**: The passing profile is a moving target — re-probe one URL per profile before a batch run instead of raising retry counts. Occasional 200s on otherwise-blocked profiles make a "retry the same profile" loop look like it works when it's just luck.
**验证要求**: Verify extracted text independently of the scraper's exit status (opinion opener present, no other-justice bleed, no site chrome, plausible word count).

---

## 创业加速器官网（ycombinator.com / techstars.com / 500.co / antler.co / alchemistaccelerator.com）

**记录日期**: 2026-09-01 · **任务**: 抓取加速器理念 / 选拔标准 / pitch 指南，产出 ai-targeted markdown

| 站点 | 特征 | 推荐方案 | 备注 |
|---|---|---|---|
| `www.ycombinator.com` | 客户端渲染（`get` 返回 200 但正文为空） | `scrapling extract fetch URL out.md --ai-targeted --network-idle` | `/library/<id>-<slug>` 页面可抓；robots 只禁 `/library?*` 查询页。id 猜错返回 404（如 `6g-...`）。`/about` `/apply` `/interviews` `/faq` `/deal` `/rfs` 均可用 |
| `paulgraham.com` | 纯静态 HTML（表格布局） | `get` | 输出开头有一段空表格噪音，正文完整 |
| `playbook.samaltman.com` | JS 渲染 | `fetch --network-idle` | `get` 返回 0 字节 |
| `www.techstars.com` | 静态 / SSR，`sitemap.xml` 914 条 | `get` | 博客路径 `/blog/founder-advice/<slug>`、`/blog/insights-and-trends/<slug>`，是选拔与 pitch 建议的主要来源 |
| `500.co` | Next.js SSR；sitemap 在 `https://strapi.500.co/api/sitemap/index.xml`（直接列页面而非子 sitemap） | `get` | `/about`→`/ethos` 重定向；`/founders`≡`/accelerators`；`/content`（博客列表）为客户端渲染，用 `fetch` 才能拿到 `/content/<slug>` 链接 |
| `www.antler.co` | Webflow，`sitemap.xml` 714 条但 `<loc>` 内含换行/空格 | `get`，`Crawl-delay: 10`（robots） | 解析 sitemap 前先 `tr -d ' \n\r\t'`；`/about` `/residency` `/investors` 带 ~100KB 导航噪音；`/blog/<slug>` 正文干净 |
| `www.alchemistaccelerator.com` | HubSpot CMS，`sitemap.xml` 302 条 | `get` | `/about` 不存在（404）；用 `/flagship-program` `/programs` `/apply` `/alchemist-investors`；博客 `/blog/<slug>`，部分 slug 带 `-2` 后缀 |

**通用做法**: 先用 `robots.txt` 找 sitemap → 关键词过滤 URL → 用 TSV 列表 + 小脚本逐条 `scrapling extract`（get 优先，空文件则改 fetch）→ 记录 `file → url → date` 清单（`SOURCES.tsv`）。

## K Framework static documentation (kframework.org)

Observed 2026-09-09: Scrapling 0.4.15 `Fetcher.get` returned full server-rendered official documentation. A single `main` element consistently scoped manual, builtin domains and tutorial/tooling pages; `Response.body` retains exact bytes while `main.get_all_text(separator='\n', strip=True)` produces a separate searchable extract. Inline code may split across lines in text, so use saved HTML when exact syntax matters. Check current robots content signals before reference use; do not assume permission for training. Follow links for tutorial slugs (`20_backends`, not guessed `20_k_cells`). No login, cookie retention or browser was needed.

---

## 学术论文 PDF（作者主页、eprint.iacr.org、cakeml.org、xavierleroy.org 等）

**站点特征**: 直接返回 PDF 字节；`scrapling extract get` 会把 PDF 当文本写入 .md，产生乱码
**推荐方案**: `curl -sL -A "Mozilla/5.0" -o paper.pdf URL` 后 `pdftotext -layout paper.pdf out.txt`（需要图表时用 pixelshot 渲染页面图片）
**备注**: dl.acm.org、link.springer.com 的 PDF 为付费墙，返回 HTML；改用作者主页、机构库（IDEALS、DASH）或 eprint 镜像，找不到就在报告中注明未检索到


## Google Developers technical documentation (developers.google.com)

Observed 2026-09-19 with Scrapling 0.4.15: public technical pages are server-rendered. Fetcher.get and the CLI with --ai-targeted work without login or browser rendering. Scope extraction to `.devsite-article-body`; retain the page URL, retrieval time, license attribution and content hashes. Normalize language/authuser query variants for bounded link discovery; REST navigation can contain retired URLs that return 404. Check robots.txt before batch collection and use a delay. Article HTML converted to Markdown may flatten code inside tables; consult the original when exact rendering matters. Google Developers CC BY notices do not automatically license help-center pages. No cookies or credentials need to be persisted.

---

## Grav CMS marketing sites (example: a public product site under /user/pages/)

**站点特征**: Grav CMS. Public pages are server-rendered. `robots.txt` often disallows `/system/`, `/user/`, `/vendor/` and allows `/user/pages/`, `/user/themes/`, `/user/images/`.
**推荐 Fetcher**: `scrapling extract get URL out.md --ai-targeted`.
**备注**: Homepage, `/about`, and `/blog` returned full visible copy on the first GET (Scrapling 0.4.15, 2026-10-09). No login and no browser. Do not crawl the disallowed admin paths.

---

## Swiss commercial-register publications (shab.ch, zefix.ch, moneyhouse.ch)

**站点特征**: SHAB publication URLs (`shab.ch/shabforms/servlet/Search?EID=7&DOCID=HR...`) and Zefix entity pages are public server-rendered register text. Moneyhouse and similar directories reprint the same SHAB wording and then paywall shareholder, revenue, and network views.
**推荐 Fetcher**: `scrapling extract get` for a single SHAB document. Prefer the official SHAB or Zefix wording over a directory reprint when both exist.
**关键陷阱**: The `shabforms/servlet/Search?EID=7&DOCID=HR..` URL returns PDF bytes (one or two pages), not HTML; save it as .pdf and read with `pdftotext -layout`. A save named .html that looks like large binary "site chrome" is a truncated PDF (2026-10-09). Plain curl with a generic UA works for these PDFs. For web search, DuckDuckGo `lite.duckduckgo.com/lite/?q=` works with plain `Fetcher.get` (html.duckduckgo.com returns 403; Bing serves junk results to automated clients; Google works only via StealthyFetcher). Search-engine extracts of the same publication are often the cleaner quotation source, then confirm the publication number against SHAB. The share register of a Swiss AG is not in these publications. Plain `curl` against some foundation marketing sites on Vercel stops at a browser checkpoint; that is separate from SHAB.
**备注**: Do not treat a directory's "shareholder" upsell as evidence that a shareholder list exists in the public register.

---

## UN Web TV / Kaltura embedded video and transcripts (webtv.un.org)

**站点特征**: Drupal 前端托管嵌入式 Kaltura 播放器 (`cdnapisec.kaltura.com`, `entryId`, `uiConfId`, `partnerId`)。页面 HTML 包含播放器配置脚本，但媒体流与字幕资源在播放器初始化后异步加载。
**推荐方案**: 使用 `DynamicFetcher` + `page_setup` 拦截网络请求：
```python
captured_urls = []
def setup(page):
    page.on("request", lambda req: captured_urls.append(req.url))

res = DynamicFetcher.fetch(url, page_setup=setup, page_action=lambda p: p.wait_for_timeout(5000), timeout=60000)
```
- 字幕/逐字稿端点特征：包含 `/service/caption_captionAsset/action/serve/captionAssetId/...`。
- 直接请求该字幕端点（或用 `Fetcher.get`）可获得包含时间戳的完整 WebVTT / SRT 逐字稿文件。
- HLS 播放清单特征：`https://cdnapisec.kaltura.com/p/{partnerId}/sp/{partnerId}00/playManifest/...`。

