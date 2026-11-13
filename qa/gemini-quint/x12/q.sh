#!/usr/bin/env bash
# Lead quint wrapper (lane rule 2026-11-12: quint is light; heavy-run in heavy.slice, no heavy lock,
# timeout 900 per command). Usage: q.sh NAME quint-args...
n="$1"; shift
QB=$HOME/.local/share/mise/installs/node/26.13.0/bin
exec "$HOME/.local/bin/heavy-run" --mem 4G --name "quint-$n" -- env PATH="$QB:$PATH" timeout 900 quint "$@"
