"""Network selection and a read-only indexer client for the network-parametrised holdouts (E12).

Evaluator-owned. The registry is config/networks.json (sealed). Selectable networks are
undeployed (local Docker at ledger 9.1.0.1), preview and mainnet. Stagenet is retired
(owner decision 2026-11-08) and selecting it raises Retired; it is never a live target.

Nothing here opens a wallet, signs or submits. The only network traffic is GraphQL reads
against the selected indexer, made by the checks that need them.
"""
import hashlib
import json
import os
import re
import urllib.parse
import urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
REGISTRY_PATH = HERE.parent / "config" / "networks.json"
ENV = "DW_HOLDOUT_NETWORK"
HEX64 = re.compile(r"[0-9a-f]{64}")
GENESIS = re.compile(r"0x[0-9a-f]{64}")


class Retired(Exception):
    """The named network is retired; it is history, not a target."""


class Unknown(Exception):
    """The name is not a network in the registry."""


class Unreachable(Exception):
    """The indexer cannot be used from this host (no endpoint, no token, transport error)."""


def registry(path=REGISTRY_PATH):
    return json.loads(Path(path).read_text())


def select(name, reg=None):
    """Return (name, config) for a selectable network; raise Retired or Unknown otherwise."""
    reg = reg or registry()
    nets = reg["networks"]
    if name not in nets:
        raise Unknown(f"unknown network {name!r}; selectable: {', '.join(reg['selectable'])}")
    cfg = nets[name]
    if cfg.get("status") == "retired" or name not in reg["selectable"]:
        raise Retired(f"RETIRED: {name} is not a target network ({cfg.get('retired', 'retired')}); "
                      f"selectable: {', '.join(reg['selectable'])}")
    return name, cfg


def from_env(default, reg=None):
    """The network named by DW_HOLDOUT_NETWORK, else `default`."""
    return select(os.environ.get(ENV) or default, reg)


def sha(b):
    return hashlib.sha256(b).hexdigest()


class Indexer:
    """Read-only Midnight indexer (GraphQL v4 API) of one selected network."""

    def __init__(self, name, cfg, opener=None):
        self.name, self.cfg = name, cfg
        self._opener = opener or urllib.request.urlopen

    def url(self):
        base = os.environ.get(self.cfg["indexerEnv"]) if self.cfg.get("indexerEnv") else None
        base = base or self.cfg.get("indexer")
        if not base:
            raise Unreachable(f"{self.name}: no indexer endpoint")
        tok_env = self.cfg.get("indexerTokenEnv")
        if tok_env:
            tok = os.environ.get(tok_env)
            if not tok:
                raise Unreachable(f"{self.name}: indexer needs a project token in {tok_env} (supply it with vault run)")
            base += ("&" if "?" in base else "?") + urllib.parse.urlencode({"project_id": tok})
        return base

    def gql(self, query):
        req = urllib.request.Request(self.url(), json.dumps({"query": query}).encode(),
                                     {"content-type": "application/json"})
        try:
            with self._opener(req, timeout=30) as r:
                body = json.load(r)
        except OSError as e:
            raise Unreachable(f"{self.name} indexer unreachable: {e}") from e
        if body.get("errors"):
            raise RuntimeError("indexer error: %r" % (body["errors"],))
        return body["data"]

    def txs(self, h):
        return self.gql('{transactions(offset:{hash:"%s"}){hash raw block{height} '
                        '... on RegularTransaction{transactionResult{status}}}}' % h)["transactions"]

    def action_at(self, address, h):
        return self.gql('{contractAction(address:"%s",offset:{transactionOffset:{hash:"%s"}}){__typename state '
                        'transaction{hash} ... on ContractCall{entryPoint}}}' % (address, h))["contractAction"]

    def latest(self, address):
        return self.gql('{contractAction(address:"%s"){__typename state transaction{hash} '
                        '... on ContractCall{entryPoint}}}' % address)["contractAction"]

    sha = staticmethod(sha)


def check_genesis(cfg, recorded):
    """A pinned network must match its sealed genesis hash; a local one must at least record one."""
    if not isinstance(recorded, str) or not GENESIS.fullmatch(recorded):
        return f"genesis hash {recorded!r} is not 0x + 64 lowercase hex"
    if cfg.get("genesisHash") and recorded != cfg["genesisHash"]:
        return f"genesis hash {recorded[:18]}... is not the sealed {cfg['networkId']} genesis {cfg['genesisHash'][:18]}..."
    return None
