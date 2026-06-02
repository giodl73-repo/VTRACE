# Run The CLI

From the VTRACE repo:

```powershell
cargo run -- validate .
cargo run -- status .
cargo run -- plan .
cargo run -- comms plan .
```

For a target repo:

```powershell
vtrace validate C:\path\to\repo
vtrace status C:\path\to\repo
vtrace comms plan C:\path\to\repo
```

Work-package commands require a `WP-*` row in `docs/vtrace/WORK_PACKAGES.md`:

```powershell
vtrace work start WP-001 C:\path\to\repo
vtrace work check WP-001 C:\path\to\repo
vtrace work close WP-001 C:\path\to\repo
```

Expected result: deterministic commands print findings or procedural packets.
Live provider, GitHub, and pulse side effects require explicit live flags.
