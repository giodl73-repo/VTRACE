# Interfaces

## Scope

Repo: VTRACE

## Controlled Interfaces

| ID | Surface | Owner | Compatibility Rule | Verification Method | Status |
|---|---|---|---|---|---|
| IF-001 | Skill frontmatter and `SKILL.md` workflow sections | VTRACE | Skills keep valid frontmatter and actionable workflow steps. | inspection | accepted |
| IF-002 | Adoption template filenames under `templates/adoption/` | VTRACE | Existing template names are stable for adopters unless changed through `CHANGE_CONTROL.md`. | inspection | accepted |
| IF-003 | VTRACE ID scheme | VTRACE | `NEED-*`, `CON-*`, `REQ-*`, `SPEC-*`, `ARCH-*`, `IF-*`, `DES-*`, `CR-*`, `WP-*`, `VER-*`, `VAL-*`, `EVID-*`, `DCR-*` remain stable conventions. | inspection | accepted |
| IF-004 | Source registry JSON | VTRACE | `sources/source-registry.json` remains parseable JSON with source custody metadata. | JSON parse | accepted |
| IF-005 | Validator CLI | VTRACE | `cargo run -- <repo>` returns nonzero for failed VTRACE checks and prints path/line findings; installed binaries expose the same behavior as `vtrace <repo>`. | cargo test / local command | accepted |
| IF-006 | CI workflow | VTRACE | `.github/workflows/ci.yml` runs the same Rust validation path used locally before merge. | workflow inspection / local command parity | accepted |
