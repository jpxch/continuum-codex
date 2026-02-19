# Language Roles

This file maps each language/domain to its responsibility in this repository.

## Active Now

`Rust` (`canon/`):
1. Canonical models for Bleach pilot entities.
2. Invariant validation and cross-record checks.
3. Lore-to-artifact compilation.

`Go` (`gateway/`):
1. Read-only API gateway over generated artifact data.
2. Transport and endpoint boundary.

`Java/Kotlin JVM` (`verification/`):
1. Independent verification of artifact consistency and contracts.
2. Secondary line of defense against schema/contract drift.

`Ruby` (`tooling/`):
1. Lore authoring helpers and DSL workflow.
2. Content creation ergonomics for structured YAML.

`YAML/JSON` (`lore/`, `artifacts/`):
1. Canon input substrate (`lore/`).
2. Generated output substrate (`artifacts/`).

## Scaffolded / Planned

`Python` (`analysis/`):
1. Interpretation, batch analysis, and research workflows over canon snapshots.

`C++` (`simulation/`):
1. High-performance simulation and scenario modeling.

`PHP` (`publishing/`):
1. Read-only publishing surface for external consumers.

`TypeScript` (`ui/`):
1. Interactive presentation and exploration UI.

## Role Contracts

1. Truth ownership remains in Rust.
2. Consumers are read-only against generated artifacts unless explicitly writing through canonical Rust workflows.
3. Any role expansion must update this file and relevant boundary docs in the same change.
