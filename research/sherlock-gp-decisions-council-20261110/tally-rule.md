# Tally rule (fixed before any review output exists)

Per decision D1..D4:
1. Classify each review lane's transport first (exit status, timeout, completeness, valid JSON in the required shape). Only completed, valid results are read.
2. A substantive verdict is `approved` or `changes_requested` on that decision. `insufficient_evidence` is recorded and does not count.
3. Picks are tallied by OPTION, not by candidate id: the lead maps each pick (a candidate's stated decision, or the reviewer's synthesis `option` text) to an option label (for D1: a/b/c-variant; D2: architecture family; D3: per point meet-as-written vs owner amendment; D4: inline processor vs separate leaf+lifts vs hybrid). The mapping is recorded next to each vote.
4. The selected option is the option with the most substantive verdicts; ties are reported as a tie with both options.
5. Status:
   - `closed`: the selected option has >= 3 substantive verdicts from distinct reviewer lanes covering >= 2 families, all of them `approved`, and no admissible `changes_requested` on that decision from any lane.
   - `blocked by dissent`: quorum is met but at least one admissible `changes_requested` exists on that decision (on any option). The finding is recorded verbatim in substance. No second round.
   - `quorum not met`: fewer than 3 substantive verdicts from >= 2 families for the selected option.
6. Approved verdicts for a different option than the selected one are recorded as dissent on option choice (not a blocking `changes_requested`).
7. Admissible `changes_requested` = completed, valid shape, and at least one finding with a location and a next action.
