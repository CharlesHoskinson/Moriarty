#!/usr/bin/env python3
"""Cache the pinned k20 Midnight ceremony SRS without changing shared parameters."""
import hashlib
from pathlib import Path
import sys
import tempfile
import urllib.request

SIZE = 201326980
SHA256 = "1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1"


def main():
    path = Path(sys.argv[1])
    if path.exists():
        if path.stat().st_size != SIZE or hashlib.file_digest(path.open("rb"), "sha256").hexdigest() != SHA256:
            raise ValueError("Existing k20 SRS differs from the pinned ceremony file")
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = None
    try:
        with tempfile.NamedTemporaryFile(dir=path.parent, delete=False) as target:
            temporary = Path(target.name)
            digest = hashlib.sha256()
            total = 0
            with urllib.request.urlopen("https://srs.midnight.network/bls_midnight_2p20", timeout=120) as source:
                while chunk := source.read(1024 * 1024):
                    total += len(chunk)
                    if total > SIZE:
                        raise ValueError("k20 SRS exceeds its pinned byte length")
                    target.write(chunk)
                    digest.update(chunk)
            if total != SIZE or digest.hexdigest() != SHA256:
                raise ValueError("Downloaded k20 SRS differs from the pinned ceremony file")
        temporary.replace(path)
    finally:
        if temporary is not None:
            temporary.unlink(missing_ok=True)


if __name__ == "__main__":
    main()
