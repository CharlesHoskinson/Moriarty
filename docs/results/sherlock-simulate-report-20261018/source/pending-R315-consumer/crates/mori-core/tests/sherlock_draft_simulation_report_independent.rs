//! R307 SOURCE_ONLY/UNRUN: contract/oracle frozen before this external consumer.
//! No report implementation body/output read; no full-event or old runtime cohort.
use mori_core::sherlock_compile_artifact::draft_execution::run_draft_host;
use mori_core::sherlock_compile_artifact::draft_simulation_report::report;
use mori_core::sherlock::helper_machine::shared::{OutcomeC, RefC};
use serde_json::{json, Value};

const CONFIG: &str = include_str!("fixtures/draft-simulation-report-independent/config.json");
const TARGET: &str = include_str!("fixtures/draft-simulation-report-independent/target.json");
const EXPECTED: &str = include_str!("fixtures/draft-simulation-report-independent/EXPECTED-REPORT.json");
const CONFIG_SHA: &str = "3406712e1327ebe87fa9c5265986a6a51ef5856f3498d99ff4a8dd958d70ac17";

#[test]
fn public_simulation_report_serializes_independent_results_and_same_run_bindings() {
    let expected: Value = serde_json::from_str(EXPECTED).unwrap();
    for (case, source) in [
        ("chain_total", include_str!("fixtures/draft-simulation-report-independent/chain-total.mori")),
        ("branch_total", include_str!("fixtures/draft-simulation-report-independent/branch-total.mori")),
    ] {
        // Count callbacks without retaining a trace, as the public report contract states.
        let mut callbacks = 0u128;
        let run = run_draft_host(source, CONFIG, TARGET, &mut |_| callbacks += 1)
            .unwrap_or_else(|failure| panic!("public run failed: {}", serde_json::to_string(&failure).unwrap()));
        let output = report(&run)
            .unwrap_or_else(|failure| panic!("public report failed: {}", serde_json::to_string(&failure).unwrap()));
        // Exercise the real public report's Serialize implementation into actual bytes.
        let bytes = serde_json::to_vec(&output).expect("public report serialization");
        let actual: Value = serde_json::from_slice(&bytes).expect("serialized report JSON");
        assert_eq!(actual["format"], expected["report_literals"]["format"]);
        assert_eq!(actual["qualification"], expected["report_literals"]["qualification"]);
        let preparation = &actual["preparation"];
        let execution = &actual["execution"];
        let image = &execution["image"];
        let summary = &execution["summary"];
        let resource = &execution["resource_report"];
        assert_eq!(execution["evaluation_performed"], true);
        assert_eq!(execution["completed"], true);
        assert_eq!(execution["event_capture"], "not-retained");
        assert!(execution.get("events").is_none());
        assert!(actual.get("events").is_none());

        // The borrowed report preserves the complete actual preparation artifact;
        // source identity remains independently fixed, rather than inferred from output.
        assert_eq!(*preparation, serde_json::to_value(&run.preparation).unwrap());
        assert_eq!(preparation["source"], expected["cases"][case]["source"]);
        assert_eq!(preparation["config"]["file_digest"], CONFIG_SHA);
        for (field, value) in expected["preparation"]["genesis"].as_object().unwrap() {
            assert_eq!(&preparation["genesis"][field], value, "genesis field {field}");
        }
        assert_eq!(preparation["observation"]["evaluation_performed"], false);
        assert_eq!(preparation["evidence"]["host_observation"], "genesis-only");
        assert_eq!(preparation["evidence"]["native"], "unrun");
        assert_eq!(preparation["evidence"]["target"], "unrun");
        assert_eq!(preparation["evidence"]["acceptance"], "unestablished");
        assert_eq!(preparation["correspondence"]["proof"]["semantic_correspondence"], "unproved");
        assert_eq!(preparation["plans"]["witness"]["status"], "missing");
        assert_eq!(preparation["plans"]["constraint"]["status"], "missing");
        assert_eq!(preparation["plans"]["export"]["status"], "unsupported");

        let observed = match run.observe() { Ok(value) => value, Err(_) => panic!("same-run observation failed") };
        let final_image = &observed.image;
        let reference = match *final_image.outcome {
            OutcomeC::Halted { value } => value,
            _ => panic!("fixed positive source did not halt"),
        };
        // Existing OwnerWire references use canonical decimal indices. These are
        // actual reference identities, not independently guessed allocation positions.
        let (reference_json, report_uint) = match reference {
            RefC::Const(index) => (json!({"Const":index.to_string()}),
                &preparation["admission"]["program_view"]["constants"][index]["U128"]["n"]),
            RefC::Data(index) => (json!({"Data":index.to_string()}), &image["data"][index]["U128"]["n"]),
        };
        let halted = json!({"Halted":{"value":reference_json.clone()}});
        assert_eq!(image["outcome"], halted);
        assert_eq!(summary["outcome"], image["outcome"]);
        assert_eq!(report_uint, &expected["cases"][case]["returnedUInt"], "independent result {case}");
        assert_eq!(image["control"], json!({"Return":{"value":reference_json}}));
        assert_ne!(image["outcome"], preparation["genesis"]["outcome"]);
        assert_eq!(image["initial_fuel"], "4096");
        assert_eq!(image["consumed_work"], final_image.consumed_work.to_string());
        assert_eq!(image["remaining_work"], final_image.remaining_work.to_string());
        assert!(final_image.consumed_work > 0);
        assert_eq!(final_image.remaining_work.checked_add(final_image.consumed_work), Some(4096));

        // Report bindings, not a new exact-count oracle or a replay of the old
        // event/prefix/occurrence assertion cohort.
        assert_eq!(callbacks, run.completed.events_forwarded);
        assert_eq!(callbacks, run.completed.summary.events);
        assert_eq!(execution["events_forwarded"], callbacks.to_string());
        for (field, value) in [
            ("events", run.completed.summary.events),
            ("consumed_work", run.completed.summary.consumed_work),
            ("helper_work", run.completed.summary.helper_work),
            ("helper_visits", run.completed.summary.helper_visits),
            ("reference_cursor", run.completed.summary.reference_cursor),
        ] {
            assert_eq!(summary[field], value.to_string(), "actual summary {field}");
            assert_eq!(resource["run_summary"][field], value.to_string(), "resource summary {field}");
        }
        for (field, value) in [
            ("initial_fuel", final_image.initial_fuel),
            ("consumed_work", final_image.consumed_work),
            ("remaining_work", final_image.remaining_work),
            ("helper_work", final_image.helper_work),
            ("helper_visits", final_image.helper_visits),
            ("reference_cursor", final_image.reference_cursor),
        ] {
            assert_eq!(image[field], value.to_string(), "actual image {field}");
            assert_eq!(resource["observed"][field], value.to_string(), "resource observation {field}");
        }
        let same_run_resource = match run.resource_report() { Ok(value) => value, Err(_) => panic!("same-run resources failed") };
        assert_eq!(*resource, serde_json::to_value(&same_run_resource).unwrap());
        for group in expected["execution_contract"]["resources_unmeasured"].as_array().unwrap() {
            assert_eq!(resource["unmeasured"][group.as_str().unwrap()]["status"], "unmeasured");
        }
    }
}
