# Moriarty Obsidian vault

Open `/home/charl/Moriarty` with **Open folder as vault** in Obsidian. Start at
[the vault overview](../wiki/overview.md), [research index](../wiki/index.md) or
[language map](../wiki/canvases/moriarty.canvas). The repository itself is the
vault, so Git and Obsidian edit the same notes and citations.

The Windows path is `\\wsl.localhost\Ubuntu-26.04\home\charl\Moriarty`.
Agent writes run in WSL. Linux Obsidian is installed on this machine; using the
Windows editor does not move the vault out of WSL.

## Workflow and tool

[Research workflow](../wiki/workflow.md) describes ingest, query, save and lint.
[WIKI_SCHEMA.md](../WIKI_SCHEMA.md) retains Moriarty's evidence contract.
[Provenance mapping](../wiki/meta/provenance.md) explains how existing source and
claim identifiers coexist with the portable ledgers.

The requested [claude-obsidian project](https://github.com/AgriciDaniel/claude-obsidian)
is pinned at `ad67087cad22ad84cc3288f915588ae42c0c2b44`. Its installed location is
`/home/charl/.local/share/claude-obsidian`, separate from the vault. Existing Codex
skills link to that checkout; no second installation is needed. The
[tool receipt](../raw/sources/claude-obsidian-2026-09-07/receipt.json) records the pin
and captured instructions. The [upstream WSL guide](https://github.com/AgriciDaniel/claude-obsidian/blob/ad67087cad22ad84cc3288f915588ae42c0c2b44/docs/windows-wsl.md)
explains its filesystem requirements.

From the repository:

```sh
python3 /home/charl/.local/share/claude-obsidian/scripts/claude-obsidian.py doctor
python3 /home/charl/.local/share/claude-obsidian/scripts/claude-obsidian.py lint --strict
```

Use the installed `wiki`, `wiki-ingest`, `wiki-query`, `save`, `wiki-lint` and
`canvas` skills in Codex. Claude Code can load the same local product with
`claude --plugin-dir /home/charl/.local/share/claude-obsidian` from this repository.
The workspace config resolves the vault relative to its own location, so a
fresh clone can also act as a vault after installing the pinned tool.

For another WSL machine, clone the tool outside Moriarty and install its portable
Codex links:

```sh
git clone https://github.com/AgriciDaniel/claude-obsidian.git ../claude-obsidian
git -C ../claude-obsidian checkout ad67087cad22ad84cc3288f915588ae42c0c2b44
bash ../claude-obsidian/bin/setup-multi-agent.sh --host codex
bash ../claude-obsidian/bin/setup-multi-agent.sh --host codex --apply
```

Inspect the first command's installation preview before applying. Substitute
that checkout's absolute path in the core commands above. Do not use the tool
checkout as a vault or overwrite an existing unrelated skill installation.

## Source control

The notes, shared vault settings, Canvas and evidence are tracked. Personal
workspace state, local transports, staged inbox inputs and transaction recovery
journals are ignored. Review source classification before publishing a new
capture. Commits and pushes are explicit; no automatic sync hooks are installed.

The graph starts with `path:wiki`. Source captures, repository clones, generated
graphs and local runtime directories are excluded from routine Obsidian search.
They remain on disk and retain their source locators. Historical execution work
is accessible through [the recovery archive](ARCHIVE.md).
