# Moriarty

At the start of every Moriarty session, load `moriarty-dev:develop` and follow
[AGENTS.md](AGENTS.md), beginning with its
[required startup procedure](AGENTS.md#required-startup-load-the-development-plugin).
If the host does not expose the skill, read and apply
[the checked-in develop skill](plugins/moriarty-dev/skills/develop/SKILL.md)
and use its guarded CLI. Restore this workflow after resume or compaction;
include it in delegated-agent handoffs. Do not wait for the user to invoke it.

Follow the current user-selected implementation and independent review routing
in `AGENTS.md`. This entry file does not select Gemini as the implementer or
change authority, resource limits, evidence rules or acceptance gates.
