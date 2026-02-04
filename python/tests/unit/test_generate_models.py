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

    @pytest.mark.slow
    def test_generate_models_preserves_init_file(self) -> None:
        """Verify that running generate_models.sh preserves __init__.py content."""
        # Capture original content
        original_content = INIT_FILE.read_text()

        # Run the generation script
        result = subprocess.run(
            [str(GENERATE_SCRIPT)],
            cwd=str(PROJECT_ROOT),
            capture_output=True,
            text=True,
            check=False,
        )

        assert result.returncode == 0, f"Script failed: {result.stderr}"

        # Verify content is preserved
        new_content = INIT_FILE.read_text()
        assert new_content == original_content, (
            "__init__.py content was modified by generate_models.sh"
        )

    @pytest.mark.slow
    def test_generate_models_no_backup_file_left(self) -> None:
        """Verify that no backup file is left after script execution."""
        backup_file = MODELS_DIR / "__init__.py.bak"

        # Run the generation script
        result = subprocess.run(
            [str(GENERATE_SCRIPT)],
            cwd=str(PROJECT_ROOT),
            capture_output=True,
            text=True,
            check=False,
        )

        assert result.returncode == 0, f"Script failed: {result.stderr}"

        # Verify no backup file remains
        assert not backup_file.exists(), (
            f"Backup file {backup_file} should not exist after script completion"
        )


class TestModelImports:
    """Tests for model import functionality."""

    def test_import_public_models(self) -> None:
        """Verify that all public models can be imported from marketschema.models."""
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

        # Verify these are actual classes, not None
        assert OHLCV is not None
        assert Quote is not None
        assert Trade is not None
        assert OrderBook is not None
        assert Instrument is not None
        assert VolumeInfo is not None
        assert DerivativeInfo is not None
        assert ExpiryInfo is not None
        assert OptionInfo is not None

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

        # Verify these are actual types, not None
        assert AssetClass is not None
        assert Currency is not None
        assert Exchange is not None
        assert Price is not None
        assert Side is not None
        assert Size is not None
        assert Symbol is not None
        assert Timestamp is not None
