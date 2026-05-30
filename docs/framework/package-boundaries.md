# Package And Language Boundaries

VTRACE implementation work must respect the repo's package, crate, module, and
language boundaries. A work package should not blur ownership just because it is
convenient for a patch.

## Boundary Unit

Use the boundary unit that matches the repo:

| Ecosystem | Typical Boundary Units |
|---|---|
| Rust | workspace, crate, feature, module, public trait/API, binary, build script |
| Python | package, module, CLI entrypoint, plugin, generated artifact |
| TypeScript/JavaScript | package, app, library, route, component, API handler, schema |
| .NET | solution, project, assembly, namespace, service, package |
| Docs/standards | template family, skill, schema, example, source registry |
| Multi-language | language adapter, FFI boundary, generated contract, CLI/API seam |

## Boundary Record

Before non-trivial implementation, record:

- package/crate/module name,
- language/toolchain,
- owner or role,
- public interfaces,
- allowed dependencies,
- forbidden dependencies,
- generated artifacts,
- validation commands,
- downstream consumers,
- change-control triggers.

## Dependency Direction

State dependency direction explicitly:

- shared primitive -> consumer,
- schema/template -> generated artifact,
- CLI/API -> downstream caller,
- source registry -> encoded concept,
- test fixture -> verifier.

Avoid product-to-product dependencies unless the reusable layer is extracted or
the dependency is explicitly approved.

## Work Package Boundary Rule

Every `WP-*` should name one of:

- the exact package/crate/module it may change,
- the interface it may change,
- the generated artifact it may regenerate,
- `discovery`, with allowed files and stop condition.

If a work package needs to cross a boundary, it should name the integration
point and require `INTEGRATION_PLAN.md`.

## Language Tailoring

Code rigor and validation commands must match the language boundary. Examples:

- Rust crate: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test`, feature-specific tests, unsafe inventory if relevant.
- Python package: `python -m py_compile`, unit tests, type checks if adopted,
  import/CLI smoke checks.
- Docs-only repo: `git diff --check`, link/source-custody checks, example
  consistency review.

## Boundary Review Questions

- Is this behavior owned by the right package or crate?
- Is the public interface controlled by `INTERFACES.md`?
- Are generated artifacts separated from source of truth?
- Does the dependency direction match the architecture?
- Does the work package touch more boundaries than its objective justifies?
- Are validation commands scoped to the package and the whole repo when needed?
