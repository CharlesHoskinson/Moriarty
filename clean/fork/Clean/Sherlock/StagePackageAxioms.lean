import Clean.Sherlock.StagePackage

open Clean.Sherlock.StagePackage

example : packageStatus (.supplied false false false false false) = .packageReady := rfl
example : packageStatus (.supplied false false false true false) = .mismatch := rfl
example : packageStatus (.supplied false true false false false) = .mismatch := rfl
example : packageStatus (.supplied true false false false false) = .mismatch := rfl
example : dependencies .boundedStepCircuit = [.fuelStepRun, .boundedEncoding] := rfl

#print axioms Clean.Sherlock.StagePackage.package_ready_without_comparison
#print axioms Clean.Sherlock.StagePackage.divergence_dominates
#print axioms Clean.Sherlock.StagePackage.comparison_does_not_grant_acceptance
#print axioms Clean.Sherlock.StagePackage.admitted_round_trip
#print axioms Clean.Sherlock.StagePackage.trace_soundness

-- JSON emitted from the definitions, for direct comparison with the live stage entrypoint.
private def quoted (s : String) : String := "\"" ++ s ++ "\""
private def array (xs : List String) : String := "[" ++ String.intercalate "," xs ++ "]"

#eval IO.println (array (tasks.map fun task ↦
  "{\"id\":" ++ quoted (taskId task) ++ ",\"status\":" ++ quoted (statusId (status task)) ++
  ",\"required\":true,\"depends\":" ++ array ((dependencies task).map (quoted ∘ taskId)) ++ "}"))
