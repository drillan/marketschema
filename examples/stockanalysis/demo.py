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

import sys
import urllib.request
from pathlib import Path

# Add project root to path for direct execution
if __name__ == "__main__":
    project_root = Path(__file__).resolve().parent.parent.parent
    if str(project_root) not in sys.path:
        sys.path.insert(0, str(project_root))

from examples.stockanalysis.adapter import StockAnalysisAdapter

STOCKANALYSIS_URL = "https://stockanalysis.com/stocks"
DEFAULT_SYMBOL = "tsla"


def fetch_html(symbol: str) -> str:
    """Fetch HTML data from stockanalysis.com.

    Args:
        symbol: Stock symbol (e.g., "tsla", "aapl", "msft")

    Returns:
        HTML content as string

    Raises:
        RuntimeError: If fetch fails
    """
    url = f"{STOCKANALYSIS_URL}/{symbol.lower()}/history/"
    print(f"GET {url}")

    try:
        request = urllib.request.Request(
            url,
            headers={
                "User-Agent": (
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
                    "AppleWebKit/537.36 (KHTML, like Gecko) "
                    "Chrome/120.0.0.0 Safari/537.36"
                )
            },
        )
        with urllib.request.urlopen(request) as response:
            content: str = response.read().decode("utf-8")
            return content
    except urllib.error.URLError as e:
        raise RuntimeError(f"Failed to fetch data: {e}") from e
    except UnicodeDecodeError as e:
        raise RuntimeError(f"Failed to decode response as UTF-8: {e}") from e


def demo_ohlcv(adapter: StockAnalysisAdapter, symbol: str) -> None:
    """Demonstrate HTML → OHLCV parsing."""
    print(f"\n{'=' * 60}")
    print(f"HTML → OHLCV ({symbol.upper()})")
    print("=" * 60)

    html_content = fetch_html(symbol)

    # Show page size info
    print(f"\nReceived {len(html_content):,} bytes of HTML")

    # Parse HTML to OHLCV models
    ohlcvs = adapter.parse_html(html_content, symbol=symbol.upper())
    print(f"Parsed {len(ohlcvs)} OHLCV records")

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
        print(f"  Volume: {ohlcv.volume.root:,.0f}")


def main() -> None:
    """Run demo."""
    print("=" * 60)
    print("stockanalysis.com HTML Stock Data Adapter Demo")
    print("=" * 60)

    adapter = StockAnalysisAdapter()

    # Get symbol from command line or use default
    symbol = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_SYMBOL

    demo_ohlcv(adapter, symbol)

    print(f"\n{'=' * 60}")
    print("Demo completed!")
    print("=" * 60)


if __name__ == "__main__":
    main()
