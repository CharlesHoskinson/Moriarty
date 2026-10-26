"""Focused integration check: the runner's actual searches must find defects.

Runs the real Quint commands, without the unrelated positive model campaigns.
The domain-expansion regression selected a valid first trace under --seed;
this check must fail if a negative search silently stops after that one trace.
"""
from pathlib import Path
import re
import shlex
import subprocess

HERE = Path(__file__).resolve().parent
source = (HERE / "check.sh").read_text()
commands = re.findall(r"^if (quint run .*?) > \S+\.log 2>&1; then", source, re.M | re.S)
assert len(commands) == 3, f"expected three negative searches, found {len(commands)}"
for command in commands:
    argv = shlex.split(command.replace("\\\n", ""))
    result = subprocess.run(argv, cwd=HERE, capture_output=True, text=True)
    output = result.stdout + result.stderr
    assert result.returncode == 1 and "error: Invariant violated" in output, (
        f"{argv[2]} did not find the expected invariant violation (exit {result.returncode}):\n{output}"
    )
    print(f"{argv[2]}: expected invariant violation confirmed")
