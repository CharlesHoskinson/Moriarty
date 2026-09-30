#!/bin/bash
# usage: run.sh id expected_exit "expected" "observed-note" cmd...
cd /tmp/moriarty-beta-developers-v3-20260930/S2/loan
id=$1; ee=$2; exp=$3; shift 3
out=outputs/$id.txt
mkdir -p ../outputs; out=../outputs/$id.txt
"$@" > $out 2>&1; ec=$?
node -e '
const fs=require("fs");const f="../command-log.json";
const a=fs.existsSync(f)?JSON.parse(fs.readFileSync(f)):[];
const o=fs.readFileSync(process.argv[5],"utf8");
a.push({command:process.argv[1],exit_code:+process.argv[2],expected:process.argv[3]+" (exit "+process.argv[6]+")",observed:o.replace(/\s+/g," ").slice(0,300),output_file:"outputs/"+process.argv[4]+".txt"});
fs.writeFileSync(f,JSON.stringify(a,null,1))' "$*" $ec "$exp" $id $out $ee
echo "[$id] exit=$ec (expected $ee)"; head -c 1500 $out | head -40
