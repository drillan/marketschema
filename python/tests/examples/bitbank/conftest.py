"""Test fixtures for bitbank adapter tests."""

from typing import Any

import pytest


@pytest.fixture
def bitbank_ticker_response() -> dict[str, Any]:
    """Fixture for bitbank ticker API response.

    https://github.com/bitbankinc/bitbank-api-docs/blob/master/public-api.md#ticker

    Example response:
    GET /public/{pair}/ticker
    """
    return {
        "sell": "9653004",
        "buy": "9651884",
        "high": "9800000",
        "low": "9550000",
        "open": "9620000",
        "last": "9652000",
        "vol": "1234.5678",
        "timestamp": 1738454400000,  # 2025-02-02T00:00:00Z
    }


@pytest.fixture
def bitbank_transactions_response() -> dict[str, Any]:
    """Fixture for bitbank transactions API response.

    https://github.com/bitbankinc/bitbank-api-docs/blob/master/public-api.md#transactions

    Example response:
    GET /public/{pair}/transactions
    """
    return {
        "transactions": [
            {
                "transaction_id": 12345678,
                "side": "buy",
                "price": "9651884",
                "amount": "0.1234",
                "executed_at": 1738454400000,  # 2025-02-02T00:00:00Z
            },
            {
                "transaction_id": 12345679,
                "side": "sell",
                "price": "9652000",
                "amount": "0.5",
                "executed_at": 1738454401000,  # 2025-02-02T00:00:01Z
            },
        ]
    }


@pytest.fixture
def bitbank_candlestick_response() -> dict[str, Any]:
    """Fixture for bitbank candlestick API response.

    https://github.com/bitbankinc/bitbank-api-docs/blob/master/public-api.md#candlestick

    Example response:
    GET /public/{pair}/candlestick/{candle_type}/{yyyymmdd}

    Note: bitbank returns candlestick as array [open, high, low, close, volume, timestamp]
    """
    return {
        "candlestick": [
            {
                "type": "1hour",
                "ohlcv": [
                    # [open, high, low, close, volume, timestamp]
                    [
                        "9620000",
                        "9680000",
                        "9600000",
                        "9650000",
                        "123.456",
                        1738454400000,
                    ],
                    [
                        "9650000",
                        "9700000",
                        "9640000",
                        "9680000",
                        "234.567",
                        1738458000000,
                    ],
                ],
            }
        ]
    }


@pytest.fixture
def bitbank_depth_response() -> dict[str, Any]:
    """Fixture for bitbank depth API response.

    https://github.com/bitbankinc/bitbank-api-docs/blob/master/public-api.md#depth

    Example response:
    GET /public/{pair}/depth

    Note: asks are in ascending order, bids are in descending order
    """
    return {
        "asks": [
            ["9653004", "0.5"],
            ["9653010", "1.0"],
            ["9653100", "2.5"],
        ],
        "bids": [
            ["9651884", "0.3"],
            ["9651800", "1.2"],
            ["9651700", "3.0"],
        ],
        "timestamp": 1738454400000,  # 2025-02-02T00:00:00Z
    }
