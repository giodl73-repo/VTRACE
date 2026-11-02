# Wave: supplied-package browser inspector

## Engineering decision and reader outcome

A maintainer should be able to open their own `docs/vtrace` Markdown artifacts,
run the existing Rust structural checks, locate each finding in the original
file/line and inspect the original package summary, work-package, review-lane
and communication rows without installing a CLI. The browser feature is a
dedicated unpublished product companion, not repo-management infrastructure.
Its requirement is REQ-BROWSER-001; SPEC-BROWSER-001 is a target until qualified.
The existing filesystem validator and CLI are current behavior and stay valid.

Reuse the original library's complete ten check passes and public extraction
functions. Separate package reads/presence from those same rules through a
small Rust reader interface. Native public entry points retain filesystem and
canonicalization behavior. Browser reads refer only to supplied file contents.
No second validator, JavaScript policy or parser is introduced. The native
CLI's orchestration commands remain their own capability.

## Trace, work package and evidence

WP-BROWSER-001 belongs to this wave while implementation is open; its target
specification is `browser/package-inspector/SPEC.md`. The existing closed native
self-package and original CLI assertion that it has no open work packages are
preserved during portability qualification. New browser evidence is scoped to
this wave; structural status must never be presented as implementation approval.

Current canonical7d4f7d2 contains two PITFALL commits beyond public main df46242.
Preserve it remotely as pitfall/integration-boundaries-20261102. Isolate work in
the owned child worktree; the canonical checkout advances only after main/live
qualification and an explicit TRACKER snapshot.

Baseline: all nine original library tests and 22 original CLI tests pass;
formatting, strict Clippy, self-validation and migration validation pass. Freeze
16 original CLI controls over self, migration, empty and absent packages before
refactoring. Preserve original tests, CLI source, root manifest and lock.

## Agent role review (not human review or external endorsement)

- Repo Maintainer: own-file inspection must expose actionable original file/line
  findings and native rows, with an immediately working synthetic example.
- Verification and Validation Lead: compare complete native checks/extractions
  with real WASM and independent original CLI controls. No-findings means only
  these structural checks found nothing; optional absent documents are skipped,
  IDs use the existing substring/token heuristics and duplicate IDs may collapse.
- Security Privacy Guardian: bound whole requests, file counts, UTF-8 bytes,
  lines/line size, outputs and compiled memory. Worker deadlines and generation,
  job and file-revision fences retain accepted reports/exports after failure.
  Supplied references and commands remain inert text.
- Source Custody Counsel: preserve original MIT software/CC BY-NC content scopes
  and pointer-first source policy. Bundle owned software/synthetic examples with
  explicit source scope; external NASA/industry standard text is not mirrored.
  VTRACE is NASA-inspired, not NASA-authored or NASA-endorsed.
- Live Evidence Custodian: hosted/main/live claims need tool, auth context,
  run/URL, commit, action, timestamp and outcome. Deterministic browser reports
  do not execute or certify listed commands, provider output, external evidence,
  actual review performance, safety/compliance or product acceptance.

## Gates and validation

1. Commit scope before implementation. Capture baseline and preserve canonical
   history. Qualify the reader separation against all original tests and controls;
   run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test`, self and migration validation, and a wasm32 library build.
2. Build the dedicated bounded reader-task companion using that library. Exercise
   all native checks/extractions, empty/malformed/duplicate/Unicode inputs,
   optional omissions and false assurance examples. Compare native/WASM results
   and independent original CLI output, including resource-limit boundaries.
3. Ship editable own-file play, native findings and source-line navigation,
   complete raw/known-field exports, input/report custody, actual read/startup/
   compute deadlines, retry recovery, keyboard/mobile and independent source/
   build/legal routes. Never truncate data or relabel caller assertions as proof.
4. Package exact corresponding source and notices, rebuild outside Git, qualify
   hosted/default/fresh live behavior and every served hash. Commit/push child
   work first; advance TRACKER only with an explicit qualified pulse.

The broader portfolio remains active. RCOUNT/RPLAN are coordinated with BISECT;
ICELINES/BISECT/TIG, owner REEL work, media and the shared Cargo cache are protected.
