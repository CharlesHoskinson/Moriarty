# Moriarty

At the start of every Moriarty session, load `moriarty-dev:develop` and follow
[the required startup procedure](AGENTS.md#required-startup-load-the-development-plugin).
If the host does not expose the skill, read and apply
[the checked-in develop skill](plugins/moriarty-dev/skills/develop/SKILL.md)
and use its guarded CLI. Restore this workflow after resume or compaction;
include it in delegated-agent handoffs. Do not wait for the user to invoke it.

Read [AGENTS.md](AGENTS.md), [WIKI_SCHEMA.md](WIKI_SCHEMA.md) and
[wiki/workflow.md](wiki/workflow.md). This repository is the Obsidian vault;
`.claude-obsidian.json` selects its root. The claude-obsidian product checkout
is external and must never become the vault.

Keep queries read-only. Apply canonical wiki writes through inspected portable
transactions. Preserve source bytes, claim identifiers and evidence scope.
Follow the current Midnight roadmap and exact independent reviewer rules.
