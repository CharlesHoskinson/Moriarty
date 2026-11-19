#!/usr/bin/env python3
"""Assemble notices for the pinned native validator and the two distributed WASM bindings."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
from tempfile import TemporaryDirectory

parser = argparse.ArgumentParser(description=__doc__)
for flag in ("native-tree", "native-metadata", "wasm-tree", "wasm-metadata"):
    parser.add_argument("--"+flag, type=Path)
for flag in ("wasm-source", "sdk", "output"):
    parser.add_argument("--"+flag, required=True, type=Path)
args = parser.parse_args()
here = Path(__file__).resolve().parent
root = here.parents[2]
rust_docs = Path(subprocess.check_output(["rustc", "--print", "sysroot"], text=True).strip()) / "share/doc/rust"
components = {}
out = args.output
(out/"texts").mkdir(parents=True, exist_ok=True)
pattern = re.compile(r"^(?:license|licence|copying|notice|copyright|unlicense|credits|third.party)(?:s|\.txt|[-._].*)?$", re.I)
supplemental = {
    "blake2b_simd": ("blake2b_simd", "https://github.com/oconnor663/blake2_simd/blob/47714dc3c424e82213d7f2eb4a833e3843017be5/LICENSE"),
    "blst": ("blst", "https://github.com/supranational/blst/blob/54e6e55674722fc2797ebb4bbb71b26d881eb4b8/LICENSE"),
    "halo2derive": ("halo2derive", "https://github.com/privacy-scaling-explorations/halo2curves/blob/4b7a69e6a3f1878b150c212d73ec82a3210c29f9/LICENSE-MIT"),
    "introspection": ("introspection", "https://github.com/vityafx/introspection/blob/4e9feac30d311411e0fbb27fcb6c5376bb892409/LICENSE"),
}
supplemental["introspection-derive"] = supplemental["introspection"]


def attach(record, name, data):
    digest = hashlib.sha256(data).hexdigest()
    relative = "texts/"+digest+(".html" if name.endswith(".html") else ".txt")
    (out/relative).write_bytes(data)
    text = {"sourceFile": name, "file": relative, "sha256": digest}
    if text not in record["texts"]:
        record["texts"].append(text)


def original_texts(folder):
    found = []
    for item in sorted(folder.rglob("*")):
        if not item.is_file() or ".git" in item.parts:
            continue
        relative = item.relative_to(folder)
        if pattern.match(item.name) or any(p.lower() in ("licenses", "licences") for p in relative.parts):
            found.append((relative.as_posix(), item.read_bytes()))
    return found


def cargo(tree, metadata, target):
    wanted = {m.groups() for line in tree.read_text().splitlines() if (m := re.match(r"^([\w-]+) v([^ ()|]+)", line))}
    seen = set()
    for pkg in json.loads(metadata.read_text())["packages"]:
        name, version = pkg["name"], pkg["version"]
        if (name, version) not in wanted:
            continue
        seen.add((name, version))
        folder = Path(pkg["manifest_path"]).parent
        source = pkg["source"] or pkg.get("repository") or "Moriarty"
        if folder.is_relative_to(args.wasm_source):
            source = "https://github.com/midnightntwrk/midnight-ledger/tree/31c90b8bac8cc841bf94a741f0ec410511f85e32"
        key = (name, version, source)
        record = components.setdefault(key, {"name": name, "version": version, "source": source, "license": pkg["license"],
                                             "authors": pkg["authors"], "targets": [], "texts": []})
        record["targets"].append(target)
        if record["texts"]:
            continue
        texts = original_texts(folder)
        if pkg["license_file"]:
            license_file = folder/pkg["license_file"]
            texts.append((license_file.name, license_file.read_bytes()))
        # Include the workspace-root license even for crates which also vendor a third-party license.
        if name.startswith("midnight-"):
            texts.append(("midnight-ledger/LICENSE", (args.wasm_source/"LICENSE").read_bytes()))
        elif name == "mori-recursion":
            texts.append(("Moriarty/LICENSE", (root/"LICENSE").read_bytes()))
        if name in supplemental:
            local, source_url = supplemental[name]
            texts.append((source_url, (here/"upstream-notices"/(local+".txt")).read_bytes()))
        if name in ("enum_index", "enum_index_derive"):
            # The published crates and upstream repository declare MIT and identify the author,
            # without supplying a standalone copyright notice. Preserve that declaration verbatim.
            declaration = (folder/"Cargo.toml").read_bytes()
            terms = (rust_docs/"licenses/MIT.txt").read_bytes().split(b"Permission is hereby granted",1)[1]
            texts.extend([("Cargo.toml", declaration), ("MIT permission and warranty terms", b"Permission is hereby granted"+terms)])
        if not texts:
            raise ValueError(f"missing license text for {key}")
        for label, data in texts:
            attach(record, label, data)
    if seen != wanted:
        raise ValueError(f"incomplete tree: {wanted - seen}")


with TemporaryDirectory() as temporary:
    for prefix, cwd, arch, extra, label in [
            ("native", root, "aarch64-unknown-linux-gnu", ["-p","mori-recursion","--features","sherlock-ledger-validation"], "native Linux arm64 verifier"),
            ("wasm", args.wasm_source, "wasm32-unknown-unknown", ["-p","midnight-ledger-wasm-v9","-p","midnight-onchain-runtime-wasm"], "ledger/onchain WASM bindings")]:
        tree = getattr(args, prefix+"_tree")
        metadata = getattr(args, prefix+"_metadata")
        if tree is None:
            tree = Path(temporary)/(prefix+"-tree")
            tree.write_bytes(subprocess.check_output(["cargo","tree","--locked","--target",arch,*extra,
                                                      "-e","normal,build","--prefix","none","--format","{p}|{l}"],cwd=cwd))
        if metadata is None:
            metadata = Path(temporary)/(prefix+"-metadata")
            features = ["--features","mori-recursion/sherlock-ledger-validation"] if prefix=="native" else []
            metadata.write_bytes(subprocess.check_output(["cargo","metadata","--locked","--format-version","1",
                                                          "--filter-platform",arch,*features],cwd=cwd))
        cargo(tree, metadata, label)
for name, version, folder in (("@midnightntwrk/ledger-v9","1.0.0","ledger-wasm"),
                              ("@midnightntwrk/onchain-runtime-v4","4.0.0","onchain-runtime-wasm"),
                              ("@midnight-ntwrk/compact-runtime","0.21.0","")):
    pkg = json.loads((args.sdk/name/"package.json").read_bytes())
    assert pkg["version"] == version
    record = {"name":name,"version":version,"source":pkg["repository"]["url"],"license":"Apache-2.0","targets":["npm"],"texts":[]}
    attach(record,"midnight-ledger/LICENSE",(args.wasm_source/"LICENSE").read_bytes())
    source = args.wasm_source/folder/"src/lib.rs" if folder else args.sdk/name/"dist/index.js"
    header = [line for line in source.read_bytes().splitlines()[:22] if line.startswith(b"//")]
    assert any(b"Copyright" in l for l in header)
    attach(record,folder+"/src/lib.rs header" if folder else "dist/index.js header",b"\n".join(header)+b"\n")
    components[(name,version,"npm")] = record
rust = {"name":"Rust standard library","version":subprocess.check_output(["rustc","--version"],text=True).strip(),
        "source":"https://github.com/rust-lang/rust","targets":["native Linux arm64 verifier"],"texts":[]}
attach(rust,"COPYRIGHT-library.html",(rust_docs/"COPYRIGHT-library.html").read_bytes())
components[("rust-std",rust["version"],"toolchain")] = rust
index = {"scope":"Resolved normal and build dependencies, Rust standard library notices and direct Midnight npm bindings. Other npm notices accompany their packages.",
         "wasmSource":"31c90b8bac8cc841bf94a741f0ec410511f85e32", "components":[components[k] for k in sorted(components)]}
(out/"index.json").write_text(json.dumps(index,indent=2)+"\n")
print(f"{len(components)} components, {len(list((out/'texts').iterdir()))} distinct texts")
