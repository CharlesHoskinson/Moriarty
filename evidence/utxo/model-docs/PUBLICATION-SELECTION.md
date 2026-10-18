# Model documentation publication selection

The original `DOWNLOAD-MANIFEST.json` is preserved byte-for-byte. It lists downloads from the local collection, including raw readiness JSONL captures and command stderr files that are not selected as readable publication files.

The readable selection consists of the official model page, model-card PDF and extracted text, model-card receipt, official CLI configuration and extension documentation, `USAGE.md`, the public readiness result, and the original download manifest. This addendum records publication selection; it does not change the download record.

Raw provider event logs are retained locally. The history publication permits only strict known public text events, with source-log hashes and extraction rules recorded separately. It never publishes provider thought/tool payloads or interprets a canary as source or native approval.

The two official CLI Markdown downloads begin with a Markdown link bracket. The conservative archive policy's JSON detector flags that prefix and then fails JSON parsing. Their exact official URLs, SHA-256 pins, parser-only classification, and separate zero credential/signing/private-envelope checks are recorded in [READABLE-CLASSIFICATION.json](../READABLE-CLASSIFICATION.json). This is a two-file provenance classification, not a general provider-log exception.
