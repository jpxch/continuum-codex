# Continuum Codex Architecture

As of February 19, 2026.

## System Intent

Continuum Codex is a polyglot canon engine for fictional universes, with Bleach as the pilot domain.

The architecture separates responsibilities by language so each subsystem has one clear ownership boundary.

## Canonical Flow

Primary runtime data path:

1. Lore authoring input in `lore/bleach/zanpakuto/*.yaml`.
2. Canon validation and compilation in `canon/` (Rust).
3. Generated artifact output in `artifacts/bleach/zanpakuto.json`.
4. Downstream consumption by `gateway/` (Go HTTP read API), `verification/` (JVM contract/invariant checks), and future consumers (`analysis/`, `publishing/`, `ui`, `simulation/`).

## Source Of Truth

Rust in `canon/` is authoritative for domain invariants and canonical validity.

All non-Rust layers may consume and verify canon outputs, but they must not redefine truth semantics independently.

## Current Implementation State

Implemented end-to-end:

1. Rust domain models and invariants (`canon/src/zanpakuto.rs`, `canon/src/registry.rs`).
2. Artifact compiler binary (`canon/src/bin/compile_bleach_zanpakuto.rs`).
3. Go read-only endpoint pipeline (`gateway/cmd/gateway/main.go`).
4. Contract tests in Rust and Go.

Partially implemented:

1. JVM verification logic exists (`verification/src/main/java/codex/Verifier.java`).
2. Gradle test configuration currently needs correction in `verification/build.gradle.kts` before JVM tests are reliable.

Scaffolded:

1. `analysis/`, `simulation/`, `publishing/`, and `ui` are present but not yet integrated into the core delivery path.

## Boundary Contracts

`lore/`:
1. Human-authored canonical input.
2. Should be edited directly or through tooling.

`canon/`:
1. Validates lore and enforces invariants.
2. Produces deterministic artifact snapshots.

`artifacts/`:
1. Generated outputs only.
2. Must not be manually edited.

`gateway/`, `verification/`, `analysis/`, `simulation/`, `publishing/`, `ui`:
1. Consumers of artifacts.
2. Must not mutate canonical truth outside Rust-managed workflows.

## Technology Role Matrix

| Language/Format | Role | Owns | Must Not Do |
| --- | --- | --- | --- |
| Rust | Canonical Truth Engine | Models, invariants, compile pipeline | UI, publishing, speculative analysis logic |
| Go | Interaction Gateway | HTTP/API access to artifacts | Canon mutation, truth redefinition |
| Java/JVM | Verification Engine | Independent contract/invariant validation | Canon ownership, UI concerns |
| Ruby | Authoring Tooling | Lore DSL and author workflows | Runtime truth enforcement |
| Python | Analysis Lab (planned) | Interpretive and batch analysis | Canon mutation/enforcement |
| C++ | Simulation Engine (planned) | High-performance simulation | Canon ownership/orchestration |
| PHP | Publishing Layer (planned) | Read-only public codex views | Canon validation/enforcement |
| TypeScript | Interactive UI (planned) | Exploration and presentation | Canon ownership/enforcement |
| YAML/JSON | Data Substrate | Lore inputs and generated artifacts | Validation logic |

## Operational Validation Commands

Run from repository root:

1. `cargo test --manifest-path canon/Cargo.toml`
2. `cd gateway && go test ./...`
3. `cd verification && ./gradlew test`
4. Optional combined flow: `make all`

## Related Documents

1. `docs/ROADMAP.md` for milestone sequencing.
2. `docs/BOUNDARY_RULES.md` for hard subsystem constraints.
3. `docs/LANGUAGE_ROLES.md` for per-language ownership summary.
