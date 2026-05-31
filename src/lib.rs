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

pub fn run_checks(root: &Path) -> Vec<Finding> {
    let root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let vtrace_dir = root.join("docs").join("vtrace");
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
