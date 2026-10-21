"""Ordinary synthetic receipts only; no proof, compiler or target execution."""
import copy
import hashlib
import json
from pathlib import Path
import shutil
import tempfile
import unittest

from acceptance import assess, canonical_digest, InputError, _known_scalars
from metrics import METRIC_UNITS, required_metric_ids, event_latencies
from workloads.generate import stage_contract

HERE = Path(__file__).resolve().parent
H = lambda raw: hashlib.sha256(raw).hexdigest()


class Fixture:
    """Synthetic transport fixture using the REAL validated workload catalog."""
    def __init__(self, root, stage="initial-deployed-baseline", label="current", compiler="compiler"):
        self.root, self.label = Path(root), label
        self.root.mkdir(parents=True, exist_ok=True)
        shutil.copytree(HERE / "workloads", self.root / "workloads", dirs_exist_ok=True)
        self.catalog = json.loads((self.root / "workloads/catalog.json").read_text())
        cases = stage_contract(self.catalog, stage)["cases"]
        self.policy = dict(schema="sherlock-benchmark-acceptance-policy/2", stage=stage,
            catalog=dict(path="workloads/catalog.json", sha256=H((self.root / "workloads/catalog.json").read_bytes())),
            cases=[], limits={}, regression=dict(mode="initial_baseline", future_tolerances="Freeze explicit per-case tolerances before any future comparison."),
            observation_fields={}, status_values={name:[name] for name in ("success","failure","censored","not_run")},
            receipt_fields={}, terminal_values={name:[True] for name in ("host","prove","local_verify","deployment","inclusion","readback")},
            command_fields=dict(execution_id="/execution_id",mode="/mode"), command_role_values=dict(prove=["prove"],local_verify=["verify"]), proof_receipt_fields={}, statement_fields=dict(pre_state="/pre_state",post_state="/post_state"))
        self.current = dict(schema="sherlock-benchmark-current/2", cases={})
        self.samples=[]
        for case in cases:
            case=copy.deepcopy(case);case.update(min_successful_samples=1,max_failures=0,max_censored=0)
            self.policy["cases"].append(case)
            self.samples.append(self.make(case, compiler))

    def write(self, path, raw):
        target=self.root/path;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(raw)
        return H(raw)

    def pin(self, path, value):
        self.write(path,json.dumps(value).encode())
        return dict(path=path,algorithm="canonical-json-sha256")

    def add(self, sample, path, value, schema="fixture-native-receipt/1"):
        raw=value if isinstance(value,bytes) else json.dumps(value).encode()
        sha=self.write(path,raw)
        sample["receipts"].append(dict(path=path,sha256=sha,schema=schema))
        return dict(receipt_sha256=sha,pointer="")

    def metric(self, name, value, source, pointer):
        unit=METRIC_UNITS[name]
        scope="rpc_elapsed" if name.startswith("network.") else "circuit_metadata" if name.startswith("circuit.") else "serialized_artifact" if unit=="bytes" else "ledger_receipt" if unit=="DUST" else "subprocess"
        src=dict(receipt_sha256=source["receipt_sha256"],pointer=pointer,scope=scope)
        if unit=="seconds":src["timer_resolution_seconds"]=0.001
        return dict(value=value,unit=unit,status="measured",source=src,reason=None)

    def make(self, case, compiler):
        name=case["workload_id"];host=case["evidence_class"]=="host"
        prefix=name+"/";identity=dict(profile="fixture/1",network=None if host else "stagenet-fixture")
        pins={}
        for key, value in (("source_sha256","source"),("compiler_sha256",compiler),("ledger_sha256","ledger"),("circuit_sha256","terminal-circuit"),("environment_sha256","same-machine"),("toolchain_sha256","same-toolchain")):
            if host and key in ("ledger_sha256","circuit_sha256"):
                identity[key]=None;continue
            path=prefix+key+".bin";identity[key]=self.write(path,value.encode());pins[key]=dict(path=path,algorithm="sha256")
        context=dict(identity=identity,identity_pins=pins,workload_digest_sha256=case["workload_digest_sha256"],
            workload_pin=dict(path="workloads/catalog.json",algorithm="canonical-json-sha256",pointer=f"/cases/{[c['workload_id'] for c in self.catalog['cases']].index(name)}/inputs"),
            expected_effects_independence="independent")
        if host:
            document=json.loads((self.root/"workloads"/case["expected_effects_path"]).read_text());effects=document["value"]
            context.update(expected_effects_scope="host-terminal-value",expected_effects_pin=dict(path="workloads/"+case["expected_effects_path"],algorithm="canonical-json-sha256",pointer="/value"))
        else:
            context.update(target_id="fixture-target",anchor_id="fixture-anchor",expected_effects_scope="complete-deployed-effects",
                expected_effect_root_pointers={role:"/roots/"+role for role in case["required_effect_root_roles"]},expected_state_pointers=dict(pre="/pre_state",post="/post_state"))
            count=2 if name=="sherlock-linked-two-step" else int(name.rsplit("-",1)[1])
            frozen=json.loads((self.root/"workloads"/case["expected_effects_path"]).read_text())
            if name=="sherlock-linked-two-step":
                states=[dict(control="Eval(Literal(7))",fuel=1),dict(control="Return(UInt(7))",fuel=0),dict(control="StepDone(UInt(7))",fuel=0)]
                semantic={key:frozen[key] for key in ("literal","program","ordered_transitions","work")}
            else:
                corpus=json.loads((self.root/"workloads"/case["inputs"][0]["path"]).read_text())
                def complete(state):
                    return {key:(H(key.encode()) if isinstance(value,dict) else value) for key,value in state.items()}
                states=[complete(corpus["batches"][0]["state_pre"])]+[complete(batch["state_post"]) for batch in corpus["batches"]]
                semantic={key:frozen[key] for key in ("balances","batches","ordered_transaction_ids","state","totals")}
            scalar_values=_known_scalars(semantic)
            context["catalog_scalar_pointers"]={key:"/catalog_scalars/"+str(index) for index,key in enumerate(scalar_values)}
            context["transition_state_pointers"]=[dict(pre="/transition_states/"+str(i),post="/transition_states/"+str(i+1)) for i in range(count)]
            context["state_scalar_fields"]={key:key for state in states for key in _known_scalars(state)}
            for state in states:state["whole_context"]={"preserved":[1,2],"optional":None}
            effects=dict(ordered_effects=[dict(step=i) for i in range(count)],pre_state=states[0],post_state=states[-1],preserved_state={"reserve":4},fees_dust=2,residual_duties=[],optional_result=None,roots={role:H(role.encode()) for role in case["required_effect_root_roles"]})
            effects["catalog_scalars"]={str(index):value for index,value in enumerate(scalar_values.values())}
            effects["transition_states"]=states
            context["expected_effects_pin"]=self.pin(prefix+"expected.json",effects)
        context["expected_effects_sha256"]=canonical_digest(effects)
        self.current["cases"][name]=context
        sample=dict(schema="sherlock-benchmark-sample/1",sample_id=self.label+"-"+name,workload_id=name,workload_digest_sha256=case["workload_digest_sha256"],evidence_class=case["evidence_class"],status="success",identity=copy.deepcopy(identity),cache_state="cold",metrics={},layers={},receipts=[],effects=dict(expected_sha256=canonical_digest(effects),observed_sha256=canonical_digest(effects),match=True),events={key:None for key in ("proof_ready","submission","ack","inclusion","readback")},extensions={})
        extension=dict(schema="sherlock-benchmark-sample-acceptance/2",observation_id=self.label+":"+name,receipt_roles={})
        sample["extensions"]["acceptance"]=extension
        if not host:
            sample["events"]={key:f"2026-10-21T00:00:0{i}Z" for i,key in enumerate(sample["events"])}
            extension.update(target_id=context["target_id"],anchor_id=context["anchor_id"],deployment_id="fixture-deployment",call_transaction_id=self.label+":"+name+":call",proof_nodes=[])
            nodes=extension["proof_nodes"]
            leaf_names=["child-0","child-1"] if name=="sherlock-linked-two-step" else [f"leaf-{i}" for i in range(count)]
            plan=[(leaf,"child-"+str(i) if name=="sherlock-linked-two-step" else "leaf",states[i],states[i+1],[]) for i,leaf in enumerate(leaf_names)]
            plan += [("parent","parent",states[0],states[-1],leaf_names),("decider","decider",states[0],states[-1],["parent"])]
            by_name={}
            for node_id,layer,pre,post,input_ids in plan:
                proof=self.add(sample,prefix+node_id+"-proof.bin",("proof:"+self.label+name+node_id).encode(),None)["receipt_sha256"]
                vk=self.add(sample,prefix+node_id+"-vk.bin",("vk:"+layer).encode(),None)["receipt_sha256"]
                statement=self.add(sample,prefix+node_id+"-statement.json",dict(pre_state=pre,post_state=post,node=node_id),"fixture-public-statement/1")["receipt_sha256"]
                node=dict(node_id=node_id,layer=layer,operation_id=self.label+":"+node_id,circuit_sha256=identity["circuit_sha256"] if layer=="decider" else H(("circuit:"+layer).encode()),proof_sha256=proof,statement_sha256=statement,verifying_key_sha256=vk,pre_state_sha256=canonical_digest(pre),post_state_sha256=canonical_digest(post),circuit_k=10,inputs=[{k:v for k,v in by_name[child].items() if k in {"node_id","layer","circuit_sha256","proof_sha256","statement_sha256","verifying_key_sha256","pre_state_sha256","post_state_sha256"}} for child in input_ids],receipt_roles={})
                values={metric:(10 if metric=="circuit.k" else 1024 if metric=="circuit.domain_capacity_rows" else next((len((self.root/ref["path"]).read_bytes()) for ref in sample["receipts"] if ref["sha256"]==node[field]),1) if (field:={"proof.bytes":"proof_sha256","verifying_key.bytes":"verifying_key_sha256","instance.bytes":"statement_sha256"}.get(metric)) else 1) for metric in {m.split(":",2)[2] for m in required_metric_ids(case) if m.startswith("layer:"+layer+":")}}
                for role in ("prove","local_verify"):
                    facts=dict(identity);facts.update({key:value for key,value in node.items() if key!="receipt_roles"});facts.update(workload_digest_sha256=case["workload_digest_sha256"],success=True,exit_code=0,metrics=values, execution_id=self.label+":"+node_id+":"+role,mode="verify" if role=="local_verify" else "prove",timings={metric:{"start":0,"end":value} for metric,value in values.items() if metric.endswith("wall_seconds")})
                    binding=self.add(sample,prefix+node_id+"-"+role+".json",facts)
                    node["receipt_roles"][role]=binding
                    self.policy["proof_receipt_fields"][role]={key:"/"+key for key in facts if key not in ("exit_code","metrics")}
                nodes.append(node);by_name[node_id]=node
                if layer not in sample["layers"]:
                    sample["layers"][layer]={metric:self.metric(metric,value,node["receipt_roles"]["local_verify" if metric.startswith("local_verify.") else "prove"],"/metrics/"+metric) for metric,value in values.items()}
            for key in ("proof_sha256","statement_sha256","verifying_key_sha256"):extension[key]=nodes[-1][key]
            if count > 1 and name != "sherlock-linked-two-step":
                leaves=[node for node in nodes if node["layer"]=="leaf"]
                values={metric:entry["value"] for metric,entry in sample["layers"]["leaf"].items()}
                lengths={ref["sha256"]:len((self.root/ref["path"]).read_bytes()) for ref in sample["receipts"]}
                for metric,field in (("proof.bytes","proof_sha256"),("verifying_key.bytes","verifying_key_sha256"),("instance.bytes","statement_sha256")):
                    values[metric]=sum(lengths[sha] for sha in {node[field] for node in leaves}) if field=="verifying_key_sha256" else sum(lengths[node[field]] for node in leaves)
                aggregation={}
                for metric in values:
                    contributors=[]
                    for node in leaves:
                        role="local_verify" if metric.startswith("local_verify.") else "prove"
                        source=dict(node["receipt_roles"][role],pointer="/metrics/"+metric)
                        contributors.append(dict(node_id=node["node_id"],source=source,start="/timings/"+metric+"/start",end="/timings/"+metric+"/end"))
                    mode="observed_parallel_span" if metric.endswith("wall_seconds") else "sum" if metric.endswith("cpu_seconds") else "maximum" if metric.endswith("peak_rss_bytes") else "common" if metric.startswith("circuit.") else "unique-verifying-keys" if metric=="verifying_key.bytes" else "sum"
                    scope="all-member-command-span" if metric.endswith("wall_seconds") else "cumulative-member-subprocess-cpu" if metric.endswith("cpu_seconds") else "maximum-member-subprocess-peak-rss" if metric.endswith("peak_rss_bytes") else "serialized-member-artifacts"
                    if metric.endswith("cpu_seconds"):values[metric]=len(leaves)
                    elif METRIC_UNITS[metric]=="bytes" and metric not in ("verifying_key.bytes","proof.bytes","instance.bytes") and not metric.endswith("peak_rss_bytes"):values[metric]=len(leaves)
                    aggregation[metric]=dict(mode=mode,scope=scope,contributors=contributors,start="/window/start",end="/window/end")
                body=dict(identity,workload_digest_sha256=case["workload_digest_sha256"],layer="leaf",nodes=[{k:v for k,v in node.items() if k in {"node_id","layer","circuit_sha256","proof_sha256","statement_sha256","verifying_key_sha256","pre_state_sha256","post_state_sha256"}} for node in leaves],success=True,metrics=values,aggregation=aggregation,window=dict(start=0,end=1))
                binding=self.add(sample,prefix+"leaf-group.json",body)
                extension["layer_observations"]={"leaf":binding}
                self.policy["layer_receipt_fields"]={key:"/"+key for key in body if key!="metrics"}
                self.policy["terminal_values"]["layer_observation"]=[True]
                sample["layers"]["leaf"]={metric:self.metric(metric,value,binding,"/metrics/"+metric) for metric,value in values.items()}
        facts=dict(identity,workload_digest_sha256=case["workload_digest_sha256"],observation_id=extension["observation_id"],cache_state="cold",status="success")
        whole_values={metric:1 for metric in required_metric_ids(case) if not metric.startswith("layer:")}
        if not host:whole_values.update(event_latencies(sample))
        facts["metrics"]=whole_values
        extension["observation"]=self.add(sample,prefix+"observation.json",facts)
        self.policy["observation_fields"]={key:"/"+key for key in facts if key!="metrics"}
        self.policy["metric_context_fields"]={key:"/"+key for key in identity if key!="circuit_sha256"}
        self.policy["metric_context_fields"]["workload_digest_sha256"]="/workload_digest_sha256"
        sample["metrics"]={metric:self.metric(metric,value,extension["observation"],"/metrics/"+metric) for metric,value in whole_values.items()}
        facts=dict(identity,workload_digest_sha256=case["workload_digest_sha256"],effects=effects,outcome="Halted",success=True)
        if not host:
            facts.update({key:extension[key] for key in ("target_id","anchor_id","deployment_id","call_transaction_id","proof_sha256","statement_sha256","verifying_key_sha256")});facts.update(sample["events"])
        for role in (["host"] if host else ["prove","local_verify","deployment","inclusion","readback"]):
            binding=self.add(sample,prefix+role+".json",dict(facts,role=role,exit_code=0,execution_id=self.label+":terminal:"+role,mode="verify" if role=="local_verify" else "prove"))
            extension["receipt_roles"][role]=binding
            self.policy["receipt_fields"][role]={key:"/"+key for key in facts}
        return sample

    def assess(self,baseline=None):return assess(self.samples,self.policy,self.current,self.root,baseline)

    def replace_receipt(self,sample,binding,mutate):
        ref=next(ref for ref in sample["receipts"] if ref["sha256"]==binding["receipt_sha256"])
        path=self.root/ref["path"];value=json.loads(path.read_text());mutate(value);raw=json.dumps(value).encode();path.write_bytes(raw)
        old=ref["sha256"];new=H(raw);ref["sha256"]=new
        def update(value):
            if isinstance(value,dict):
                for key,child in value.items():
                    if key=="receipt_sha256" and child==old:value[key]=new
                    else:update(child)
            elif isinstance(value,list):
                for child in value:update(child)
        update(sample)


class AcceptanceTests(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();self.addCleanup(self.tmp.cleanup);self.f=Fixture(self.tmp.name)

    def test_real_catalog_policy_with_all_layer_prefixed_metrics_passes(self):
        result=self.f.assess();self.assertTrue(result["complete"],result)
        self.assertFalse(result["performance_target_satisfied"]);self.assertFalse(result["full_goal_complete"])

    def test_catalog_kind_metrics_case_and_layer_omissions_cannot_pass(self):
        for mutate in [lambda p:p["cases"][0].update(kind="moriarty-nominal-source"),lambda p:p["cases"][0].update(required_metrics=["prove.wall_seconds"]),lambda p:p.update(cases=[]),lambda p:p["cases"][0].update(required_layers=[])]:
            with self.subTest(mutate=mutate):
                policy=copy.deepcopy(self.f.policy);mutate(policy)
                self.assertFalse(assess(self.f.samples,policy,self.f.current,self.f.root)["complete"])

    def test_initial_baseline_and_full_series_are_distinct(self):
        self.f.policy["stage"]="full-rollup-series"
        self.assertFalse(self.f.assess()["complete"])

    def test_host_compiler_stage_uses_host_receipts_without_network(self):
        fixture=Fixture(Path(self.tmp.name)/"host","host-compiler-coverage")
        result=fixture.assess();self.assertTrue(result["complete"],result)
        self.assertTrue(all(s["identity"]["ledger_sha256"] is None and not s["events"]["submission"] for s in fixture.samples))

    def test_full_series_and_full_suite_contracts_have_positive_fixture_paths(self):
        for stage in ("full-rollup-series","full-suite"):
            fixture=Fixture(Path(self.tmp.name)/stage,stage)
            result=fixture.assess();self.assertTrue(result["complete"],result)
            self.assertFalse(result["full_goal_complete"])

    def test_host_or_model_cannot_replace_deployed_recursive_chain(self):
        self.f.samples[0]["evidence_class"]="host"
        self.assertFalse(self.f.assess()["complete"])

    def test_environment_toolchain_pins_qualify_and_drift_invalidates(self):
        context=next(iter(self.f.current["cases"].values()))
        path=self.f.root/context["identity_pins"]["environment_sha256"]["path"];path.write_bytes(b"different-machine")
        self.assertFalse(self.f.assess()["complete"])

    def test_per_layer_false_verifier_even_exit_zero_is_incomplete(self):
        sample=self.f.samples[0];binding=sample["extensions"]["acceptance"]["proof_nodes"][0]["receipt_roles"]["local_verify"]
        self.f.replace_receipt(sample,binding,lambda d:d.update(success=False))
        self.assertFalse(self.f.assess()["complete"])

    def test_missing_child_proof_or_vk_artifact_is_incomplete(self):
        for field in ("proof_sha256","verifying_key_sha256","statement_sha256"):
            sample=copy.deepcopy(self.f.samples[0]);sample["extensions"]["acceptance"]["proof_nodes"][0][field]="f"*64
            self.assertFalse(assess([sample],self.f.policy,self.f.current,self.f.root)["complete"])

    def test_parent_child_statement_and_whole_state_continuity_required(self):
        sample=self.f.samples[0];sample["extensions"]["acceptance"]["proof_nodes"][2]["inputs"][0]["post_state_sha256"]="f"*64
        self.assertFalse(self.f.assess()["complete"])

    def test_unavailable_oracle_sentinel_cannot_claim_complete_effects(self):
        context=next(iter(self.f.current["cases"].values()));path=self.f.root/context["expected_effects_pin"]["path"]
        value=json.loads(path.read_text());value["roots"]["pre_state_root"]={"status":"unavailable","value":None};path.write_text(json.dumps(value));context["expected_effects_sha256"]=canonical_digest(value)
        self.assertFalse(self.f.assess()["complete"])

    def test_legitimate_null_option_values_remain_valid(self):
        self.assertTrue(self.f.assess()["complete"])

    def test_null_required_root_is_incomplete(self):
        context=next(iter(self.f.current["cases"].values()));path=self.f.root/context["expected_effects_pin"]["path"]
        value=json.loads(path.read_text());value["roots"]["ledger_state_root"]=None;path.write_text(json.dumps(value));context["expected_effects_sha256"]=canonical_digest(value)
        self.assertFalse(self.f.assess()["complete"])

    def test_network_duration_must_match_receipt_bound_events(self):
        self.f.samples[0]["metrics"]["network.submission_to_readback_seconds"]["value"]=0.001
        with self.assertRaises(InputError):self.f.assess()

    def test_unprefixed_limit_applies_to_matching_layer_metrics(self):
        self.f.policy["limits"]={"prove.wall_seconds":dict(max=0.5,unit="seconds")}
        self.assertFalse(self.f.assess()["complete"])

    def test_decimal_scaling_uses_original_receipt_tokens(self):
        sample=self.f.samples[0];metric=sample["layers"]["child-0"]["prove.wall_seconds"]
        binding=sample["extensions"]["acceptance"]["proof_nodes"][0]["receipt_roles"]["prove"]
        self.f.replace_receipt(sample,binding,lambda d:d["metrics"].update({"prove.wall_seconds":3}))
        metric["value"]=0.3;metric["source"]["scale"]=0.1
        self.assertTrue(self.f.assess()["complete"])

    def test_all_failure_identity_and_numeric_lineage_is_checked(self):
        failed=copy.deepcopy(self.f.samples[0]);failed.update(sample_id="failure",status="failure");failed["identity"]["source_sha256"]="f"*64
        self.f.policy["cases"][0]["max_failures"]=1;self.f.samples.append(failed)
        self.assertFalse(self.f.assess()["complete"])

    def test_duplicate_observation_is_retained_without_count_inflation(self):
        self.f.samples.append(copy.deepcopy(self.f.samples[0]))
        result=self.f.assess();self.assertTrue(result["complete"],result)
        self.assertEqual(result["cases"][0]["successful_observations"],1);self.assertEqual(result["cases"][0]["duplicate_observations"],1)
        self.f.policy["cases"][0]["min_successful_samples"]=2
        self.assertFalse(self.f.assess()["complete"])

    def test_historical_optimized_compiler_uses_own_identity_pins(self):
        prior=Fixture(self.f.root/"prior",label="prior",compiler="old-compiler")
        self.f.policy["regression"]={"mode":"compare","allowed_changed_subject_fields":["compiler_sha256"],"tolerances":{"sherlock-linked-two-step":{"layer:decider:prove.wall_seconds":{"statistic":"median","relative_increase":0,"absolute_increase":0,"min_samples":1}}}}
        baseline=dict(schema="sherlock-benchmark-baseline/2",root_prefix="prior",samples=prior.samples,policy=prior.policy,current=prior.current)
        result=self.f.assess(baseline);self.assertTrue(result["complete"],result)
        self.assertEqual(result["comparisons"][0]["changed_subject_fields"],["compiler_sha256"])
        context=next(iter(prior.current["cases"].values()));(prior.root/context["identity_pins"]["compiler_sha256"]["path"]).write_bytes(b"drift")
        self.assertFalse(self.f.assess(baseline)["complete"])

    def test_current_observation_cannot_supply_own_baseline(self):
        self.f.policy["regression"]={"mode":"compare","tolerances":{"sherlock-linked-two-step":{"layer:decider:prove.wall_seconds":{"statistic":"median","relative_increase":0,"absolute_increase":0,"min_samples":1}}}}
        baseline=dict(schema="sherlock-benchmark-baseline/2",root_prefix="self",samples=self.f.samples,policy=copy.deepcopy(self.f.policy),current=self.f.current)
        baseline["policy"]["regression"]=dict(mode="initial_baseline",future_tolerances="explicit future policy")
        shutil.copytree(self.f.root,self.f.root/"self",ignore=shutil.ignore_patterns("self"))
        self.assertFalse(self.f.assess(baseline)["complete"])

    def failure_sample(self,fixture,label="retained-failure"):
        failed=copy.deepcopy(fixture.samples[0]);failed.update(sample_id=label,status="failure")
        extension=failed["extensions"]["acceptance"]
        old=next(ref for ref in failed["receipts"] if ref["sha256"]==extension["observation"]["receipt_sha256"])
        body=json.loads((fixture.root/old["path"]).read_text());body.update(status="failure",observation_id=label)
        extension["observation_id"]=label
        extension["observation"]=fixture.add(failed,failed["workload_id"]+"/"+label+".json",body)
        for metric in failed["metrics"].values():metric["source"]["receipt_sha256"]=extension["observation"]["receipt_sha256"]
        return failed

    def test_retained_failed_baseline_does_not_break_shared_compare(self):
        prior=Fixture(self.f.root/"prior",label="prior",compiler="old-compiler")
        prior.policy["cases"][0]["max_failures"]=1
        prior.samples.append(self.failure_sample(prior))
        self.f.policy["regression"]={"mode":"compare","allowed_changed_subject_fields":["compiler_sha256"],"tolerances":{"sherlock-linked-two-step":{"layer:decider:prove.wall_seconds":{"statistic":"median","relative_increase":0,"absolute_increase":0,"min_samples":1}}}}
        baseline=dict(schema="sherlock-benchmark-baseline/2",root_prefix="prior",samples=prior.samples,policy=prior.policy,current=prior.current)
        result=self.f.assess(baseline);self.assertTrue(result["complete"],result)
        self.assertEqual(result["baseline_cases"][0]["counts"]["failure"],1)

    def test_failed_metric_receipt_context_and_value_are_checked(self):
        failed=self.failure_sample(self.f);failed["layers"]["child-0"]["prove.wall_seconds"]["value"]=999
        self.f.policy["cases"][0]["max_failures"]=1;self.f.samples.append(failed)
        self.assertFalse(self.f.assess()["complete"])

    def test_artifact_ingestion_bound_is_explicit_and_separate(self):
        self.f.policy["ingestion_limits"]={"max_artifact_bytes":1}
        with self.assertRaises(InputError):self.f.assess()
        self.f.policy["ingestion_limits"]["max_artifact_bytes"]=1024
        self.assertTrue(self.f.assess()["complete"])

    def test_missing_referenced_receipts_are_input_errors(self):
        sample=self.f.samples[0];ref=sample["receipts"][-1];path=self.f.root/ref["path"];path.unlink()
        with self.assertRaises(InputError):self.f.assess()

    def test_prover_result_cannot_be_its_verifier(self):
        node=self.f.samples[0]["extensions"]["acceptance"]["proof_nodes"][0]
        node["receipt_roles"]["local_verify"]=copy.deepcopy(node["receipt_roles"]["prove"])
        self.assertFalse(self.f.assess()["complete"])

    def test_distinct_command_results_in_one_bundle_are_valid(self):
        sample=self.f.samples[0];node=sample["extensions"]["acceptance"]["proof_nodes"][0]
        old=copy.deepcopy(node["receipt_roles"]);body={}
        for role,binding in old.items():
            ref=next(ref for ref in sample["receipts"] if ref["sha256"]==binding["receipt_sha256"])
            body[role]=json.loads((self.f.root/ref["path"]).read_text())
        bundle=self.f.add(sample,"bundle.json",dict(sample["identity"],workload_digest_sha256=sample["workload_digest_sha256"],runs=body))
        for role in old:
            node["receipt_roles"][role]=dict(bundle,pointer="/runs/"+role)
        for name,metric in sample["layers"]["child-0"].items():
            role="local_verify" if name.startswith("local_verify.") else "prove"
            metric["source"].update(receipt_sha256=bundle["receipt_sha256"],pointer="/runs/"+role+"/metrics/"+name)
        self.assertTrue(self.f.assess()["complete"])

    def test_catalog_scalar_mapping_and_actual_value_are_required(self):
        context=next(iter(self.f.current["cases"].values()))
        for mutate in (lambda c:c.update(catalog_scalar_pointers={}),lambda c:c["catalog_scalar_pointers"].update({"/literal":"/fees_dust"})):
            current=copy.deepcopy(self.f.current);mutate(next(iter(current["cases"].values())))
            self.assertFalse(assess(self.f.samples,self.f.policy,current,self.f.root)["complete"])

    def test_root_roles_have_distinct_fields_but_can_equal_values(self):
        context=next(iter(self.f.current["cases"].values()));roots=context["expected_effect_root_pointers"]
        roots["ledger_state_root"]=roots["post_state_root"]
        self.assertFalse(self.f.assess()["complete"])

    def test_linked_transition_scalar_progress_is_frozen(self):
        context=next(iter(self.f.current["cases"].values()))
        context["transition_state_pointers"][0]["post"]=context["transition_state_pointers"][0]["pre"]
        self.assertFalse(self.f.assess()["complete"])

    def test_multi_member_duration_must_match_actual_member_span(self):
        fixture=Fixture(self.f.root/"full","full-rollup-series")
        sample=next(s for s in fixture.samples if s["workload_id"]=="fenrir-transfer-batches-2")
        node=sample["extensions"]["acceptance"]["proof_nodes"][1]
        fixture.replace_receipt(sample,node["receipt_roles"]["prove"],lambda d:d["metrics"].update({"prove.wall_seconds":500}))
        self.assertFalse(fixture.assess()["complete"])

    def test_multi_member_aggregation_cannot_drop_a_member(self):
        fixture=Fixture(self.f.root/"full","full-rollup-series")
        sample=next(s for s in fixture.samples if s["workload_id"]=="fenrir-transfer-batches-2")
        binding=sample["extensions"]["acceptance"]["layer_observations"]["leaf"]
        fixture.replace_receipt(sample,binding,lambda d:d["aggregation"]["witness.wall_seconds"]["contributors"].pop())
        self.assertFalse(fixture.assess()["complete"])

    def test_failure_cannot_use_unbound_extra_numeric_receipt(self):
        failed=self.failure_sample(self.f);self.f.policy["cases"][0]["max_failures"]=1
        extra=self.f.add(failed,"extra.json",dict(failed["identity"],workload_digest_sha256=failed["workload_digest_sha256"],value=1))
        failed["metrics"]["compile.frontend.wall_seconds"]["source"].update(receipt_sha256=extra["receipt_sha256"],pointer="/value")
        self.f.samples.append(failed);self.assertFalse(self.f.assess()["complete"])

    def test_partial_failure_needs_no_successful_proof_or_deployment(self):
        failed=self.failure_sample(self.f);ext=failed["extensions"]["acceptance"]
        ext["receipt_roles"]={};ext["proof_nodes"]=[]
        failed["layers"]={};self.f.samples.append(failed);self.f.policy["cases"][0]["max_failures"]=1
        result=self.f.assess();self.assertTrue(result["complete"],result["cases"])
        self.assertEqual(result["cases"][0]["counts"]["failure"],1)

    def test_censored_partial_requires_observed_reason_without_success_artifacts(self):
        failed=self.failure_sample(self.f,"timeout");failed["status"]="censored"
        ext=failed["extensions"]["acceptance"];ext["receipt_roles"]={};ext["proof_nodes"]=[];failed["layers"]={}
        self.f.replace_receipt(failed,ext["observation"],lambda d:d.update(status="censored",censoring_reason="bounded command timeout"))
        self.f.policy["observation_fields"]["censoring_reason"]="/censoring_reason"
        self.f.samples.append(failed);self.f.policy["cases"][0]["max_censored"]=1
        self.assertTrue(self.f.assess()["complete"])

    def test_token_duplicates_never_enter_comparison_statistics(self):
        prior=Fixture(self.f.root/"prior",label="prior",compiler="old-compiler")
        name=self.f.samples[0]["workload_id"];metric="compile.frontend.wall_seconds"
        sample=self.f.samples[0];ext=sample["extensions"]["acceptance"]
        self.f.replace_receipt(sample,ext["observation"],lambda d:d["metrics"].update({metric:100}))
        sample["metrics"][metric]["value"]=100
        for i in range(2):
            duplicate=copy.deepcopy(sample);duplicate["sample_id"]="alias"+str(i);de=duplicate["extensions"]["acceptance"];de["observation_id"]="alias"+str(i)
            ref=next(r for r in duplicate["receipts"] if r["sha256"]==de["observation"]["receipt_sha256"])
            body=json.loads((self.f.root/ref["path"]).read_text());body.update(observation_id=de["observation_id"]);body["metrics"][metric]=1
            de["observation"]=self.f.add(duplicate,"alias"+str(i)+".json",body)
            for entry in duplicate["metrics"].values():entry["source"]["receipt_sha256"]=de["observation"]["receipt_sha256"]
            duplicate["metrics"][metric]["value"]=1;self.f.samples.append(duplicate)
        self.f.policy["regression"]={"mode":"compare","allowed_changed_subject_fields":["compiler_sha256"],"tolerances":{name:{metric:{"statistic":"median","relative_increase":0,"absolute_increase":0,"min_samples":1}}}}
        baseline=dict(schema="sherlock-benchmark-baseline/2",root_prefix="prior",samples=prior.samples,policy=prior.policy,current=prior.current)
        result=self.f.assess(baseline);self.assertFalse(result["complete"])
        self.assertEqual(result["cases"][0]["counts"]["success"],1)
        self.assertEqual(result["cases"][0]["duplicate_observations"],2)

    def test_comparison_reports_selected_scope_without_universal_sla(self):
        prior=Fixture(self.f.root/"prior",label="prior",compiler="old-compiler")
        name=self.f.samples[0]["workload_id"];metric="layer:decider:prove.wall_seconds"
        self.f.policy["regression"]={"mode":"compare","allowed_changed_subject_fields":["compiler_sha256"],"tolerances":{name:{metric:{"statistic":"median","relative_increase":0,"absolute_increase":0,"min_samples":1}}}}
        baseline=dict(schema="sherlock-benchmark-baseline/2",root_prefix="prior",samples=prior.samples,policy=prior.policy,current=prior.current)
        result=self.f.assess(baseline);self.assertTrue(result["complete"],result["findings"])
        self.assertEqual(result["comparison_scope"][0]["compared_metrics"],[metric])
        self.assertGreater(len(result["comparison_scope"][0]["measured_without_tolerance"]),1)
        self.assertFalse(result["performance_target_satisfied"])


    def test_comparison_requires_case_minimum_within_each_selected_cache_profile(self):
        prior=Fixture(self.f.root/"prior",label="prior",compiler="old-compiler")
        for fixture in (self.f,prior):
            second=Fixture(fixture.root/"second",label=fixture.label+"-second",compiler="compiler" if fixture is self.f else "old-compiler")
            sample=second.samples[0];ext=sample["extensions"]["acceptance"]
            sample["cache_state"]="warm";sample["cache_identity_sha256"]=H(b"cache")
            second.replace_receipt(sample,ext["observation"],lambda d:d.update(cache_state="warm",cache_identity_sha256=H(b"cache")))
            fixture.policy["observation_fields"]["cache_identity_sha256"]="/cache_identity_sha256"
            for ref in sample["receipts"]:ref["path"]="second/"+ref["path"]
            fixture.samples.append(sample);fixture.policy["cases"][0]["min_successful_samples"]=2
        name=self.f.samples[0]["workload_id"];metric="layer:decider:prove.wall_seconds"
        self.f.policy["regression"]={"mode":"compare","allowed_changed_subject_fields":["compiler_sha256"],"tolerances":{name:{metric:{"statistic":"median","relative_increase":0,"absolute_increase":0,"min_samples":1}}}}
        baseline=dict(schema="sherlock-benchmark-baseline/2",root_prefix="prior",samples=prior.samples,policy=prior.policy,current=prior.current)
        result=self.f.assess(baseline);self.assertFalse(result["complete"])
        self.assertIn("per-profile",str(result["findings"]))


    def test_witness_keygen_and_key_artifacts_require_their_own_layer_source(self):
        sample=self.f.samples[0];ext=sample["extensions"]["acceptance"]
        for name in ("witness.wall_seconds","keygen.cpu_seconds","proving_key.bytes","artifact.bytes"):
            other=copy.deepcopy(sample)
            other["layers"]["decider"][name]["source"].update(receipt_sha256=ext["proof_nodes"][0]["receipt_roles"]["prove"]["receipt_sha256"])
            self.assertFalse(assess([other],self.f.policy,self.f.current,self.f.root)["complete"],name)

    def test_failed_observation_cannot_be_reused_across_baseline_and_current(self):
        prior=Fixture(self.f.root/"prior",label="prior")
        for fixture in (self.f,prior):
            fixture.samples.append(self.failure_sample(fixture,"same-failed-observation"));fixture.policy["cases"][0]["max_failures"]=1
        name=self.f.samples[0]["workload_id"];metric="layer:decider:prove.wall_seconds"
        self.f.policy["regression"]={"mode":"compare","tolerances":{name:{metric:{"statistic":"median","relative_increase":0,"absolute_increase":0,"min_samples":1}}}}
        baseline=dict(schema="sherlock-benchmark-baseline/2",root_prefix="prior",samples=prior.samples,policy=prior.policy,current=prior.current)
        result=self.f.assess(baseline);self.assertFalse(result["complete"])
        self.assertIn("reused as historical",str(result["findings"]))


if __name__=="__main__":unittest.main()
