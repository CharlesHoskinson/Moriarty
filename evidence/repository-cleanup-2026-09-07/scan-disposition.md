# Publication pattern scan disposition

The recorded scan inspected 9,842 historical blobs (1,631,447,411 bytes) reachable from the recovery commit, plus existing changed files. It searched explicit private-key headers and known GitHub, AWS access-ID and provider-key forms. It did not perform entropy or seed-phrase detection and does not establish absence of every secret.

The one historical match is an AWS access ID in a public source capture, `raw/sources/cake-working-group-2026-09-03/frontier-home.html`. Inspection places it in an image URL's `X-Amz-Credential` field, with signing date 2024-10-31 and an 86,400-second expiry. This is an expired public presigned image URL; the record contains no associated AWS secret key. Preserve the original source bytes. No changed-file match was found.

The historical `takeover/environment.json` contains only cwd, base, quint_version, source_hash_command and scope fields. The named session database, exported sessions and model logs are covered by the historical blob scan. That scan's limits apply to them too. The origin repository is already public and contains all history before the one local report/roadmap commit; this cleanup does not change repository visibility.

No private local wallet files or worktree recovery contents are included in publication. The bundle remains outside the repository. The archive tag exposes the already-public historical tree plus the report/roadmap commit.
