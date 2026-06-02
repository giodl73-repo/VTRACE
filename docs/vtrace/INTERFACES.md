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
| IF-007 | CLI orchestrator commands | VTRACE | New commands must preserve deterministic validation behavior, avoid silent artifact edits, and write evidence/review changes only through explicit user-selected operations. | CLI tests / design review | accepted |
| IF-008 | Provider adapter boundary | VTRACE | Provider adapters may draft, summarize, or review; they must not mark evidence passed, close work packages, bypass source custody, or promote claims without review. | role review / security review | accepted |
| IF-009 | Communications strategy artifact | VTRACE | `COMMUNICATIONS_STRATEGY.md` rows use stable `COMMS-*` IDs and map source IDs to user-facing docs surfaces without treating docs as specifications. | validator / inspection | accepted |
