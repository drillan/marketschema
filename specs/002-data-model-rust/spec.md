# Rust Implementation Specification: Data Model

**Feature Branch**: `002-data-model-rust`
**Parent Spec**: [002-data-model](../002-data-model/spec.md)
**Status**: Active

## Overview

JSON Schema から Rust struct を生成するための仕様。

## Code Generation Tool

### cargo-typify

- **Tool**: [typify](https://github.com/oxidecomputer/typify)
- **Install**: `cargo install cargo-typify`

## Limitations

### External $ref Resolution

typify has limitations on resolving external `$ref`. Schemas must be bundled beforehand.

### Draft 2020-12 Support

Due to lack of explicit Draft 2020-12 support, it's recommended to define both `$defs` and `definitions` for compatibility.

### unevaluatedProperties Support

typify does not support JSON Schema Draft 2020-12 `unevaluatedProperties`.
Even when `unevaluatedProperties: false` is specified in the schema,
`#[serde(deny_unknown_fields)]` is not added to generated Rust code.

This issue is tracked in issue #39.

Current behavior:
- Unknown fields are silently ignored during deserialization
- Strict FR-010 compliance is not guaranteed in Rust implementation

### Limited Support

The following JSON Schema features have limited support:
- `anyOf`
- `if/then/else`

## Schema Bundling

### Necessity

typify requires self-contained schemas. Use `json-refs` tool to resolve and inline all `$ref`.

### Bundling Tool

```bash
# Install json-refs
npm install json-refs

# Execute bundling
npx json-refs resolve schema.json > bundled-schema.json
```

## Auto-generated serde Attributes

| Attribute | Condition |
|-----------|-----------|
| `#[derive(Serialize, Deserialize, Debug, Clone)]` | All types |
| `#[serde(default)]` | Properties not in `required` |
| `#[serde(deny_unknown_fields)]` | Requires manual addition (see issue #39) |

## Type Checking

### cargo check

Verify generated code compiles without errors.

```bash
cargo check
```

### Expected Results

- Compile errors: 0
- Warnings: Allowed (but reducing is recommended)

## Notes

### regress Crate

The `regress` crate may be required for schema pattern validation.

```toml
# Cargo.toml
[dependencies]
regress = "0.10"
```

### Symbol Type Duplication

When multiple schemas define `Symbol` type, type names may conflict after bundling. Use renaming or type aliases as needed.

## Recommended Workflow

1. Bundle schemas
2. Generate Rust code
3. Verify compilation with cargo check
4. Manual adjustments as needed

## Execution

See [docs/code-generation.md](../../docs/code-generation.md) for actual command execution.
