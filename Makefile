.PHONY: build
build:
	cargo build

.PHONY: run
run:
	cargo run

.PHONY: clippy
clippy:
	cargo clippy -- -D warnings

.PHONY: format-check
format-check:
	cargo fmt --all --check

.PHONY: test
test:
	cargo test --no-fail-fast

.PHONY: fix
fix:
	cargo fix --allow-staged --allow-dirty --all-targets
