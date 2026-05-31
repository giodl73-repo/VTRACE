# Language Profiles

## Scope

Repo: VTRACE

## Active Profiles

| Profile ID | Applicability | L0 | L1 | L2 |
|---|---|---|---|---|
| PROFILE-DOCS-001 | VTRACE framework, templates, roles, source maps, and self-trace docs. | `git diff --check` | `py tools\vtrace_check.py .`; source-custody inspection when sources change | Role/gate review in `docs/vtrace/REVIEW.md` |
| PROFILE-PYTHON-001 | `tools/vtrace_check.py`, Python examples, and validator tests. | `py -m py_compile tools\vtrace_check.py`; `py -m py_compile examples\hello-world\src\hello_world.py` | `py -m unittest discover -s tests -p "test_*.py"`; example CLI smoke checks | Validator run against VTRACE and example repos |
| PROFILE-GEN-001 | Future schemas, generated reports, and generated evidence. | Source-of-truth and generation command identified | Regeneration produces expected diff or no diff | Generated output consumed by validator/example evidence |
| PROFILE-MULTI-001 | Future target repos with multiple language boundaries. | Per-language L0 checks named in `PACKAGE_BOUNDARIES.md` | Boundary integration command | End-to-end validation scenario |

## Tailoring Notes

| Profile ID | Local Override | Rationale | Reviewer |
|---|---|---|---|
| PROFILE-DOCS-001 | VTRACE self-trace requires more artifacts than target repo minimum slice. | VTRACE must prove itself thoroughly. | Systems Engineering Steward |
| PROFILE-PYTHON-001 | Validator stays stdlib-only. | Keeps adoption lightweight and offline. | Software Assurance Guardian |
