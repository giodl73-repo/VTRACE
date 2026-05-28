# VTRACE Product Plan

## Thesis

VTRACE helps software codebases adopt systems engineering discipline without
turning every project into a heavyweight compliance program. The core artifact
is a traceability spine: every meaningful build decision should connect to a
need, requirement, design element, implementation surface, verification method,
validation method, and objective evidence.

## Users

- Codebase maintainers who need stronger engineering gates.
- Agent operators who want repeatable review and implementation discipline.
- Portfolio owners who need comparable readiness evidence across repos.
- Teams preparing code for safety, compliance, assurance, or high-cost change.

## Foundation Scope

VTRACE begins as a standards/protocols repo:

- document a NASA-inspired V-model development framework,
- create reusable Codex skills for applying the framework,
- define practical artifacts for requirements, V&V, reviews, and evidence,
- keep implementation guidance adaptable to each target repo.

## Non-Goals

- Do not claim NASA endorsement.
- Do not copy NASA lifecycle governance verbatim.
- Do not require a specific programming language or build system.
- Do not replace domain-specific safety, security, legal, or compliance review.
- Do not create product-to-product runtime dependencies across the portfolio.

## Waves

### Wave 1: Foundation

Create the repo, source-grounded framework notes, wave/pulse structure, and the
first reusable skill for codebase rigor.

### Wave 2: Traceability Artifacts

Define practical templates for:

- mission/stakeholder needs,
- requirement records,
- requirement verification matrices,
- interface control records,
- V&V plans,
- objective evidence ledgers.

### Wave 2A: Source Custody and Encoding

Define the source registry, rights posture, and encoded concept shape before
downloading or deriving from standards. Keep public NASA guidance, NASA
standards, and copyrighted industry standards in separate handling lanes.

### Wave 3: Repo Adoption Path

Apply VTRACE to one existing repo and record the before/after evidence:

- initial rigor assessment,
- missing trace links,
- proposed gates,
- implemented docs or checks,
- validation command results.

The process is now codified as focused skills plus stage templates. The next
step is applying it to a real repo and tightening the artifacts based on use.

### Wave 4: Tooling Candidates

Prototype lightweight validators only after the document contracts stabilize.
Candidates include Markdown checks, trace matrix consistency checks, and evidence
ledger completeness checks.

## Dependency Posture

VTRACE is a standards/protocols repo. It may later use PROOF for Markdown and
report validation, MDPATH for stable references, ROLES for review panels, and
CROP/PEBBLE for portable context packs. None are required in the foundation
commit.
