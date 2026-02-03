# Code Generation Guide

This document explains how to generate code from the JSON Schema definitions.

> **Note**: 言語固有の仕様・制約については [specs/002-data-model/lang/](../specs/002-data-model/lang/) を参照してください。本ドキュメントは実行手順に焦点を当てています。

## Python (pydantic v2)

仕様詳細: [specs/002-data-model/lang/python.md](../specs/002-data-model/lang/python.md)

### Prerequisites

```bash
uv sync --group dev
```

### Generation Command

```bash
./scripts/generate_models.sh
# or
make generate-models
```

### Manual Generation

```bash
datamodel-codegen \
  --input src/marketschema/schemas/ \
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
  --output src/marketschema/models/
```

### Configuration

Options are defined in `pyproject.toml` under `[tool.datamodel-codegen]`.

### Key Options

| Option | Description |
|--------|-------------|
| `--output-model-type pydantic_v2.BaseModel` | Generate pydantic v2 models |
| `--use-annotated` | Use `typing.Annotated` for field constraints |
| `--field-constraints` | Include min/max constraints from schema |
| `--reuse-model` | Reuse identical models across files |
| `--snake-case-field` | Convert field names to snake_case |

## Rust (serde)

仕様詳細: [specs/002-data-model/lang/rust.md](../specs/002-data-model/lang/rust.md)

### Prerequisites

```bash
# Install typify
cargo install cargo-typify

# Install json-refs for schema bundling
npm install
```

### Generation Command

```bash
./scripts/bundle_schemas.sh   # Bundle schemas first
./scripts/generate_rust.sh    # Generate Rust code
# or
make generate-rust
```

### Manual Generation

1. Bundle schemas to resolve `$ref`:

```bash
npx json-refs resolve src/marketschema/schemas/quote.json > bundled_quote.json
```

2. Generate Rust code:

```bash
cargo typify bundled_quote.json --output src/types/quote.rs
```

### Why Bundling?

typify requires schemas to be self-contained. The `json-refs` tool resolves all `$ref` references and inlines them into a single file.

### Generated Code Features

- `#[derive(Serialize, Deserialize, Clone, Debug)]` on all types
- Builder pattern for struct construction
- String newtypes for validated fields (e.g., Symbol, Currency)
- Optional fields use `Option<T>`

## Validation

### Validate Schemas with ajv-cli

```bash
# Validate a single schema
npx ajv validate \
  --spec=draft2020 \
  -s src/marketschema/schemas/quote.json \
  -r src/marketschema/schemas/definitions.json \
  -d sample_data.json

# Validate all schemas
make validate-schemas
```

## Troubleshooting

### Python: `httpx` Module Error

If datamodel-codegen tries to fetch remote URLs:

```
Exception: Please run `pip install 'datamodel-code-generator[http]`'
```

This means your schemas have absolute URL `$ref`. Use relative paths like `definitions.json#/$defs/Symbol` instead.

### Rust: `regress` Crate Missing

Add to `Cargo.toml`:

```toml
regress = "0.10"
```

This is needed for schema pattern validation.

### Schema Reference Errors

If validation tools can't resolve `$ref`:

1. Use relative paths: `definitions.json#/$defs/Symbol`
2. Run from the schemas directory
3. Ensure definitions.json is in the same directory
