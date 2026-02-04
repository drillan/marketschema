"""Shared pytest fixtures for marketschema tests."""

from pathlib import Path
from typing import Any

import pytest

# Repository root directory
REPO_ROOT = Path(__file__).parent.parent

# Schema directory paths
SCHEMAS_DIR = REPO_ROOT / "src" / "marketschema" / "schemas"
CONTRACTS_DIR = REPO_ROOT / "specs" / "002-data-model" / "contracts"
FIXTURES_DIR = Path(__file__).parent / "fixtures"


@pytest.fixture
def schemas_dir() -> Path:
    """Return the path to the schemas directory."""
    return SCHEMAS_DIR


@pytest.fixture
def contracts_dir() -> Path:
    """Return the path to the contracts directory."""
    return CONTRACTS_DIR


@pytest.fixture
def fixtures_dir() -> Path:
    """Return the path to the test fixtures directory."""
    return FIXTURES_DIR


@pytest.fixture
def valid_quote() -> dict[str, Any]:
    """Return a valid Quote data sample."""
    return {
        "symbol": "7203.T",
        "timestamp": "2026-02-02T09:00:00.000Z",
        "bid": 2850.0,
        "ask": 2851.0,
        "bid_size": 1000,
        "ask_size": 500,
    }


@pytest.fixture
def valid_ohlcv() -> dict[str, Any]:
    """Return a valid OHLCV data sample."""
    return {
        "symbol": "BTCUSDT",
        "timestamp": "2026-02-02T00:00:00.000Z",
        "open": 50000.0,
        "high": 51500.0,
        "low": 49800.0,
        "close": 51200.0,
        "volume": 12345.67,
        "quote_volume": 628000000.0,
    }


@pytest.fixture
def valid_trade() -> dict[str, Any]:
    """Return a valid Trade data sample."""
    return {
        "symbol": "AAPL",
        "timestamp": "2026-02-02T14:30:00.123Z",
        "price": 175.50,
        "size": 100,
        "side": "buy",
    }


@pytest.fixture
def valid_orderbook() -> dict[str, Any]:
    """Return a valid OrderBook data sample."""
    return {
        "symbol": "USDJPY",
        "timestamp": "2026-02-02T09:00:00.000Z",
        "bids": [
            {"price": 149.50, "size": 1000000},
            {"price": 149.49, "size": 2000000},
        ],
        "asks": [
            {"price": 149.51, "size": 1500000},
            {"price": 149.52, "size": 3000000},
        ],
    }


@pytest.fixture
def valid_instrument() -> dict[str, Any]:
    """Return a valid Instrument data sample."""
    return {
        "symbol": "7203.T",
        "asset_class": "equity",
        "currency": "JPY",
        "exchange": "XJPX",
    }


@pytest.fixture
def valid_derivative_info() -> dict[str, Any]:
    """Return a valid DerivativeInfo data sample."""
    return {
        "multiplier": 1000,
        "tick_size": 5.0,
        "underlying_symbol": "NK225",
        "underlying_type": "index",
        "settlement_method": "cash",
    }


@pytest.fixture
def valid_expiry_info() -> dict[str, Any]:
    """Return a valid ExpiryInfo data sample."""
    return {
        "expiry": "2026-03",
        "expiration_date": "2026-03-13",
        "last_trading_day": "2026-03-12",
    }


@pytest.fixture
def valid_option_info() -> dict[str, Any]:
    """Return a valid OptionInfo data sample."""
    return {
        "strike_price": 40000.0,
        "option_type": "call",
        "exercise_style": "european",
    }


@pytest.fixture
def valid_volume_info() -> dict[str, Any]:
    """Return a valid VolumeInfo data sample."""
    return {
        "symbol": "BTCUSDT",
        "timestamp": "2026-02-02T00:00:00.000Z",
        "volume": 12345.67,
        "quote_volume": 628000000.0,
    }
