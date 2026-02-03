# Python Implementation Specification: Data Model

**Feature Branch**: `002-data-model-python`
**Parent Spec**: [002-data-model](../002-data-model/spec.md)
**Status**: Active

## Overview

JSON Schema から Python pydantic v2 モデルを生成するための仕様。

## Code Generation Tool

### datamodel-code-generator

- **Tool**: [datamodel-code-generator](https://github.com/koxudaxi/datamodel-code-generator)
- **Version**: Latest recommended

### Required Options

| Option | Value | Description |
|--------|-------|-------------|
| `--output-model-type` | `pydantic_v2.BaseModel` | pydantic v2 model output |
| `--use-annotated` | - | Use `typing.Annotated` |
| `--field-constraints` | - | Include min/max constraints |
| `--use-standard-collections` | - | Use `list`, `dict` (not `List`, `Dict`) |
| `--use-union-operator` | - | Use `X | Y` syntax |
| `--snake-case-field` | - | Convert field names to snake_case |
| `--use-schema-description` | - | Use schema description as docstring |
| `--use-field-description` | - | Use field description in Field |
| `--reuse-model` | - | Reuse identical models |
| `--disable-timestamp` | - | Disable generation timestamp comment |

### Option Configuration

Configuration is managed in `[tool.datamodel-codegen]` section of `pyproject.toml`.
See [docs/code-generation.md](../../docs/code-generation.md) for details.

## pydantic v2 Configuration

### unevaluatedProperties Mapping

JSON Schema `unevaluatedProperties` maps to pydantic `model_config`:

| JSON Schema | pydantic v2 |
|------------|-------------|
| `unevaluatedProperties: false` | `model_config = ConfigDict(extra='forbid')` |
| `unevaluatedProperties: true` | `model_config = ConfigDict(extra='allow')` |

### Nullable Fields

```python
# JSON Schema: {"type": ["string", "null"]}
field: str | None = None
```

## Type Checking

### mypy

Generated code must pass mypy type checking.

```bash
uv run mypy src/marketschema/models/
```

### Expected Results

- Errors: 0
- Warnings: Allowed (but reducing warnings in `--strict` mode is recommended)

## Notes

### $ref Resolution

Specify `$ref` as relative paths. Using absolute URLs requires the httpx module.

```json
// OK
{"$ref": "definitions.json#/$defs/Symbol"}

// NG (requires httpx)
{"$ref": "https://example.com/schemas/definitions.json#/$defs/Symbol"}
```

### Auto-formatting

Recommended to format with ruff after generation:

```bash
uv run ruff format src/marketschema/models/
uv run ruff check --fix src/marketschema/models/
```

## Execution

See [docs/code-generation.md](../../docs/code-generation.md) for actual command execution.
