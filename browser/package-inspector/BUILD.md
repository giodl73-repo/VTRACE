# Build and qualify the package inspector

Use Python 3.11+, Rust 1.95.0 with wasm32, wasm-bindgen-cli 0.2.127, Node 22+
and Git. Cargo/npm download the dependencies pinned in committed locks. No
TRACKER checkout, provider, account or service is required for local execution.

```powershell
rustup toolchain install 1.95.0 --profile minimal
rustup target add wasm32-unknown-unknown --toolchain 1.95.0
cargo +1.95.0 install wasm-bindgen-cli --version 0.2.127 --locked
cargo +1.95.0 fmt --check
cargo +1.95.0 clippy --all-targets --locked -- -D warnings
cargo +1.95.0 test --locked
cargo +1.95.0 run --locked -- .
cargo +1.95.0 run --locked -- examples/existing-repo-migration
cargo +1.95.0 test --manifest-path browser/package-inspector/Cargo.toml --all-targets --locked
cargo +1.95.0 clippy --manifest-path browser/package-inspector/Cargo.toml --all-targets --locked -- -D warnings
cargo +1.95.0 clippy --manifest-path browser/package-inspector/Cargo.toml --lib --target wasm32-unknown-unknown --locked -- -D warnings
python browser/package-inspector/build-prototype.py --bindgen <path-to-wasm-bindgen>
cd browser/package-inspector
npm ci
npx playwright install chromium
npm run test:pages
```

The test runner serves `target/package-prototype` under `/VTRACE/`. Do not open
the HTML with a file URL. Use `CARGO_TARGET_DIR` to reuse a chosen cache; all
first-party paths stay inside the repo or extracted source tree.

The recipe checks original native source/test/manifest/lock identity, builds
native and memory-bounded release WASM, repeats native outcomes and compares
admitted findings with the independent original CLI. Owned Git document fixtures
use canonical LF for checkout/archive replay; explicit CRLF/BOM/lone-CR cases
retain their original bytes. Native reports normalize only displayed path
separators. Browser tests execute the real module worker and check memory limits,
own-file edits, source navigation, byte custody, deadlines and stale-result fences.

From a clean Git checkout, `python browser/package-inspector/package-site.py`
packages exact committed source and original notices. Then run
`python browser/package-inspector/verify-release.py` for complete packaged
delivery, and `python browser/package-inspector/verify-source.py --bindgen
<path-to-wasm-bindgen>` to rebuild and replay delivered source outside Git.
Use the same commands on Linux; install Chromium system dependencies with
`npx playwright install --with-deps chromium` where needed.

Source replay uses the `vtrace-replay` Cargo profile inside the same target cache.
Native tests embed their source directory at compile time, so temporary-source
test executables must not replace the ordinary checkout's dev-profile tests.
Each extraction refreshes first-party Rust timestamps before compilation and
clears any inherited live-site URL before testing its own rebuilt browser files.

Original opened bytes, draft input, normalized editor text, accepted input and
accepted report remain distinct. Findings refer to accepted source. Source,
compiled-WASM, build and licensing routes work independently of engine startup.
No-findings describes only existing structural checks; see MODEL.md for native
heuristics, admission limits and unverified caller-assertion boundaries.
