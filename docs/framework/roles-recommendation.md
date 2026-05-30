# Role Recommendations

VTRACE target repos should use the ROLES repo convention when review quality
matters. The minimum ROLES-conformant target is `.roles/ROLE.md`; recommended
panels use `.roles/parliament/`, `.roles/editorial/`, and
`.roles/stakeholders/` with role frontmatter.

## Minimum VTRACE Panel

For a non-trivial implementation repo, recommend:

```text
.roles/
  ROLE.md
  parliament/
    systems-engineering-steward.md
    requirements-traceability-auditor.md
    verification-validation-lead.md
    software-assurance-guardian.md
  stakeholders/
    repo-maintainer.md
    future-agent.md
```

## Add Roles By Risk

| Trigger | Add Role |
|---|---|
| Public API, CLI, schema, file format, or downstream consumer | Interface / Boundary Reviewer |
| Rust crate, package architecture, module split, dependency direction | Package Boundary Steward |
| Security, privacy, auth, secrets, network, file operations, dependencies | Security Privacy Guardian |
| Safety, civic, financial, legal, operational, medical, or irreversible impact | Safety Risk Officer |
| Source registry, standards, license, provenance, data custody | Source Custody Counsel |
| Generated code, parsers, schemas, build outputs | Generated Artifact Auditor |
| Public README/release claims | Public Claims Editor |

## Role Frontmatter

Use the ROLES frontmatter style:

```yaml
---
name: Package Boundary Steward
slug: package-boundary-steward
tier: parliament
applies_to: [architecture, package-boundaries, dependencies]
---
```

## Review Order

For VTRACE work, use this default order:

1. Systems engineering.
2. Requirements traceability.
3. Package/interface boundary review.
4. Software assurance / code rigor.
5. Security/privacy and safety/risk where required.
6. V&V.
7. Source custody where required.
8. Stakeholder/editorial review for public or adoption-facing changes.

## Assisted Level

A repo reaches ROLES `Assisted` level when it has local skills or commands that
know how to select and run relevant roles. VTRACE recommends adding a local
`roles-review` skill once the repo has more than three active role files.
