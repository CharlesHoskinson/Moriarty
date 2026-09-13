---
source_url: "https://developers.openai.com/api/reference/cli/resources/models/methods/retrieve"
captured_at: "2026-09-12T15:51:54+00:00"
source_text_sha256: "03ea99305529e606c35f8369e84f7b9d4260f2aa40399220e96dedf3b308932d"
---

API Reference
Models
Copy Markdown
Open in
Claude
Open in
ChatGPT
Open in
Cursor
Copy Markdown
View as Markdown
Retrieve model
$
openai models retrieve
GET
/models/{model}
Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
Parameters
Expand
Collapse
--model
:
string
The ID of the model to use for this request
Returns
Expand
Collapse
model
:
object
{
id
,
created
,
object
,
2 more
}
Describes an OpenAI model offering that can be used with the API.
id
:
string
The model identifier, which can be referenced in the API endpoints.
created
:
number
The Unix timestamp (in seconds) when the model was created.
object
:
"model"
The object type, which is always “model”.
owned_by
:
string
The organization that owns the model.
shutdown_date
:
optional
string
The date when the model will shut down, or null if not announced.
Retrieve model
CLI Tool
HTTP
HTTP
Python
Python
TypeScript
TypeScript
Go
Go
Ruby
Ruby
Java
Java
CLI Tool
CLI Tool
openai
models
retrieve
\
--api-key
'My API Key'
\
--model
gpt-6-astra
{
"id"
:
"gpt-6-astra"
,
"object"
:
"model"
,
"created"
:
1686935002
,
"owned_by"
:
"openai"
,
"shutdown_date"
:
"2026-10-23"
}
Returns Examples
{
"id"
:
"gpt-6-astra"
,
"object"
:
"model"
,
"created"
:
1686935002
,
"owned_by"
:
"openai"
,
"shutdown_date"
:
"2026-10-23"
}
