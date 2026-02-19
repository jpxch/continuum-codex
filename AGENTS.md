# AGENTS.md

This repository is a polyglot monorepo containing multiple language domains:

- `canon/` – Rust core library and binaries
- `gateway/` – Go service
- `verification/` – Kotlin/JVM (Gradle)
- `analysis/` – Python tooling
- `tooling/` – Ruby CLI utilities
- `simulation/` – C++ (CMake)
- `publishing/` – PHP (Composer)
- `lore/`, `artifacts/`, `docs/` – schemas, generated data, and documentation

Agentic coding agents should respect per‑language conventions and avoid cross‑cutting changes unless required.

---

## Build, Lint, and Test Commands

Always run commands from the repository root unless otherwise noted.

### Rust (`canon/`)

- Build: `cargo build --manifest-path canon/Cargo.toml`
- Release build: `cargo build --release --manifest-path canon/Cargo.toml`
- Test (all): `cargo test --manifest-path canon/Cargo.toml`
- Test (single): `cargo test --manifest-path canon/Cargo.toml <test_name>`
- Lint: `cargo clippy --all-targets --all-features --manifest-path canon/Cargo.toml`
- Format: `cargo fmt --manifest-path canon/Cargo.toml`
- Run binary: `cargo run --manifest-path canon/Cargo.toml --bin compile_bleach_zanpakuto`

Notes:
- Prefer fixing Clippy warnings rather than suppressing them.
- Do not commit `target/` artifacts.

### Go (`gateway/`)

- Build: `go build ./...` (run inside `gateway/`)
- Test (all): `go test ./...`
- Test (single package): `go test ./path/to/package`
- Test (single test): `go test -run TestName ./path/to/package`
- Format: `go fmt ./...`
- Vet: `go vet ./...`

Use Go modules (`go.mod`). Do not vendor dependencies unless explicitly required.

### Kotlin / JVM (`verification/`)

- Build: `./gradlew build`
- Test (all): `./gradlew test`
- Test (single class): `./gradlew test --tests "com.example.MyTest"`
- Test (single method): `./gradlew test --tests "com.example.MyTest.myMethod"`
- Clean: `./gradlew clean`

Use the Gradle wrapper; do not rely on a system Gradle installation.

### Python (`analysis/`)

- Install (editable): `pip install -e .` (inside `analysis/`)
- Test (all): `pytest`
- Test (single file): `pytest path/to/test_file.py`
- Test (single test): `pytest path/to/test_file.py::test_name`
- Lint (if configured): `ruff check .` or `flake8` (check `pyproject.toml`)

Use virtual environments; do not install globally.

### Ruby (`tooling/`)

- Install deps: `bundle install`
- Run CLI: `bundle exec ruby bin/codex`
- Test (if present): `bundle exec rspec`

Respect `Gemfile` versions.

### C++ (`simulation/`)

- Configure: `cmake -S simulation -B build/simulation`
- Build: `cmake --build build/simulation`
- Test (if enabled): `ctest --test-dir build/simulation`

Do not commit build directories.

### PHP (`publishing/`)

- Install deps: `composer install`
- Autoload dump: `composer dump-autoload`
- Test (if configured): `vendor/bin/phpunit`

Do not modify `vendor/` directly.

---

## Code Style and Conventions

Follow language‑idiomatic standards. When in doubt, prefer consistency with existing code.

### General Principles

- Keep changes minimal and scoped.
- Avoid drive‑by refactors.
- Do not introduce new frameworks without strong justification.
- Prefer explicitness over cleverness.
- Add tests for new behavior or bug fixes.

---

## Rust Guidelines (`canon/`)

- Edition: As specified in `Cargo.toml`.
- Formatting: Enforced by `rustfmt`.
- Imports: Group `std`, external crates, then internal modules.
- Error handling: Prefer `Result<T, E>` with domain‑specific error enums.
- Avoid `unwrap()` in library code; use `?` and propagate errors.
- Naming:
  - Types/traits: `PascalCase`
  - Functions/modules: `snake_case`
  - Constants: `SCREAMING_SNAKE_CASE`
- Derive common traits (`Debug`, `Clone`, `Serialize`, etc.) where appropriate.
- Keep modules cohesive; avoid overly large files.

---

## Go Guidelines (`gateway/`)

- Format with `go fmt`; do not manually align.
- Follow standard Go layout and package naming (short, lowercase).
- Errors:
  - Return `error` as last value.
  - Wrap with `%w` when adding context.
  - Avoid panics in request paths.
- Interfaces should be small and consumer‑defined.
- Use context propagation (`context.Context`) for request‑scoped operations.

---

## Kotlin / JVM Guidelines (`verification/`)

- Follow Kotlin style guide (official JetBrains conventions).
- Prefer immutable data (`val`) over `var`.
- Use data classes for value objects.
- Nullability must be explicit; avoid platform types.
- Tests should be deterministic and not rely on wall‑clock time.

---

## Python Guidelines (`analysis/`)

- Follow PEP 8.
- Use type hints for public APIs.
- Prefer `dataclasses` or `pydantic` (if present) for structured data.
- Avoid dynamic monkey‑patching.
- Raise specific exceptions; avoid bare `except:`.

---

## Ruby Guidelines (`tooling/`)

- Follow community Ruby style.
- Use frozen string literals if already adopted.
- Prefer small, composable classes over large scripts.

---

## C++ Guidelines (`simulation/`)

- Follow modern C++ (C++17 or later if configured).
- Prefer RAII and smart pointers over raw ownership.
- Minimize header dependencies; use forward declarations.

---

## PHP Guidelines (`publishing/`)

- Follow PSR‑12.
- Use Composer autoloading; avoid manual `require` chains.
- Keep business logic out of global scope.

---

## Schemas and Data (`lore/`, `artifacts/`)

- YAML schemas define canonical structures.
- Generated JSON in `artifacts/` should not be edited manually.
- If regeneration is required, use the appropriate Rust binary in `canon/`.

---

## Documentation

- Architecture decisions belong in `docs/`.
- Keep README high‑level; avoid duplicating detailed docs.
- Update docs alongside behavior changes.

---

## Agent Operating Rules

- Never commit build artifacts or dependency directories.
- Run language‑specific formatters before finalizing changes.
- Prefer adding or adjusting tests over weakening them.
- If multiple subsystems are affected, validate each independently.
- Do not rewrite history or force‑push unless explicitly instructed.

This file is authoritative guidance for automated coding agents operating in this repository.
