"""Independent public-policy checks. Run only on the finished frozen candidate."""
import copy
import json
from moriarty_dev.policy import assess

base = dict(authorityCurrent=True, entryEligible=True, candidateCurrent=True,
            resourceAdmitted=True, sameDefectFailures=0, adminCycles=0,
            adminSeconds=0, primaryActive=False, reproducerVerified=False,
            approachChanged=False, nextActionId="driver-reproduce", missingEvidence=[])
act = dict(id="driver-implement", requirement="SP05", capability="driver",
           kind="implement", candidate="candidate-a", admissionRef="admission.json",
           commandRef="commands.json#driver", evidenceProfile="local-runtime")
results = []

def check(name, changes, kind, wanted):
    snapshot = copy.deepcopy(base)
    snapshot.update(changes)
    action = dict(act, kind=kind)
    result = assess(snapshot, action)
    ok = isinstance(result, dict) and result.get("allow") is wanted
    results.append(dict(name=name, passed=ok, decision=result))

check("first implementation eligible", {}, "implement", True)
for field in ["authorityCurrent", "entryEligible", "candidateCurrent", "resourceAdmitted"]:
    check("missing " + field, {field: False}, "implement", False)
for failures in [0, 1, 2, 7, 100]:
    check(f"broad failures={failures}", {"sameDefectFailures": failures}, "implement", failures < 2)
    check(f"reproduce failures={failures}", {"sameDefectFailures": failures}, "reproduce", True)
for reproduced in [False, True]:
    for changed in [False, True]:
        check(f"repair {reproduced=} {changed=}", dict(sameDefectFailures=2,
              reproducerVerified=reproduced, approachChanged=changed), "repair", reproduced and changed)
for seconds in [0, 1799, 1800, 1801, 999999]:
    check(f"admin seconds={seconds}", {"adminSeconds": seconds}, "admin", seconds < 1800)
for cycles in [0, 1, 2, 3]:
    check(f"admin cycles={cycles}", {"adminCycles": cycles}, "admin", cycles < 2)
check("repair remains possible after administration", {"adminCycles": 2}, "repair", True)
check("report remains possible with stale authority", {"authorityCurrent": False}, "report", True)
check("second primary forbidden", {"primaryActive": True}, "implement", False)
check("repair also owns primary implementation slot", {"primaryActive": True}, "repair", False)
check("verified changed repair cannot bypass occupied primary slot", dict(primaryActive=True,
      sameDefectFailures=2, reproducerVerified=True, approachChanged=True), "repair", False)

for field in ["sameDefectFailures", "adminCycles", "adminSeconds"]:
    for malformed in [True, -1, 1.5, "0", None]:
        s = dict(base, **{field: malformed})
        try:
            decision = assess(s, act)
            ok = isinstance(decision, dict) and decision.get("allow") is False
        except (TypeError, ValueError):
            ok, decision = True, "boundary rejected"
        results.append(dict(name=f"malformed {field}={malformed!r}", passed=ok, decision=decision))

print(json.dumps({"passed": sum(r["passed"] for r in results), "total": len(results),
                  "failures": [r for r in results if not r["passed"]]}, indent=2))
raise SystemExit(0 if all(r["passed"] for r in results) else 1)
