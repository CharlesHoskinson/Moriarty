#!/bin/bash
# usage: obuild.sh <context> [lake args...]   -- sync and build one oracle on odin
ctx=$1; shift
rsync -a --exclude .lake /home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/ odin:sherlock/formal-e2e/ || exit 9
ssh odin "cd ~/sherlock/formal-e2e/$ctx/lean && PATH=\$HOME/.elan/bin:\$PATH flock ~/sherlock/formal-e2e/.build.lock timeout 1500 systemd-run --user --scope -q -p MemoryMax=24G nice -n 10 lake build --wfail $* 2>&1 | grep -v '^✔\|^⣿\|Replayed' | tail -60"
