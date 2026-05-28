---
name: vtrace-pulse
description: Execute the next VTRACE wave pulse with docs, skills, validation, and commit-ready updates.
allowed-tools:
  - Read
  - Write
  - Glob
  - Grep
  - Bash
---

# VTRACE Pulse

Use this skill for VTRACE development pulses.

## Workflow

1. Read `context/waves/PHASES.md`.
2. Read the active wave `WAVE.md`.
3. Read the target pulse under `pulses/`.
4. Implement the smallest complete slice.
5. Keep NASA-inspired guidance distinct from official NASA process.
6. Update docs and pulse status.
7. Run validation commands and `git diff --check`.
