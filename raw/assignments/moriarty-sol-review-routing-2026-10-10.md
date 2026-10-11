# Moriarty language review routing

The user instructed: "we don't need astra in the review, use Gpt 6.1 high sol".

For the current Moriarty language roadmap work, use a fresh independent
`gpt-6.1-sol` reviewer at high effort alongside the existing Grok 4.6 high review.
This replaces Astra in the required review pair. Preserve earlier Astra verdicts
as historical evidence; they do not satisfy this updated requirement.

Record the requested model and effort separately from any available returned
identity. A task label does not independently establish the returned model.
Both reviewers must assess the same final candidate. Preserve language scope:
update Moriarty itself; external Fenrir implementation is outside this assignment.
