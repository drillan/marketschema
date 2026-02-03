#!/usr/bin/env python3
"""Demo script for bitbank Public API adapter.

This script demonstrates how to use the BitbankAdapter to fetch and parse
data from bitbank's Public API using async/await.

Usage:
    uv run python -m examples.bitbank.demo
    # Or directly:
    .venv/bin/python examples/bitbank/demo.py
"""

from __future__ import annotations

import asyncio
import sys
from datetime import UTC, datetime
from pathlib import Path

# Add project root to path for direct execution
if __name__ == "__main__":
    project_root = Path(__file__).resolve().parent.parent.parent
    if str(project_root) not in sys.path:
        sys.path.insert(0, str(project_root))

from examples.bitbank.adapter import BitbankAdapter

DEFAULT_PAIR = "btc_jpy"


async def demo_ticker(adapter: BitbankAdapter, pair: str) -> None:
    """Demonstrate ticker → Quote parsing."""
    print(f"\n{'=' * 60}")
    print(f"Ticker → Quote ({pair})")
    print("=" * 60)

    quote = await adapter.fetch_ticker(pair)

    print(f"\nQuote: bid={quote.bid.root}, ask={quote.ask.root}")
    print(f"Timestamp: {quote.timestamp.root}")


async def demo_transactions(adapter: BitbankAdapter, pair: str) -> None:
    """Demonstrate transactions → Trade parsing."""
    print(f"\n{'=' * 60}")
    print(f"Transactions → Trade ({pair})")
    print("=" * 60)

    trades = await adapter.fetch_transactions(pair)

    if not trades:
        print("No recent transactions")
        return

    for i, trade in enumerate(trades[:3], 1):
        print(f"\nTrade {i}:")
        print(f"  Price: {trade.price.root}")
        print(f"  Size: {trade.size.root}")
        print(f"  Side: {trade.side.value}")
        print(f"  Timestamp: {trade.timestamp.root}")


async def demo_candlestick(adapter: BitbankAdapter, pair: str) -> None:
    """Demonstrate candlestick → OHLCV parsing."""
    print(f"\n{'=' * 60}")
    print(f"Candlestick → OHLCV ({pair})")
    print("=" * 60)

    today = datetime.now(UTC).strftime("%Y%m%d")
    ohlcvs = await adapter.fetch_candlestick(pair, "1hour", today)

    if not ohlcvs:
        print("No candlestick data available")
        return

    for i, ohlcv in enumerate(ohlcvs[:3], 1):
        print(f"\nOHLCV {i}:")
        print(f"  Open: {ohlcv.open.root}")
        print(f"  High: {ohlcv.high.root}")
        print(f"  Low: {ohlcv.low.root}")
        print(f"  Close: {ohlcv.close.root}")
        print(f"  Volume: {ohlcv.volume.root}")
        print(f"  Timestamp: {ohlcv.timestamp.root}")


async def demo_depth(adapter: BitbankAdapter, pair: str) -> None:
    """Demonstrate depth → OrderBook parsing."""
    print(f"\n{'=' * 60}")
    print(f"Depth → OrderBook ({pair})")
    print("=" * 60)

    orderbook = await adapter.fetch_depth(pair)

    print(f"\nTimestamp: {orderbook.timestamp.root}")
    print("\nTop 3 Asks (ascending price):")
    for i, level in enumerate(orderbook.asks[:3], 1):
        print(f"  {i}. Price: {level.price.root}, Size: {level.size.root}")

    print("\nTop 3 Bids (descending price):")
    for i, level in enumerate(orderbook.bids[:3], 1):
        print(f"  {i}. Price: {level.price.root}, Size: {level.size.root}")


async def main() -> None:
    """Run all demos."""
    print("=" * 60)
    print("bitbank Public API Adapter Demo")
    print("=" * 60)

    async with BitbankAdapter() as adapter:
        pair = DEFAULT_PAIR

        await demo_ticker(adapter, pair)
        await demo_transactions(adapter, pair)
        await demo_candlestick(adapter, pair)
        await demo_depth(adapter, pair)

    print(f"\n{'=' * 60}")
    print("Demo completed!")
    print("=" * 60)


if __name__ == "__main__":
    asyncio.run(main())
