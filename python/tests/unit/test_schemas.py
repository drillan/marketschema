"""Test JSON Schema validation using Python jsonschema library."""

import json
import re
from pathlib import Path
from typing import Any

import pytest
from jsonschema import Draft202012Validator  # type: ignore[import-untyped]
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT202012

REPO_ROOT = Path(__file__).parent.parent.parent.parent
SCHEMAS_DIR = REPO_ROOT / "schemas"
FIXTURES_DIR = Path(__file__).parent.parent / "fixtures"


def load_definitions() -> dict[str, Any]:
    """Load the definitions.json schema."""
    with open(SCHEMAS_DIR / "definitions.json", encoding="utf-8") as f:
        result: dict[str, Any] = json.load(f)
        return result


def bundle_schema(
    schema: dict[str, Any], definitions: dict[str, Any]
) -> dict[str, Any]:
    """Bundle a schema by inlining definitions.

    Replaces $ref: "definitions.json#/$defs/X" with the actual definition.
    """
    bundled = json.loads(json.dumps(schema))  # Deep copy

    # Add $defs from definitions.json to the schema
    if "$defs" not in bundled:
        bundled["$defs"] = {}
    bundled["$defs"].update(definitions.get("$defs", {}))

    # Replace all references to definitions.json with local $defs
    def replace_refs(obj: Any) -> Any:
        if isinstance(obj, dict):
            if "$ref" in obj:
                ref = obj["$ref"]
                # Match definitions.json#/$defs/X
                match = re.match(r"definitions\.json#/\$defs/(\w+)", ref)
                if match:
                    return {**obj, "$ref": f"#/$defs/{match.group(1)}"}
            return {k: replace_refs(v) for k, v in obj.items()}
        elif isinstance(obj, list):
            return [replace_refs(item) for item in obj]
        return obj

    result: dict[str, Any] = replace_refs(bundled)
    return result


def create_registry() -> Registry[Any]:
    """Create a JSON Schema registry with bundled schema files."""
    resources: list[tuple[str, Resource[Any]]] = []
    definitions = load_definitions()

    for schema_file in SCHEMAS_DIR.glob("*.json"):
        with open(schema_file, encoding="utf-8") as f:
            schema = json.load(f)

        # Bundle the schema to inline definitions
        if schema_file.name != "definitions.json":
            schema = bundle_schema(schema, definitions)

        resource = Resource.from_contents(schema, default_specification=DRAFT202012)

        # Register by $id if present
        if "$id" in schema:
            resources.append((schema["$id"], resource))

        # Also register by filename
        resources.append((schema_file.name, resource))

    return Registry().with_resources(resources)


def load_schema(schema_name: str) -> dict[str, Any]:
    """Load a JSON Schema file by name."""
    schema_path = SCHEMAS_DIR / schema_name
    with open(schema_path, encoding="utf-8") as f:
        result: dict[str, Any] = json.load(f)
        return result


def load_fixture(fixture_path: Path) -> dict[str, Any]:
    """Load a test fixture file."""
    with open(fixture_path, encoding="utf-8") as f:
        result: dict[str, Any] = json.load(f)
        return result


def validate_data(schema_name: str, data: dict[str, Any]) -> list[str]:
    """Validate data against a schema and return list of errors.

    Args:
        schema_name: Name of the schema file (e.g., "quote.json")
        data: Data to validate

    Returns:
        List of validation error messages (empty if valid)
    """
    schema = load_schema(schema_name)
    definitions = load_definitions()

    # Bundle the schema to inline definitions
    if schema_name != "definitions.json":
        schema = bundle_schema(schema, definitions)

    validator = Draft202012Validator(schema)

    errors: list[str] = []
    for error in validator.iter_errors(data):
        errors.append(str(error.message))
    return errors


def assert_valid(schema_name: str, data: dict[str, Any]) -> None:
    """Assert that data is valid against a schema."""
    errors = validate_data(schema_name, data)
    assert not errors, f"Validation errors: {errors}"


def assert_invalid(schema_name: str, data: dict[str, Any]) -> None:
    """Assert that data is invalid against a schema."""
    errors = validate_data(schema_name, data)
    assert errors, "Expected validation to fail but it passed"


class TestValidData:
    """Test that valid data passes validation."""

    def test_valid_quote(self) -> None:
        """Valid quote data should pass validation."""
        data = load_fixture(FIXTURES_DIR / "valid" / "quote.json")
        assert_valid("quote.json", data)

    def test_valid_ohlcv(self) -> None:
        """Valid OHLCV data should pass validation."""
        data = load_fixture(FIXTURES_DIR / "valid" / "ohlcv.json")
        assert_valid("ohlcv.json", data)

    def test_valid_trade(self) -> None:
        """Valid trade data should pass validation."""
        data = load_fixture(FIXTURES_DIR / "valid" / "trade.json")
        assert_valid("trade.json", data)

    def test_valid_orderbook(self) -> None:
        """Valid orderbook data should pass validation."""
        data = load_fixture(FIXTURES_DIR / "valid" / "orderbook.json")
        assert_valid("orderbook.json", data)

    def test_valid_instrument(self) -> None:
        """Valid instrument data should pass validation."""
        data = load_fixture(FIXTURES_DIR / "valid" / "instrument.json")
        assert_valid("instrument.json", data)

    def test_valid_derivative_info(self) -> None:
        """Valid derivative info data should pass validation."""
        data = load_fixture(FIXTURES_DIR / "valid" / "derivative_info.json")
        assert_valid("derivative_info.json", data)

    def test_valid_expiry_info(self) -> None:
        """Valid expiry info data should pass validation."""
        data = load_fixture(FIXTURES_DIR / "valid" / "expiry_info.json")
        assert_valid("expiry_info.json", data)

    def test_valid_option_info(self) -> None:
        """Valid option info data should pass validation."""
        data = load_fixture(FIXTURES_DIR / "valid" / "option_info.json")
        assert_valid("option_info.json", data)

    def test_valid_volume_info(self) -> None:
        """Valid volume info data should pass validation."""
        data = load_fixture(FIXTURES_DIR / "valid" / "volume_info.json")
        assert_valid("volume_info.json", data)


class TestInvalidData:
    """Test that invalid data fails validation."""

    def test_quote_missing_required_field(self) -> None:
        """Quote missing required field should fail validation."""
        data = load_fixture(FIXTURES_DIR / "invalid" / "quote_missing_symbol.json")
        assert_invalid("quote.json", data)

    def test_quote_extra_field_rejected(self) -> None:
        """Quote with extra field should fail validation (unevaluatedProperties)."""
        data = load_fixture(FIXTURES_DIR / "invalid" / "quote_extra_field.json")
        assert_invalid("quote.json", data)

    def test_trade_invalid_side_enum(self) -> None:
        """Trade with invalid side value should fail validation."""
        data = load_fixture(FIXTURES_DIR / "invalid" / "trade_invalid_side.json")
        assert_invalid("trade.json", data)

    def test_instrument_invalid_currency_format(self) -> None:
        """Instrument with invalid currency format should fail validation."""
        data = load_fixture(
            FIXTURES_DIR / "invalid" / "instrument_invalid_currency.json"
        )
        assert_invalid("instrument.json", data)

    def test_ohlcv_missing_volume(self) -> None:
        """OHLCV missing required volume field should fail validation."""
        data = load_fixture(FIXTURES_DIR / "invalid" / "ohlcv_missing_volume.json")
        assert_invalid("ohlcv.json", data)


class TestSchemaFilesExist:
    """Test that all schema files exist and are loadable."""

    @pytest.mark.parametrize(
        "schema_name",
        [
            "definitions.json",
            "quote.json",
            "ohlcv.json",
            "trade.json",
            "orderbook.json",
            "instrument.json",
            "derivative_info.json",
            "expiry_info.json",
            "option_info.json",
            "volume_info.json",
        ],
    )
    def test_schema_file_exists(self, schema_name: str) -> None:
        """Schema file should exist in schemas directory."""
        schema_path = SCHEMAS_DIR / schema_name
        assert schema_path.exists(), f"Schema file not found: {schema_path}"

    @pytest.mark.parametrize(
        "schema_name",
        [
            "definitions.json",
            "quote.json",
            "ohlcv.json",
            "trade.json",
            "orderbook.json",
            "instrument.json",
            "derivative_info.json",
            "expiry_info.json",
            "option_info.json",
            "volume_info.json",
        ],
    )
    def test_schema_is_valid_json(self, schema_name: str) -> None:
        """Schema file should be valid JSON."""
        schema = load_schema(schema_name)
        assert isinstance(schema, dict)


class TestValidDataWithFixtures:
    """Test using pytest fixtures from conftest.py."""

    def test_valid_quote_fixture(self, valid_quote: dict[str, Any]) -> None:
        """Valid quote fixture should pass validation."""
        assert_valid("quote.json", valid_quote)

    def test_valid_ohlcv_fixture(self, valid_ohlcv: dict[str, Any]) -> None:
        """Valid OHLCV fixture should pass validation."""
        assert_valid("ohlcv.json", valid_ohlcv)

    def test_valid_trade_fixture(self, valid_trade: dict[str, Any]) -> None:
        """Valid trade fixture should pass validation."""
        assert_valid("trade.json", valid_trade)

    def test_valid_orderbook_fixture(self, valid_orderbook: dict[str, Any]) -> None:
        """Valid orderbook fixture should pass validation."""
        assert_valid("orderbook.json", valid_orderbook)

    def test_valid_instrument_fixture(self, valid_instrument: dict[str, Any]) -> None:
        """Valid instrument fixture should pass validation."""
        assert_valid("instrument.json", valid_instrument)
