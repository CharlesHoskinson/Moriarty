**Verdict: CHANGES_REQUESTED (reader: FLAGS)**

All three documents are internally consistent on the points I could check by hand. Requirement counts match the table of contents. Repayment step counts, guard counts, the worked ProRata example and the obligation update equations all check out. Version pairings, bound lists and the EBNF precedence are consistent between prose and grammar. The findings below are comprehension breaks, not correctness errors.

**Important findings**

1. `requirements.html`, ZKIRv4 intro paragraph ("The next backend contract must express..."). "MNR refinements" is used before the abbreviation is ever expanded, and the recursion section later calls them only "Mina-derived refinements". A reader has to infer the link. Minimal correction: expand once at first use, e.g. "The MNR (Mina-derived recursion) refinements below...", and keep MNR01–MNR08 unchanged. Severity: medium.

2. `language.html`, "Terms and frames" paragraph and the Statement grammar block. The metavariable O is used for two different things: the frame component "observations" in σ = ⟨pre, W, L, A, O, D⟩ and the operation name in Emit(O, e) and Operation_O(v) in S-EMIT. For a PL reader, this reads as Emit taking the observation namespace as an argument. Minimal correction: rename the operation-name metavariable in Emit and S-EMIT (for example to op), leaving the frame and Γ definitions as they are. Severity: medium.

3. `language.html`, "Typed financial reads" paragraph and table. The syntax section presents the /5 grammar, including six POST reads and Originate/Accrue, and the lifecycle boundary paragraph says /5 binds Core /4. The semantics section then formalizes only /3 PRE reads elaborating to Core /2, and nothing states where the /4 and /5 POST-read, Originate and Accrue rules live or that they are deliberately not restated here. The lede's promise of following a source expression to its operational meaning is not delivered for the current profile, and the reader is left unsure whether that is an omission. Minimal correction: one sentence at the start of "Typed financial reads" saying that the /4 and /5 extensions (POST reads, Originate, Accrue) are specified in the linked source /5 contract and not restated in this document. Severity: medium.

4. `language.html`, end of "Typed financial reads" ("Successful debit is E + N."). "Debit" is undefined in this context and collides with the financial sense of the word used throughout the requirements. The earlier sentence uses "charges expression work E plus kernel action count N". Minimal correction: "Successful action work is E + N." Severity: medium.

**Optional preferences**

- `requirements.html`, MPLR-011 title "Causality concurrency and interference" reads as a missing comma after "Causality". Purely typographic.
- `language.html`, "Expression small-step semantics" intro says the contract and static semantics "are proposals with independent design approval; neither is a registered profile", while the syntax table lists expression-source/1 as an implemented profile. The distinction between an implemented source profile and unregistered Core contract documents is real but easy to misread. A clause such as "the source profile is implemented; its Core contract documents remain unregistered" would remove the apparent contradiction.
- `language.html`, "Presentation bound" refers to "the earlier single-expansion presentation", which does not appear in the document. Harmless, but the comparison is unusable for a new reader.
- `README.md` and both HTML pages each state that no Lean theorem prover is required. The repetition is fine for standalone reading; readers unfamiliar with Lean get no context, but the claim is negative so nothing is lost.

No changes proposed to equations, grammar productions, requirement text or claim status.