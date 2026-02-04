"""Integration tests for StockAnalysisAdapter HTTP client."""

import httpx
import pytest
import respx

from examples.stockanalysis.adapter import (
    STOCKANALYSIS_BASE_URL,
    STOCKANALYSIS_USER_AGENT,
    StockAnalysisAdapter,
)
from marketschema.http.exceptions import (
    HttpConnectionError,
    HttpRateLimitError,
    HttpStatusError,
    HttpTimeoutError,
)


class TestFetchHistory:
    """Test fetch_history method with HTTP mocking."""

    @respx.mock
    @pytest.mark.asyncio
    async def test_fetch_history_success(self, stockanalysis_html_content: str) -> None:
        """Fetch history returns HTML content on success."""
        route = respx.get(f"{STOCKANALYSIS_BASE_URL}/tsla/history/").mock(
            return_value=httpx.Response(200, text=stockanalysis_html_content)
        )

        async with StockAnalysisAdapter() as adapter:
            result = await adapter.fetch_history("TSLA")

        assert route.called
        assert result == stockanalysis_html_content

    @respx.mock
    @pytest.mark.asyncio
    async def test_fetch_history_with_custom_symbol(
        self, stockanalysis_html_content: str
    ) -> None:
        """Fetch history works with different symbols."""
        route = respx.get(f"{STOCKANALYSIS_BASE_URL}/aapl/history/").mock(
            return_value=httpx.Response(200, text=stockanalysis_html_content)
        )

        async with StockAnalysisAdapter() as adapter:
            result = await adapter.fetch_history("AAPL")

        assert route.called
        assert result == stockanalysis_html_content

    @respx.mock
    @pytest.mark.asyncio
    async def test_fetch_history_sends_user_agent(
        self, stockanalysis_html_content: str
    ) -> None:
        """Fetch history sends correct User-Agent header."""
        route = respx.get(f"{STOCKANALYSIS_BASE_URL}/tsla/history/").mock(
            return_value=httpx.Response(200, text=stockanalysis_html_content)
        )

        async with StockAnalysisAdapter() as adapter:
            await adapter.fetch_history("TSLA")

        assert route.called
        request = route.calls[0].request
        assert request.headers.get("User-Agent") == STOCKANALYSIS_USER_AGENT

    @respx.mock
    @pytest.mark.asyncio
    async def test_fetch_history_symbol_lowercased(
        self, stockanalysis_html_content: str
    ) -> None:
        """Symbol is lowercased in URL."""
        route = respx.get(f"{STOCKANALYSIS_BASE_URL}/msft/history/").mock(
            return_value=httpx.Response(200, text=stockanalysis_html_content)
        )

        async with StockAnalysisAdapter() as adapter:
            await adapter.fetch_history("MSFT")

        assert route.called

    @respx.mock
    @pytest.mark.asyncio
    async def test_fetch_history_http_error(self) -> None:
        """Fetch history raises HttpStatusError on HTTP error."""
        respx.get(f"{STOCKANALYSIS_BASE_URL}/invalid/history/").mock(
            return_value=httpx.Response(404, text="Not Found")
        )

        async with StockAnalysisAdapter() as adapter:
            with pytest.raises(HttpStatusError) as exc_info:
                await adapter.fetch_history("INVALID")

        assert exc_info.value.status_code == 404

    @respx.mock
    @pytest.mark.asyncio
    async def test_fetch_history_timeout_error(self) -> None:
        """Fetch history raises HttpTimeoutError on timeout."""
        respx.get(f"{STOCKANALYSIS_BASE_URL}/tsla/history/").mock(
            side_effect=httpx.TimeoutException("Connection timeout")
        )

        async with StockAnalysisAdapter() as adapter:
            with pytest.raises(HttpTimeoutError):
                await adapter.fetch_history("TSLA")

    @respx.mock
    @pytest.mark.asyncio
    async def test_fetch_history_connection_error(self) -> None:
        """Fetch history raises HttpConnectionError on connection failure."""
        respx.get(f"{STOCKANALYSIS_BASE_URL}/tsla/history/").mock(
            side_effect=httpx.ConnectError("Connection refused")
        )

        async with StockAnalysisAdapter() as adapter:
            with pytest.raises(HttpConnectionError):
                await adapter.fetch_history("TSLA")

    @respx.mock
    @pytest.mark.asyncio
    async def test_fetch_history_rate_limit_error(self) -> None:
        """Fetch history raises HttpRateLimitError on 429."""
        respx.get(f"{STOCKANALYSIS_BASE_URL}/tsla/history/").mock(
            return_value=httpx.Response(
                429,
                text="Too Many Requests",
                headers={"Retry-After": "60"},
            )
        )

        async with StockAnalysisAdapter() as adapter:
            with pytest.raises(HttpRateLimitError) as exc_info:
                await adapter.fetch_history("TSLA")

        assert exc_info.value.retry_after == 60.0


class TestAdapterContextManager:
    """Test adapter context manager for resource management."""

    @respx.mock
    @pytest.mark.asyncio
    async def test_context_manager_closes_client(
        self, stockanalysis_html_content: str
    ) -> None:
        """Context manager properly closes HTTP client."""
        respx.get(f"{STOCKANALYSIS_BASE_URL}/tsla/history/").mock(
            return_value=httpx.Response(200, text=stockanalysis_html_content)
        )

        adapter = StockAnalysisAdapter()
        async with adapter:
            await adapter.fetch_history("TSLA")
            # After use, client should exist
            assert adapter._http_client is not None

        # After context exit, client should be closed
        assert adapter._http_client is None

    @respx.mock
    @pytest.mark.asyncio
    async def test_context_manager_closes_on_exception(self) -> None:
        """Context manager closes client even on exception."""
        respx.get(f"{STOCKANALYSIS_BASE_URL}/tsla/history/").mock(
            return_value=httpx.Response(500, text="Server Error")
        )

        adapter = StockAnalysisAdapter()
        with pytest.raises(HttpStatusError) as exc_info:
            async with adapter:
                await adapter.fetch_history("TSLA")

        assert exc_info.value.status_code == 500
        # After context exit, client should be closed
        assert adapter._http_client is None


class TestFetchAndParse:
    """Test fetch and parse integration."""

    @respx.mock
    @pytest.mark.asyncio
    async def test_fetch_and_parse_ohlcv(self, stockanalysis_html_content: str) -> None:
        """Fetch and parse works together for OHLCV."""
        respx.get(f"{STOCKANALYSIS_BASE_URL}/tsla/history/").mock(
            return_value=httpx.Response(200, text=stockanalysis_html_content)
        )

        async with StockAnalysisAdapter() as adapter:
            html = await adapter.fetch_history("TSLA")
            ohlcvs = adapter.parse_html(html, symbol="TSLA")

        assert len(ohlcvs) == 2
        assert ohlcvs[0].symbol.root == "TSLA"

    @respx.mock
    @pytest.mark.asyncio
    async def test_fetch_and_parse_extended_ohlcv(
        self, stockanalysis_html_content: str
    ) -> None:
        """Fetch and parse works together for ExtendedOHLCV."""
        respx.get(f"{STOCKANALYSIS_BASE_URL}/tsla/history/").mock(
            return_value=httpx.Response(200, text=stockanalysis_html_content)
        )

        async with StockAnalysisAdapter() as adapter:
            html = await adapter.fetch_history("TSLA")
            ohlcvs = adapter.parse_html_extended(html, symbol="TSLA")

        assert len(ohlcvs) == 2
        assert ohlcvs[0].symbol.root == "TSLA"
        assert ohlcvs[0].adj_close.root == 269.96
