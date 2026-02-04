"""Tests for generate_models.sh script behavior.

These tests ensure that the model generation script preserves
the manually maintained __init__.py file with public exports.
"""

import subprocess
from pathlib import Path

import pytest

# Project paths
# __file__ is python/tests/unit/test_generate_models.py
# .parent x3 = python/, .parent x4 = marketschema/ (PROJECT_ROOT)
PYTHON_ROOT = Path(__file__).parent.parent.parent
PROJECT_ROOT = PYTHON_ROOT.parent
MODELS_DIR = PYTHON_ROOT / "src" / "marketschema" / "models"
INIT_FILE = MODELS_DIR / "__init__.py"
GENERATE_SCRIPT = PROJECT_ROOT / "scripts" / "generate_models.sh"

# Expected public exports that must be preserved
EXPECTED_EXPORTS = [
    "OHLCV",
    "Quote",
    "Trade",
    "OrderBook",
    "Instrument",
    "VolumeInfo",
    "DerivativeInfo",
    "ExpiryInfo",
    "OptionInfo",
]


class TestInitFilePrerequisites:
    """Tests for __init__.py prerequisites."""

    def test_init_file_exists_before_generation(self) -> None:
        """Verify that __init__.py exists in models directory."""
        assert INIT_FILE.exists(), f"Expected {INIT_FILE} to exist"

    def test_init_file_has_public_exports(self) -> None:
        """Verify that __init__.py contains expected public exports."""
        content = INIT_FILE.read_text()

        for export in EXPECTED_EXPORTS:
            assert export in content, (
                f"Expected '{export}' to be exported in __init__.py"
            )

    def test_all_in_list_contains_exports(self) -> None:
        """Verify that __all__ list contains the expected exports."""
        content = INIT_FILE.read_text()

        assert "__all__" in content, "__all__ should be defined in __init__.py"

        for export in EXPECTED_EXPORTS:
            # Check that export is in __all__ list
            assert f'"{export}"' in content, f'Expected "{export}" in __all__ list'


class TestGenerateModelsScript:
    """Tests for generate_models.sh script execution."""

    # Timeout for script execution (2 minutes)
    SCRIPT_TIMEOUT_SECONDS = 120

    @pytest.mark.slow
    def test_generate_models_preserves_init_file_and_cleans_backup(self) -> None:
        """Verify that generate_models.sh preserves __init__.py and cleans up backup."""
        # Capture original content
        original_content = INIT_FILE.read_text()
        backup_file = MODELS_DIR / "__init__.py.bak"

        # Run the generation script
        result = subprocess.run(
            [str(GENERATE_SCRIPT)],
            cwd=str(PROJECT_ROOT),
            capture_output=True,
            text=True,
            check=False,
            timeout=self.SCRIPT_TIMEOUT_SECONDS,
        )

        assert result.returncode == 0, (
            f"Script failed with return code {result.returncode}\n"
            f"stdout:\n{result.stdout}\n"
            f"stderr:\n{result.stderr}"
        )

        # Verify content is preserved
        new_content = INIT_FILE.read_text()
        assert new_content == original_content, (
            "__init__.py content was modified by generate_models.sh"
        )

        # Verify no backup file remains
        assert not backup_file.exists(), (
            f"Backup file {backup_file} should not exist after script completion"
        )


class TestModelImports:
    """Tests for model import functionality."""

    def test_import_public_models(self) -> None:
        """Verify that all public models can be imported as Pydantic models."""
        from pydantic import BaseModel

        from marketschema.models import (
            OHLCV,
            DerivativeInfo,
            ExpiryInfo,
            Instrument,
            OptionInfo,
            OrderBook,
            Quote,
            Trade,
            VolumeInfo,
        )

        # Verify these are Pydantic model classes
        assert issubclass(OHLCV, BaseModel), "OHLCV should be a Pydantic model"
        assert issubclass(Quote, BaseModel), "Quote should be a Pydantic model"
        assert issubclass(Trade, BaseModel), "Trade should be a Pydantic model"
        assert issubclass(OrderBook, BaseModel), "OrderBook should be a Pydantic model"
        assert issubclass(Instrument, BaseModel), (
            "Instrument should be a Pydantic model"
        )
        assert issubclass(VolumeInfo, BaseModel), (
            "VolumeInfo should be a Pydantic model"
        )
        assert issubclass(DerivativeInfo, BaseModel), (
            "DerivativeInfo should be a Pydantic model"
        )
        assert issubclass(ExpiryInfo, BaseModel), (
            "ExpiryInfo should be a Pydantic model"
        )
        assert issubclass(OptionInfo, BaseModel), (
            "OptionInfo should be a Pydantic model"
        )

    def test_import_common_types(self) -> None:
        """Verify that common types can be imported from marketschema.models."""
        from marketschema.models import (
            AssetClass,
            Currency,
            Exchange,
            Price,
            Side,
            Size,
            Symbol,
            Timestamp,
        )

        # Verify these are actual types (enums or type aliases)
        assert AssetClass is not None
        assert Currency is not None
        assert Exchange is not None
        assert Price is not None
        assert Side is not None
        assert Size is not None
        assert Symbol is not None
        assert Timestamp is not None
