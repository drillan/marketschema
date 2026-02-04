#!/bin/bash
# Generate pydantic models from JSON Schema files
# Usage: ./scripts/generate_models.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
SCHEMAS_DIR="$PROJECT_ROOT/schemas"
MODELS_DIR="$PROJECT_ROOT/python/src/marketschema/models"
PYTHON_DIR="$PROJECT_ROOT/python"

echo "Generating pydantic models from JSON Schema..."
echo "  Schemas: $SCHEMAS_DIR"
echo "  Output:  $MODELS_DIR"

# Ensure output directory exists
mkdir -p "$MODELS_DIR"

# Run datamodel-codegen with options from pyproject.toml plus additional flags
cd "$PYTHON_DIR"
uv run datamodel-codegen \
  --input "$SCHEMAS_DIR" \
  --input-file-type jsonschema \
  --output-model-type pydantic_v2.BaseModel \
  --target-python-version 3.13 \
  --use-annotated \
  --field-constraints \
  --use-standard-collections \
  --use-union-operator \
  --snake-case-field \
  --use-schema-description \
  --use-field-description \
  --reuse-model \
  --disable-timestamp \
  --output "$MODELS_DIR"

echo "Done! Models generated in $MODELS_DIR"

# Format generated code
echo "Formatting generated code..."
uv run ruff format "$MODELS_DIR" --quiet

echo "Running ruff check with auto-fix..."
if ! uv run ruff check "$MODELS_DIR" --fix --quiet; then
    echo "WARNING: ruff check found issues that could not be auto-fixed" >&2
    echo "Please review the generated code in $MODELS_DIR" >&2
fi

echo "All done!"
