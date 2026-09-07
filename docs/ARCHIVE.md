# Historical work

Current development follows the [Midnight language roadmap](../ROADMAP.md).
Superseded implementations and execution campaigns are preserved at
[archive/pre-cleanup-2026-09-07](https://github.com/CharlesHoskinson/Moriarty/tree/archive/pre-cleanup-2026-09-07).

That tag contains the former Python Core, Quint S01/S02 and Candidate A models,
K/ZKIR experiments, their tests, execution plans and retained outputs. Their
historical successes and failures keep their original scope. Archiving does not
complete an unfinished plan or admit a current language package.

The [archive manifest](../evidence/repository-cleanup-2026-09-07/archive-manifest.json)
records each removed path, Git blob, size and former branch tip. Primary source
captures, financial studies, the research wiki, current Midnight experiments and
MC01-MC08 evidence remain in the current tree.

To inspect the old tree without changing current work:

```sh
git fetch origin tag archive/pre-cleanup-2026-09-07
git worktree add --detach ../Moriarty-history archive/pre-cleanup-2026-09-07
```

Open historical receipts in that checkout to resolve their original relative
paths. Their bytes have not been rewritten to match the new layout. A separate
verified local Git bundle preserves all pre-cleanup refs, including checkpoint
refs. Private runtime files and uncommitted drafts stay in local recovery storage.

The cleanup reduces the checked-out tree. The archive remains in Git history,
so historical clone size is unchanged. No history rewrite is part of this cleanup.
