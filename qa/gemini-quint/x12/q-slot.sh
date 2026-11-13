#!/usr/bin/env bash
# Lead quint wrapper through heavy-slot (T13 rule 2026-11-13: heavy jobs only through heavy-slot).
# Usage: q-slot.sh NAME quint-args...
n="$1"; shift
QB=$HOME/.local/share/mise/installs/node/26.13.0/bin
exec "$HOME/.local/bin/heavy-slot" --mem 4G --name "t13-$n" -- env PATH="$QB:$PATH" timeout 1800 quint "$@"
