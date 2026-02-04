# Code Generation Guide

This document explains how to generate code from the JSON Schema definitions.

> **Note**: 言語固有の仕様・制約については各言語の spec を参照してください。本ドキュメントは実行手順に焦点を当てています。

## Python (pydantic v2)

仕様詳細: [specs/002-data-model-python/spec.md](https://github.com/drillan/marketschema/tree/main/specs/002-data-model-python/spec.md)

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
  --input schemas/ \
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
  --output python/src/marketschema/models/
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

仕様詳細: [specs/002-data-model-rust/spec.md](https://github.com/drillan/marketschema/tree/main/specs/002-data-model-rust/spec.md)

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
npx json-refs resolve schemas/quote.json > bundled_quote.json
```

2. Generate Rust code:

```bash
cargo typify bundled_quote.json --output src/types/quote.rs
```

### Why Bundling?

typify requires schemas to be self-contained. The `json-refs` tool resolves all `$ref` references and inlines them into a single file.

### unevaluatedProperties Conversion

typify does not support JSON Schema Draft 2020-12's `unevaluatedProperties` keyword ([typify#579](https://github.com/oxidecomputer/typify/issues/579)). To ensure `#[serde(deny_unknown_fields)]` is generated, the bundling script converts `unevaluatedProperties` to `additionalProperties`:

```bash
npx json-refs resolve "$schema" | \
    jq 'walk(if type == "object" and has("unevaluatedProperties")
        then .additionalProperties = .unevaluatedProperties | del(.unevaluatedProperties)
        else . end)' > "$OUTPUT"
```

> **Note**: The `walk` function is a built-in since jq 1.6.

**How it works**:

1. `json-refs resolve` resolves all `$ref` references
2. `jq walk(...)` recursively traverses the JSON tree
3. For each object with `unevaluatedProperties`, copies the value to `additionalProperties` and removes the original

After bundling, the schemas are semantically equivalent because all `$ref` references have been inlined.

See [ADR-001](adr/codegen/001-unevaluated-properties-workaround.md) for the full decision record.

### Generated Code Features

- `#[derive(Serialize, Deserialize, Clone, Debug)]` on all types
- Builder pattern for struct construction
- String newtypes for validated fields (e.g., Symbol, Currency)
- Optional fields use `Option<T>`

### Known Limitations

typify has limited support for some JSON Schema Draft 2020-12 features:

| Feature | Support | Workaround |
|---------|---------|------------|
| `unevaluatedProperties` | Not supported | Convert to `additionalProperties` during bundling (see above) |
| `anyOf` | Limited | May generate enum variants; complex unions may fail |
| `if/then/else` | Not supported | Use `oneOf` or `anyOf` instead |
| `$dynamicRef` | Not supported | Use static `$ref` |

For the latest status, see [typify#579: The Big Plan for 2020-12 support](https://github.com/oxidecomputer/typify/issues/579).

## Validation

### Validate Schemas with ajv-cli

```bash
# Validate a single schema
npx ajv validate \
  --spec=draft2020 \
  -s schemas/quote.json \
  -r schemas/definitions.json \
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
