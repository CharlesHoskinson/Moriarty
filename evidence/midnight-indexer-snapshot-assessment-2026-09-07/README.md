# Preprod snapshot metadata assessment

Status: metadata-only; Preprod restore is not an execution target. The latest
user direction is “let's go all in on preview”. No archive download, container
start or database restore was performed.

Experiment observation: one HTTP GET to the supplied snapshot URL requested
`Range: bytes=0-4095` and `Accept-Encoding: identity`, with a 20-second timeout
and a hard application read limit of 4096 bytes. The response was HTTP 403 from
AmazonS3, containing 111 bytes of XML. The connection was closed after the bounded
read. See `metadata.json` and `range-response-prefix.bin`. This does not establish
whether the underlying archive exists or is valid. No access bypass was attempted.

Repository/environment observation at 2026-09-07T03:20:53Z: `/home/charl` had
660,729,655,296 bytes available, about 660.73 GB decimal. The parent reported
that Docker and `/home` share this filesystem. The unverified claimed 199 GB
archive plus 450 GB restored database would consume 649 GB decimal, leaving only
11.73 GB before WAL, restore scratch space, indexes beyond the estimate, Docker
images, ongoing chain growth and other concurrent work. If the quoted sizes are
GiB, the combined amount is 696,858,443,776 bytes and exceeds available space by
about 36.13 GB. Neither interpretation supports a comfortable local restore.

Inference: the URL identifies a Preprod indexer dump, while the selected work is
Preview. An indexer database snapshot is not a wallet's private synchronization
state and does not itself restore the selected wallet's keys or scanning state.
No deeper migration/config compatibility review was performed after the parent
relayed the updated Preview-only direction.

Open questions: archive content, exact archive/restored size, integrity digest,
authenticity, dump format, PostgreSQL compatibility, schema/migration version,
expected block height and wallet-table contents remain unverified. PostgreSQL
17.9-bookworm, pg_restore 17, eight restore jobs, block height 1,907,432,
successful migration maximum 3 and wallet count zero are user-supplied claims or
instructions relayed by the parent, not results of this assessment.

Recommendation: continue Preview. Preserve the Preprod instructions as historical
input; do not infer permission or suitability for a Preprod restore from them.
