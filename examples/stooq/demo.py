#!/usr/bin/env python3
"""Demo script for stooq.com stock data adapter.

This script demonstrates how to use the StooqAdapter to fetch and parse
CSV data from stooq.com using AsyncHttpClient.

Usage:
    uv run python -m examples.stooq.demo
    uv run python examples/stooq/demo.py
    uv run python examples/stooq/demo.py aapl.us
"""

from __future__ import annotations

import asyncio
import sys
from pathlib import Path

# Add project root to path for direct execution
if __name__ == "__main__":
    project_root = Path(__file__).resolve().parent.parent.parent
    if str(project_root) not in sys.path:
        sys.path.insert(0, str(project_root))

from examples.stooq.adapter import STOOQ_BASE_URL, StooqAdapter
from marketschema.exceptions import AdapterError
from marketschema.http.exceptions import (
    HttpConnectionError,
    HttpStatusError,
    HttpTimeoutError,
)

DEFAULT_SYMBOL = "spy.us"


async def demo_ohlcv(adapter: StooqAdapter, symbol: str) -> None:
    """Demonstrate CSV → OHLCV parsing."""
    print(f"\n{'=' * 60}")
    print(f"CSV → OHLCV ({symbol})")
    print("=" * 60)

    print(f"GET {STOOQ_BASE_URL}?s={symbol}&i=d")
    csv_content = await adapter.fetch_csv(symbol)

    # Show first few lines of raw CSV
    lines = csv_content.strip().split("\n")
    print(f"\nRaw CSV ({len(lines)} lines):")
    for line in lines[:4]:
        print(f"  {line}")
    if len(lines) > 4:
        print("  ...")

    # Parse CSV to OHLCV models
    ohlcvs = adapter.parse_csv(csv_content, symbol=symbol)
    print(f"\nParsed {len(ohlcvs)} OHLCV records")

    # Show most recent records (last 5)
    print("\nMost recent 5 records:")
    for ohlcv in ohlcvs[-5:]:
        print(f"\n  Date: {ohlcv.timestamp.root.date()}")
        print(f"  Open: {ohlcv.open.root}")
        print(f"  High: {ohlcv.high.root}")
        print(f"  Low: {ohlcv.low.root}")
        print(f"  Close: {ohlcv.close.root}")
        print(f"  Volume: {ohlcv.volume.root:,.0f}")


async def main() -> None:
    """Run demo."""
    print("=" * 60)
    print("stooq.com Stock Data Adapter Demo")
    print("=" * 60)

    # Get symbol from command line or use default
    symbol = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_SYMBOL

    try:
        async with StooqAdapter() as adapter:
            await demo_ohlcv(adapter, symbol)
    except HttpStatusError as e:
        if e.status_code == 404:
            print(f"\nError: Symbol '{symbol}' not found on stooq.com")
        else:
            print(f"\nError: HTTP {e.status_code} - {e.message}")
        sys.exit(1)
    except HttpTimeoutError as e:
        print(f"\nError: Request timed out: {e}")
        sys.exit(1)
    except HttpConnectionError as e:
        print(f"\nError: Could not connect to stooq.com: {e}")
        sys.exit(1)
    except AdapterError as e:
        print(f"\nError: Failed to parse CSV response: {e}")
        sys.exit(1)

    print(f"\n{'=' * 60}")
    print("Demo completed!")
    print("=" * 60)


if __name__ == "__main__":
    asyncio.run(main())
