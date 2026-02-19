UNIVERSE ?= bleach

build:
	cd canon && cargo run --bin compile_bleach_zanpakuto

verify-rust:
	cd canon && cargo test

verify-go:
	cd gateway && go test ./...

verify-contract:
	@echo "Rebuilding artifact..."
	cd canon && cargo run --bin compile_bleach_zanpakuto
	@git diff --exit-code artifacts/$(UNIVERSE)/zanpakuto.json || \
	  (echo "Artifact out of sync. Regenerate before committing." && exit 1)

all: build verify-rust verify-go verify-contract