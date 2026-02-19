# Continuum Codex Roadmap

As of February 19, 2026.

## Current Phase

Architecture and foundation. The Bleach pilot has one working pipeline:

1. Author lore in `lore/bleach/zanpakuto/*.yaml`.
2. Validate and compile in Rust (`canon/`) into `artifacts/bleach/zanpakuto.json`.
3. Serve artifact through Go (`gateway/`).

Current health snapshot:

1. Rust tests pass.
2. Go tests pass.
3. JVM verification build is currently blocked by Gradle DSL configuration in `verification/build.gradle.kts`.

## Milestone 1: Reliable Canon Contract Pipeline

Goal: make Rust, Go, and JVM verification paths all green and reproducible.

Deliverables:

1. Fix `verification/build.gradle.kts` test task configuration.
2. Add JVM tests for artifact contract and duplicate invariants.
3. Ensure one standard check flow from repo root (Makefile target or script).
4. Keep generated artifact stable and deterministic.

Exit criteria:

1. `cargo test --manifest-path canon/Cargo.toml` passes.
2. `cd gateway && go test ./...` passes.
3. `cd verification && ./gradlew test` passes.
4. Rebuild of `artifacts/bleach/zanpakuto.json` is deterministic.

## Milestone 2: Authoring and Schema Hardening

Goal: tighten lore authoring quality and validation boundaries.

Deliverables:

1. Expand Ruby authoring tooling under `tooling/` for safer YAML generation.
2. Add schema-level checks for required multilingual fields and enum constraints.
3. Add regression tests for edge cases (missing romaji, invalid IDs, duplicate names).
4. Document canonical authoring workflow end-to-end.

Exit criteria:

1. New lore entries can be created via tooling, not manual YAML only.
2. Invalid lore is rejected with clear error messages.
3. Contract tests protect artifact structure across Rust and Go consumers.

## Milestone 3: Read-Only Publication Slice

Goal: provide a stable read-only presentation path for canon data.

Deliverables:

1. Define API response contract owned by `gateway/`.
2. Implement minimal read-only publish path in `publishing/` or `ui/`.
3. Version the artifact/API contract and add compatibility checks.

Exit criteria:

1. Consumers can query canon data without reading raw artifacts directly.
2. Contract version changes are explicit and tested.

## Milestone 4: Advanced Engines (After M1-M3)

Goal: activate currently scaffolded domains without breaking boundaries.

Planned tracks:

1. `analysis/` for batch interpretation and embeddings.
2. `simulation/` for high-performance scenario simulation.
3. Extended verification rules in `verification/`.

Rule: no advanced engine work should bypass canonical truth ownership in Rust.

## Suggested Next Task

Fix `verification/build.gradle.kts` and get `./gradlew test` green first.  
This is the highest-leverage blocker to making the polyglot contract pipeline reliable.
