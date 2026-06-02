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

fn command_output(output: &std::process::Output) -> String {
    format!(
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn unique_temp_repo() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("vtrace-cli-test-{unique}"))
}

fn git(root: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .expect("git command should run")
}

fn committed_worktree_repo() -> PathBuf {
    let root = unique_temp_repo();
    let vtrace_dir = root.join("docs").join("vtrace");
    fs::create_dir_all(&vtrace_dir).unwrap();
    fs::write(
        vtrace_dir.join("WORK_PACKAGES.md"),
        "# Work Packages\n\n| ID | Objective | Parent IDs | Affected Surfaces | Entry Criteria | Exit Criteria | L0 / L1 / L2 | Status |\n|---|---|---|---|---|---|---|---|\n| WP-001 | Test worktree creation. | REQ-001 / SPEC-001 | docs/vtrace | ready | done | L0: diff / L1: test / L2: review | proposed |\n",
    )
    .unwrap();
    assert!(git(&root, &["init"]).status.success());
    assert!(git(&root, &["add", "."]).status.success());
    assert!(git(
        &root,
        &[
            "-c",
            "user.name=VTRACE Test",
            "-c",
            "user.email=vtrace@example.invalid",
            "commit",
            "-m",
            "seed",
        ],
    )
    .status
    .success());
    root
}

fn committed_close_ready_repo() -> PathBuf {
    let root = unique_temp_repo();
    let vtrace_dir = root.join("docs").join("vtrace");
    fs::create_dir_all(&vtrace_dir).unwrap();
    fs::write(vtrace_dir.join("MISSION.md"), "# Mission\n").unwrap();
    fs::write(
        vtrace_dir.join("REQUIREMENTS.md"),
        "# Requirements\n\n| ID | Requirement |\n|---|---|\n| REQ-001 | The repo shall do the thing. |\n",
    )
    .unwrap();
    fs::write(
        vtrace_dir.join("SPECIFICATION_BASELINE.md"),
        "# Specification Baseline\n\n| Spec ID | Parent REQ IDs | Status |\n|---|---|---|\n| SPEC-001 | REQ-001 | accepted |\n",
    )
    .unwrap();
    fs::write(
        vtrace_dir.join("TRACE.md"),
        "# Trace\n\n| Requirement ID | Specification Item | Work Package | Evidence Pointer | Status |\n|---|---|---|---|---|\n| REQ-001 | SPEC-001 | WP-001 | EVID-001 | verified |\n",
    )
    .unwrap();
    fs::write(
        vtrace_dir.join("WORK_PACKAGES.md"),
        "# Work Packages\n\n| ID | Objective | Parent IDs | Affected Surfaces | Entry Criteria | Exit Criteria | L0 / L1 / L2 | Status |\n|---|---|---|---|---|---|---|---|\n| WP-001 | Close a ready package. | REQ-001 / SPEC-001 | docs/vtrace | ready | done | L0: check / L1: test / L2: review | complete |\n",
    )
    .unwrap();
    fs::write(vtrace_dir.join("VERIFICATION.md"), "# Verification\n").unwrap();
    fs::write(
        vtrace_dir.join("REVIEW.md"),
        "# Review\n\n| Lane | Required | Decision |\n|---|---|---|\n| Systems engineering | yes | pass |\n",
    )
    .unwrap();
    fs::write(
        vtrace_dir.join("EVIDENCE.md"),
        "# Evidence\n\n| Evidence ID | Source / Command | Status |\n|---|---|---|\n| EVID-001 | command | passed |\n",
    )
    .unwrap();
    assert!(git(&root, &["init"]).status.success());
    assert!(git(&root, &["add", "."]).status.success());
    assert!(git(
        &root,
        &[
            "-c",
            "user.name=VTRACE Test",
            "-c",
            "user.email=vtrace@example.invalid",
            "commit",
            "-m",
            "seed",
        ],
    )
    .status
    .success());
    root
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
fn work_close_reports_readiness_before_blocking() {
    let root = committed_worktree_repo();
    let root_arg = root.to_string_lossy().to_string();

    let output = run(&["work", "close", "WP-001", &root_arg]);
    assert!(!output.status.success());
    let out = stdout(&output);
    assert!(out.contains("VTRACE work close: WP-001"));
    assert!(out.contains("closure readiness:"));
    assert!(out.contains("validator findings:"));
    assert!(out.contains("work-package status: proposed"));
    assert!(out.contains("required review lanes:"));
    assert!(out.contains("git scope:"));
    assert!(out.contains("expected evidence:"));
    assert!(out.contains("closure blocked:"));

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn work_close_blocks_dirty_git_scope() {
    let root = committed_close_ready_repo();
    fs::write(root.join("uncommitted.txt"), "dirty\n").unwrap();
    let root_arg = root.to_string_lossy().to_string();

    let output = run(&["work", "close", "WP-001", &root_arg]);
    assert!(!output.status.success());
    let out = stdout(&output);
    assert!(out.contains("validator findings: 0"));
    assert!(out.contains("work-package status: complete"));
    assert!(out.contains("git scope: dirty"));
    assert!(out.contains("closure blocked: git scope must be clean before closure"));

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn work_close_passes_ready_clean_package() {
    let root = committed_close_ready_repo();
    let root_arg = root.to_string_lossy().to_string();

    let output = run(&["work", "close", "WP-001", &root_arg]);
    assert!(output.status.success(), "{}", command_output(&output));
    let out = stdout(&output);
    assert!(out.contains("validator findings: 0"));
    assert!(out.contains("work-package status: complete"));
    assert!(out.contains("git scope: clean"));
    assert!(out.contains("closure gate passed for WP-001"));

    let _ = fs::remove_dir_all(&root);
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
fn worktree_create_creates_isolated_worktree() {
    let root = committed_worktree_repo();
    let target = root.with_file_name(format!(
        "{}-wp-001",
        root.file_name().unwrap().to_string_lossy()
    ));
    let root_arg = root.to_string_lossy().to_string();
    let target_arg = target.to_string_lossy().to_string();

    let output = run(&["worktree", "create", "WP-001", &root_arg, &target_arg]);
    assert!(output.status.success(), "{}", command_output(&output));
    let out = stdout(&output);
    assert!(out.contains("VTRACE worktree created: WP-001"));
    assert!(target
        .join("docs")
        .join("vtrace")
        .join("WORK_PACKAGES.md")
        .exists());
    let record = fs::read_to_string(target.join(".vtrace").join("worktree.md")).unwrap();
    assert!(record.contains("Work package: WP-001"));
    assert!(record.contains("Closeout commands:"));
    let brief = fs::read_to_string(target.join(".vtrace").join("agent-brief.md")).unwrap();
    assert!(brief.contains("# VTRACE Agent Brief: WP-001"));
    assert!(brief.contains("Stop conditions:"));

    let status = run(&["worktree", "status", &root_arg]);
    assert!(status.status.success(), "{}", command_output(&status));
    let status_out = stdout(&status);
    assert!(status_out.contains("VTRACE worktree status"));
    assert!(status_out.contains("record: present"));
    assert!(status_out.contains("wp: WP-001"));
    assert!(status_out.contains("duplicate: no"));

    let duplicate = root.with_file_name(format!(
        "{}-wp-001-duplicate",
        root.file_name().unwrap().to_string_lossy()
    ));
    let duplicate_arg = duplicate.to_string_lossy().to_string();
    let duplicate_output = run(&["worktree", "create", "WP-001", &root_arg, &duplicate_arg]);
    assert!(!duplicate_output.status.success());
    assert!(command_output(&duplicate_output).contains("already has active worktree"));
    assert!(!duplicate.exists());

    let allowed_duplicate = run(&[
        "worktree",
        "create",
        "WP-001",
        &root_arg,
        &duplicate_arg,
        "--allow-duplicate",
    ]);
    assert!(
        allowed_duplicate.status.success(),
        "{}",
        command_output(&allowed_duplicate)
    );
    let duplicate_record =
        fs::read_to_string(duplicate.join(".vtrace").join("worktree.md")).unwrap();
    assert!(duplicate_record.contains("Duplicate ownership allowed: yes"));

    let duplicate_status = run(&["worktree", "status", &root_arg]);
    assert!(
        duplicate_status.status.success(),
        "{}",
        command_output(&duplicate_status)
    );
    assert!(stdout(&duplicate_status).contains("duplicate: yes"));

    let duplicate_remove = run(&["worktree", "remove", &duplicate_arg]);
    assert!(
        duplicate_remove.status.success(),
        "{}",
        command_output(&duplicate_remove)
    );
    assert!(!duplicate.exists());

    let remove = run(&["worktree", "remove", &target_arg]);
    assert!(remove.status.success(), "{}", command_output(&remove));
    assert!(stdout(&remove).contains("VTRACE worktree removed:"));
    assert!(!target.exists());

    let _ = fs::remove_dir_all(&root);
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
