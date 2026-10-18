---
source_url: "https://docs.x.ai/developers/grok-4-6"
captured_at: "2026-09-12T17:29:46+00:00"
source_text_sha256: "6d2661e4e756510fabfa7a06b377ebb9363b75dfd6e5f42fb2a575a30b1143c8"
---

Get Started
Grok 4.6
Copy for LLM
View as Markdown
Create API key
Meet grok-4.6
Grok 4.6 is SpaceXAI's frontier model built for coding, agentic tasks, and knowledge work.
Using the API
If you already have an
API key
, set the model name to
grok-4.6
:
Python
JavaScript
JavaScript (OpenAI)
Bash
import
os
from
xai_sdk
import
Client
from
xai_sdk.chat
import
user
client
=
Client(
api_key
=
os.getenv(
"XAI_API_KEY"
))
chat
=
client.chat.create(
model
=
"grok-4.6"
)
chat.append(user(
"Find and fix the bug, then explain it: function median(a){a.sort();return a[a.length/2]}"
))
response
=
chat.sample()
print
(response.content)
import
{ xai }
from
'@ai-sdk/xai'
;
import
{ generateText }
from
'ai'
;
const
{
text
}
=
await
generateText
({
model: xai.
responses
(
'grok-4.6'
),
prompt:
'Find and fix the bug, then explain it: function median(a){a.sort();return a[a.length/2]}'
,
});
console.
log
(text);
import
OpenAI
from
'openai'
;
const
client
=
new
OpenAI
({
apiKey: process.env.
XAI_API_KEY
,
baseURL:
'https://api.x.ai/v1'
,
});
const
response
=
await
client.responses.
create
({
model:
'grok-4.6'
,
input: [
{
role:
'user'
,
content:
'Find and fix the bug, then explain it: function median(a){a.sort();return a[a.length/2]}'
,
},
],
});
console.
log
(response.output_text);
curl
https://api.x.ai/v1/responses
\
-H
"Content-Type: application/json"
\
-H
"Authorization: Bearer
$XAI_API_KEY
"
\
-d
'{
"model": "grok-4.6",
"input": "Find and fix the bug, then explain it: function median(a){a.sort();return a[a.length/2]}"
}'
New to the xAI API? Follow the
Quickstart
to create an account and make your first request.
At a glance
Property
Value
Model name
grok-4.6
Context window
500,000 tokens
Knowledge cutoff
February 1, 2026
Modalities
Text and image input; text output
Output limit
No text output limit
Input price
$2.00 / 1M tokens
Output price
$6.00 / 1M tokens
Reasoning
Low, medium, high (default), or xhigh
APIs
Responses API
,
Chat Completions
Tools
Function calling
,
web search
,
X search
,
code execution
Rate limits and live pricing for your team are on the
model detail page
and
Pricing
.
For benchmark results and demos, see the
announcement
.
Important details
We highly recommend setting a
prompt_cache_key
(Responses API;
x-grok-conv-id
header on Chat Completions). It routes a conversation's requests to the same server, making cache hits reliable; without it you often pay full input price on a cache-cold server. See
What Breaks Caching
for common mistakes.
Long agent loops
additionally benefit from
context compaction
; for tool-heavy workloads see
function calling
.
Where it runs
xAI API
: get a key from the
console
Grok Build
: the default model of the
coding agent
, on the API and CLI
Cursor
: available on all plans
Model gateways
: OpenRouter, Vercel, and Cloudflare
Learn more
Reasoning
- controlling
reasoning_effort
, including
"xhigh"
Announcement
- launch post with demos and full benchmark figures
Models
- compare available models and their capabilities
Pricing
- token pricing for all models
Last updated: September 2, 2026
