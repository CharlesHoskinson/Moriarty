# Obsidian migration evidence

The Moriarty repository in WSL is the Obsidian vault. The existing wiki, raw source captures, evidence and roadmap keep their paths. The user requested AgriciDaniel/claude-obsidian after GitHub cleanup; SRC-0077 records that instruction and SRC-0078 pins the tool instructions.

The installed portable core and all 15 Codex skill links were already present at the pinned commit. The migration uses the core's inspected transactions for adoption, note metadata and navigation, portable provenance records, settings and Canvas. Exact changed paths and file hashes appear in the transaction receipts. Personal Obsidian state, inbox inputs and transaction recovery data are ignored by Git.

The legacy source inventory and all observed claim identifiers remain available. Portable source records preserve each complete legacy row and separately record observed file hashes. Legacy claims are indexed by location, not automatically extracted or accepted into the new claim ledger. Original confidence and S0-S7 scope stay in the notes.

The initial deterministic lint found four incompatible heading links and missing metadata on 33 notes. The migration repairs navigation and supplies Obsidian metadata. Doctor resolves the workspace and reports ready. Final strict lint passes with zero findings after removal of the redundant optional Canvas catalog. Its original creation and subsequent scoped repository cleanup are both retained. `lint-after.json` and `verification.json` describe the initial candidate; `lint-final.json` and `preservation.json` supersede them for the final state.

`verify-preservation.py` compares every legacy note with cleanup commit `8df38d2`, records baseline/current file and body hashes, and checks exact body equality after the enumerated navigation additions and fragment and portable-source-link repairs. It also verifies that original frontmatter properties and source inventory rows are unchanged, all 620 claim identifiers remain indexed, and the raw sources, roadmap and Canvas targets are intact. `legacy-note-bodies.diff` contains every legacy prose difference. Humanizer touched newly authored notes and repository guidance, not legacy note bodies.

New source intake uses `.raw/captured/` consistently and updates both the SRC inventory and portable mapping. Source backlinks include newly citing notes. Obsidian creates Markdown links and does not automatically rewrite links on rename; orphans remain visible in the graph.
 This operation does not establish a native recursive proof, ledger correspondence or any MC01-MC08 acceptance.

The fresh-checkout test initially exposed links into ignored `repos/` and
`graphs/` directories. Agda links now use the exact previously cited GitHub
commit, verified against its tree. The historical decision graph is retained
byte-for-byte in `wiki/meta/legacy-decision-graph.json`. Its provenance and the
complete link replacements are recorded; neither change reinterprets a claim.

Independent final Fable 5.1 medium and GPT-6 high reviews approve this migration
within its maintenance scope. `audits/admission.json` identifies the reviewed
files and actual provider receipts. `candidate-binding.json` confirms those
files match the working tree and the fresh staged checkout byte-for-byte.

The four old heading fragments used GitHub-style slugs that the portable
Obsidian parser did not resolve. Those links now retain their note targets and
visible labels; claim IDs remain searchable through the claim index. This
trades precise heading jumps for links that work in both readers. The external
Agda link check is recorded in `source-link-verification.json`; structural lint
alone does not check remote URLs. Desktop opening and rendering remain untested.
