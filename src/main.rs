use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

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
        Some("plan") => plan(path_arg(&args, 1)),
        Some("work") => work(&args[1..]),
        Some("worktree") => worktree(&args[1..]),
        Some("evidence") => evidence(&args[1..]),
        Some("roles") => roles(&args[1..]),
        Some("agent") => agent(&args[1..]),
        Some("provider") => provider(&args[1..]),
        Some("report") => report(&args[1..]),
        Some("comms") => comms(&args[1..]),
        Some("github") => github(&args[1..]),
        Some("pulse") => pulse(&args[1..]),
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
  vtrace plan [repo]
  vtrace work start <WP-ID> [repo]
  vtrace work check <WP-ID> [repo]
  vtrace work close <WP-ID> [repo]
  vtrace worktree status [repo]
  vtrace worktree plan <WP-ID> [repo]
  vtrace worktree create <WP-ID> [repo] [path] [--allow-duplicate]
  vtrace worktree remove <path> [--force]
  vtrace evidence receipt <WP-ID> [repo]
  vtrace roles review <WP-ID> [repo]
  vtrace roles run <WP-ID> [repo]
  vtrace agent brief <WP-ID> [repo]
  vtrace provider list
  vtrace provider check <codex|claude|copilot>
  vtrace provider draft <WP-ID> --provider <name> [repo] [--live]
  vtrace provider review <WP-ID> --provider <name> [repo] [--live]
  vtrace report adoption [repo]
  vtrace comms plan [repo]
  vtrace github issue <WP-ID> [repo] [--dry-run|--live]
  vtrace github pr-review <WP-ID> [repo] [--dry-run|--live]
  vtrace pulse sync <WP-ID> [repo] [--dry-run|--live]"
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

fn plan(root: &Path) -> Result<(), String> {
    let findings = vtrace::run_checks(root);
    let work_packages = vtrace::work_packages(root);
    let open: Vec<_> = work_packages
        .iter()
        .filter(|wp| {
            !matches!(
                wp.status.to_ascii_lowercase().as_str(),
                "complete" | "closed" | "passed"
            )
        })
        .collect();

    println!("VTRACE plan");
    println!("validator findings: {}", findings.len());
    if !findings.is_empty() {
        println!("next: fix validator findings before starting new work packages");
        let display_root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
        for finding in findings {
            println!("{}", finding.render(&display_root));
        }
        std::process::exit(1);
    }

    if open.is_empty() {
        println!("open work packages: none");
        println!("next: define a DCR and proposed WP before implementation, or run `vtrace status` for readiness.");
        return Ok(());
    }

    println!("open work packages:");
    for wp in open {
        println!("- {} [{}] {}", wp.id, wp.status, wp.objective);
        println!("  parents: {}", wp.parent_ids);
        println!("  next: vtrace work start {}", wp.id);
    }
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
            let required_lanes: Vec<String> = vtrace::review_lanes(root)
                .into_iter()
                .filter(|lane| lane.required.eq_ignore_ascii_case("yes"))
                .map(|lane| lane.lane)
                .collect();
            let git_scope = git_scope(root);

            println!("closure readiness:");
            println!("- validator findings: {}", findings.len());
            println!("- work-package status: {}", wp.status);
            println!("- affected surfaces: {}", wp.affected_surfaces);
            println!("- required review lanes: {}", required_lanes.len());
            println!(
                "- required review lane names: {}",
                if required_lanes.is_empty() {
                    "none".to_string()
                } else {
                    required_lanes.join(", ")
                }
            );
            println!("- git scope: {}", git_scope.status);
            println!(
                "- changed paths: {}",
                if git_scope.changed_paths.is_empty() {
                    "none".to_string()
                } else {
                    git_scope.changed_paths.join(", ")
                }
            );
            println!("- expected evidence: verification, validation, evidence, review, and work-package status updates");

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
            if git_scope.status.starts_with("dirty ") {
                println!("closure blocked: git scope must be clean before closure");
                std::process::exit(1);
            }
            println!("closure gate passed for {}", wp.id);
            Ok(())
        }
        other => Err(format!("unknown work action `{other}`")),
    }
}

struct GitScope {
    status: String,
    changed_paths: Vec<String>,
}

fn git_scope(root: &Path) -> GitScope {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("status")
        .arg("--short")
        .output();
    let Ok(output) = output else {
        return GitScope {
            status: "not inspected: git status failed to run".to_string(),
            changed_paths: Vec::new(),
        };
    };
    if !output.status.success() {
        return GitScope {
            status: "not a git worktree or git status unavailable".to_string(),
            changed_paths: Vec::new(),
        };
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let changed_paths: Vec<String> = stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .collect();
    let status = if changed_paths.is_empty() {
        "clean".to_string()
    } else {
        format!("dirty ({} changed path(s))", changed_paths.len())
    };
    GitScope {
        status,
        changed_paths,
    }
}

fn roles(args: &[String]) -> Result<(), String> {
    let action = args
        .first()
        .map(String::as_str)
        .ok_or("missing roles action")?;
    let wp_id = args.get(1).ok_or("missing work package ID")?;
    let root = args.get(2).map(Path::new).unwrap_or_else(|| Path::new("."));
    let wp = vtrace::work_package(root, wp_id)
        .ok_or_else(|| format!("{wp_id} was not found in docs/vtrace/WORK_PACKAGES.md"))?;
    let lanes = vtrace::review_lanes(root);

    match action {
        "review" => {
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
        "run" => {
            println!("VTRACE roles run: {}", wp.id);
            println!("objective: {}", wp.objective);
            println!("advisory: role packets do not close REVIEW.md lanes");
            if lanes.is_empty() {
                println!("review packets: none");
                return Ok(());
            }
            for lane in lanes {
                println!(
                    "- lane: {} | required: {} | decision: {} | role: {} | output: review_needed",
                    lane.lane,
                    lane.required,
                    lane.decision,
                    role_path_for_lane(&lane.lane)
                );
            }
            Ok(())
        }
        other => Err(format!("unknown roles action `{other}`")),
    }
}

fn role_path_for_lane(lane: &str) -> String {
    let normalized = lane.to_ascii_lowercase();
    let known = match normalized.as_str() {
        "systems engineering" => Some(".roles/parliament/systems-engineering-steward.md"),
        "requirements traceability" => {
            Some(".roles/parliament/requirements-traceability-auditor.md")
        }
        "v&v" => Some(".roles/parliament/verification-validation-lead.md"),
        "software assurance" => Some(".roles/parliament/software-assurance-guardian.md"),
        "security/privacy" => Some(".roles/parliament/security-privacy-guardian.md"),
        "safety/mission impact" => Some(".roles/parliament/safety-risk-officer.md"),
        "source custody" => Some(".roles/parliament/source-custody-counsel.md"),
        "adoption guide" => Some(".roles/editorial/adoption-guide-editor.md"),
        "template minimalism" => Some(".roles/editorial/template-minimalism-editor.md"),
        "repo maintainer" => Some(".roles/stakeholders/repo-maintainer.md"),
        "future agent" => Some(".roles/stakeholders/future-agent.md"),
        _ => None,
    };
    if let Some(path) = known {
        return path.to_string();
    }
    let slug = lane
        .to_ascii_lowercase()
        .replace('&', "")
        .replace('/', "-")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-");
    format!(".roles/{slug}.md")
}

fn worktree(args: &[String]) -> Result<(), String> {
    let action = args
        .first()
        .map(String::as_str)
        .ok_or("missing worktree action")?;
    if action == "status" {
        let root = args.get(1).map(Path::new).unwrap_or_else(|| Path::new("."));
        return worktree_status(root);
    }
    if action == "remove" {
        let target = args.get(1).map(Path::new).ok_or("missing worktree path")?;
        let force = args.iter().any(|arg| arg == "--force");
        return remove_worktree(target, force);
    }
    let wp_id = args.get(1).ok_or("missing work package ID")?;
    let root = args.get(2).map(Path::new).unwrap_or_else(|| Path::new("."));
    let wp = vtrace::work_package(root, wp_id)
        .ok_or_else(|| format!("{wp_id} was not found in docs/vtrace/WORK_PACKAGES.md"))?;
    let spec = worktree_spec(root, &wp);

    match action {
        "plan" => {
            print_worktree_plan(&wp, &spec);
            Ok(())
        }
        "create" => {
            let allow_duplicate = args.iter().any(|arg| arg == "--allow-duplicate");
            let duplicate_existing = existing_worktree_for_wp(root, &wp.id)?.is_some();
            if !allow_duplicate {
                if let Some(existing_path) = existing_worktree_for_wp(root, &wp.id)? {
                    return Err(format!(
                        "work package {} already has active worktree {}; pass --allow-duplicate to override",
                        wp.id, existing_path
                    ));
                }
            }
            let branch = if allow_duplicate && duplicate_existing {
                format!("{}-duplicate", spec.branch)
            } else {
                spec.branch.clone()
            };
            let target = args
                .iter()
                .skip(3)
                .find(|arg| !arg.starts_with("--"))
                .map(Path::new)
                .unwrap_or(&spec.path);
            create_worktree(root, &branch, target)?;
            write_worktree_record(target, root, &wp, &branch, allow_duplicate)?;
            write_agent_brief_record(target, &wp)?;
            println!("VTRACE worktree created: {}", wp.id);
            println!("branch: {}", branch);
            println!("path: {}", target.display());
            println!(
                "record: {}",
                target.join(".vtrace").join("worktree.md").display()
            );
            println!(
                "brief: {}",
                target.join(".vtrace").join("agent-brief.md").display()
            );
            Ok(())
        }
        other => Err(format!("unknown worktree action `{other}`")),
    }
}

fn existing_worktree_for_wp(root: &Path, wp_id: &str) -> Result<Option<String>, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("worktree")
        .arg("list")
        .arg("--porcelain")
        .output()
        .map_err(|err| format!("failed to inspect git worktrees: {err}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        let Some(path) = line.strip_prefix("worktree ") else {
            continue;
        };
        let record = Path::new(path).join(".vtrace").join("worktree.md");
        if worktree_ownership(&record).work_package.as_deref() == Some(wp_id) {
            return Ok(Some(path.to_string()));
        }
    }
    Ok(None)
}

fn worktree_status(root: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("worktree")
        .arg("list")
        .arg("--porcelain")
        .output()
        .map_err(|err| format!("failed to inspect git worktrees: {err}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    println!("VTRACE worktree status");
    let text = String::from_utf8_lossy(&output.stdout);
    let mut current_path: Option<String> = None;
    let mut current_branch: Option<String> = None;
    for line in text.lines().chain([""]) {
        if let Some(path) = line.strip_prefix("worktree ") {
            print_worktree_status_row(current_path.take(), current_branch.take());
            current_path = Some(path.to_string());
        } else if let Some(branch) = line.strip_prefix("branch ") {
            current_branch = Some(branch.trim_start_matches("refs/heads/").to_string());
        } else if line.is_empty() {
            print_worktree_status_row(current_path.take(), current_branch.take());
        }
    }
    Ok(())
}

fn print_worktree_status_row(path: Option<String>, branch: Option<String>) {
    let Some(path) = path else {
        return;
    };
    let record = Path::new(&path).join(".vtrace").join("worktree.md");
    let ownership = worktree_ownership(&record);
    let record_status = if ownership.work_package.is_some() {
        "present"
    } else {
        "absent"
    };
    println!(
        "- path: {} | branch: {} | record: {} | wp: {} | duplicate: {}",
        path,
        branch.unwrap_or_else(|| "detached".to_string()),
        record_status,
        ownership
            .work_package
            .unwrap_or_else(|| "unknown".to_string()),
        ownership
            .duplicate_allowed
            .unwrap_or_else(|| "unknown".to_string())
    );
}

struct WorktreeOwnership {
    work_package: Option<String>,
    duplicate_allowed: Option<String>,
}

fn worktree_ownership(record: &Path) -> WorktreeOwnership {
    let Ok(text) = fs::read_to_string(record) else {
        return WorktreeOwnership {
            work_package: None,
            duplicate_allowed: None,
        };
    };
    WorktreeOwnership {
        work_package: text
            .lines()
            .find_map(|line| line.strip_prefix("Work package: "))
            .map(ToOwned::to_owned),
        duplicate_allowed: text
            .lines()
            .find_map(|line| line.strip_prefix("Duplicate ownership allowed: "))
            .map(ToOwned::to_owned),
    }
}

struct WorktreeSpec {
    root: std::path::PathBuf,
    branch: String,
    path: std::path::PathBuf,
}

fn worktree_spec(root: &Path, wp: &vtrace::WorkPackage) -> WorktreeSpec {
    let root_path = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let branch = format!("vtrace/{}", wp.id.to_ascii_lowercase());
    let root_name = root_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("repo");
    let default_path =
        root_path.with_file_name(format!("{}-{}", root_name, wp.id.to_ascii_lowercase()));

    WorktreeSpec {
        root: root_path,
        branch,
        path: default_path,
    }
}

fn print_worktree_plan(wp: &vtrace::WorkPackage, spec: &WorktreeSpec) {
    println!("VTRACE worktree plan: {}", wp.id);
    println!("objective: {}", wp.objective);
    println!("repo: {}", spec.root.display());
    println!("branch: {}", spec.branch);
    println!("path: {}", spec.path.display());
    println!("command:");
    println!(
        "git -C {} worktree add -b {} {} HEAD",
        quote_arg(&spec.root.display().to_string()),
        quote_arg(&spec.branch),
        quote_arg(&spec.path.display().to_string())
    );
    println!("agent brief:");
    println!(
        "vtrace agent brief {} {}",
        wp.id,
        quote_arg(&spec.root.display().to_string())
    );
    println!("close check:");
    println!(
        "vtrace work check {} {}",
        wp.id,
        quote_arg(&spec.root.display().to_string())
    );
}

fn create_worktree(root: &Path, branch: &str, target: &Path) -> Result<(), String> {
    if target.exists() {
        return Err(format!(
            "worktree target already exists: {}",
            target.display()
        ));
    }

    let status = Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("status")
        .arg("--porcelain")
        .output()
        .map_err(|err| format!("failed to inspect git status: {err}"))?;
    if !status.status.success() {
        return Err(String::from_utf8_lossy(&status.stderr).trim().to_string());
    }
    if !status.stdout.is_empty() {
        return Err("refusing to create worktree from dirty repo".to_string());
    }

    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("worktree")
        .arg("add")
        .arg("-b")
        .arg(branch)
        .arg(target)
        .arg("HEAD")
        .output()
        .map_err(|err| format!("failed to create worktree: {err}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    Ok(())
}

fn remove_worktree(target: &Path, force: bool) -> Result<(), String> {
    if !target.exists() {
        return Err(format!(
            "worktree target does not exist: {}",
            target.display()
        ));
    }
    let record = target.join(".vtrace").join("worktree.md");
    if !record.exists() && !force {
        return Err(format!(
            "refusing to remove worktree without VTRACE ownership record: {}",
            record.display()
        ));
    }

    let source_root =
        source_root_from_worktree_record(&record).unwrap_or_else(|| target.to_path_buf());
    let mut command = Command::new("git");
    command
        .arg("-C")
        .arg(source_root)
        .arg("worktree")
        .arg("remove");
    if force || record.exists() {
        command.arg("--force");
    }
    let output = command
        .arg(target)
        .output()
        .map_err(|err| format!("failed to remove worktree: {err}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    println!("VTRACE worktree removed: {}", target.display());
    Ok(())
}

fn source_root_from_worktree_record(record: &Path) -> Option<std::path::PathBuf> {
    let text = fs::read_to_string(record).ok()?;
    text.lines()
        .find_map(|line| line.strip_prefix("Source repo: "))
        .map(std::path::PathBuf::from)
}

fn write_worktree_record(
    target: &Path,
    source_root: &Path,
    wp: &vtrace::WorkPackage,
    branch: &str,
    allow_duplicate: bool,
) -> Result<(), String> {
    let record_dir = target.join(".vtrace");
    fs::create_dir_all(&record_dir)
        .map_err(|err| format!("failed to create worktree record directory: {err}"))?;
    let source = source_root
        .canonicalize()
        .unwrap_or_else(|_| source_root.to_path_buf());
    let target_path = target
        .canonicalize()
        .unwrap_or_else(|_| target.to_path_buf());
    let content = format!(
        "# VTRACE Worktree\n\nWork package: {}\nObjective: {}\nBranch: {}\nDuplicate ownership allowed: {}\nSource repo: {}\nWorktree path: {}\n\nCloseout commands:\n\n```powershell\nvtrace work check {} {}\nvtrace evidence receipt {} {}\n```\n",
        wp.id,
        wp.objective,
        branch,
        if allow_duplicate { "yes" } else { "no" },
        source.display(),
        target_path.display(),
        wp.id,
        quote_arg(&target_path.display().to_string()),
        wp.id,
        quote_arg(&target_path.display().to_string())
    );
    fs::write(record_dir.join("worktree.md"), content)
        .map_err(|err| format!("failed to write worktree record: {err}"))
}

fn write_agent_brief_record(target: &Path, wp: &vtrace::WorkPackage) -> Result<(), String> {
    let record_dir = target.join(".vtrace");
    fs::create_dir_all(&record_dir)
        .map_err(|err| format!("failed to create agent brief directory: {err}"))?;
    let content = agent_brief_markdown(wp);
    fs::write(record_dir.join("agent-brief.md"), content)
        .map_err(|err| format!("failed to write agent brief: {err}"))
}

fn quote_arg(value: &str) -> String {
    if value.contains(' ') {
        format!("\"{}\"", value.replace('"', "\\\""))
    } else {
        value.to_string()
    }
}

fn evidence(args: &[String]) -> Result<(), String> {
    let action = args
        .first()
        .map(String::as_str)
        .ok_or("missing evidence action")?;
    if action != "receipt" {
        return Err(format!("unknown evidence action `{action}`"));
    }
    let wp_id = args.get(1).ok_or("missing work package ID")?;
    let root = args.get(2).map(Path::new).unwrap_or_else(|| Path::new("."));
    let wp = vtrace::work_package(root, wp_id)
        .ok_or_else(|| format!("{wp_id} was not found in docs/vtrace/WORK_PACKAGES.md"))?;
    let findings = vtrace::run_checks(root);

    println!("# VTRACE Evidence Receipt");
    println!();
    println!("Work package: {}", wp.id);
    println!("Objective: {}", wp.objective);
    println!("Parent IDs: {}", wp.parent_ids);
    println!("Status: {}", wp.status);
    println!("Validation expectation: {}", wp.validation_levels);
    println!("Validator findings: {}", findings.len());
    println!();
    println!("Evidence row draft:");
    println!(
        "| Evidence ID | Type | Source / Command | Expected Result | Actual Result | Status |"
    );
    println!("|---|---|---|---|---|---|");
    println!(
        "| EVID-TBD | command/review | vtrace work check {} | Required package checks and trace validation complete. | validator findings: {} | TBD |",
        wp.id,
        findings.len()
    );
    println!();
    println!("Closeout targets:");
    println!("- WORK_PACKAGES.md status");
    println!("- VERIFICATION.md status and evidence pointer");
    println!("- VALIDATION.md scenario impact");
    println!("- EVIDENCE.md command/review receipt");
    println!("- REVIEW.md required role lanes");
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

    print!("{}", agent_brief_markdown(&wp));
    Ok(())
}

fn agent_brief_markdown(wp: &vtrace::WorkPackage) -> String {
    format!(
        "# VTRACE Agent Brief: {}\n\nObjective: {}\nParent IDs: {}\nAllowed surfaces: {}\nEntry criteria: {}\nExit criteria: {}\nRequired validation: {}\nCurrent status: {}\n\nStop conditions:\n- Parent IDs are missing or conflict with the requested change.\n- Required evidence cannot be produced or must be accepted with risk.\n- Required review lanes are pending or blocked.\n- Git status shows unrelated changes that would be staged by the package.\n\nCloseout:\n- Update verification, validation, evidence, review, and work-package status.\n- Run `vtrace work check {}` before closure.\n- Keep child repo commits separate from tracker pointer commits when applicable.\n",
        wp.id,
        wp.objective,
        wp.parent_ids,
        wp.affected_surfaces,
        wp.entry_criteria,
        wp.exit_criteria,
        wp.validation_levels,
        wp.status,
        wp.id
    )
}

fn provider(args: &[String]) -> Result<(), String> {
    let action = args
        .first()
        .map(String::as_str)
        .ok_or("missing provider action")?;
    match action {
        "list" => {
            println!("VTRACE providers");
            for name in provider_names() {
                println!("- {name}");
            }
            Ok(())
        }
        "check" => {
            let name = args.get(1).ok_or("missing provider name")?;
            ensure_provider_name(name)?;
            println!("VTRACE provider check: {name}");
            println!("available: {}", provider_available(name));
            println!("mode: advisory");
            Ok(())
        }
        "draft" | "review" => {
            let wp_id = args.get(1).ok_or("missing work package ID")?;
            let provider_name = option_value(args, "--provider").ok_or("missing --provider")?;
            ensure_provider_name(provider_name)?;
            let live = has_flag(args, "--live");
            let root = positional_repo_arg(args, 2);
            let wp = vtrace::work_package(root, wp_id)
                .ok_or_else(|| format!("{wp_id} was not found in docs/vtrace/WORK_PACKAGES.md"))?;
            if live && !provider_available(provider_name) {
                return Err(format!(
                    "provider `{provider_name}` is not available for live mode"
                ));
            }
            print_provider_packet(action, provider_name, &wp, live);
            Ok(())
        }
        other => Err(format!("unknown provider action `{other}`")),
    }
}

fn provider_names() -> [&'static str; 3] {
    ["codex", "claude", "copilot"]
}

fn ensure_provider_name(name: &str) -> Result<(), String> {
    if provider_names().contains(&name) {
        Ok(())
    } else {
        Err(format!("unknown provider `{name}`"))
    }
}

fn provider_available(name: &str) -> bool {
    match name {
        "codex" => command_available("codex"),
        "claude" => command_available("claude"),
        "copilot" => command_available("gh"),
        _ => false,
    }
}

fn command_available(name: &str) -> bool {
    Command::new(name)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn print_provider_packet(action: &str, provider_name: &str, wp: &vtrace::WorkPackage, live: bool) {
    println!("VTRACE provider {action}: {}", wp.id);
    println!("provider: {provider_name}");
    println!("mode: {}", if live { "live" } else { "dry-run" });
    println!("status: review_needed");
    println!("canonical: false");
    println!("objective: {}", wp.objective);
    println!("parents: {}", wp.parent_ids);
    println!("surfaces: {}", wp.affected_surfaces);
    println!("checks: {}", wp.validation_levels);
    println!("instruction: produce advisory output only; do not mark evidence passed or close work packages");
}

fn report(args: &[String]) -> Result<(), String> {
    let action = args
        .first()
        .map(String::as_str)
        .ok_or("missing report action")?;
    if action != "adoption" {
        return Err(format!("unknown report action `{action}`"));
    }
    let root = args.get(1).map(Path::new).unwrap_or_else(|| Path::new("."));
    let findings = vtrace::run_checks(root);
    let summary = vtrace::package_summary(root);
    println!("VTRACE adoption report");
    println!("requirements: {}", summary.requirements);
    println!("specs: {}", summary.specs);
    println!("work packages: {}", summary.work_packages);
    println!("evidence rows: {}", summary.evidence_rows);
    println!("validator findings: {}", findings.len());
    println!(
        "open work packages: {}",
        if summary.open_work_packages.is_empty() {
            "none".to_string()
        } else {
            summary.open_work_packages.join(", ")
        }
    );
    println!(
        "readiness: {}",
        if findings.is_empty() {
            "ready_for_review"
        } else {
            "blocked"
        }
    );
    Ok(())
}

fn comms(args: &[String]) -> Result<(), String> {
    let action = args
        .first()
        .map(String::as_str)
        .ok_or("missing comms action")?;
    if action != "plan" {
        return Err(format!("unknown comms action `{action}`"));
    }
    let root = args.get(1).map(Path::new).unwrap_or_else(|| Path::new("."));
    let strategy_path = root
        .join("docs")
        .join("vtrace")
        .join("COMMUNICATIONS_STRATEGY.md");
    let surfaces = vtrace::communication_surfaces(root);

    println!("VTRACE comms plan");
    println!(
        "strategy: {}",
        if strategy_path.exists() {
            "present"
        } else {
            "missing"
        }
    );
    println!("boundary: specs control engineering intent; docs explain accepted features to users");
    println!("source mapping:");
    println!("- mission need -> docs/README.md and audience promises");
    println!("- CONOPS scenario -> tutorial, walkthrough, trace, or deck");
    println!("- requirement/spec -> concept, how-to, reference page, or example");
    println!(
        "- work package/evidence -> release note, trace walkthrough, corpus row, or deck update"
    );
    println!("recommended surfaces:");
    println!("- docs/README.md: map the repo documentation package");
    println!("- docs/concepts/: explain mental models, boundaries, and feature meaning");
    println!("- docs/how-to/: answer focused user tasks");
    println!("- docs/tutorials/: teach sequenced learning paths");
    println!("- docs/examples/: provide copyable examples and expected outputs");
    println!("- docs/traces/: show end-to-end walkthroughs tied to evidence");
    println!("- docs/decks/: summarize stakeholder, release, or adoption narratives");
    println!("- docs/CORPUS.md: govern ownership, cadence, and update obligations");

    if surfaces.is_empty() {
        println!("declared surfaces: none");
        println!("next: add docs/vtrace/COMMUNICATIONS_STRATEGY.md from templates/adoption/COMMUNICATIONS_STRATEGY.md");
        return Ok(());
    }

    println!("declared surfaces:");
    for surface in surfaces {
        println!(
            "- {} | sources: {} | audience: {} | docs: {} | cadence: {} | status: {}",
            surface.id,
            surface.source_ids,
            surface.audience,
            surface.generated_docs,
            surface.cadence,
            surface.status
        );
    }
    Ok(())
}

fn github(args: &[String]) -> Result<(), String> {
    let action = args
        .first()
        .map(String::as_str)
        .ok_or("missing github action")?;
    if !matches!(action, "issue" | "pr-review") {
        return Err(format!("unknown github action `{action}`"));
    }
    let wp_id = args.get(1).ok_or("missing work package ID")?;
    let root = positional_repo_arg(args, 2);
    let wp = vtrace::work_package(root, wp_id)
        .ok_or_else(|| format!("{wp_id} was not found in docs/vtrace/WORK_PACKAGES.md"))?;
    let live = has_flag(args, "--live");
    let body = github_packet(action, &wp);
    if !live {
        println!("VTRACE GitHub {action} dry-run: {}", wp.id);
        println!("{body}");
        return Ok(());
    }
    ensure_gh_auth()?;
    match action {
        "issue" => run_gh(
            root,
            &["issue", "create", "--title", &wp.id, "--body", &body],
        ),
        "pr-review" => run_gh(root, &["pr", "comment", "--body", &body]),
        _ => unreachable!(),
    }
}

fn github_packet(action: &str, wp: &vtrace::WorkPackage) -> String {
    format!(
        "# VTRACE {action}: {}\n\nObjective: {}\nParent IDs: {}\nAffected surfaces: {}\nChecks: {}\nClose criteria: {}\nEvidence targets: WORK_PACKAGES.md, VERIFICATION.md, VALIDATION.md, EVIDENCE.md, REVIEW.md\n",
        wp.id,
        wp.objective,
        wp.parent_ids,
        wp.affected_surfaces,
        wp.validation_levels,
        wp.exit_criteria
    )
}

fn ensure_gh_auth() -> Result<(), String> {
    let output = Command::new("gh")
        .arg("auth")
        .arg("status")
        .output()
        .map_err(|err| format!("failed to run gh auth status: {err}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

fn run_gh(root: &Path, args: &[&str]) -> Result<(), String> {
    let output = Command::new("gh")
        .current_dir(root)
        .args(args)
        .output()
        .map_err(|err| format!("failed to run gh: {err}"))?;
    if output.status.success() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

fn pulse(args: &[String]) -> Result<(), String> {
    let action = args
        .first()
        .map(String::as_str)
        .ok_or("missing pulse action")?;
    if action != "sync" {
        return Err(format!("unknown pulse action `{action}`"));
    }
    let wp_id = args.get(1).ok_or("missing work package ID")?;
    let root = positional_repo_arg(args, 2);
    let wp = vtrace::work_package(root, wp_id)
        .ok_or_else(|| format!("{wp_id} was not found in docs/vtrace/WORK_PACKAGES.md"))?;
    let live = has_flag(args, "--live");
    let target = pulse_target(root);
    let content = pulse_packet(&wp);
    println!("VTRACE pulse sync: {}", wp.id);
    println!("mode: {}", if live { "live" } else { "dry-run" });
    println!("target: {}", target.display());
    if live {
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("failed to create pulse directory: {err}"))?;
        }
        fs::write(&target, content)
            .map_err(|err| format!("failed to write {}: {err}", target.display()))?;
        println!("written: yes");
    } else {
        println!("written: no");
        println!("{content}");
    }
    Ok(())
}

fn pulse_target(root: &Path) -> std::path::PathBuf {
    let context = root.join("context").join("waves");
    if context.exists() {
        context.join("PULSE_EXECUTION.md")
    } else {
        root.join("waves").join("PULSE_EXECUTION.md")
    }
}

fn pulse_packet(wp: &vtrace::WorkPackage) -> String {
    format!(
        "# VTRACE Pulse Execution\n\nWork package: {}\nObjective: {}\nParent IDs: {}\nAffected surfaces: {}\nChecks: {}\nStatus: {}\n",
        wp.id,
        wp.objective,
        wp.parent_ids,
        wp.affected_surfaces,
        wp.validation_levels,
        wp.status
    )
}

fn option_value<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].as_str())
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|arg| arg == name)
}

fn positional_repo_arg(args: &[String], start: usize) -> &Path {
    args.iter()
        .skip(start)
        .find(|arg| !arg.starts_with("--") && provider_names().iter().all(|name| arg != name))
        .map(Path::new)
        .unwrap_or_else(|| Path::new("."))
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
