
.PHONY: help build test clean format lint check run install dev audit cargo-build cargo-clean

help:
	@echo "Available targets:"
	@echo "  make build         - Format, lint, and build the project (Spin + Cargo)"
	@echo "  make cargo-build   - Run cargo build for all workspace members"
	@echo "  make cargo-clean   - Run cargo clean for all workspace members"
	@echo "  make test          - Run all tests"
	@echo "  make rebuild       - Clean build artifacts (Spin)"
	@echo "  make check         - Run cargo check and format check"
	@echo "  make run           - Run the application"
	@echo "  make audit         - Run security audit"
	@echo "  make all           - Check, test, and build"
	@echo "  make ci            - Run checks and tests for CI"

build:
	cargo fmt --all
	cargo clippy --all -- -D warnings
	cargo build --workspace
	spin build

cargo-build:
	cargo build --workspace

cargo-clean:
	cargo clean --workspace

run:
	spin up

test:
	spin test

check:
	cargo check --workspace
	cargo fmt --all --check

audit:
	cargo audit

all: check test build

ci: check test