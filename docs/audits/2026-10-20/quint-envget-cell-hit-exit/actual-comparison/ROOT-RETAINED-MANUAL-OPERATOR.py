import json,sys
from pathlib import Path
sys.path.insert(0,sys.argv[3])
from core2_await_boundary_wire import boundary_observations_equal
rust=json.loads(Path(sys.argv[1]).read_text())
quint=json.loads(Path(sys.argv[2]).read_text())
left=rust['window']
assert set(left)=={'before','after_cell','after_exit','events'} and set(quint)==set(left)
answer=boundary_observations_equal(left,quint)
print(json.dumps({'wholeWindowEqual':answer,'genuineRustSelector':'$.window','computedQuintSelector':'$','hostCount':3,'eventCount':2,'existingWholePrefixDisposition':'MISMATCH retained','comparisonKind':'actual-to-actual finite whole native window'}))
