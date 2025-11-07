.PHONY: run test fmt clippy pre-commit help

# Default target
help:
	@echo "Available targets:"
	@echo "  run         - Build and run the project"
	@echo "  test        - Run all tests"
	@echo "  fmt         - Format code with rustfmt"
	@echo "  clippy      - Run clippy linter"
	@echo "  pre-commit  - Run all checks (fmt, clippy, test)"

# Build and run the project
run:
	cargo run

# Run all tests
test:
	cargo test

# Format code
fmt:
	cargo fmt

# Run clippy
clippy:
	cargo clippy -- -D warnings

# Pre-commit checks: format, lint, and test
pre-commit: fmt clippy test
	@echo "✓ All pre-commit checks passed!"
