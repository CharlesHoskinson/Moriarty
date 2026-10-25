#!/usr/bin/env python3
"""Prepare a separate maintainer-only Loki package; never patch the public workspace."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tomllib


def toml_value(value):
    if isinstance(value, dict):
        return '{' + ', '.join(json.dumps(key) + ' = ' + toml_value(item) for key, item in value.items()) + '}'
    return json.dumps(value)


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def source_pins(root):
    files = subprocess.check_output(['git', '-C', str(root), 'ls-files', '-co', '--exclude-standard', '-z']).split(b'\0')
    return {raw.decode(): sha(root / raw.decode()) for raw in sorted(set(files)) if raw and (root / raw.decode()).is_file()}


def recursion_pins(root):
    files = [root / 'Cargo.toml', root / 'Cargo.lock']
    files.extend(path for path in (root / 'crates/mori-recursion').rglob('*')
                 if path.is_file() and path.suffix in ('.rs', '.toml', '.json'))
    return {str(path.relative_to(root)): sha(path) for path in sorted(files)}


def render_runner(source, prover, gpu):
    runner = source.read_text().replace('fn main() -> Result<()> {', 'fn rollup_main() -> Result<()> {', 1)
    runner = re.sub(r'include_bytes!\("([^"]+)"\)',
                    lambda match: 'include_bytes!(' + json.dumps(str((source.parent / match[1]).resolve())) + ')', runner)
    # The generated package is always Loki, even if an operator omits --config.
    # Missing identity is a compile error; wrong identity/features are a startup
    # error; reference-verifier modes are unconditionally refused by this wrapper.
    runner += '\nfn main() -> Result<()> {\n'
    runner += '    const ID: &str = env!("SHERLOCK_PROVER_ID", "Loki build requires the generated --config file");\n'
    runner += '    if ID != ' + json.dumps(prover) + ' || cfg!(feature="gpu") != ' + str(gpu).lower() + ' {\n'
    runner += '        return Err(error("Loki prover identity/features differ from preparation"));\n    }\n'
    runner += '    if std::env::args().any(|a| a == "verify" || a == "prefix-verify") {\n'
    runner += '        return Err(error("Loki requires the retained stock reference verifier"));\n    }\n'
    runner += '    rollup_main()\n}\n'
    return runner


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', choices=('prepare', 'stamp'))
    parser.add_argument('--loki', type=Path, default=Path('/home/charles/repos/loki-proofs'))
    parser.add_argument('--output', type=Path, default=Path('target/sherlock-loki'))
    parser.add_argument('--gpu', action='store_true')
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    output = args.output.resolve()
    loki = args.loki.resolve()
    if not output.is_relative_to(root / 'target'):
        parser.error('generated maintainer package must be under this worktree target directory')
    if args.mode == 'stamp':
        manifest = json.loads((output / 'preparation.json').read_bytes())
        if source_pins(loki) != manifest['loki_source_sha256']:
            parser.error('Loki source changed during build; retain this attempt and prepare a new build')
        binary = output / 'target/release/rollup_loki'
        identity = json.loads(subprocess.check_output([str(binary), 'identity']))
        if identity != {'prover': manifest['prover'], 'fixed_key_prefix': True, 'gpu': manifest['gpu']}:
            parser.error('built prover identity/features differ from preparation')
        if sha(root / 'crates/mori-recursion/examples/rollup.rs') != manifest['rollup_source_sha256']:
            parser.error('rollup source changed during build; prepare a new build')
        if sha(output / 'main.rs') != manifest['runner_source_sha256']:
            parser.error('generated runner changed during build')
        if recursion_pins(root) != manifest['recursion_source_sha256']:
            parser.error('recursion source changed during build; prepare a new build')
        manifest.update({'schema': 'sherlock-loki-build/1', 'binary_sha256': sha(binary),
                         'cargo_lock_sha256': sha(output / 'Cargo.lock'),
                         'rustc': subprocess.check_output(['rustc', '-Vv'], text=True),
                         'binary': str(binary), 'independent_verifier': 'retained stock B3 binary required'})
        (output / 'build.json').write_text(json.dumps(manifest, indent=2, sort_keys=True) + '\n')
        print(output / 'build.json')
        return
    output.mkdir(parents=True, exist_ok=False)
    workspace = tomllib.loads((root / 'Cargo.toml').read_text())
    crate = tomllib.loads((root / 'crates/mori-recursion/Cargo.toml').read_text())
    dependencies = {'mori-recursion': {'path': str(root / 'crates/mori-recursion'),
                                     'features': ['sherlock-rollup-prefix']}}
    enabled = crate['features']['sherlock-recursion']
    for name, local in crate['dependencies'].items():
        if 'dep:' + name not in enabled:
            continue
        inherited = workspace['workspace']['dependencies'][name]
        entry = {'version': inherited} if isinstance(inherited, str) else dict(inherited)
        entry['features'] = sorted(set(entry.get('features', []) + local.get('features', [])))
        dependencies[name] = entry
    patches = dict(workspace['patch']['crates-io'])
    patches['midnight-proofs'] = {'path': str(loki)}
    lines = ['[package]', 'name = "sherlock-rollup-loki"', 'version = "0.1.0"', 'edition = "2024"',
             '[workspace]', '[features]', 'default = ["sherlock-rollup-prefix"]',
             'sherlock-rollup-prefix = ["mori-recursion/sherlock-rollup-prefix"]',
             'gpu = ["midnight-proofs/cuda", "midnight-proofs/cuda-ntt"]', '[dependencies]']
    lines.extend(json.dumps(name) + ' = ' + toml_value(value) for name, value in dependencies.items())
    lines.append('[patch.crates-io]')
    lines.extend(json.dumps(name) + ' = ' + toml_value(value) for name, value in patches.items())
    lines.extend(['[[bin]]', 'name = "rollup_loki"', 'path = "main.rs"'])
    (output / 'Cargo.toml').write_text('\n'.join(lines) + '\n')
    head = subprocess.check_output(['git', '-C', str(loki), 'rev-parse', 'HEAD'], text=True).strip()
    prover = f'Loki {"GPU MSM/NTT" if args.gpu else "CPU"} {head}'
    source = root / 'crates/mori-recursion/examples/rollup.rs'
    (output / 'main.rs').write_text(render_runner(source, prover, args.gpu))
    config = {'build': {'target-dir': str(output / 'target'), 'jobs': 4},
              'env': {'SHERLOCK_PROVER_ID': {'value': prover, 'force': True}, 'RAYON_NUM_THREADS': '4'}}
    lines = []
    for section, values in config.items():
        lines.append(f'[{section}]')
        lines.extend(json.dumps(key) + ' = ' + toml_value(value) for key, value in values.items())
    (output / 'config.toml').write_text('\n'.join(lines) + '\n')
    preparation = {'schema': 'sherlock-loki-preparation/1', 'prover': prover, 'gpu': args.gpu,
                   'loki_head': head, 'loki_source_sha256': source_pins(loki),
                   'rollup_source_sha256': sha(source), 'runner_source_sha256': sha(output / 'main.rs'),
                   'recursion_source_sha256': recursion_pins(root),
                   'workspace_head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
                   'public_workspace_patch': False}
    (output / 'preparation.json').write_text(json.dumps(preparation, indent=2, sort_keys=True) + '\n')
    print(output / 'Cargo.toml')
    print('GPU feature must match preparation:', args.gpu)


if __name__ == '__main__':
    main()
