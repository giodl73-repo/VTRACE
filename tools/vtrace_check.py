#!/usr/bin/env python3
"""Validate a repo-local VTRACE package.

The checker is intentionally small and dependency-free. It verifies the
contracts that VTRACE currently relies on: required artifacts, trace visibility
for requirements, specification coverage, evidence pointers, work-package
shape, and review-lane closure.
"""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path


REQUIRED_FILES = (
    "MISSION.md",
    "REQUIREMENTS.md",
    "SPECIFICATION_BASELINE.md",
    "TRACE.md",
    "VERIFICATION.md",
    "REVIEW.md",
)

ID_RE = re.compile(r"\b[A-Z]+(?:-[A-Z]+)*-\d{3}\b")
REQ_RE = re.compile(r"\bREQ(?:-[A-Z]+)*-\d{3}\b")
SPEC_RE = re.compile(r"\bSPEC(?:-[A-Z]+)*-\d{3}\b")
EVID_RE = re.compile(r"\bEVID(?:-[A-Z]+)*-\d{3}\b")


@dataclass(frozen=True)
class Finding:
    level: str
    path: Path
    line: int
    message: str

    def render(self, root: Path) -> str:
        try:
            shown = self.path.relative_to(root)
        except ValueError:
            shown = self.path
        return f"{self.level}: {shown}:{self.line}: {self.message}"


def read_lines(path: Path) -> list[str]:
    return path.read_text(encoding="utf-8").splitlines()


def split_row(line: str) -> list[str]:
    stripped = line.strip()
    if not stripped.startswith("|") or not stripped.endswith("|"):
        return []
    return [cell.strip() for cell in stripped.strip("|").split("|")]


def is_separator(cells: list[str]) -> bool:
    return bool(cells) and all(set(cell.replace(" ", "")) <= {"-", ":"} for cell in cells)


def table_rows(path: Path) -> list[tuple[int, dict[str, str]]]:
    rows: list[tuple[int, dict[str, str]]] = []
    header: list[str] | None = None

    for line_no, line in enumerate(read_lines(path), start=1):
        cells = split_row(line)
        if not cells:
            header = None
            continue
        if is_separator(cells):
            continue
        if header is None:
            header = cells
            continue
        if len(cells) != len(header):
            continue
        rows.append((line_no, dict(zip(header, cells))))

    return rows


def collect_ids(path: Path, pattern: re.Pattern[str]) -> dict[str, int]:
    ids: dict[str, int] = {}
    if not path.exists():
        return ids
    for line_no, line in enumerate(read_lines(path), start=1):
        for item in pattern.findall(line):
            ids.setdefault(item, line_no)
    return ids


def add(finding_list: list[Finding], level: str, path: Path, line: int, message: str) -> None:
    finding_list.append(Finding(level, path, line, message))


def check_required_files(root: Path, vtrace_dir: Path, findings: list[Finding]) -> None:
    for name in REQUIRED_FILES:
        path = vtrace_dir / name
        if not path.exists():
            add(findings, "ERROR", path, 1, f"missing required VTRACE artifact {name}")


def check_requirements_trace(vtrace_dir: Path, findings: list[Finding]) -> None:
    req_path = vtrace_dir / "REQUIREMENTS.md"
    trace_path = vtrace_dir / "TRACE.md"
    req_ids = collect_ids(req_path, REQ_RE)
    trace_text = trace_path.read_text(encoding="utf-8") if trace_path.exists() else ""

    for req_id, line_no in sorted(req_ids.items()):
        if req_id not in trace_text:
            add(findings, "ERROR", req_path, line_no, f"{req_id} is not visible in TRACE.md")


def check_spec_trace(vtrace_dir: Path, findings: list[Finding]) -> None:
    spec_path = vtrace_dir / "SPECIFICATION_BASELINE.md"
    trace_path = vtrace_dir / "TRACE.md"
    spec_ids = collect_ids(spec_path, SPEC_RE)
    trace_text = trace_path.read_text(encoding="utf-8") if trace_path.exists() else ""

    for spec_id, line_no in sorted(spec_ids.items()):
        if spec_id not in trace_text:
            add(findings, "ERROR", spec_path, line_no, f"{spec_id} is not visible in TRACE.md")


def check_evidence_pointers(vtrace_dir: Path, findings: list[Finding]) -> None:
    evidence_path = vtrace_dir / "EVIDENCE.md"
    trace_path = vtrace_dir / "TRACE.md"
    if not evidence_path.exists() or not trace_path.exists():
        return

    evidence_ids = set(collect_ids(evidence_path, EVID_RE))
    for line_no, row in table_rows(trace_path):
        pointer = row.get("Evidence Pointer", "")
        for evid_id in EVID_RE.findall(pointer):
            if evid_id not in evidence_ids:
                add(findings, "ERROR", trace_path, line_no, f"{evid_id} is not defined in EVIDENCE.md")


def check_evidence_ledger(vtrace_dir: Path, findings: list[Finding]) -> None:
    evidence_path = vtrace_dir / "EVIDENCE.md"
    if not evidence_path.exists():
        return

    saw_evidence = False
    for line_no, row in table_rows(evidence_path):
        evidence_id = row.get("Evidence ID", "")
        if not evidence_id.startswith("EVID-"):
            continue
        saw_evidence = True
        status = row.get("Status", "").lower()
        if status in {"", "pending", "planned"}:
            add(findings, "ERROR", evidence_path, line_no, f"{evidence_id} has incomplete evidence status")
        if not row.get("Source / Command", row.get("Pointer", "")).strip():
            add(findings, "ERROR", evidence_path, line_no, f"{evidence_id} is missing evidence source or command")
    if not saw_evidence:
        add(findings, "ERROR", evidence_path, 1, "EVIDENCE.md has no EVID-* rows")


def check_work_packages(vtrace_dir: Path, findings: list[Finding]) -> None:
    wp_path = vtrace_dir / "WORK_PACKAGES.md"
    if not wp_path.exists():
        return

    for line_no, row in table_rows(wp_path):
        wp_id = row.get("ID", "")
        if not wp_id.startswith("WP-"):
            continue
        for column in ("Parent IDs", "Entry Criteria", "Exit Criteria", "L0 / L1 / L2"):
            if not row.get(column, "").strip():
                add(findings, "ERROR", wp_path, line_no, f"{wp_id} has blank {column}")
        levels = row.get("L0 / L1 / L2", "")
        for level in ("L0", "L1", "L2"):
            if level not in levels:
                add(findings, "ERROR", wp_path, line_no, f"{wp_id} is missing {level} validation expectation")


def check_review_lanes(vtrace_dir: Path, findings: list[Finding]) -> None:
    review_path = vtrace_dir / "REVIEW.md"
    if not review_path.exists():
        return

    seen_lane = False
    for line_no, row in table_rows(review_path):
        if not {"Lane", "Required", "Decision"}.issubset(row):
            continue
        seen_lane = True
        lane = row.get("Lane", "")
        required = row.get("Required", "").lower()
        decision = row.get("Decision", "").lower()
        if required == "yes" and decision in {"", "pending", "blocked"}:
            add(findings, "ERROR", review_path, line_no, f"required review lane {lane} is not closed")
    if not seen_lane:
        add(findings, "ERROR", review_path, 1, "review lane table is missing")


def check_review_checklists(vtrace_dir: Path, findings: list[Finding]) -> None:
    checklist_path = vtrace_dir / "REVIEW_CHECKLISTS.md"
    if not checklist_path.exists():
        return

    saw_required = False
    for line_no, row in table_rows(checklist_path):
        if not {"Gate", "Item", "Required", "Decision"}.issubset(row):
            continue
        if row.get("Required", "").lower() != "yes":
            continue
        saw_required = True
        decision = row.get("Decision", "").lower()
        if decision in {"", "pending", "blocked"}:
            add(findings, "ERROR", checklist_path, line_no, f"required checklist item is not closed for {row.get('Gate', 'unknown gate')}")
    if not saw_required:
        add(findings, "ERROR", checklist_path, 1, "REVIEW_CHECKLISTS.md has no required checklist rows")


def check_language_profiles(vtrace_dir: Path, findings: list[Finding]) -> None:
    profile_path = vtrace_dir / "LANGUAGE_PROFILES.md"
    if not profile_path.exists():
        return

    saw_profile = False
    required_columns = ("Profile ID", "Applicability", "L0", "L1", "L2")
    for line_no, row in table_rows(profile_path):
        if not set(required_columns).issubset(row):
            continue
        profile_id = row.get("Profile ID", "")
        if not profile_id.startswith("PROFILE-"):
            continue
        saw_profile = True
        for column in required_columns:
            if not row.get(column, "").strip():
                add(findings, "ERROR", profile_path, line_no, f"{profile_id} has blank {column}")
    if not saw_profile:
        add(findings, "ERROR", profile_path, 1, "LANGUAGE_PROFILES.md has no PROFILE-* rows")


def run_checks(root: Path) -> list[Finding]:
    root = root.resolve()
    vtrace_dir = root / "docs" / "vtrace"
    findings: list[Finding] = []

    if not vtrace_dir.exists():
        add(findings, "ERROR", vtrace_dir, 1, "missing docs/vtrace directory")
        return findings

    check_required_files(root, vtrace_dir, findings)
    check_requirements_trace(vtrace_dir, findings)
    check_spec_trace(vtrace_dir, findings)
    check_evidence_pointers(vtrace_dir, findings)
    check_evidence_ledger(vtrace_dir, findings)
    check_work_packages(vtrace_dir, findings)
    check_review_lanes(vtrace_dir, findings)
    check_review_checklists(vtrace_dir, findings)
    check_language_profiles(vtrace_dir, findings)
    return findings


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Validate repo-local VTRACE artifacts.")
    parser.add_argument("repo", nargs="?", default=".", help="repository root to validate")
    args = parser.parse_args(argv)

    root = Path(args.repo)
    findings = run_checks(root)
    for finding in findings:
        print(finding.render(root.resolve()))

    if findings:
        print(f"VTRACE validation failed: {len(findings)} finding(s)")
        return 1

    print("VTRACE validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
