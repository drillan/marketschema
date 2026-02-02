#!/usr/bin/env python3
"""Demo script for bitbank Public API adapter.

This script demonstrates how to use the BitbankAdapter to fetch and parse
data from bitbank's Public API.

Usage:
    uv run python -m examples.bitbank.demo
"""

from __future__ import annotations

import json
import sys
import urllib.request
from pathlib import Path
from typing import Any

# Add project root to path for direct execution
if __name__ == "__main__":
    project_root = Path(__file__).resolve().parent.parent.parent
    if str(project_root) not in sys.path:
        sys.path.insert(0, str(project_root))

from examples.bitbank.adapter import BitbankAdapter

BITBANK_API_BASE = "https://public.bitbank.cc"
DEFAULT_PAIR = "btc_jpy"


def fetch_json(url: str) -> dict[str, Any]:
    """Fetch JSON from URL.

    Args:
        url: API endpoint URL

    Returns:
        Parsed JSON response

    Raises:
        RuntimeError: If API returns error
    """
    with urllib.request.urlopen(url) as response:
        data: dict[str, Any] = json.loads(response.read().decode())
        if data.get("success") != 1:
            raise RuntimeError(f"API error: {data}")
        result: dict[str, Any] = data["data"]
        return result


def demo_ticker(adapter: BitbankAdapter, pair: str) -> None:
    """Demonstrate ticker → Quote parsing."""
    print(f"\n{'=' * 60}")
    print(f"Ticker → Quote ({pair})")
    print("=" * 60)

    url = f"{BITBANK_API_BASE}/{pair}/ticker"
    print(f"GET {url}")

    ticker = fetch_json(url)
    quote = adapter.parse_quote(ticker, symbol=pair)

    print(f"\nRaw ticker: bid={ticker['buy']}, ask={ticker['sell']}")
    print(f"Quote: bid={quote.bid.root}, ask={quote.ask.root}")
    print(f"Timestamp: {quote.timestamp.root}")


def demo_transactions(adapter: BitbankAdapter, pair: str) -> None:
    """Demonstrate transactions → Trade parsing."""
    print(f"\n{'=' * 60}")
    print(f"Transactions → Trade ({pair})")
    print("=" * 60)

    url = f"{BITBANK_API_BASE}/{pair}/transactions"
    print(f"GET {url}")

    data = fetch_json(url)
    transactions = data.get("transactions", [])

    if not transactions:
        print("No recent transactions")
        return

    trades = adapter.parse_trades(transactions[:3], symbol=pair)

    for i, trade in enumerate(trades, 1):
        print(f"\nTrade {i}:")
        print(f"  Price: {trade.price.root}")
        print(f"  Size: {trade.size.root}")
        print(f"  Side: {trade.side.value}")
        print(f"  Timestamp: {trade.timestamp.root}")


def demo_candlestick(adapter: BitbankAdapter, pair: str) -> None:
    """Demonstrate candlestick → OHLCV parsing."""
    print(f"\n{'=' * 60}")
    print(f"Candlestick → OHLCV ({pair})")
    print("=" * 60)

    # Get today's date for the API call
    from datetime import UTC, datetime

    today = datetime.now(UTC).strftime("%Y%m%d")
    url = f"{BITBANK_API_BASE}/{pair}/candlestick/1hour/{today}"
    print(f"GET {url}")

    try:
        data = fetch_json(url)
        candlesticks = data.get("candlestick", [])

        if not candlesticks or not candlesticks[0].get("ohlcv"):
            print("No candlestick data available")
            return

        ohlcv_arrays = candlesticks[0]["ohlcv"][:3]
        ohlcvs = adapter.parse_ohlcv_batch(ohlcv_arrays, symbol=pair)

        for i, ohlcv in enumerate(ohlcvs, 1):
            print(f"\nOHLCV {i}:")
            print(f"  Open: {ohlcv.open.root}")
            print(f"  High: {ohlcv.high.root}")
            print(f"  Low: {ohlcv.low.root}")
            print(f"  Close: {ohlcv.close.root}")
            print(f"  Volume: {ohlcv.volume.root}")
            print(f"  Timestamp: {ohlcv.timestamp.root}")
    except RuntimeError as e:
        print(f"Error fetching candlestick: {e}")


def demo_depth(adapter: BitbankAdapter, pair: str) -> None:
    """Demonstrate depth → OrderBook parsing."""
    print(f"\n{'=' * 60}")
    print(f"Depth → OrderBook ({pair})")
    print("=" * 60)

    url = f"{BITBANK_API_BASE}/{pair}/depth"
    print(f"GET {url}")

    depth = fetch_json(url)
    orderbook = adapter.parse_orderbook(depth, symbol=pair)

    print(f"\nTimestamp: {orderbook.timestamp.root}")
    print("\nTop 3 Asks (ascending price):")
    for i, level in enumerate(orderbook.asks[:3], 1):
        print(f"  {i}. Price: {level.price.root}, Size: {level.size.root}")

    print("\nTop 3 Bids (descending price):")
    for i, level in enumerate(orderbook.bids[:3], 1):
        print(f"  {i}. Price: {level.price.root}, Size: {level.size.root}")


def main() -> None:
    """Run all demos."""
    print("=" * 60)
    print("bitbank Public API Adapter Demo")
    print("=" * 60)

    adapter = BitbankAdapter()
    pair = DEFAULT_PAIR

    demo_ticker(adapter, pair)
    demo_transactions(adapter, pair)
    demo_candlestick(adapter, pair)
    demo_depth(adapter, pair)

    print(f"\n{'=' * 60}")
    print("Demo completed!")
    print("=" * 60)


if __name__ == "__main__":
    main()
