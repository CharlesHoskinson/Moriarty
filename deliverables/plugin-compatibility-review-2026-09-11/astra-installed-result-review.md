# Installed-result independent review

Verdict: **APPROVE** — scoped installed result for `moriarty-dev` version `0.1.0+codex.20260911184750`.

Reviewer: **gpt-6-astra**, independent delegated reviewer `/root/final_astra_review_02`. This follow-up audits actual installation and host-result evidence. The prior full candidate source audit remains separate. No production status, campaign dispatch, configuration change, source edit, cache modification or operational-state write was performed for this audit.

## Independently checked current bytes

All 32 frozen entries match `candidate-02.json` at each of these roots, with no extra package files excluding Python bytecode caches:

- `/home/charl/Moriarty-plugin-compatibility-20260911/plugins/moriarty-dev`
- `/home/charl/Moriarty/plugins/moriarty-dev`
- `/home/charl/plugins/moriarty-dev`
- `/home/charl/.codex/plugins/cache/personal/moriarty-dev/0.1.0+codex.20260911184750`

The manifest SHA-256 remains `5c8163265ef8369a518c105652a73ee8504f95111718d760fdd98b5737dda275`.

Both retained roots ending `.20260911061236` and `.20260911181517` match all 28 recorded pre-upgrade file hashes. Both separate backups and the preserved marketplace-source backup also match their recorded hashes. The earlier `.20260911061236` path contains a manifest identifying `.20260911181517`; that is the preserved baseline state, not evidence that this update installed new bytes under the old path.

Current `/home/charl/.codex/hooks.json` and `/home/charl/.codex/config.toml` hashes match `host-config-before-hashes.json`. This confirms unchanged recorded configuration bytes, rather than inferring trust from installation.

`install-result.json` records successful `moriarty-dev@personal` installation at the current cache path, exit zero and empty stderr. `installed-doctor.json` accurately keeps `activation` and `hookTrust` unknown and `hostCoverage` at `installed-unverified`.

## Actual host evidence

Inspected the raw captured hook notifications and the collector/checker source, not only `host-comparison.json`. The capture is produced by a Codex app-server ephemeral thread. The fixture command catalog maps the marker action to exactly `python3 marker.py`; the marker script would write `unexpected-marker` if run.

In post-upgrade thread `01a091dd-9e35-7491-b7fd-b0d22a366525`, every Moriarty hook event identifies the `.20260911184750/hooks/hooks.json` source:

- Registered raw dispatch: PreToolUse call `exec-206ee63f-440e-4683-b1dc-ca773d5ef454` is `blocked`, with the explicit registered-dispatch/guarded-CLI reason. The marker is absent in the recorded result and remains absent at independent inspection.
- `pwd`: call `exec-cdf8ac90-b233-42cf-99f9-5ac0ba4301d1` has completed PreToolUse and PostToolUse events and a completed command with exit zero and the expected fixture directory output.
- `printf compatibility-denied-marker`: call `exec-de895e9a-b333-4a4e-8a4d-55e21910ff87` has completed PreToolUse and PostToolUse events and a completed command with exit zero and the exact expected output.
- SessionStart completed with fixture context. Stop completed in 26 ms with no error or continuation entry; the turn completed.

Recomputed Moriarty completion counts agree with the comparison: before upgrade, four completed and two blocked; after upgrade, six completed and one blocked. Before-upgrade events identify `.20260911181517`, including the extra denial for the harmless action-ID mention. These records support the specific false-denial fix while retaining the registered raw-dispatch denial.

The event capture does not retain the denied command's original tool-input record. Its attribution to the marker request is corroborated by the probe's closed instructions, the exact raw-argv-only denial message, fixture catalog and absence of an executed marker command. I found no contradictory observation.

## Test-result evidence and limits

Validated the complete recorded test-result lines and terminal summaries: integrated suite **199 OK**, 13.214 seconds; installed package suite **23 OK**, 1.184 seconds. These are inspected author-run results for this installed-result review, not falsely claimed fresh reviewer executions. The separate source audit independently ran the same full frozen suite successfully.

No blocking installed-result findings. Approval establishes matching installed bytes, retention of the recorded older packages, unchanged recorded config bytes and the specific captured host behavior. It does not establish all shell forms, native delegation, code-mode composition, another host/platform, pending-notification delivery in this smoke, generic interception coverage, or financial/proof acceptance. The ephemeral session was not represented as a persistent transcript, and no new host turn was launched by this reviewer.

## Audited evidence hashes

```json
{
  "install-result.json": "803e3e2a0a9ac91242861ad15ffd1135e9d88fd42456f62f83ba758bfa71588b",
  "candidate-02.json": "5c8163265ef8369a518c105652a73ee8504f95111718d760fdd98b5737dda275",
  "preserved-installations.json": "48667809704487897cb53908e1976414d937640ca03dfb839bcab4c8ef2113cf",
  "installed-doctor.json": "9227a05878a41c3c6e733342b2b8111856629789e96bc8b0047e2c8a6221d344",
  "host-before-upgrade-events.json": "ee71359e28f50f8144c54294c17be9281ed642744632162b8024ccc04968bdc4",
  "host-before-upgrade-result.json": "1b4398c834644255f5ad82261c2ba0ae28674bde33f51026c2f5456c69b96025",
  "host-after-upgrade-events.json": "933b788b5658cf7b7709fe50dabd006f5a59a9f63fc9e5894828234489a8ef69",
  "host-after-upgrade-result.json": "ca6953fa2819a44c837bb584e8af8c37c2e56d18e493ef389e279ca41fbf095f",
  "host-comparison.json": "cf6cce1f74af02a06b5a795addde821a14064e476cc3ff4823b0ebb0294a6b9f",
  "plugin-tests-integrated.txt": "4504bb9d0dc30894d20877e5c9a5d0ea5e37d710ade2125c5245612687767822",
  "installed-package-tests.txt": "083467144c2ae7664822a1c92601e047e8692044f149f77606abe8c7a9e2bb69",
  "host-config-before-hashes.json": "9b51be2c5b2c2fae5167f6daeb06abfa5f23515772ff250b6c8ac5f4aa49b186",
  "host_probe.py": "3bbd36b768a190ce1406ee279e3576167eab75b6a40935e4501709a81a79a3dd",
  "check_host_results.py": "5d1b48666cc94492966f0605bf74f16c7df11c6f85a6082362b16dcb171f47a8",
  "install_reviewed_candidate.py": "5e2b50534638d149a0fb5c273790e5cb7f29ed14a788bfacf26720a49532b481"
}
```
