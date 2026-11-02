//! The same original rules/extractions over independently supplied text and disk.
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use vtrace::*;

const ARTIFACTS: [&str; 11] = [
    "MISSION.md",
    "REQUIREMENTS.md",
    "SPECIFICATION_BASELINE.md",
    "TRACE.md",
    "VERIFICATION.md",
    "REVIEW.md",
    "EVIDENCE.md",
    "WORK_PACKAGES.md",
    "REVIEW_CHECKLISTS.md",
    "LANGUAGE_PROFILES.md",
    "COMMUNICATIONS_STRATEGY.md",
];

struct Supplied {
    files: BTreeMap<String, String>,
}
impl PackageReader for Supplied {
    fn exists(&self, path: &Path) -> bool {
        let directory = Path::new("supplied").join("docs/vtrace");
        path == directory
            || path
                .strip_prefix(directory)
                .ok()
                .and_then(Path::to_str)
                .is_some_and(|name| self.files.contains_key(name))
    }
    fn read_to_string(&self, path: &Path) -> std::io::Result<String> {
        path.file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| self.files.get(name))
            .cloned()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
    }
}

struct Fixture(PathBuf);
impl Fixture {
    fn new(files: &BTreeMap<String, String>, directory: bool) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root =
            std::env::temp_dir().join(format!("vtrace-reader-{}-{nonce}", std::process::id()));
        fs::create_dir(&root).unwrap();
        if directory {
            fs::create_dir_all(root.join("docs/vtrace")).unwrap();
            for (name, text) in files {
                fs::write(root.join("docs/vtrace").join(name), text).unwrap();
            }
        }
        Self(root.canonicalize().unwrap())
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn comparable(findings: Vec<Finding>, root: &Path) -> Vec<(String, String, usize, String)> {
    findings
        .into_iter()
        .map(|finding| {
            (
                finding.level,
                finding
                    .path
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/"),
                finding.line,
                finding.message,
            )
        })
        .collect()
}

fn compare(files: BTreeMap<String, String>) -> Vec<(String, String, usize, String)> {
    let disk = Fixture::new(&files, true);
    let supplied = Supplied { files };
    let root = Path::new("supplied");
    let actual = comparable(run_checks_with_reader(&supplied, root), root);
    assert_eq!(actual, comparable(run_checks(&disk.0), &disk.0));
    assert_eq!(
        package_summary_with_reader(&supplied, root),
        package_summary(&disk.0)
    );
    assert_eq!(
        work_packages_with_reader(&supplied, root),
        work_packages(&disk.0)
    );
    assert_eq!(
        work_package_with_reader(&supplied, root, "WP-009"),
        work_package(&disk.0, "WP-009")
    );
    assert_eq!(
        review_lanes_with_reader(&supplied, root),
        review_lanes(&disk.0)
    );
    assert_eq!(
        communication_surfaces_with_reader(&supplied, root),
        communication_surfaces(&disk.0)
    );
    actual
}

#[test]
fn complete_self_package_matches_every_native_query() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/vtrace");
    let files = ARTIFACTS
        .into_iter()
        .filter_map(|name| {
            fs::read_to_string(directory.join(name))
                .ok()
                .map(|text| (name.into(), text))
        })
        .collect();
    assert!(compare(files).is_empty());
}

#[test]
fn every_original_check_pass_keeps_its_findings_and_line_locations() {
    let files = [
        ("REQUIREMENTS.md", "| ID | Requirement |\n|---|---|\n| REQ-001 | Missing trace. |\n"),
        ("SPECIFICATION_BASELINE.md", "| Spec ID | Text |\n|---|---|\n| SPEC-001 | Missing trace. |\n"),
        ("TRACE.md", "| Evidence Pointer |\n|---|\n| EVID-999 |\n"),
        ("VERIFICATION.md", ""),
        ("EVIDENCE.md", "| Evidence ID | Source / Command | Status |\n|---|---|---|\n| EVID-002 | | pending |\n"),
        ("WORK_PACKAGES.md", "| ID | Parent IDs | Entry Criteria | Exit Criteria | L0 / L1 / L2 |\n|---|---|---|---|---|\n| WP-001 | | | | L0 |\n"),
        ("REVIEW.md", "| Lane | Required | Decision |\n|---|---|---|\n| Assurance | yes | pending |\n"),
        ("REVIEW_CHECKLISTS.md", "| Gate | Item | Required | Decision |\n|---|---|---|---|\n| Design | Review | yes | blocked |\n"),
        ("LANGUAGE_PROFILES.md", ""),
        ("COMMUNICATIONS_STRATEGY.md", ""),
    ].into_iter().map(|(name, text)| (name.into(), text.into())).collect();
    let findings = compare(files);
    for expected in [
        "missing required VTRACE artifact MISSION.md",
        "REQ-001 is not visible",
        "SPEC-001 is not visible",
        "EVID-999 is not defined",
        "incomplete evidence status",
        "missing evidence source or command",
        "blank Parent IDs",
        "missing L1",
        "required review lane Assurance is not closed",
        "required checklist item is not closed",
        "LANGUAGE_PROFILES.md has no PROFILE-* rows",
        "COMMUNICATIONS_STRATEGY.md has no COMMS-* rows",
    ] {
        assert!(
            findings.iter().any(|finding| finding.3.contains(expected)),
            "{expected}"
        );
    }
}

#[test]
fn existing_duplicate_substring_and_optional_omission_semantics_are_visible() {
    let mut files: BTreeMap<String, String> = ARTIFACTS[..6]
        .iter()
        .map(|name| (name.to_string(), String::new()))
        .collect();
    files.insert(
        "REQUIREMENTS.md".into(),
        "| ID | Text |\r\n|---|---|\r\n| REQ-001 | first |\r\n| REQ-001 | second |\r\n".into(),
    );
    files.insert("TRACE.md".into(), "REQ-0010 EVID-999".into());
    files.insert(
        "REVIEW.md".into(),
        "| Lane | Required | Decision |\n|---|---|---|\n| Assurance | yes | passed |\n".into(),
    );
    assert!(compare(files.clone()).is_empty());
    let source = Supplied { files };
    assert_eq!(
        package_summary_with_reader(&source, Path::new("supplied")).requirements,
        1
    );
}

#[test]
fn reader_can_represent_a_missing_package_directory() {
    struct Absent;
    impl PackageReader for Absent {
        fn exists(&self, _: &Path) -> bool {
            false
        }
        fn read_to_string(&self, _: &Path) -> std::io::Result<String> {
            panic!("Absent source must not be read")
        }
    }
    let disk = Fixture::new(&BTreeMap::new(), false);
    assert_eq!(
        comparable(
            run_checks_with_reader(&Absent, Path::new("supplied")),
            Path::new("supplied")
        ),
        comparable(run_checks(&disk.0), &disk.0)
    );
}
