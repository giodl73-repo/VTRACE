# Complete tracked source snapshot and original grant scopes

The source ZIP contains every tracked Git blob at its recorded commit. This
includes the native library/CLI, all original tests and added reader tests,
the browser companion, locks, UI/build/qualification code, templates, examples,
self-trace records, role files, source registries and ordinary documentation.
The native CLI tests need those repo-local templates, docs and role records;
shipping only the small library would not reproduce the complete native suite.

`source-files.json` records each included Git blob, mode and SHA256. The ZIP
includes the same manifest in `.source-record.json` for extraction/replay outside
Git. Ignored/local files, downloaded source caches, node_modules, Git history,
compiled targets and QA caches are excluded. No NASA handbook, industry standard
or other external source is fetched while packaging. Existing pointer-first
registries and locally authored derived guidance remain in their original scope.

VTRACE's LICENSE retains MIT software and ordinary software documentation,
CC BY-NC original non-software content and separate third-party grants. This
full source snapshot does not give every file blanket MIT terms, change prior
grants, confer NASA endorsement or turn locally authored guidance into an
official standard. Read the original LICENSE and any file-specific notices.

Dependencies are pinned and fetched by Cargo/npm, not vendored in this ZIP.
Original dependency notices and Rust runtime notices accompany the compiled
site. Test-only npm tools are not browser runtime assets. The licenses page and
build record also link the complete Git archive at the same recorded commit.
