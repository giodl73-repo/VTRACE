use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const REQUIRED_FILES: [&str; 6] = [
    "MISSION.md",
    "REQUIREMENTS.md",
    "SPECIFICATION_BASELINE.md",
    "TRACE.md",
    "VERIFICATION.md",
    "REVIEW.md",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub level: String,
    pub path: PathBuf,
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageSummary {
    pub requirements: usize,
    pub specs: usize,
    pub work_packages: usize,
    pub evidence_rows: usize,
    pub open_work_packages: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkPackage {
    pub id: String,
    pub objective: String,
    pub parent_ids: String,
    pub affected_surfaces: String,
    pub entry_criteria: String,
    pub exit_criteria: String,
    pub validation_levels: String,
    pub status: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewLane {
    pub lane: String,
    pub required: String,
    pub decision: String,
    pub evidence: String,
}

impl Finding {
    pub fn render(&self, root: &Path) -> String {
        let shown = self
            .path
            .strip_prefix(root)
            .unwrap_or(&self.path)
            .display()
            .to_string();
        format!("{}: {}:{}: {}", self.level, shown, self.line, self.message)
    }
}

fn add(
    findings: &mut Vec<Finding>,
    level: &str,
    path: impl Into<PathBuf>,
    line: usize,
    message: impl Into<String>,
) {
    findings.push(Finding {
        level: level.to_string(),
        path: path.into(),
        line,
        message: message.into(),
    });
}

fn read_lines(path: &Path) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(ToOwned::to_owned)
        .collect()
}

fn split_row(line: &str) -> Vec<String> {
    let stripped = line.trim();
    if !stripped.starts_with('|') || !stripped.ends_with('|') {
        return Vec::new();
    }
    stripped
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}

fn is_separator(cells: &[String]) -> bool {
    !cells.is_empty()
        && cells.iter().all(|cell| {
            let compact = cell.replace(' ', "");
            !compact.is_empty() && compact.chars().all(|ch| matches!(ch, '-' | ':'))
        })
}

fn table_rows(path: &Path) -> Vec<(usize, BTreeMap<String, String>)> {
    let mut rows = Vec::new();
    let mut header: Option<Vec<String>> = None;

    for (line_index, line) in read_lines(path).iter().enumerate() {
        let line_no = line_index + 1;
        let cells = split_row(line);
        if cells.is_empty() {
            header = None;
            continue;
        }
        if is_separator(&cells) {
            continue;
        }
        match &header {
            None => header = Some(cells),
            Some(columns) if columns.len() == cells.len() => {
                rows.push((line_no, columns.iter().cloned().zip(cells).collect()));
            }
            Some(_) => {}
        }
    }

    rows
}

fn collect_ids(path: &Path, prefix: &str) -> BTreeMap<String, usize> {
    let mut ids = BTreeMap::new();
    if !path.exists() {
        return ids;
    }

    for (line_index, line) in read_lines(path).iter().enumerate() {
        let line_no = line_index + 1;
        for token in id_tokens(line) {
            if id_has_prefix(&token, prefix) {
                ids.entry(token).or_insert(line_no);
            }
        }
    }
    ids
}

fn id_tokens(line: &str) -> Vec<String> {
    line.split(|ch: char| !(ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '-'))
        .filter(|token| is_vtrace_id(token))
        .map(ToOwned::to_owned)
        .collect()
}

fn is_vtrace_id(token: &str) -> bool {
    let Some((head, number)) = token.rsplit_once('-') else {
        return false;
    };
    number.len() == 3
        && number.chars().all(|ch| ch.is_ascii_digit())
        && head
            .split('-')
            .all(|part| !part.is_empty() && part.chars().all(|ch| ch.is_ascii_uppercase()))
}

fn id_has_prefix(token: &str, prefix: &str) -> bool {
    token == prefix || token.starts_with(&format!("{prefix}-"))
}

fn check_required_files(vtrace_dir: &Path, findings: &mut Vec<Finding>) {
    for name in REQUIRED_FILES {
        let path = vtrace_dir.join(name);
        if !path.exists() {
            add(
                findings,
                "ERROR",
                path,
                1,
                format!("missing required VTRACE artifact {name}"),
            );
        }
    }
}

fn check_requirements_trace(vtrace_dir: &Path, findings: &mut Vec<Finding>) {
    let req_path = vtrace_dir.join("REQUIREMENTS.md");
    let trace_path = vtrace_dir.join("TRACE.md");
    let trace_text = fs::read_to_string(&trace_path).unwrap_or_default();

    for (req_id, line_no) in collect_ids(&req_path, "REQ") {
        if !trace_text.contains(&req_id) {
            add(
                findings,
                "ERROR",
                req_path.clone(),
                line_no,
                format!("{req_id} is not visible in TRACE.md"),
            );
        }
    }
}

fn check_spec_trace(vtrace_dir: &Path, findings: &mut Vec<Finding>) {
    let spec_path = vtrace_dir.join("SPECIFICATION_BASELINE.md");
    let trace_path = vtrace_dir.join("TRACE.md");
    let trace_text = fs::read_to_string(&trace_path).unwrap_or_default();

    for (spec_id, line_no) in collect_ids(&spec_path, "SPEC") {
        if !trace_text.contains(&spec_id) {
            add(
                findings,
                "ERROR",
                spec_path.clone(),
                line_no,
                format!("{spec_id} is not visible in TRACE.md"),
            );
        }
    }
}

fn check_evidence_pointers(vtrace_dir: &Path, findings: &mut Vec<Finding>) {
    let evidence_path = vtrace_dir.join("EVIDENCE.md");
    let trace_path = vtrace_dir.join("TRACE.md");
    if !evidence_path.exists() || !trace_path.exists() {
        return;
    }

    let evidence_ids: BTreeSet<String> = collect_ids(&evidence_path, "EVID").into_keys().collect();
    for (line_no, row) in table_rows(&trace_path) {
        let pointer = row
            .get("Evidence Pointer")
            .map(String::as_str)
            .unwrap_or("");
        for evid_id in id_tokens(pointer)
            .into_iter()
            .filter(|token| id_has_prefix(token, "EVID"))
        {
            if !evidence_ids.contains(&evid_id) {
                add(
                    findings,
                    "ERROR",
                    trace_path.clone(),
                    line_no,
                    format!("{evid_id} is not defined in EVIDENCE.md"),
                );
            }
        }
    }
}

fn check_evidence_ledger(vtrace_dir: &Path, findings: &mut Vec<Finding>) {
    let evidence_path = vtrace_dir.join("EVIDENCE.md");
    if !evidence_path.exists() {
        return;
    }

    let mut saw_evidence = false;
    for (line_no, row) in table_rows(&evidence_path) {
        let evidence_id = row.get("Evidence ID").map(String::as_str).unwrap_or("");
        if !evidence_id.starts_with("EVID-") {
            continue;
        }
        saw_evidence = true;

        let status = row
            .get("Status")
            .map(|value| value.to_ascii_lowercase())
            .unwrap_or_default();
        if matches!(status.as_str(), "" | "pending" | "planned") {
            add(
                findings,
                "ERROR",
                evidence_path.clone(),
                line_no,
                format!("{evidence_id} has incomplete evidence status"),
            );
        }

        let source = row
            .get("Source / Command")
            .or_else(|| row.get("Pointer"))
            .map(String::as_str)
            .unwrap_or("")
            .trim();
        if source.is_empty() {
            add(
                findings,
                "ERROR",
                evidence_path.clone(),
                line_no,
                format!("{evidence_id} is missing evidence source or command"),
            );
        }
    }

    if !saw_evidence {
        add(
            findings,
            "ERROR",
            evidence_path,
            1,
            "EVIDENCE.md has no EVID-* rows",
        );
    }
}

fn check_work_packages(vtrace_dir: &Path, findings: &mut Vec<Finding>) {
    let wp_path = vtrace_dir.join("WORK_PACKAGES.md");
    if !wp_path.exists() {
        return;
    }

    for (line_no, row) in table_rows(&wp_path) {
        let wp_id = row.get("ID").map(String::as_str).unwrap_or("");
        if !wp_id.starts_with("WP-") {
            continue;
        }
        for column in [
            "Parent IDs",
            "Entry Criteria",
            "Exit Criteria",
            "L0 / L1 / L2",
        ] {
            if row
                .get(column)
                .map(|value| value.trim())
                .unwrap_or("")
                .is_empty()
            {
                add(
                    findings,
                    "ERROR",
                    wp_path.clone(),
                    line_no,
                    format!("{wp_id} has blank {column}"),
                );
            }
        }

        let levels = row.get("L0 / L1 / L2").map(String::as_str).unwrap_or("");
        for level in ["L0", "L1", "L2"] {
            if !levels.contains(level) {
                add(
                    findings,
                    "ERROR",
                    wp_path.clone(),
                    line_no,
                    format!("{wp_id} is missing {level} validation expectation"),
                );
            }
        }
    }
}

fn check_review_lanes(vtrace_dir: &Path, findings: &mut Vec<Finding>) {
    let review_path = vtrace_dir.join("REVIEW.md");
    if !review_path.exists() {
        return;
    }

    let mut seen_lane = false;
    for (line_no, row) in table_rows(&review_path) {
        if !(row.contains_key("Lane")
            && row.contains_key("Required")
            && row.contains_key("Decision"))
        {
            continue;
        }
        seen_lane = true;

        let required = row
            .get("Required")
            .map(|value| value.to_ascii_lowercase())
            .unwrap_or_default();
        let decision = row
            .get("Decision")
            .map(|value| value.to_ascii_lowercase())
            .unwrap_or_default();
        if required == "yes" && matches!(decision.as_str(), "" | "pending" | "blocked") {
            let lane = row.get("Lane").map(String::as_str).unwrap_or("");
            add(
                findings,
                "ERROR",
                review_path.clone(),
                line_no,
                format!("required review lane {lane} is not closed"),
            );
        }
    }

    if !seen_lane {
        add(
            findings,
            "ERROR",
            review_path,
            1,
            "review lane table is missing",
        );
    }
}

fn check_review_checklists(vtrace_dir: &Path, findings: &mut Vec<Finding>) {
    let checklist_path = vtrace_dir.join("REVIEW_CHECKLISTS.md");
    if !checklist_path.exists() {
        return;
    }

    let mut saw_required = false;
    for (line_no, row) in table_rows(&checklist_path) {
        if !(row.contains_key("Gate")
            && row.contains_key("Item")
            && row.contains_key("Required")
            && row.contains_key("Decision"))
        {
            continue;
        }
        if row
            .get("Required")
            .map(|value| value.to_ascii_lowercase())
            .unwrap_or_default()
            != "yes"
        {
            continue;
        }
        saw_required = true;

        let decision = row
            .get("Decision")
            .map(|value| value.to_ascii_lowercase())
            .unwrap_or_default();
        if matches!(decision.as_str(), "" | "pending" | "blocked") {
            let gate = row
                .get("Gate")
                .map(String::as_str)
                .unwrap_or("unknown gate");
            add(
                findings,
                "ERROR",
                checklist_path.clone(),
                line_no,
                format!("required checklist item is not closed for {gate}"),
            );
        }
    }

    if !saw_required {
        add(
            findings,
            "ERROR",
            checklist_path,
            1,
            "REVIEW_CHECKLISTS.md has no required checklist rows",
        );
    }
}

fn check_language_profiles(vtrace_dir: &Path, findings: &mut Vec<Finding>) {
    let profile_path = vtrace_dir.join("LANGUAGE_PROFILES.md");
    if !profile_path.exists() {
        return;
    }

    let required_columns = ["Profile ID", "Applicability", "L0", "L1", "L2"];
    let mut saw_profile = false;
    for (line_no, row) in table_rows(&profile_path) {
        if !required_columns
            .iter()
            .all(|column| row.contains_key(*column))
        {
            continue;
        }
        let profile_id = row.get("Profile ID").map(String::as_str).unwrap_or("");
        if !profile_id.starts_with("PROFILE-") {
            continue;
        }
        saw_profile = true;

        for column in required_columns {
            if row
                .get(column)
                .map(|value| value.trim())
                .unwrap_or("")
                .is_empty()
            {
                add(
                    findings,
                    "ERROR",
                    profile_path.clone(),
                    line_no,
                    format!("{profile_id} has blank {column}"),
                );
            }
        }
    }

    if !saw_profile {
        add(
            findings,
            "ERROR",
            profile_path,
            1,
            "LANGUAGE_PROFILES.md has no PROFILE-* rows",
        );
    }
}

fn vtrace_dir(root: &Path) -> PathBuf {
    root.join("docs").join("vtrace")
}

pub fn package_summary(root: &Path) -> PackageSummary {
    let vtrace_dir = vtrace_dir(root);
    let requirements = collect_ids(&vtrace_dir.join("REQUIREMENTS.md"), "REQ").len();
    let specs = collect_ids(&vtrace_dir.join("SPECIFICATION_BASELINE.md"), "SPEC").len();
    let evidence_rows = collect_ids(&vtrace_dir.join("EVIDENCE.md"), "EVID").len();

    let mut work_packages = 0;
    let mut open_work_packages = Vec::new();
    for (_, row) in table_rows(&vtrace_dir.join("WORK_PACKAGES.md")) {
        let id = row.get("ID").map(String::as_str).unwrap_or("");
        if !id.starts_with("WP-") {
            continue;
        }
        work_packages += 1;
        let status = row
            .get("Status")
            .map(|value| value.to_ascii_lowercase())
            .unwrap_or_default();
        if !matches!(status.as_str(), "complete" | "closed" | "passed") {
            open_work_packages.push(id.to_string());
        }
    }

    PackageSummary {
        requirements,
        specs,
        work_packages,
        evidence_rows,
        open_work_packages,
    }
}

pub fn work_package(root: &Path, wanted_id: &str) -> Option<WorkPackage> {
    work_packages(root)
        .into_iter()
        .find(|work_package| work_package.id == wanted_id)
}

pub fn work_packages(root: &Path) -> Vec<WorkPackage> {
    let wp_path = vtrace_dir(root).join("WORK_PACKAGES.md");
    table_rows(&wp_path)
        .into_iter()
        .filter_map(|(line, row)| {
            let id = row.get("ID")?;
            if !id.starts_with("WP-") {
                return None;
            }
            Some(WorkPackage {
                id: id.to_string(),
                objective: row.get("Objective").cloned().unwrap_or_default(),
                parent_ids: row.get("Parent IDs").cloned().unwrap_or_default(),
                affected_surfaces: row.get("Affected Surfaces").cloned().unwrap_or_default(),
                entry_criteria: row.get("Entry Criteria").cloned().unwrap_or_default(),
                exit_criteria: row.get("Exit Criteria").cloned().unwrap_or_default(),
                validation_levels: row.get("L0 / L1 / L2").cloned().unwrap_or_default(),
                status: row.get("Status").cloned().unwrap_or_default(),
                line,
            })
        })
        .collect()
}

pub fn review_lanes(root: &Path) -> Vec<ReviewLane> {
    let review_path = vtrace_dir(root).join("REVIEW.md");
    table_rows(&review_path)
        .into_iter()
        .filter_map(|(_, row)| {
            let lane = row.get("Lane")?;
            Some(ReviewLane {
                lane: lane.to_string(),
                required: row.get("Required").cloned().unwrap_or_default(),
                decision: row.get("Decision").cloned().unwrap_or_default(),
                evidence: row
                    .get("Evidence / Rationale")
                    .or_else(|| row.get("Evidence"))
                    .cloned()
                    .unwrap_or_default(),
            })
        })
        .collect()
}

pub fn run_checks(root: &Path) -> Vec<Finding> {
    let root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let vtrace_dir = vtrace_dir(&root);
    let mut findings = Vec::new();

    if !vtrace_dir.exists() {
        add(
            &mut findings,
            "ERROR",
            vtrace_dir,
            1,
            "missing docs/vtrace directory",
        );
        return findings;
    }

    check_required_files(&vtrace_dir, &mut findings);
    check_requirements_trace(&vtrace_dir, &mut findings);
    check_spec_trace(&vtrace_dir, &mut findings);
    check_evidence_pointers(&vtrace_dir, &mut findings);
    check_evidence_ledger(&vtrace_dir, &mut findings);
    check_work_packages(&vtrace_dir, &mut findings);
    check_review_lanes(&vtrace_dir, &mut findings);
    check_review_checklists(&vtrace_dir, &mut findings);
    check_language_profiles(&vtrace_dir, &mut findings);
    findings
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TempRepo {
        root: PathBuf,
    }

    impl TempRepo {
        fn new() -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let root = std::env::temp_dir().join(format!("vtrace-check-{unique}"));
            fs::create_dir_all(root.join("docs").join("vtrace")).unwrap();
            Self { root }
        }

        fn vtrace_dir(&self) -> PathBuf {
            self.root.join("docs").join("vtrace")
        }

        fn write(&self, name: &str, content: &str) {
            fs::write(
                self.vtrace_dir().join(name),
                format!("{}\n", content.trim()),
            )
            .unwrap();
        }

        fn write_minimal_valid_package(&self) {
            self.write("MISSION.md", "# Mission");
            self.write(
                "REQUIREMENTS.md",
                r#"
                # Requirements

                | ID | Requirement |
                |---|---|
                | REQ-001 | The repo shall do the thing. |
                "#,
            );
            self.write(
                "SPECIFICATION_BASELINE.md",
                r#"
                # Specification Baseline

                | Spec ID | Parent REQ IDs | Status |
                |---|---|---|
                | SPEC-001 | REQ-001 | accepted |
                "#,
            );
            self.write(
                "TRACE.md",
                r#"
                # Trace

                | Requirement ID | Specification Item | Work Package | Evidence Pointer | Status |
                |---|---|---|---|---|
                | REQ-001 | SPEC-001 | WP-001 | EVID-001 | verified |
                "#,
            );
            self.write(
                "WORK_PACKAGES.md",
                r#"
                # Work Packages

                | ID | Parent IDs | Entry Criteria | Exit Criteria | L0 / L1 / L2 |
                |---|---|---|---|---|
                | WP-001 | REQ-001 / SPEC-001 | ready | done | L0: check / L1: test / L2: review |
                "#,
            );
            self.write("VERIFICATION.md", "# Verification");
            self.write(
                "REVIEW.md",
                r#"
                # Review

                | Lane | Required | Decision |
                |---|---|---|
                | Systems engineering | yes | pass |
                "#,
            );
            self.write(
                "EVIDENCE.md",
                r#"
                # Evidence

                | Evidence ID | Source / Command | Status |
                |---|---|---|
                | EVID-001 | command | passed |
                "#,
            );
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    fn messages(findings: &[Finding]) -> Vec<&str> {
        findings
            .iter()
            .map(|finding| finding.message.as_str())
            .collect()
    }

    #[test]
    fn valid_package_passes() {
        let repo = TempRepo::new();
        repo.write_minimal_valid_package();
        assert_eq!(run_checks(&repo.root), Vec::<Finding>::new());
    }

    #[test]
    fn missing_required_artifact_fails() {
        let repo = TempRepo::new();
        repo.write_minimal_valid_package();
        fs::remove_file(repo.vtrace_dir().join("TRACE.md")).unwrap();
        assert!(messages(&run_checks(&repo.root))
            .contains(&"missing required VTRACE artifact TRACE.md"));
    }

    #[test]
    fn deferred_requirement_must_remain_trace_visible() {
        let repo = TempRepo::new();
        repo.write_minimal_valid_package();
        fs::write(
            repo.vtrace_dir().join("REQUIREMENTS.md"),
            "# Requirements\n\n| ID | Requirement |\n|---|---|\n| REQ-001 | ok |\n| REQ-VAL-001 | Validator work is deferred. |\n",
        )
        .unwrap();
        assert!(
            messages(&run_checks(&repo.root)).contains(&"REQ-VAL-001 is not visible in TRACE.md")
        );
    }

    #[test]
    fn required_review_lane_must_close() {
        let repo = TempRepo::new();
        repo.write_minimal_valid_package();
        repo.write(
            "REVIEW.md",
            r#"
            # Review

            | Lane | Required | Decision |
            |---|---|---|
            | Systems engineering | yes | pending |
            "#,
        );
        assert!(messages(&run_checks(&repo.root))
            .contains(&"required review lane Systems engineering is not closed"));
    }

    #[test]
    fn trace_evidence_must_exist_in_ledger() {
        let repo = TempRepo::new();
        repo.write_minimal_valid_package();
        repo.write(
            "EVIDENCE.md",
            r#"
            # Evidence

            | Evidence ID | Status |
            |---|---|
            | EVID-999 | passed |
            "#,
        );
        assert!(
            messages(&run_checks(&repo.root)).contains(&"EVID-001 is not defined in EVIDENCE.md")
        );
    }

    #[test]
    fn evidence_status_must_be_complete() {
        let repo = TempRepo::new();
        repo.write_minimal_valid_package();
        repo.write(
            "EVIDENCE.md",
            r#"
            # Evidence

            | Evidence ID | Source / Command | Status |
            |---|---|---|
            | EVID-001 | command | pending |
            "#,
        );
        assert!(
            messages(&run_checks(&repo.root)).contains(&"EVID-001 has incomplete evidence status")
        );
    }

    #[test]
    fn review_checklist_required_item_must_close() {
        let repo = TempRepo::new();
        repo.write_minimal_valid_package();
        repo.write(
            "REVIEW_CHECKLISTS.md",
            r#"
            # Review Checklists

            | Gate | Item | Required | Decision |
            |---|---|---|---|
            | Specification Review | Requirements are testable. | yes | pending |
            "#,
        );
        assert!(messages(&run_checks(&repo.root))
            .contains(&"required checklist item is not closed for Specification Review"));
    }

    #[test]
    fn language_profiles_need_validation_levels() {
        let repo = TempRepo::new();
        repo.write_minimal_valid_package();
        repo.write(
            "LANGUAGE_PROFILES.md",
            r#"
            # Language Profiles

            | Profile ID | Applicability | L0 | L1 | L2 |
            |---|---|---|---|---|
            | PROFILE-PYTHON-001 | Python package | py_compile | unittest |  |
            "#,
        );
        assert!(messages(&run_checks(&repo.root)).contains(&"PROFILE-PYTHON-001 has blank L2"));
    }
}
