# Boundary Rules

This document defines hard subsystem boundaries for Continuum Codex.

## Core Principle

Rust (`canon/`) is the source of truth for canon invariants and data validity.  
All other layers consume or check canon; they do not redefine canon logic.

## Allowed Data Flow

1. Authoring input: `tooling/` and `lore/`.
2. Canon validation + compilation: `canon/`.
3. Generated output: `artifacts/`.
4. Serving/consuming: `gateway/`, `verification/`, `publishing/`, `ui`, `analysis/`, `simulation/`.

Flow: `lore/*` -> `canon/*` -> `artifacts/*` -> downstream consumers.

## Must / Must Not By Layer

`canon/` (Rust):
1. Must own domain models and invariants.
2. Must validate cross-record consistency.
3. Must not include UI, serving, or heuristic analysis logic.

`gateway/` (Go):
1. Must expose read-only access to canon artifacts.
2. Must not mutate canon artifacts.
3. Must not encode business invariants that conflict with Rust.

`verification/` (JVM):
1. Must perform contract and consistency checks over produced artifacts.
2. Must not become the canonical source of truth.
3. Must not bypass published artifact contract.

`tooling/` (Ruby):
1. Must help author lore safely and consistently.
2. Must not bypass schema or invariant expectations.
3. Must not directly edit generated artifacts.

`analysis/` (Python):
1. Must treat artifacts as read-only inputs.
2. Must not mutate truth or enforcement rules.

`simulation/` (C++):
1. Must consume canon snapshots as inputs.
2. Must not write canonical truth back into lore/artifacts without Rust validation.

`publishing/` (PHP) and `ui/`:
1. Must remain read-only presentation layers.
2. Must not contain canon enforcement logic.

## Artifact Ownership

1. `artifacts/` is generated output.
2. Manual edits to generated JSON are prohibited.
3. Regeneration should happen through canonical build commands in `canon/`.

## Cross-Cutting Rules

1. Prefer explicit contracts over implicit coupling.
2. Add tests when changing behavior across subsystem boundaries.
3. Keep per-language changes scoped; avoid drive-by refactors.
4. If a boundary is unclear, document it in `docs/` before implementation expands.
