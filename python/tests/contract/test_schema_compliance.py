"""Test JSON Schema files are compliant with Draft 2020-12."""

import json
from pathlib import Path

import pytest

SCHEMAS_DIR = Path(__file__).parent.parent.parent.parent / "schemas"

EXPECTED_SCHEMAS = [
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
]


class TestSchemaCompliance:
    """Test that all schemas are Draft 2020-12 compliant."""

    def test_all_schemas_exist(self) -> None:
        """All expected schema files should exist."""
        for schema_name in EXPECTED_SCHEMAS:
            schema_path = SCHEMAS_DIR / schema_name
            assert schema_path.exists(), f"Schema file missing: {schema_name}"

    @pytest.mark.parametrize("schema_name", EXPECTED_SCHEMAS)
    def test_schema_is_valid_json(self, schema_name: str) -> None:
        """Each schema should be valid JSON."""
        schema_path = SCHEMAS_DIR / schema_name
        with open(schema_path, encoding="utf-8") as f:
            schema = json.load(f)
        assert isinstance(schema, dict)

    @pytest.mark.parametrize("schema_name", EXPECTED_SCHEMAS)
    def test_schema_has_draft_2020_12(self, schema_name: str) -> None:
        """Each schema should declare Draft 2020-12."""
        schema_path = SCHEMAS_DIR / schema_name
        with open(schema_path, encoding="utf-8") as f:
            schema = json.load(f)

        assert "$schema" in schema
        assert "draft/2020-12" in schema["$schema"]

    # NOTE: $id is intentionally omitted from schemas/ to allow relative $ref
    # resolution without conflicts. The specs/002-data-model/contracts/ directory
    # contains the formal schemas with absolute URI $id for specification purposes.

    @pytest.mark.parametrize("schema_name", EXPECTED_SCHEMAS)
    def test_schema_has_title(self, schema_name: str) -> None:
        """Each schema should have a title."""
        schema_path = SCHEMAS_DIR / schema_name
        with open(schema_path, encoding="utf-8") as f:
            schema = json.load(f)

        assert "title" in schema
        assert len(schema["title"]) > 0

    @pytest.mark.parametrize("schema_name", EXPECTED_SCHEMAS)
    def test_schema_has_description(self, schema_name: str) -> None:
        """Each schema should have a description."""
        schema_path = SCHEMAS_DIR / schema_name
        with open(schema_path, encoding="utf-8") as f:
            schema = json.load(f)

        assert "description" in schema
        assert len(schema["description"]) > 0

    def test_definitions_schema_has_defs(self) -> None:
        """The definitions.json should use $defs (Draft 2020-12 style)."""
        schema_path = SCHEMAS_DIR / "definitions.json"
        with open(schema_path, encoding="utf-8") as f:
            schema = json.load(f)

        assert "$defs" in schema
        assert "Timestamp" in schema["$defs"]
        assert "Symbol" in schema["$defs"]
        assert "Price" in schema["$defs"]
        assert "Size" in schema["$defs"]
        assert "Side" in schema["$defs"]

    @pytest.mark.parametrize(
        "schema_name",
        [s for s in EXPECTED_SCHEMAS if s != "definitions.json"],
    )
    def test_leaf_schemas_have_unevaluated_properties(self, schema_name: str) -> None:
        """Leaf schemas should have unevaluatedProperties: false."""
        schema_path = SCHEMAS_DIR / schema_name
        with open(schema_path, encoding="utf-8") as f:
            schema = json.load(f)

        assert "unevaluatedProperties" in schema
        assert schema["unevaluatedProperties"] is False
