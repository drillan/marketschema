"""Integration tests for StooqAdapter HTTP client migration (Issue #17).

Tests verify:
- fetch_csv returns CSV content
- Correct query parameters are used
- HTTP errors are properly propagated
- Integration between fetch and parse
- Resource cleanup on adapter close
"""

import httpx
import pytest
import respx

from examples.stooq.adapter import (
    STOOQ_BASE_URL,
    STOOQ_INTERVAL_DAILY,
    StooqAdapter,
)
from marketschema.http import HttpRateLimitError, HttpStatusError

# Sample CSV data for testing
SAMPLE_CSV = """Date,Open,High,Low,Close,Volume
2024-01-02,100.0,105.0,99.0,104.0,1000000
2024-01-03,104.0,106.0,103.0,105.0,1200000
"""


class TestStooqAdapterFetchCsv:
    """Tests for StooqAdapter.fetch_csv method."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_csv_returns_csv_content(self) -> None:
        """fetch_csv should return CSV content as string."""
        respx.get(STOOQ_BASE_URL).mock(
            return_value=httpx.Response(200, text=SAMPLE_CSV)
        )

        async with StooqAdapter() as adapter:
            result = await adapter.fetch_csv("spy.us")

        assert result == SAMPLE_CSV

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_csv_uses_correct_params(self) -> None:
        """fetch_csv should use correct query parameters."""
        route = respx.get(STOOQ_BASE_URL).mock(
            return_value=httpx.Response(200, text=SAMPLE_CSV)
        )

        async with StooqAdapter() as adapter:
            await adapter.fetch_csv("aapl.us")

        assert route.called
        request = route.calls.last.request
        assert request.url.params["s"] == "aapl.us"
        assert request.url.params["i"] == STOOQ_INTERVAL_DAILY

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_csv_propagates_http_500_error(self) -> None:
        """fetch_csv should propagate HTTP 500 errors."""
        respx.get(STOOQ_BASE_URL).mock(
            return_value=httpx.Response(500, text="Internal Server Error")
        )

        async with StooqAdapter() as adapter:
            with pytest.raises(HttpStatusError) as exc_info:
                await adapter.fetch_csv("spy.us")

        assert exc_info.value.status_code == 500

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_csv_handles_404(self) -> None:
        """fetch_csv should raise HttpStatusError for 404."""
        respx.get(STOOQ_BASE_URL).mock(
            return_value=httpx.Response(404, text="Not Found")
        )

        async with StooqAdapter() as adapter:
            with pytest.raises(HttpStatusError) as exc_info:
                await adapter.fetch_csv("invalid")

        assert exc_info.value.status_code == 404

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_csv_handles_rate_limit(self) -> None:
        """fetch_csv should raise HttpRateLimitError for 429."""
        respx.get(STOOQ_BASE_URL).mock(
            return_value=httpx.Response(
                429,
                text="Too Many Requests",
                headers={"Retry-After": "60"},
            )
        )

        async with StooqAdapter() as adapter:
            with pytest.raises(HttpRateLimitError) as exc_info:
                await adapter.fetch_csv("spy.us")

        assert exc_info.value.status_code == 429
        assert exc_info.value.retry_after == 60.0


class TestStooqAdapterFetchAndParse:
    """Tests for StooqAdapter.fetch_and_parse method."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_and_parse_integration(self) -> None:
        """fetch_and_parse should fetch CSV and return parsed OHLCV models."""
        respx.get(STOOQ_BASE_URL).mock(
            return_value=httpx.Response(200, text=SAMPLE_CSV)
        )

        async with StooqAdapter() as adapter:
            ohlcvs = await adapter.fetch_and_parse("spy.us")

        assert len(ohlcvs) == 2
        assert ohlcvs[0].symbol.root == "spy.us"
        assert ohlcvs[0].open.root == 100.0
        assert ohlcvs[0].close.root == 104.0
        assert ohlcvs[1].close.root == 105.0


class TestStooqAdapterResourceManagement:
    """Tests for StooqAdapter resource management."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_adapter_closes_http_client(self) -> None:
        """Adapter should close HTTP client when exiting context."""
        respx.get(STOOQ_BASE_URL).mock(
            return_value=httpx.Response(200, text=SAMPLE_CSV)
        )

        adapter = StooqAdapter()
        async with adapter:
            await adapter.fetch_csv("spy.us")
            # Verify client was created
            assert adapter._http_client is not None

        # After context exit, client should be None (closed)
        assert adapter._http_client is None

    @pytest.mark.asyncio
    @respx.mock
    async def test_adapter_reuses_http_client(self) -> None:
        """Adapter should reuse the same HTTP client for multiple requests."""
        respx.get(STOOQ_BASE_URL).mock(
            return_value=httpx.Response(200, text=SAMPLE_CSV)
        )

        async with StooqAdapter() as adapter:
            await adapter.fetch_csv("spy.us")
            client1 = adapter.http_client

            await adapter.fetch_csv("aapl.us")
            client2 = adapter.http_client

        assert client1 is client2
