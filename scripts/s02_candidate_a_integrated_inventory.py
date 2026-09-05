from itertools import product

PROFILES = (("after", "SignAfterResolve"), ("before", "SignBeforeResolve"))
I_MODES = ("choice2", "timeout100", "timeout101", "refuse100", "refuse101")
I_CONTROLS = ("no-cancel", "unsigned", "old-nonce",
              "fresh-duplicate-cancel", "unused-successor")
S_MUTANTS = ("unused-successor", "reversed-effects", "reductions", "neutral-chooser")
S_CONTROLS = ("stale-signing", *S_MUTANTS, "second-plan",
              "wrong-Core-chooser", "wrong-signer", "wrong-nonce", "stale-facts")

def i_route(residual=None, mode=None):
    route = ["I:prepare-parent", "I:sign-parent", "I:propose:first",
             "I:propose:cancel", "I:verify:first", "I:verify:cancel"]
    if residual is None:
        return route + ["I:commit:first", "I:reject-stale:cancel",
                        "I:propose:second", "I:verify:second", "I:commit:second"]
    route += (["I:commit:first", "I:reject-stale:cancel"] if residual else
              ["I:commit:cancel", "I:reject-stale:first"])
    if residual:
        route += ["I:propose:fresh-cancel", "I:verify:fresh-cancel", "I:commit:fresh-cancel"]
    if mode != "choice2":
        route += ["I:advance"]
    route += ["I:prepare-recovery", "I:sign-recovery", "I:propose:recovery"]
    return route + (["I:reject-recovery"] if mode.startswith("refuse") else
                    ["I:verify:recovery", "I:commit:recovery"])

def s_funding(funded):
    route = []
    for number, owner in ((1, "Alice"), (2, "Bob")):
        if funded >= number:
            if number == 2:
                route += ["S:advance:2"]
            name = f"fund{number}"
            route += [f"S:prepare:{name}:{owner}", f"S:sign:{name}:{owner}",
                      f"S:propose:{name}", f"S:verify:{name}", f"S:commit:{name}"]
    return route

def s_route(funded, mode, time=2):
    route = s_funding(funded)
    if mode not in ("settle", "refund"):
        route += [f"S:advance:{time}"]
    if funded and mode != "refuse":
        for owner in ("Alice", "Bob")[:funded]:
            route += [f"S:prepare:disposition:{owner}", f"S:sign:disposition:{owner}"]
        return route + ["S:propose:disposition", "S:verify:disposition", "S:commit:disposition"]
    return route + ["S:propose:disposition", "S:reject-proposed:disposition"]

def replay(ids):
    return [f"P:{verb}:{name}" for name in ids for verb in ("propose", "commit")]

def scenarios():
    yield "installment", "two-fills", None, None, None
    for residual, mode in product((0, 1), I_MODES):
        yield "installment", f"recover-r{residual}-{mode}", residual, mode, None
    for mode in ("settle", "refund"):
        yield "swap", f"funded2-{mode}", 2, mode, 2
    for funded, time in product((0, 1, 2), (100, 101)):
        yield "swap", f"funded{funded}-timeout{time}", funded, "timeout", time
    for time, chosen in product((100, 101), (0, 1)):
        yield "swap", f"funded2-refuse{time}-choice{chosen}", 2, "refuse", time

def suffix(control, profile, lifecycle):
    if lifecycle == "installment":
        return {
            "no-cancel": ["D:prepare-recovery", "D:financial-recovery", "C:propose:recovery",
                                "D:verify:recovery:normal", "C:reject:recovery:normal"],
            "unsigned": ["C:propose:recovery", "D:verify:recovery:normal",
                                  "C:reject:recovery:normal", "D:prepare-parent"],
            "old-nonce": ["C:propose:recovery", "D:verify:recovery:parent",
                          "C:reject:recovery:parent"],
            "fresh-duplicate-cancel": ["C:duplicate-cancel", "D:verify:fresh-cancel:normal",
                                       "C:reject:fresh-cancel:normal"],
            "unused-successor": ["M:proposed", "D:verify:first:normal", "C:reject:first:normal",
                                 "M:verified", "P:commit:first", "C:reject-verified:first"],
        }[control]
    if control in S_MUTANTS:
        return ["M:proposed", "D:verify:disposition:normal", "C:reject:disposition:normal",
                "M:verified", "P:commit:disposition", "C:reject-verified:disposition"]
    return {
        "stale-signing": ["S:prepare:disposition:Alice", "S:advance:100", "D:sign-disposition"],
        "second-plan": ["D:plan", "D:bad-prepare" if profile == "after" else "C:bad-prepare"]
            + (["C:prepare:fund1:Alice"] if profile == "after" else [])
            + ["C:sign:fund1:Alice", "C:bad-plan-propose", "D:verify:fund1:normal", "C:reject:fund1:normal"],
        "wrong-Core-chooser": ["D:core"],
        "wrong-signer": ["D:verify:disposition:signer", "C:reject:disposition:signer"],
        "wrong-nonce": ["D:verify:disposition:nonce", "C:reject:disposition:nonce"],
        "stale-facts": ["M:proposed", "D:verify:disposition:normal", "C:reject:disposition:normal"],
    }[control]

def instructions():
    for lifecycle in ("installment", "swap"):
        for life, name, count, mode, time in scenarios():
            if life != lifecycle:
                continue
            route = i_route(count, mode) if life == "installment" else s_route(count, mode, time)
            if life == "installment":
                ids = (["first", "second"] if count is None else
                       (["first", "fresh-cancel"] if count else ["cancel"]) +
                       ([] if mode.startswith("refuse") else ["recovery"]))
                tail = replay(ids) + ([] if count is None else ["P:cancel-parent", "P:slot:1", "P:slot:2"])
            else:
                tail = replay([f"fund{i}" for i in range(1, count + 1)] + ["disposition"])
            for short, profile in PROFILES:
                yield descriptor(life, name, "ordinary", short, profile), ["B:start", *route, *tail, "B:end"]
        if lifecycle == "swap":
            stale = s_route(2, "settle")[:17] + ["S:advance:100", "P:commit:disposition", "S:reject-verified:disposition"]
            for short, profile in PROFILES:
                yield descriptor(lifecycle, "funded2-settle", "verified-stale", short, profile), ["B:start", *stale, "B:end"]
        for control in I_CONTROLS if lifecycle == "installment" else S_CONTROLS:
            scenario = ("two-fills" if control == "unused-successor" else "recover-r0-choice2") if lifecycle == "installment" else "funded2-settle"
            for short, profile in PROFILES:
                if lifecycle == "installment":
                    prefix = i_route()[:4] if control == "unused-successor" else i_route(0, "choice2")[:2 if control == "no-cancel" else 8]
                else:
                    count = 0 if control == "second-plan" else 11 if control in ("stale-signing", "wrong-Core-chooser") else 16
                    prefix = s_route(2, "settle")[:count]
                yield descriptor(lifecycle, scenario, control, short, profile), ["B:start", *prefix, *suffix(control, short, lifecycle), "B:end"]

def descriptor(lifecycle, scenario, control, short, profile):
    return {"case_id": f"{lifecycle}/{scenario}/{control}/{profile}", "lifecycle": lifecycle,
            "profile": profile, "scenario": scenario, "control": control}

def inventory():
    cases = [{**d, "event_count": len(steps)} for d, steps in instructions()]
    assert len(cases) == 78
    assert sum(c["event_count"] for c in cases) == 1557
    assert len({c["case_id"] for c in cases}) == len(cases)
    return {"schema_version": 2, "cases": cases}
