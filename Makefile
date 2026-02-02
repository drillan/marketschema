.PHONY: all install lint format typecheck test test-cov validate-schemas generate-models generate-rust clean help

# Default target
all: lint typecheck test

# Install dependencies
install:
	uv sync --group dev

# Lint code
lint:
	uv run ruff check src tests

# Format code
format:
	uv run ruff format src tests
	uv run ruff check --fix src tests

# Type check
typecheck:
	uv run mypy src/marketschema

# Run all tests
test:
	uv run pytest tests -v

# Run tests with coverage
test-cov:
	uv run pytest tests -v --cov=src/marketschema --cov-report=term-missing --cov-report=html

# Validate JSON schemas
validate-schemas:
	@echo "Validating JSON Schemas..."
	@cd src/marketschema/schemas && for f in *.json; do \
		echo "  Checking $$f..."; \
		npx ajv compile --spec=draft2020 -s "$$f" -r definitions.json || exit 1; \
	done
	@echo "All schemas valid!"

# Generate Python pydantic models from JSON Schema
generate-models:
	./scripts/generate_models.sh

# Bundle schemas and generate Rust structs
generate-rust: bundle-schemas
	./scripts/generate_rust.sh

# Bundle schemas for Rust code generation
bundle-schemas:
	./scripts/bundle_schemas.sh

# Check Rust crate
rust-check:
	cd rust && cargo check

# Run Rust tests
rust-test:
	cd rust && cargo test

# Clean generated files
clean:
	rm -rf rust/bundled/*.json
	rm -rf src/marketschema/models/*.py
	rm -rf rust/src/types/*.rs
	rm -rf .pytest_cache .mypy_cache .ruff_cache
	rm -rf htmlcov .coverage
	find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true

# Full CI check
ci: lint typecheck test rust-check rust-test

# Help
help:
	@echo "Available targets:"
	@echo "  all             - Run lint, typecheck, and tests"
	@echo "  install         - Install dependencies with uv"
	@echo "  lint            - Run ruff linter"
	@echo "  format          - Format code with ruff"
	@echo "  typecheck       - Run mypy type checker"
	@echo "  test            - Run pytest tests"
	@echo "  test-cov        - Run tests with coverage report"
	@echo "  validate-schemas - Validate JSON Schema files"
	@echo "  generate-models - Generate Python pydantic models"
	@echo "  generate-rust   - Generate Rust structs"
	@echo "  bundle-schemas  - Bundle schemas for Rust"
	@echo "  rust-check      - Check Rust crate compiles"
	@echo "  rust-test       - Run Rust tests"
	@echo "  clean           - Remove generated files"
	@echo "  ci              - Full CI check"
	@echo "  help            - Show this help"
