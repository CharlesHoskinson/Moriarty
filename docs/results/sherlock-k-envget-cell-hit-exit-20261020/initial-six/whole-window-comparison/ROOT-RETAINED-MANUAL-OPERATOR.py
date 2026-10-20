import json,sys
from pathlib import Path
sys.path.insert(0,sys.argv[3])
from core2_await_boundary_wire import boundary_observations_equal
rust=json.loads(Path(sys.argv[1]).read_text())
computed=json.loads(Path(sys.argv[2]).read_text())
assert set(rust['window'])=={'before','after_cell','after_exit','events'} and set(computed)==set(rust['window'])
answer=boundary_observations_equal(rust['window'],computed)
print(json.dumps({'wholeWindowEqual':answer,'genuineRustSelector':'$.window','computedKSelector':'$','hostCount':3,'eventCount':2,'existingWholePrefixDisposition':'MISMATCH retained','comparisonKind':'actual-to-actual finite whole native window'}))
