# Moriarty end-to-end audit and mid-roadmap review

Authority: direct user request in this conversation, 2026-10-14.

> Alright, let's do a complete end to end audit and mid-roadmap review to update it based upon what we learned. Launch a bunch of luna agents to get our knowledge graph complete and then have 10 agents (5 opus 5 gpt) brainstorm how we can do the audit and what the output needs to be to effectively update the roadmap

Later controlling security steering:

> have opus do all the security work since gpt refuses

Execution scope: six Luna mapping passes, five GPT and five Opus independent
initial proposals, Opus-owned security follow-up, source/evidence reconciliation,
audit output plan and minimal roadmap/crosswalk update. Requested models are
`gpt-6-luna`, `gpt-6.1-sol` and `claude-opus-5-5`, at high effort. Initial ten
proposals remain distinct; a security follow-up is not counted as another
initial seat. Security steering redirects the GPT native-security seat to
nonsecurity toolchain/evidence integration. All security work is Opus-owned.

The original direct M1-M17 table remains controlling: Phase0 requires M1-M10
through U0/U1/U2 with M10 release tooling; Phase1 requires M11-M17 through U2/U3,
part of U5 and U6. Implement the general language, not Fenrir applications.
The canonical crosswalk preserves each original U/MC/L mapping. The user has
reported that Fenrir has no versioning system yet. No endpoint, network ID,
Anchor, deployed registry, committee identity or target tuple is invented.

This work does not authorize a native retry, expand an exhausted budget,
reset a contract, waive any U/M/MC/SP/G requirement or claim a completed full
runtime audit. Source baseline and actual review scopes are in the audit
package. Existing implementation routing outside this review is not silently
replaced; this task's security work is explicitly assigned to Opus.
