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


import json

IDS = {"first": "FirstFillAttempt", "second": "SecondFillAttempt", "cancel": "CancelAttempt",
       "fresh-cancel": "FreshCancelAttempt", "recovery": "RecoveryAttempt", "fund1": "FundingOneAttempt",
       "fund2": "FundingTwoAttempt", "disposition": "DispositionAttempt"}
MODES = dict(zip(I_MODES, ("Choice2I", "Timeout100I", "Timeout101I", "Refuse100I", "Refuse101I")))
EMODES = {"normal": "NormalEvidenceA4", "parent": "ParentEvidenceA4",
          "signer": "WrongSignerEvidenceA4", "nonce": "WrongNonceEvidenceA4"}

def quint_instruction(selector):
    parts = selector.split(":")
    head, verb = parts[:2]
    if head == "I":
        simple = {"prepare-parent": "PrepareParentI", "sign-parent": "SignParentI",
                  "prepare-recovery": "PrepareRecoveryI", "sign-recovery": "SignRecoveryI",
                  "advance": "AdvanceI", "reject-recovery": "RejectRecoveryI"}
        cmd = simple.get(verb)
        if cmd is None:
            tag = {"propose": "ProposeI", "verify": "VerifyI", "commit": "CommitI", "reject-stale": "RejectStaleI"}[verb]
            cmd = f"{tag}({IDS[parts[2]]})"
        return f"LifecycleIA4({cmd})"
    if head == "S":
        if verb in ("prepare", "sign"):
            cmd = f"{'PrepareS' if verb == 'prepare' else 'SignS'}({{id: {IDS[parts[2]]}, principal: {parts[3]}}})"
        elif verb == "advance":
            cmd = f"AdvanceS(Time{parts[2]})"
        else:
            tag = {"propose": "ProposeS", "verify": "VerifyS", "commit": "CommitS",
                   "reject-proposed": "RejectProposedS", "reject-verified": "RejectVerifiedS"}[verb]
            cmd = f"{tag}({IDS[parts[2]]})"
        return f"LifecycleSA4({cmd})"
    if head == "P":
        if verb == "cancel-parent": return "ParentCancellationA4"
        if verb == "slot": return f"ParentSlotA4({int(parts[2])})"
        return f"{'ReplayProposalA4' if verb == 'propose' else 'ReplayCommitA4'}({IDS[parts[2]]})"
    if head == "M":
        return f"MutateA4({'ProposedDerivationA4' if verb == 'proposed' else 'VerifiedDerivationA4'})"
    if head == "C" and verb == "propose":
        return f"DirectLifecycleIA4(ProposeI({IDS[parts[2]]}))"
    if head == "C" and verb in ("prepare", "sign"):
        tag = "PrepareS" if verb == "prepare" else "SignS"
        return f"DirectLifecycleSA4({tag}({{id: {IDS[parts[2]]}, principal: {parts[3]}}}))"
    simple = {"prepare-recovery": "LifecycleIA4(PrepareRecoveryI)", "prepare-parent": "DirectLifecycleIA4(PrepareParentI)",
              "financial-recovery": "RecoveryFinancialA4", "duplicate-cancel": "DuplicateCancellationA4",
              "sign-disposition": "StaleSignA4", "plan": "BadPlanA4", "bad-prepare": "BadPrepareA4",
              "bad-plan-propose": "BadProposeA4", "core": "WrongCoreA4"}
    if verb in simple: return simple[verb]
    if verb == "reject-verified": return f"RejectConstructedA4({IDS[parts[2]]})"
    if verb in ("verify", "reject"):
        tag = "VerifyEvidenceA4" if verb == "verify" else "RejectEvidenceA4"
        return f"{tag}({{id: {IDS[parts[2]]}, mode: {EMODES[parts[3]]}}})"
    raise ValueError(f"unknown selector {selector}")

def quint_scenario(desc):
    name = desc["scenario"]
    if desc["lifecycle"] == "installment":
        if name == "two-fills": return "InstallmentScenarioA4(TwoFillsI)"
        _, residual, mode = name.split("-", 2)
        return f"InstallmentScenarioA4(RecoverI({{residual: {'true' if residual == 'r1' else 'false'}, mode: {MODES[mode]}}}))"
    funded = int(name[6])
    mode = name.split("-", 1)[1]
    if mode == "settle": value = "SettleS"
    elif mode == "refund": value = "RefundS"
    elif mode.startswith("timeout"): value = f"TimeoutS(Time{mode[7:]})"
    else: value = f"RefuseS({{now: Time{mode[6:9]}, chosen: {mode[-1]}}})"
    return f"SwapScenarioA4({{funded: {funded}, mode: {value}}})"

def quint_case(desc, steps):
    descriptor = "{" + ", ".join(f"{key}: {json.dumps(desc[source])}" for key, source in
        (("caseId", "case_id"), ("lifecycle", "lifecycle"), ("profile", "profile"), ("scenario", "scenario"), ("control", "control"))) + "}"
    lowered = []
    for selector in steps[1:-1]:
        kind = "adversarial-derivation" if selector.startswith("M:") else "denied-probe" if selector.startswith(("D:", "P:")) else "transition"
        lowered.append(f'{{kind: "{kind}", instruction: {checked_quint_instruction(selector)}}}')
    return f"{{descriptor: {descriptor}, scenario: {quint_scenario(desc)}, profile: {desc['profile']}, steps: List(" + ",\n".join(lowered) + ")}"

def quint_cases():
    imports = "\n".join(f'  import {name}.* from "./{name}"' for name in
        ("effects", "consumption", "observations", "policies", "authorization", "execution",
         "candidate_a_types", "candidate_a_programs", "candidate_a_core", "candidate_a_projection",
         "candidate_a_authority_adapter", "candidate_a_authority_boundary",
         "candidate_a_authority_installment_fixtures", "candidate_a_authority_installment",
         "candidate_a_authority_swap_fixtures", "candidate_a_authority_swap", "candidate_a_integrated_observer",
         "candidate_a_integrated_lowering"))
    body = []
    for lifecycle, name in (("installment", "INSTALLMENT_CASES_A4"), ("swap", "SWAP_CASES_A4")):
        values = [quint_case(d, steps) for d, steps in instructions() if d["lifecycle"] == lifecycle]
        body.append(f"pure val {name}: List[A4Case] = List(\n" + ",\n".join(values) + ")")
    return "module candidate_a_integrated_cases {\n" + imports + "\n" + "\n".join(body) + "\n}\n"

def checked_quint_instruction(selector):
    allowed = {s for _, steps in instructions() for s in steps[1:-1]}
    if type(selector) is not str or selector not in allowed:
        raise ValueError("selector outside fixed inventory")
    return quint_instruction(selector)

def case_wrapper(global_index):
    from pathlib import Path
    rows = inventory()["cases"]
    if type(global_index) is not int or not 0 <= global_index < len(rows):
        raise ValueError("fixed case index")
    desc, steps = list(instructions())[global_index]
    if {**desc, "event_count": len(steps)} != rows[global_index]:
        raise ValueError("fixed case instruction correspondence")
    name = f"candidate_a_integrated_case_{global_index:03d}"
    template = (Path(__file__).resolve().parents[1] /
                "specs/quint/s02/candidate_a_integrated_driver.qnt").read_text(encoding="utf-8")
    header = "module candidate_a_integrated_driver {\n"
    if not template.startswith(header) or not template.endswith("}\n"):
        raise ValueError("driver template framing")
    body = template[len(header):-2]
    replacements = {
        "const CASE_A4: A4Case\n": f"pure val CASE_A4: A4Case = {quint_case(desc, steps)}\n",
        "const CASE_INDEX_A4: int\n": f"pure val CASE_INDEX_A4: int = {global_index}\n",
    }
    for original, literal in replacements.items():
        if body.count(original) != 1:
            raise ValueError("driver template parameter inventory")
        body = body.replace(original, literal)
    return f"module {name} {{\n" + body + "}\n"
