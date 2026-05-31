import shutil
import tempfile
import unittest
from pathlib import Path

from tools.vtrace_check import run_checks


class VTraceCheckTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tmp = Path(tempfile.mkdtemp(prefix="vtrace-check-"))
        self.vtrace = self.tmp / "docs" / "vtrace"
        self.vtrace.mkdir(parents=True)

    def tearDown(self) -> None:
        shutil.rmtree(self.tmp)

    def write(self, name: str, content: str) -> None:
        (self.vtrace / name).write_text(content.strip() + "\n", encoding="utf-8")

    def write_minimal_valid_package(self) -> None:
        self.write("MISSION.md", "# Mission")
        self.write(
            "REQUIREMENTS.md",
            """
            # Requirements

            | ID | Requirement |
            |---|---|
            | REQ-001 | The repo shall do the thing. |
            """,
        )
        self.write(
            "SPECIFICATION_BASELINE.md",
            """
            # Specification Baseline

            | Spec ID | Parent REQ IDs | Status |
            |---|---|---|
            | SPEC-001 | REQ-001 | accepted |
            """,
        )
        self.write(
            "TRACE.md",
            """
            # Trace

            | Requirement ID | Specification Item | Work Package | Evidence Pointer | Status |
            |---|---|---|---|---|
            | REQ-001 | SPEC-001 | WP-001 | EVID-001 | verified |
            """,
        )
        self.write(
            "WORK_PACKAGES.md",
            """
            # Work Packages

            | ID | Parent IDs | Entry Criteria | Exit Criteria | L0 / L1 / L2 |
            |---|---|---|---|---|
            | WP-001 | REQ-001 / SPEC-001 | ready | done | L0: check / L1: test / L2: review |
            """,
        )
        self.write("VERIFICATION.md", "# Verification")
        self.write(
            "REVIEW.md",
            """
            # Review

            | Lane | Required | Decision |
            |---|---|---|
            | Systems engineering | yes | pass |
            """,
        )
        self.write(
            "EVIDENCE.md",
            """
            # Evidence

            | Evidence ID | Source / Command | Status |
            |---|---|---|
            | EVID-001 | command | passed |
            """,
        )

    def test_valid_package_passes(self) -> None:
        self.write_minimal_valid_package()

        self.assertEqual(run_checks(self.tmp), [])

    def test_missing_required_artifact_fails(self) -> None:
        self.write_minimal_valid_package()
        (self.vtrace / "TRACE.md").unlink()

        findings = run_checks(self.tmp)

        self.assertTrue(any("missing required VTRACE artifact TRACE.md" in item.message for item in findings))

    def test_deferred_requirement_must_remain_trace_visible(self) -> None:
        self.write_minimal_valid_package()
        with (self.vtrace / "REQUIREMENTS.md").open("a", encoding="utf-8") as handle:
            handle.write("| REQ-VAL-001 | Validator work is deferred. |\n")

        findings = run_checks(self.tmp)

        self.assertTrue(any("REQ-VAL-001 is not visible in TRACE.md" in item.message for item in findings))

    def test_required_review_lane_must_close(self) -> None:
        self.write_minimal_valid_package()
        self.write(
            "REVIEW.md",
            """
            # Review

            | Lane | Required | Decision |
            |---|---|---|
            | Systems engineering | yes | pending |
            """,
        )

        findings = run_checks(self.tmp)

        self.assertTrue(any("required review lane Systems engineering is not closed" in item.message for item in findings))

    def test_trace_evidence_must_exist_in_ledger(self) -> None:
        self.write_minimal_valid_package()
        self.write(
            "EVIDENCE.md",
            """
            # Evidence

            | Evidence ID | Status |
            |---|---|
            | EVID-999 | passed |
            """,
        )

        findings = run_checks(self.tmp)

        self.assertTrue(any("EVID-001 is not defined in EVIDENCE.md" in item.message for item in findings))

    def test_evidence_status_must_be_complete(self) -> None:
        self.write_minimal_valid_package()
        self.write(
            "EVIDENCE.md",
            """
            # Evidence

            | Evidence ID | Source / Command | Status |
            |---|---|---|
            | EVID-001 | command | pending |
            """,
        )

        findings = run_checks(self.tmp)

        self.assertTrue(any("EVID-001 has incomplete evidence status" in item.message for item in findings))

    def test_review_checklist_required_item_must_close(self) -> None:
        self.write_minimal_valid_package()
        self.write(
            "REVIEW_CHECKLISTS.md",
            """
            # Review Checklists

            | Gate | Item | Required | Decision |
            |---|---|---|---|
            | Specification Review | Requirements are testable. | yes | pending |
            """,
        )

        findings = run_checks(self.tmp)

        self.assertTrue(any("required checklist item is not closed" in item.message for item in findings))

    def test_language_profiles_need_validation_levels(self) -> None:
        self.write_minimal_valid_package()
        self.write(
            "LANGUAGE_PROFILES.md",
            """
            # Language Profiles

            | Profile ID | Applicability | L0 | L1 | L2 |
            |---|---|---|---|---|
            | PROFILE-PYTHON-001 | Python package | py_compile | unittest |  |
            """,
        )

        findings = run_checks(self.tmp)

        self.assertTrue(any("PROFILE-PYTHON-001 has blank L2" in item.message for item in findings))


if __name__ == "__main__":
    unittest.main()
