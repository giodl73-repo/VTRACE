use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        None => validate(Path::new(".")),
        Some("help" | "-h" | "--help") => {
            print_usage();
            Ok(())
        }
        Some("validate") => validate(path_arg(&args, 1)),
        Some("status") => status(path_arg(&args, 1)),
        Some("init") => init(path_arg(&args, 1)),
        Some("work") => work(&args[1..]),
        Some("roles") => roles(&args[1..]),
        Some("agent") => agent(&args[1..]),
        Some(path_or_command) if looks_like_path(path_or_command) => {
            validate(Path::new(path_or_command))
        }
        Some(command) => Err(format!("unknown command `{command}`")),
    };

    if let Err(message) = result {
        eprintln!("error: {message}");
        print_usage();
        std::process::exit(2);
    }
}

fn path_arg(args: &[String], index: usize) -> &Path {
    args.get(index)
        .map(Path::new)
        .unwrap_or_else(|| Path::new("."))
}

fn looks_like_path(value: &str) -> bool {
    value == "."
        || value == ".."
        || value.contains('/')
        || value.contains('\\')
        || Path::new(value).exists()
}

fn print_usage() {
    println!(
        "Usage:
  vtrace [validate] [repo]
  vtrace status [repo]
  vtrace init [repo]
  vtrace work start <WP-ID> [repo]
  vtrace work check <WP-ID> [repo]
  vtrace work close <WP-ID> [repo]
  vtrace roles review <WP-ID> [repo]
  vtrace agent brief <WP-ID> [repo]"
    );
}

fn validate(root: &Path) -> Result<(), String> {
    let findings = vtrace::run_checks(root);
    let display_root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());

    for finding in &findings {
        println!("{}", finding.render(&display_root));
    }

    if findings.is_empty() {
        println!("VTRACE validation passed");
        Ok(())
    } else {
        println!("VTRACE validation failed: {} finding(s)", findings.len());
        std::process::exit(1);
    }
}

fn status(root: &Path) -> Result<(), String> {
    let findings = vtrace::run_checks(root);
    let summary = vtrace::package_summary(root);

    println!("VTRACE status");
    println!("requirements: {}", summary.requirements);
    println!("specs: {}", summary.specs);
    println!("work packages: {}", summary.work_packages);
    println!("evidence rows: {}", summary.evidence_rows);
    println!("validator findings: {}", findings.len());

    if summary.open_work_packages.is_empty() {
        println!("open work packages: none");
    } else {
        println!(
            "open work packages: {}",
            summary.open_work_packages.join(", ")
        );
    }

    if findings.is_empty() {
        println!("decision: pass");
        Ok(())
    } else {
        println!("decision: blocked");
        let display_root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
        for finding in findings {
            println!("{}", finding.render(&display_root));
        }
        std::process::exit(1);
    }
}

fn init(root: &Path) -> Result<(), String> {
    let vtrace_dir = root.join("docs").join("vtrace");
    fs::create_dir_all(&vtrace_dir)
        .map_err(|err| format!("failed to create {}: {err}", vtrace_dir.display()))?;

    let files = [
        ("MISSION.md", "# Mission\n\n| ID | Need | Success Criteria | Status |\n|---|---|---|---|\n| NEED-001 | Define the repo mission. | Mission is traceable to requirements. | draft |\n"),
        ("REQUIREMENTS.md", "# Requirements\n\n| ID | Requirement | Parent Need / Scenario | Rationale | Priority | Owner | Verification Method | Status |\n|---|---|---|---|---|---|---|---|\n| REQ-001 | The repo shall define a first controlled requirement. | NEED-001 | Seed requirement for VTRACE adoption. | must | maintainer | inspection | draft |\n"),
        ("SPECIFICATION_BASELINE.md", "# Specification Baseline\n\n| Spec ID | Parent REQ IDs | Type | Current / Target / Deprecated / Unknown | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |\n|---|---|---|---|---|---|---|---|---|---|\n| SPEC-001 | REQ-001 | process | target | First controlled behavior is documented before implementation. | inspection | adoption scenario | maintainer | medium | draft |\n"),
        ("TRACE.md", "# Trace Matrix\n\n| Requirement ID | Specification Item | Work Package | Evidence Pointer | Status |\n|---|---|---|---|---|\n| REQ-001 | SPEC-001 | WP-001 | EVID-001 | draft |\n"),
        ("WORK_PACKAGES.md", "# Work Packages\n\n| ID | Objective | Parent IDs | Affected Surfaces | Entry Criteria | Exit Criteria | L0 / L1 / L2 | Status |\n|---|---|---|---|---|---|---|---|\n| WP-001 | Close the first VTRACE adoption slice. | REQ-001 / SPEC-001 | docs/vtrace | Required artifacts exist. | Evidence and review are recorded. | L0: git diff --check / L1: vtrace validate / L2: role review if public claim changes | proposed |\n"),
        ("VERIFICATION.md", "# Verification\n\n| Requirement ID | Method | Command / Inspection | Expected Result | Status | Evidence |\n|---|---|---|---|---|---|\n| REQ-001 | inspection | Inspect docs/vtrace | Required artifacts exist. | draft | EVID-001 |\n"),
        ("VALIDATION.md", "# Validation\n\n| ID | User / Actor | Scenario | Success Criteria | Evidence | Status |\n|---|---|---|---|---|---|\n| VAL-001 | maintainer | Apply VTRACE first slice. | Trace and evidence are complete. | EVID-001 | draft |\n"),
        ("EVIDENCE.md", "# Evidence Ledger\n\n| Evidence ID | Type | Source / Command | Expected Result | Actual Result | Status |\n|---|---|---|---|---|---|\n| EVID-001 | inspection | docs/vtrace | First VTRACE package exists. | pending | pending |\n"),
        ("REVIEW.md", "# Review\n\n| Lane | Required | Decision | Evidence / Rationale |\n|---|---|---|---|\n| Systems engineering | yes | pending | Initial VTRACE package needs review. |\n"),
    ];

    let mut created = Vec::new();
    let mut kept = Vec::new();
    for (name, content) in files {
        let path = vtrace_dir.join(name);
        if path.exists() {
            kept.push(name);
            continue;
        }
        fs::write(&path, content)
            .map_err(|err| format!("failed to write {}: {err}", path.display()))?;
        created.push(name);
    }

    println!("VTRACE init complete");
    println!("created: {}", list_or_none(&created));
    println!("kept existing: {}", list_or_none(&kept));
    Ok(())
}

fn list_or_none(items: &[&str]) -> String {
    if items.is_empty() {
        "none".to_string()
    } else {
        items.join(", ")
    }
}

fn work(args: &[String]) -> Result<(), String> {
    let action = args
        .first()
        .map(String::as_str)
        .ok_or("missing work action")?;
    let wp_id = args.get(1).ok_or("missing work package ID")?;
    let root = args.get(2).map(Path::new).unwrap_or_else(|| Path::new("."));
    let wp = vtrace::work_package(root, wp_id)
        .ok_or_else(|| format!("{wp_id} was not found in docs/vtrace/WORK_PACKAGES.md"))?;

    match action {
        "start" => {
            print_work_package("start", &wp);
            println!("next: satisfy entry criteria, keep changes inside affected surfaces, then run `vtrace work check {wp_id}`");
            Ok(())
        }
        "check" => {
            print_work_package("check", &wp);
            println!("expected checks: {}", wp.validation_levels);
            validate(root)
        }
        "close" => {
            print_work_package("close", &wp);
            let findings = vtrace::run_checks(root);
            if !findings.is_empty() {
                println!("closure blocked: validator findings must be fixed or accepted with risk");
                let display_root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
                for finding in findings {
                    println!("{}", finding.render(&display_root));
                }
                std::process::exit(1);
            }
            if matches!(
                wp.status.to_ascii_lowercase().as_str(),
                "proposed" | "planned" | "draft"
            ) {
                println!("closure blocked: {} is still `{}`", wp.id, wp.status);
                println!("update work-package status, evidence, verification, validation, and review before closure");
                std::process::exit(1);
            }
            println!("closure gate passed for {}", wp.id);
            Ok(())
        }
        other => Err(format!("unknown work action `{other}`")),
    }
}

fn roles(args: &[String]) -> Result<(), String> {
    let action = args
        .first()
        .map(String::as_str)
        .ok_or("missing roles action")?;
    if action != "review" {
        return Err(format!("unknown roles action `{action}`"));
    }
    let wp_id = args.get(1).ok_or("missing work package ID")?;
    let root = args.get(2).map(Path::new).unwrap_or_else(|| Path::new("."));
    let wp = vtrace::work_package(root, wp_id)
        .ok_or_else(|| format!("{wp_id} was not found in docs/vtrace/WORK_PACKAGES.md"))?;
    let lanes = vtrace::review_lanes(root);

    println!("VTRACE roles review: {}", wp.id);
    println!("objective: {}", wp.objective);
    if lanes.is_empty() {
        println!("review lanes: none found");
        return Ok(());
    }
    for lane in lanes {
        println!(
            "- {} | required: {} | decision: {} | evidence: {}",
            lane.lane, lane.required, lane.decision, lane.evidence
        );
    }
    Ok(())
}

fn agent(args: &[String]) -> Result<(), String> {
    let action = args
        .first()
        .map(String::as_str)
        .ok_or("missing agent action")?;
    if action != "brief" {
        return Err(format!("unknown agent action `{action}`"));
    }
    let wp_id = args.get(1).ok_or("missing work package ID")?;
    let root = args.get(2).map(Path::new).unwrap_or_else(|| Path::new("."));
    let wp = vtrace::work_package(root, wp_id)
        .ok_or_else(|| format!("{wp_id} was not found in docs/vtrace/WORK_PACKAGES.md"))?;

    println!("# VTRACE Agent Brief: {}", wp.id);
    println!();
    println!("Objective: {}", wp.objective);
    println!("Parent IDs: {}", wp.parent_ids);
    println!("Allowed surfaces: {}", wp.affected_surfaces);
    println!("Entry criteria: {}", wp.entry_criteria);
    println!("Exit criteria: {}", wp.exit_criteria);
    println!("Required validation: {}", wp.validation_levels);
    println!("Current status: {}", wp.status);
    println!();
    println!("Stop conditions:");
    println!("- Parent IDs are missing or conflict with the requested change.");
    println!("- Required evidence cannot be produced or must be accepted with risk.");
    println!("- Required review lanes are pending or blocked.");
    println!("- Git status shows unrelated changes that would be staged by the package.");
    println!();
    println!("Closeout:");
    println!("- Update verification, validation, evidence, review, and work-package status.");
    println!("- Run `vtrace work check {}` before closure.", wp.id);
    println!("- Keep child repo commits separate from tracker pointer commits when applicable.");
    Ok(())
}

fn print_work_package(action: &str, wp: &vtrace::WorkPackage) {
    println!("VTRACE work {action}: {}", wp.id);
    println!("line: {}", wp.line);
    println!("objective: {}", wp.objective);
    println!("parents: {}", wp.parent_ids);
    println!("surfaces: {}", wp.affected_surfaces);
    println!("entry: {}", wp.entry_criteria);
    println!("exit: {}", wp.exit_criteria);
    println!("validation: {}", wp.validation_levels);
    println!("status: {}", wp.status);
}
