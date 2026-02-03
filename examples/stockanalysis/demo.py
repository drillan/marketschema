#!/usr/bin/env python3
"""Demo script for stockanalysis.com HTML stock data adapter.

This script demonstrates how to use the StockAnalysisAdapter to fetch and parse
HTML data from stockanalysis.com.

Usage:
    uv run python -m examples.stockanalysis.demo
    uv run python examples/stockanalysis/demo.py
    uv run python examples/stockanalysis/demo.py aapl
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

from examples.stockanalysis.adapter import STOCKANALYSIS_BASE_URL, StockAnalysisAdapter
from marketschema.exceptions import AdapterError
from marketschema.http.exceptions import (
    HttpConnectionError,
    HttpRateLimitError,
    HttpStatusError,
    HttpTimeoutError,
)

DEFAULT_SYMBOL = "tsla"


async def demo_ohlcv(adapter: StockAnalysisAdapter, symbol: str) -> None:
    """Demonstrate HTML → ExtendedOHLCV parsing."""
    print(f"\n{'=' * 60}")
    print(f"HTML → ExtendedOHLCV ({symbol.upper()})")
    print("=" * 60)

    url = f"{STOCKANALYSIS_BASE_URL}/{symbol.lower()}/history/"
    print(f"\nGET {url}")

    html_content = await adapter.fetch_history(symbol)

    # Show page size info
    print(f"\nReceived {len(html_content):,} bytes of HTML")

    # Parse HTML to ExtendedOHLCV models (includes Adj Close)
    ohlcvs = adapter.parse_html_extended(html_content, symbol=symbol.upper())
    print(f"Parsed {len(ohlcvs)} ExtendedOHLCV records")

    if not ohlcvs:
        print("\nNo data found. The page may have a different structure.")
        return

    # Show most recent records (first 5, as page shows newest first)
    print("\nMost recent 5 records:")
    for ohlcv in ohlcvs[:5]:
        print(f"\n  Date: {ohlcv.timestamp.root.date()}")
        print(f"  Open: {ohlcv.open.root}")
        print(f"  High: {ohlcv.high.root}")
        print(f"  Low: {ohlcv.low.root}")
        print(f"  Close: {ohlcv.close.root}")
        print(f"  Adj Close: {ohlcv.adj_close.root}")
        print(f"  Volume: {ohlcv.volume.root:,.0f}")


async def main() -> None:
    """Run demo."""
    print("=" * 60)
    print("stockanalysis.com HTML Stock Data Adapter Demo")
    print("=" * 60)

    # Get symbol from command line or use default
    symbol = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_SYMBOL

    try:
        async with StockAnalysisAdapter() as adapter:
            await demo_ohlcv(adapter, symbol)
    except HttpRateLimitError as e:
        retry_msg = f" Retry after {e.retry_after}s." if e.retry_after else ""
        print(f"\nError: Rate limited.{retry_msg}")
        sys.exit(1)
    except HttpStatusError as e:
        if e.status_code == 404:
            print(f"\nError: Symbol '{symbol.upper()}' not found on stockanalysis.com")
        else:
            print(f"\nError: HTTP {e.status_code} - {e.message}")
        sys.exit(1)
    except HttpTimeoutError:
        print("\nError: Request timed out. Please check your network connection.")
        sys.exit(1)
    except HttpConnectionError:
        print(
            "\nError: Could not connect to stockanalysis.com. Please check your network."
        )
        sys.exit(1)
    except AdapterError as e:
        print(f"\nError: Failed to parse HTML response: {e}")
        sys.exit(1)

    print(f"\n{'=' * 60}")
    print("Demo completed!")
    print("=" * 60)


if __name__ == "__main__":
    asyncio.run(main())
