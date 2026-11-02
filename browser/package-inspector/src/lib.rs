//! Browser admission and transport over the original package validator.
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use vtrace::PackageReader;
use wasm_bindgen::prelude::*;

pub const REQUEST_BYTES: usize = 1_048_576;
const REPORT_BYTES: usize = 2_097_152;
const FILE_BYTES: usize = 65_536;
const PACKAGE_BYTES: usize = 262_144;
const LINE_BYTES: usize = 2048;
const FILE_LINES: usize = 2048;
// Browser input admission contract: every artifact read by the current core.
pub const ARTIFACTS: [&str; 11] = [
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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    files: Vec<Document>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    name: String,
    text: String,
}

struct Supplied {
    files: BTreeMap<String, String>,
}
impl Supplied {
    fn name<'a>(&self, path: &'a Path) -> Option<&'a str> {
        path.strip_prefix(package_dir()).ok()?.to_str()
    }
}
fn root() -> &'static Path {
    Path::new("supplied")
}
fn package_dir() -> PathBuf {
    root().join("docs").join("vtrace")
}
impl PackageReader for Supplied {
    fn exists(&self, path: &Path) -> bool {
        path == package_dir()
            || self
                .name(path)
                .is_some_and(|name| self.files.contains_key(name))
    }
    fn read_to_string(&self, path: &Path) -> std::io::Result<String> {
        self.name(path)
            .and_then(|name| self.files.get(name))
            .cloned()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
    }
}
fn bounded(text: &str, max: usize, label: &str) -> Result<(), String> {
    if text.len() > max {
        Err(format!("{label} exceeds {max} UTF-8 bytes"))
    } else {
        Ok(())
    }
}
fn inspect(request: Request) -> Result<Value, String> {
    if request.files.len() > ARTIFACTS.len() {
        return Err("At most eleven named artifacts".into());
    }
    let mut total = 0;
    let mut files = BTreeMap::new();
    let mut inputs = Vec::new();
    for document in request.files {
        bounded(&document.name, 64, "Artifact filename")?;
        if !ARTIFACTS.contains(&document.name.as_str()) {
            return Err(format!(
                "Unknown artifact name {:?}; supply an exact docs/vtrace filename",
                document.name
            ));
        }
        bounded(&document.text, FILE_BYTES, &document.name)?;
        total += document.text.len();
        if total > PACKAGE_BYTES {
            return Err("Complete package exceeds 256 KiB".into());
        }
        let mut lines = 0;
        for line in document.text.lines() {
            lines += 1;
            bounded(line, LINE_BYTES, "Artifact line")?;
        }
        if lines > FILE_LINES {
            return Err(format!("{} exceeds {FILE_LINES} lines", document.name));
        }
        inputs.push(json!({"name":document.name,"bytes":document.text.len(),"lines":lines}));
        if files.insert(document.name, document.text).is_some() {
            return Err("Duplicate artifact filename; no last-file-wins import".into());
        }
    }
    let source = Supplied { files };
    let findings = vtrace::run_checks_with_reader(&source, root());
    let summary = vtrace::package_summary_with_reader(&source, root());
    let work = vtrace::work_packages_with_reader(&source, root());
    let reviews = vtrace::review_lanes_with_reader(&source, root());
    let comms = vtrace::communication_surfaces_with_reader(&source, root());
    let findings = findings.into_iter().map(|finding|json!({
        "level":finding.level,
        "path":finding.path.strip_prefix(root()).unwrap_or(&finding.path).to_string_lossy().replace('\\',"/"),
        "line":finding.line,"message":finding.message
    })).collect::<Vec<_>>();
    Ok(json!({
        "kind":"report","findings":findings,"supplied_files":inputs,"accepted_artifact_names":ARTIFACTS,
        "summary":{"requirements":summary.requirements,"specs":summary.specs,"work_packages":summary.work_packages,"evidence_rows":summary.evidence_rows,"open_work_packages":summary.open_work_packages},
        "work_packages":work.into_iter().map(|w|json!({"id":w.id,"objective":w.objective,"parent_ids":w.parent_ids,"affected_surfaces":w.affected_surfaces,"entry_criteria":w.entry_criteria,"exit_criteria":w.exit_criteria,"validation_levels":w.validation_levels,"status":w.status,"line":w.line})).collect::<Vec<_>>(),
        "review_lanes":reviews.into_iter().map(|r|json!({"lane":r.lane,"required":r.required,"decision":r.decision,"evidence":r.evidence})).collect::<Vec<_>>(),
        "communication_surfaces":comms.into_iter().map(|c|json!({"id":c.id,"source_ids":c.source_ids,"audience":c.audience,"user_question":c.user_question,"generated_docs":c.generated_docs,"cadence":c.cadence,"owner":c.owner,"status":c.status})).collect::<Vec<_>>(),
        "scope":"Structural checks over supplied docs/vtrace text. Optional omissions, substring visibility, duplicate-ID collapse and skipped malformed rows retain native semantics. Listed evidence, status strings, review decisions, commands and URLs are caller assertions; no execution, authenticity, completeness, product readiness, safety, compliance or NASA endorsement is established. Original/current inputs retain unknown Markdown content separately from these native known-field views."
    }))
}

#[wasm_bindgen]
pub fn evaluate(input: &str) -> String {
    let value = if input.len() > REQUEST_BYTES {
        json!({"kind":"browser_refusal","message":"Request exceeds 1 MiB"})
    } else {
        match serde_json::from_str::<Request>(input) {
            Err(error) => json!({"kind":"input_error","message":error.to_string()}),
            Ok(request) => match inspect(request) {
                Ok(report) => report,
                Err(message) => json!({"kind":"browser_refusal","message":message}),
            },
        }
    };
    let report = value.to_string();
    if report.len() > REPORT_BYTES {
        json!({"kind":"browser_refusal","message":"Complete report exceeds 2 MiB; no partial result accepted"}).to_string()
    } else {
        report
    }
}
