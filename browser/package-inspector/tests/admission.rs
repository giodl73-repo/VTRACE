use serde_json::{json, Value};
use vtrace_package_inspector::evaluate;

fn run(value: Value) -> Value {
    serde_json::from_str(&evaluate(&value.to_string())).unwrap()
}
fn document(name: &str, text: &str) -> Value {
    json!({"name":name,"text":text})
}
fn file_text(bytes: usize) -> String {
    let mut text = String::new();
    while text.len() < bytes {
        let take = (bytes - text.len()).min(1024);
        text.push_str(&"x".repeat(take.saturating_sub(1)));
        text.push('\n');
    }
    text
}

#[test]
fn empty_logical_package_uses_all_original_required_file_checks() {
    let report = run(json!({"files":[]}));
    assert_eq!(report["kind"], "report");
    assert_eq!(report["findings"].as_array().unwrap().len(), 6);
    assert_eq!(report["findings"][0]["path"], "docs/vtrace/MISSION.md");
    assert_eq!(report["summary"]["requirements"], 0);
}

#[test]
fn own_edit_changes_the_original_requirement_finding_at_its_source_line() {
    let requirement = "| ID | Text |\r\n|---|---|\r\n| REQ-001 | A concrete behavior. |\r\n";
    let mut request =
        json!({"files":[document("REQUIREMENTS.md",requirement),document("TRACE.md","unrelated")]});
    let before = run(request.clone());
    let finding = before["findings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|x| x["message"].as_str().unwrap().contains("REQ-001"))
        .unwrap();
    assert_eq!(finding["line"], 3);
    assert_eq!(finding["path"], "docs/vtrace/REQUIREMENTS.md");
    request["files"][1]["text"] = json!("REQ-001");
    let after = run(request);
    assert!(!after["findings"]
        .as_array()
        .unwrap()
        .iter()
        .any(|x| x["message"].as_str().unwrap().contains("REQ-001")));
    assert_eq!(after["summary"]["requirements"], 1);
}

#[test]
fn strict_unknown_duplicate_and_path_inputs_are_whole_refusals() {
    for files in [
        vec![document("../TRACE.md", "")],
        vec![document("TRACE.md", ""), document("TRACE.md", "other")],
        vec![document("trace.md", "")],
        vec![document(&"x".repeat(65), "")],
        vec![document("MISSION.md", ""); 12],
    ] {
        let report = run(json!({"files":files}));
        assert_eq!(report["kind"], "browser_refusal");
        assert!(report.get("findings").is_none());
    }
    assert_eq!(
        run(json!({"files":[],"ignored":true}))["kind"],
        "input_error"
    );
    assert_eq!(
        run(json!({"files":[{"name":"TRACE.md","text":"","extra":true}]}))["kind"],
        "input_error"
    );
}

#[test]
fn byte_line_and_package_boundaries_refuse_without_truncation() {
    assert_eq!(
        run(json!({"files":[document("MISSION.md",&file_text(65536))]}))["kind"],
        "report"
    );
    assert_eq!(
        run(json!({"files":[document("MISSION.md",&file_text(65537))]}))["kind"],
        "browser_refusal"
    );
    assert_eq!(
        run(json!({"files":[document("MISSION.md",&"é".repeat(1024))]}))["kind"],
        "report"
    );
    assert_eq!(
        run(json!({"files":[document("MISSION.md",&"é".repeat(1025))]}))["kind"],
        "browser_refusal"
    );
    assert_eq!(
        run(json!({"files":[document("MISSION.md",&"\n".repeat(2048))]}))["kind"],
        "report"
    );
    assert_eq!(
        run(json!({"files":[document("MISSION.md",&"\n".repeat(2049))]}))["kind"],
        "browser_refusal"
    );
    let files: Vec<Value> = [
        "MISSION.md",
        "REQUIREMENTS.md",
        "TRACE.md",
        "VERIFICATION.md",
    ]
    .iter()
    .map(|name| document(name, &file_text(65536)))
    .collect();
    assert_eq!(run(json!({"files":files}))["kind"], "report");
    let mut files = files;
    files.push(document("REVIEW.md", "x"));
    assert_eq!(run(json!({"files":files}))["kind"], "browser_refusal");
}

#[test]
fn original_unknown_markdown_and_claimed_status_stay_unverified_native_views() {
    let text="| ID | Objective | Status | Extra column |\n|---|---|---|---|\n| WP-001 | <script>claim</script> | passed | owned extension |\n";
    let report = run(json!({"files":[document("WORK_PACKAGES.md",text)]}));
    assert_eq!(
        report["work_packages"][0]["objective"],
        "<script>claim</script>"
    );
    assert_eq!(report["work_packages"][0]["status"], "passed");
    assert!(report["work_packages"][0].get("Extra column").is_none());
    assert!(report["scope"]
        .as_str()
        .unwrap()
        .contains("caller assertions"));
}

#[test]
fn amplified_native_findings_exceeding_report_budget_are_rejected_whole() {
    let id = format!("WP-{}", "\0".repeat(1800));
    let row = format!("| {id} | | | | |\n");
    let text=format!("| ID | Parent IDs | Entry Criteria | Exit Criteria | L0 / L1 / L2 |\n|---|---|---|---|---|\n{}",row.repeat(35));
    assert!(text.len() < 65536);
    let report = run(json!({"files":[document("WORK_PACKAGES.md",&text)]}));
    assert_eq!(report["kind"], "browser_refusal");
    assert!(report["message"]
        .as_str()
        .unwrap()
        .contains("Complete report"));
    assert!(report.get("findings").is_none());
}

#[test]
fn raw_request_and_native_multibyte_prefix_admission_are_identical() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let request = json!({"files":[document("MISSION.md",&"é".repeat(600000))]}).to_string();
    let expected = evaluate(&request);
    assert_eq!(
        serde_json::from_str::<Value>(&expected).unwrap()["kind"],
        "browser_refusal"
    );
    let mut child = Command::new(env!("CARGO_BIN_EXE_vtrace-package-control"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Err(error) = child.stdin.take().unwrap().write_all(request.as_bytes()) {
        assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
    }
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), expected);
}
