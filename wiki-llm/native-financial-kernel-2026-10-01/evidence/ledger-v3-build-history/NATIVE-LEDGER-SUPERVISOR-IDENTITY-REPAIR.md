# Supervisor source/binary identity receipt repair — source-only draft

## Observed defect and retained evidence

Fully inspected v3 wrapper, v3 build log/receipt, resource proposal, and independent native finalized-call/preparation contracts. Guarded status retains stale admission/accounting/history stops and no pending transactions. Actual build exited101 after9 Rust compiler errors; no executable was produced. Build receipt falsely says source_identity_preserved:false and stop_reason:"source identity changed during phase" because final source-verify block also unconditionally reads/hashes a missing executable against binary_pin:null. No evidence in that error establishes a source/runtime mutation.

Original wrapper SHA2562ce2ae7b80b4930de7a32b9093b9745b0ab8933fdad557703f94ce59c89a340b and original receipt SHA256c7d40c0f9311c5f2ce48e6cbdca1f3a1d52e6526197e586b89248c76d0e09758 remain untouched. Build took38.361s, sampledRSS1536049152bytes/CPU73.41s; target grew4515479003->5549887375bytes. The consumed reservation, actual compile errors and observed resource debt remain. This repair does not rewrite the old receipt or reset authority.

## Minimal repair

Separate final verify() source/runtime manifest rehash from executable identity. Source_identity_preserved reports verify() only, retaining its own error. Failed build has binary_artifact_status:not_produced_build_failed, produced_binary_sha256:null, binary_identity_preserved:null; a pre-existing stale target binary is never adopted as that failed build's artifact. Nonzero exit remains101 and authoritative failure even if stop_reason:null. Successful build still must produce a regular executable and freeze its real hash; unavailable artifact retains the existing executable-postcondition stop, not a false source stop. Frozen produced/prerequisite binary is rehashed separately after successful build or every helper completion; helper prelaunch priorbuild/binary checks are unchanged. Binary mismatch gets distinct binary_identity_error and binary stopreason, without changing source identity.

All caps, network namespace, command arrays, monitored limits, phase prerequisite, reservation/cleanup/signal ownership and artifact constraints remain unchanged. Draft retains OLD manifest/runtime pins and v3 phase names deliberately; it is NOT an executable v4 allocation. Root must integrate repaired source pin, actual runtime identity and new distinct v4 names, then freeze exact candidate for fresh substantive Astra/Grok source/resource votes. No target/sourcecaller/manifest mutation, compilation, helper/export, proof/key/SRS operation, or draft execution occurred. Python ast.parse syntax only is verified; lifecycle behavior not executed or accepted.

Expected receipt matrix (specified): compile101+unchangedsource => source:true, binary:null, not_produced_build_failed, exit101; compile0+missingartifact => source:true, binary:null, produced_identity_unavailable, executable-postcondition stop; helper failure+unchangedinputs/binary => source:true,binary:true with childfailure retained; source mutation => source:false independent of binary; binary mutation alone => source:true,binary:false and binary-specific stop. No state/proof/ledger acceptance follows these receipt predicates.

## Exact unified diff recipe

```diff
--- run-native-ledger-v3-bounded.py
+++ native-ledger-v4-wrapper-draft.py
@@ -155,6 +155,15 @@
   binary_pin=digest.hexdigest()
  except OSError as error:
   stop='produced executable identity unavailable';supervisor_error={'type':type(error).__name__,'message':str(error)}
+# Binary artifact identity is independent from source/runtime identity.
+# A failed build never adopts an existing/stale executable from the shared target.
+binary_identity_ok=None;binary_identity_error=None
+if phase=='build' and exitcode!=0:
+ binary_artifact_status='not_produced_build_failed'
+elif binary_pin is None:
+ binary_artifact_status='produced_identity_unavailable'
+else:
+ binary_artifact_status='produced_identity_frozen' if phase=='build' else 'prerequisite_binary_frozen'
 export_artifacts=None;helper_receipt_pin=None
 if phase=='export' and exitcode==0 and stop is None:
  try:
@@ -179,12 +188,25 @@
  except (AssertionError,OSError,ValueError,KeyError,TypeError) as error:
   stop='preparation postcondition refused';supervisor_error={'type':type(error).__name__,'message':str(error)}
 if phase in ('export','prepare') and size(export_dir)+size(prepare_dir)>32*M:stop=stop or 'final preparation output disk limit'
+source_identity_error=None
 try:
  verify()
- assert hashlib.sha256((target/'debug/beta-native-ledger-consumer').read_bytes()).hexdigest()==binary_pin,'binary identity changed during helper phase'
  identity_ok=True
-except (AssertionError,OSError,ValueError,TypeError,KeyError):identity_ok=False;stop=stop or 'source identity changed during phase'
-r={'command':cmd,'adapter_manifest_sha256':manifest_pin,'produced_binary_path':str(binary_path),'produced_binary_sha256':binary_pin,'initial_disk':initial,'final_disk':final,'exit_code':exitcode,'stop_reason':stop,'source_identity_preserved':identity_ok,'child_launched':p is not None,'supervisor_error':supervisor_error,'attempt_reservation':str(reservation),'elapsed_seconds':time.monotonic()-start,'peak_sampled_group_rss_bytes':peak,'peak_sampled_group_cpu_seconds':maxcpu,'global_nonloopback_received_byte_delta':final_rx-initial_rx,'disk_samples':samples,'limits':{'wall_seconds':wall,'group_cpu_seconds':cpu_limit,'group_rss_bytes':rss_limit,'per_process_address_space_bytes':rss_limit,'incremental_source_cache_bytes':G,'incremental_target_bytes':2*G,'total_target_bytes':8*G,'free_floor_bytes':10*G,'disk_sample_interval_seconds':5,'memory_cpu_network_sample_interval_seconds':.1,'cargo_jobs':2,'rayon_threads':2},'monitor_limitations':'RSS/CPU/traffic sampled at0.1s and disk at5s; stop/receipt records overshoot. Traffic is global nonloopback RX, a conservative upper bound including other host activity, not isolated Cargo body measurement. Build/check run in verified no-network user namespace.','qualification':'bounded compile or no-key typed native preparation only; no SRS/keygen/proof/well_formed/apply/Preview acceptance','helper_receipt_sha256':helper_receipt_pin,'export_artifacts_sha256':export_artifacts,'runtime_inputs_manifest_sha256':runtime_inputs_pin,'phase':phase,'config_sha256':config_pin if phase=='prepare' else None,'preparation_output_limit_bytes':32*M}
+except (AssertionError,OSError,ValueError,TypeError,KeyError) as error:
+ identity_ok=False;stop=stop or 'source identity changed during phase'
+ source_identity_error={'type':type(error).__name__,'message':str(error)}
+# Successful build freezes its produced binary above. Helpers check their frozen
+# prerequisite before launch above and again here, even if the helper failed.
+# Failed build: binary_pin remains None; missing binary is not a source mutation.
+if binary_pin is not None:
+ try:
+  assert hashlib.sha256(binary_path.read_bytes()).hexdigest()==binary_pin,'binary identity changed during phase'
+  binary_identity_ok=True
+ except (AssertionError,OSError) as error:
+  binary_identity_ok=False;binary_artifact_status='binary_identity_refused'
+  stop=stop or 'produced binary identity changed during phase'
+  binary_identity_error={'type':type(error).__name__,'message':str(error)}
+r={'command':cmd,'adapter_manifest_sha256':manifest_pin,'produced_binary_path':str(binary_path),'produced_binary_sha256':binary_pin,'initial_disk':initial,'final_disk':final,'exit_code':exitcode,'stop_reason':stop,'source_identity_preserved':identity_ok,'source_identity_error':source_identity_error,'binary_artifact_status':binary_artifact_status,'binary_identity_preserved':binary_identity_ok,'binary_identity_error':binary_identity_error,'child_launched':p is not None,'supervisor_error':supervisor_error,'attempt_reservation':str(reservation),'elapsed_seconds':time.monotonic()-start,'peak_sampled_group_rss_bytes':peak,'peak_sampled_group_cpu_seconds':maxcpu,'global_nonloopback_received_byte_delta':final_rx-initial_rx,'disk_samples':samples,'limits':{'wall_seconds':wall,'group_cpu_seconds':cpu_limit,'group_rss_bytes':rss_limit,'per_process_address_space_bytes':rss_limit,'incremental_source_cache_bytes':G,'incremental_target_bytes':2*G,'total_target_bytes':8*G,'free_floor_bytes':10*G,'disk_sample_interval_seconds':5,'memory_cpu_network_sample_interval_seconds':.1,'cargo_jobs':2,'rayon_threads':2},'monitor_limitations':'RSS/CPU/traffic sampled at0.1s and disk at5s; stop/receipt records overshoot. Traffic is global nonloopback RX, a conservative upper bound including other host activity, not isolated Cargo body measurement. Build/check run in verified no-network user namespace.','qualification':'bounded compile or no-key typed native preparation only; no SRS/keygen/proof/well_formed/apply/Preview acceptance','helper_receipt_sha256':helper_receipt_pin,'export_artifacts_sha256':export_artifacts,'runtime_inputs_manifest_sha256':runtime_inputs_pin,'phase':phase,'config_sha256':config_pin if phase=='prepare' else None,'preparation_output_limit_bytes':32*M}
 fd=os.open(receipt,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o644)
 with os.fdopen(fd,'w') as recorded:
  json.dump(r,recorded,indent=2);recorded.write('\n');recorded.flush();os.fsync(recorded.fileno())
```
