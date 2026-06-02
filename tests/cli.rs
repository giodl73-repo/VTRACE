use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn vtrace_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_vtrace"))
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(vtrace_bin())
        .current_dir(repo_root())
        .args(args)
        .output()
        .expect("vtrace command should run")
}

fn stdout(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn unique_temp_repo() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("vtrace-cli-test-{unique}"))
}

#[test]
fn explicit_validate_command_passes_self_package() {
    let output = run(&["validate", "."]);
    assert!(output.status.success(), "{}", stdout(&output));
    assert!(stdout(&output).contains("VTRACE validation passed"));
}

#[test]
fn path_compatibility_still_validates_self_package() {
    let output = run(&["."]);
    assert!(output.status.success(), "{}", stdout(&output));
    assert!(stdout(&output).contains("VTRACE validation passed"));
}

#[test]
fn status_reports_counts_and_decision() {
    let output = run(&["status", "."]);
    assert!(output.status.success(), "{}", stdout(&output));
    let out = stdout(&output);
    assert!(out.contains("VTRACE status"));
    assert!(out.contains("validator findings: 0"));
    assert!(out.contains("decision: pass"));
}

#[test]
fn plan_reports_no_open_self_work_packages() {
    let output = run(&["plan", "."]);
    assert!(output.status.success(), "{}", stdout(&output));
    let out = stdout(&output);
    assert!(out.contains("VTRACE plan"));
    assert!(out.contains("validator findings: 0"));
    assert!(out.contains("open work packages: none"));
}

#[test]
fn work_start_reports_selected_work_package() {
    let output = run(&["work", "start", "WP-009", "."]);
    assert!(output.status.success(), "{}", stdout(&output));
    let out = stdout(&output);
    assert!(out.contains("VTRACE work start: WP-009"));
    assert!(out.contains("parents: DCR-009"));
}

#[test]
fn worktree_plan_reports_branch_and_command() {
    let output = run(&["worktree", "plan", "WP-009", "."]);
    assert!(output.status.success(), "{}", stdout(&output));
    let out = stdout(&output);
    assert!(out.contains("VTRACE worktree plan: WP-009"));
    assert!(out.contains("branch: vtrace/wp-009"));
    assert!(out.contains("git -C"));
}

#[test]
fn roles_review_reports_required_lanes() {
    let output = run(&["roles", "review", "WP-009", "."]);
    assert!(output.status.success(), "{}", stdout(&output));
    let out = stdout(&output);
    assert!(out.contains("VTRACE roles review: WP-009"));
    assert!(out.contains("Systems engineering"));
}

#[test]
fn evidence_receipt_reports_draft_row() {
    let output = run(&["evidence", "receipt", "WP-009", "."]);
    assert!(output.status.success(), "{}", stdout(&output));
    let out = stdout(&output);
    assert!(out.contains("# VTRACE Evidence Receipt"));
    assert!(out.contains("Work package: WP-009"));
    assert!(out.contains("Evidence row draft:"));
}

#[test]
fn agent_brief_reports_stop_conditions() {
    let output = run(&["agent", "brief", "WP-009", "."]);
    assert!(output.status.success(), "{}", stdout(&output));
    let out = stdout(&output);
    assert!(out.contains("# VTRACE Agent Brief: WP-009"));
    assert!(out.contains("Stop conditions:"));
}

#[test]
fn init_creates_minimum_package_without_overwriting_existing_files() {
    let root = unique_temp_repo();
    let mission = root.join("docs").join("vtrace").join("MISSION.md");
    fs::create_dir_all(mission.parent().unwrap()).unwrap();
    fs::write(&mission, "# Existing Mission\n").unwrap();

    let root_arg = root.to_string_lossy().to_string();
    let output = run(&["init", &root_arg]);
    assert!(output.status.success(), "{}", stdout(&output));
    let out = stdout(&output);
    assert!(out.contains("VTRACE init complete"));
    assert!(out.contains("kept existing: MISSION.md"));
    assert_eq!(
        fs::read_to_string(&mission).unwrap(),
        "# Existing Mission\n"
    );
    assert!(root.join("docs").join("vtrace").join("TRACE.md").exists());

    let _ = fs::remove_dir_all(root);
}
