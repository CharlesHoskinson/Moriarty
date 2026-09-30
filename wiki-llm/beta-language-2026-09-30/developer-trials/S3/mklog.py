import subprocess,json
M='./node_modules/.bin/mori'
C=[
(f"{M} init starter-rerun-not-run",None,None,None),
]
C=[
("check src/pool_oracle.mori --json",0,"AuthoringChecked, 4 SpecifiedOnly actions","check_pool_oracle.json"),
("check src/swap_fee.mori --json",0,"AuthoringChecked, LocalS0","check_swap_fee.json"),
("inspect src/pool_oracle.mori",0,"atoms 50000/2400/150/50150/3000, no premises listed for specified actions","inspect_pool_oracle.txt"),
("inspect src/swap_fee.mori",0,"25075/75/25000 atoms, 4 premises, 4 unverified bindings","inspect_swap_fee.txt"),
("simulate src/swap_fee.mori --action settle_swap_fee --scenario scenarios/swap_fee_ok.json",0,"PreparedUnqualified; Trader 74925, Vault 30000, Protocol 75 (hand-derived)","sim_swap_fee_ok.json"),
("simulate src/swap_fee.mori --action settle_swap_fee --scenario scenarios/swap_fee_underfunded.json",1,"CoreRejected, no effects","sim_swap_fee_underfunded.json"),
("simulate src/pool_oracle.mori --action swap_eur_for_xau --scenario scenarios/swap_fee_ok.json",1,"Unsupported BETA_PROFILE_UNSUPPORTED","sim_pool_swap_eur_for_xau.json"),
("simulate src/pool_oracle.mori --action read_fix --scenario scenarios/swap_fee_ok.json",1,"Unsupported","sim_pool_read_fix.json"),
("expand src/pool_oracle.mori --action swap_eur_for_xau --scenario scenarios/swap_fee_ok.json",1,"Unsupported","expand_pool_swap.json"),
("test cases/good",0,"4 pass","test_good.json"),
("test cases/bad",0,"5 intended rejections match","test_bad.json"),
]
for n in ["bad_nominal_fee","bad_scale_spelling","bad_amm_unknown_asset","bad_amm_missing_fee_cap","bad_amm_floor_mixed_assets","probe_floor_wrong_asset","probe_output_same_as_input","probe_owner_is_pool","probe_oracle_odd_fields","probe_output_not_in_pool"]:
    C.append((f"check variants/{n}.mori --json",1,"AuthoringRejected",f"check_{n}.json"))
log=[]
for cmd,exp,obs,out in C:
    r=subprocess.run(f"{M} {cmd}",shell=True,capture_output=True,text=True)
    open('outputs/_rerun_'+out,'w').write(r.stdout+r.stderr)
    status=''
    try:
        j=json.loads(r.stdout); status=j.get('status','')
        d=j.get('diagnostics') or []
        if d: status+=' '+d[0].get('code','')
        if 'result' in j and 'rejection' in j['result']: status+=' '+j['result']['rejection']['code']
    except Exception: status='(non-JSON text)'
    log.append(dict(command=f"./node_modules/.bin/mori {cmd}",exit_code=r.returncode,expected=f"exit {exp}: {obs}",observed=f"exit {r.returncode}; {status}",output_file="outputs/"+out))
    assert r.returncode==exp,(cmd,r.returncode)
log.append(dict(command="./node_modules/.bin/mori init starter",exit_code=0,expected="starter created",observed="Initialized local-stipulation-only (files read, not reused)",output_file=None))
log.append(dict(command="./node_modules/.bin/mori fmt src/swap_fee.mori | fmt again",exit_code=0,expected="idempotent",observed="idempotent; expanded to multi-line; sourceHash changed",output_file="outputs/swap_fee.fmt.mori"))
log.append(dict(command="python3 mcp_client.py (real stdio MCP: initialize,tools/list,preview,check)",exit_code=0,expected="preview Prepared, check AuthoringChecked",observed="as expected",output_file="outputs/mcp_session.json"))
log.append(dict(command="mori test <copy of cases/good with balance 74900>",exit_code=1,expected="TestsFailed",observed="TestsFailed; no field-level diff shown",output_file="outputs/test_wrong_expectation.json"))
json.dump(log,open('command-log.json','w'),indent=1)
