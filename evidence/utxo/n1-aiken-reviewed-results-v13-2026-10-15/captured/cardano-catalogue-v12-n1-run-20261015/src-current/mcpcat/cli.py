"""Command line for the v1.2 UNIT checker (catalogue editions 1.2 current, 1.1 legacy).

python3 -I src/mcpcat/cli.py validate-catalogue CATALOGUE MANIFEST V0 V1 [--v11 V11] --expected-manifest-digest H
python3 -I src/mcpcat/cli.py check-admission  CATALOGUE MANIFEST V0 V1 RECORDS POLICY [--v11 V11] --expected-manifest-digest H
python3 -I src/mcpcat/cli.py check-bundle     CATALOGUE MANIFEST V0 V1 RECORDS BUNDLE POLICY [--v11 V11] --expected-manifest-digest H
python3 -I src/mcpcat/cli.py check-settlement [anything]

The expected catalogue manifest digest is mandatory (review A2); base pins are compiled in and cannot be
overridden. --v11 supplies the bytes of the pinned v1.1 base; an edition 1.2 catalogue needs it (CAT_BASE_INPUT
otherwise) and its bytes must equal pins.V11_PIN. Profiles come from pinned code via the policy's profile_id. All
JSON inputs are parsed with MCJ/1. stdout is always exactly one ASCII JSON object, including usage errors (review
A7; v1.2 L7: non-ASCII text is \\u-escaped, never written raw, and a surrogate-escaped non-UTF-8 argv byte is shown
as backslash text, so the decoded JSON holds Unicode scalar values only).
Exit status: 0 VALID / ADMITTED_STRUCTURAL (all records) / STRUCTURALLY_CONSISTENT_PROOF_UNVERIFIED;
1 INVALID / REFUSED; 2 usage error or UNSUPPORTED. No exit status means "verified".
"""
import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from mcpcat import api, mcj, semantic  # noqa: E402
from mcpcat.admission import check_admission  # noqa: E402
from mcpcat.bundle import OUTCOME_OK  # noqa: E402
from mcpcat.catalogue import validate_catalogue  # noqa: E402
from mcpcat.settlement import check_settlement  # noqa: E402


class _Usage(Exception):
    pass


class _Parser(argparse.ArgumentParser):
    def error(self, message):
        raise _Usage(message)

    def exit(self, status=0, message=None):
        raise _Usage(message or "help/exit requested")


def _scalar(x):
    """Unicode scalar values only: a lone surrogate (a surrogate-escaped non-UTF-8 argv byte) becomes visible ASCII
    backslash text, so the printed JSON decodes to text inside MCJ/1's string domain (logs/FAILURE-ANALYSIS-01.json)."""
    if isinstance(x, str):
        return x.encode("utf-8", "backslashreplace").decode("utf-8")
    if isinstance(x, list):
        return [_scalar(v) for v in x]
    if isinstance(x, dict):
        return {_scalar(k): _scalar(v) for k, v in x.items()}
    return x


def _emit(obj, code):
    obj.setdefault("qualification", "UNIT")
    # ASCII bytes whatever the stdio encoding/error handler, and no lone-surrogate escapes (review fdecbe41 L7)
    print(json.dumps(_scalar(obj), indent=2, ensure_ascii=True))
    return code


def _load(p):
    return mcj.parse(Path(p).read_bytes())


def _parser():
    ap = _Parser(prog="mcpcat", add_help=False)
    sub = ap.add_subparsers(dest="cmd", required=True, parser_class=_Parser)

    def common(p, *extra):
        for a in ("catalogue", "manifest", "v0", "v1", *extra):
            p.add_argument(a)
        p.add_argument("--v11", default=None)
        p.add_argument("--expected-manifest-digest", required=True)
    common(sub.add_parser("validate-catalogue", add_help=False))
    common(sub.add_parser("check-admission", add_help=False), "records", "policy")
    common(sub.add_parser("check-bundle", add_help=False), "records", "bundle", "policy")
    s = sub.add_parser("check-settlement", add_help=False)
    s.add_argument("rest", nargs="*")
    return ap


def main(argv=None):
    try:
        args = _parser().parse_args(argv)
    except _Usage as e:
        return _emit({"outcome": "REFUSED", "codes": ["CLI_USAGE"], "detail": str(e)}, 2)
    try:
        if args.cmd == "check-settlement":
            return _emit(check_settlement(args.rest), 2)
        try:
            cat_bytes = Path(args.catalogue).read_bytes()
            doc = mcj.parse(cat_bytes)
            manifest = _load(args.manifest)
            bases = {"v0": Path(args.v0).read_bytes(), "v1": Path(args.v1).read_bytes()}
            if args.v11 is not None:
                bases["v11"] = Path(args.v11).read_bytes()
        except mcj.MCJError as e:
            return _emit({"outcome": "REFUSED", "codes": ["INPUT_CANONICAL_UNSUPPORTED", e.code]}, 1)
        cat = validate_catalogue(doc, manifest, bases, args.expected_manifest_digest)
        if isinstance(manifest, dict) and manifest.get("catalogueFileSha256") != semantic.sha256_hex(cat_bytes):
            cat = dict(cat, outcome="INVALID", codes=sorted(set(cat["codes"]) | {"CAT_MANIFEST_FILE_DIGEST_MISMATCH"}),
                       errors=cat["errors"] + [{"code": "CAT_MANIFEST_FILE_DIGEST_MISMATCH", "where": "manifest",
                                                "detail": "catalogue file sha256 differs from manifest catalogueFileSha256"}])
        if args.cmd == "validate-catalogue":
            return _emit(cat, 0 if cat["outcome"] == "VALID" else 1)
        if cat["outcome"] != "VALID":
            return _emit({"outcome": "REFUSED", "codes": ["CATALOGUE_INVALID"] + cat["codes"],
                          "errors": cat.get("errors", [])[:40]}, 1)
        try:
            records = _load(args.records)
            policy = _load(args.policy)
        except mcj.MCJError as e:
            return _emit({"outcome": "REFUSED", "codes": ["INPUT_CANONICAL_UNSUPPORTED", e.code]}, 1)
        ctxres = api.open_context(doc, manifest, bases, policy)
        if ctxres["outcome"] != "READY":
            return _emit({"outcome": "REFUSED", "codes": ctxres["codes"], "notes": ctxres["notes"]}, 1)
        ctx = ctxres["context"]
        if args.cmd == "check-admission":
            if not isinstance(records, list) or not records:
                return _emit({"outcome": "REFUSED", "codes": ["ADM_RECORDS_SCHEMA"]}, 1)
            res = [check_admission(ctx, records, i) for i in range(len(records))]
            ok = all(r["outcome"] == "ADMITTED_STRUCTURAL" for r in res)
            return _emit({"outcome": "ADMITTED_STRUCTURAL" if ok else "REFUSED",
                          "codes": sorted({c for r in res for c in r["codes"]}), "results": res,
                          "cryptographicVerification": "NOT_PERFORMED"}, 0 if ok else 1)
        from mcpcat.bundle import check_bundle
        res = check_bundle(ctx, records, Path(args.bundle).read_bytes())
        return _emit(res, 0 if res["outcome"] == OUTCOME_OK else 1)
    except OSError as e:
        return _emit({"outcome": "REFUSED", "codes": ["CLI_INPUT_ERROR"], "detail": type(e).__name__}, 2)
    except Exception as e:  # noqa: BLE001 - fail closed with a structured refusal; tests treat this code as a defect
        return _emit({"outcome": "REFUSED", "codes": ["CLI_INTERNAL_ERROR"], "detail": f"{type(e).__name__}: {e}"}, 2)


if __name__ == "__main__":
    sys.exit(main())
