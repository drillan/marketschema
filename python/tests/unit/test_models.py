"""Test pydantic models generated from JSON Schema."""

from datetime import UTC, datetime

import pytest
from pydantic import ValidationError

from marketschema.models import (
    OHLCV,
    AssetClass,
    Currency,
    DerivativeInfo,
    Exchange,
    ExerciseStyle,
    ExpiryInfo,
    Instrument,
    OptionInfo,
    OptionType,
    OrderBook,
    Price,
    PriceLevel,
    Quote,
    SettlementMethod,
    Side,
    Size,
    Symbol,
    Timestamp,
    Trade,
    UnderlyingType,
    VolumeInfo,
)
from marketschema.models.derivative_info import SettlementPrice
from marketschema.models.volume_info import OpenInterest


class TestQuoteModel:
    """Test Quote pydantic model."""

    def test_create_quote_with_all_fields(self) -> None:
        """Quote can be created with all fields."""
        quote = Quote(
            symbol=Symbol("7203.T"),
            timestamp=Timestamp(datetime(2026, 2, 2, 9, 0, 0, tzinfo=UTC)),
            bid=Price(2850.0),
            ask=Price(2851.0),
            bid_size=Size(1000.0),
            ask_size=Size(500.0),
        )
        assert quote.symbol.root == "7203.T"
        assert quote.bid.root == 2850.0
        assert quote.ask.root == 2851.0

    def test_create_quote_without_optional_fields(self) -> None:
        """Quote can be created without optional bid_size and ask_size."""
        quote = Quote(
            symbol=Symbol("AAPL"),
            timestamp=Timestamp(datetime(2026, 2, 2, 14, 30, 0, tzinfo=UTC)),
            bid=Price(175.0),
            ask=Price(175.50),
        )
        assert quote.bid_size is None
        assert quote.ask_size is None

    def test_quote_rejects_extra_fields(self) -> None:
        """Quote rejects unknown fields due to extra='forbid'."""
        with pytest.raises(ValidationError) as exc_info:
            Quote(
                symbol=Symbol("AAPL"),
                timestamp=Timestamp(datetime(2026, 2, 2, 14, 30, 0, tzinfo=UTC)),
                bid=Price(175.0),
                ask=Price(175.50),
                extra_field="should fail",  # type: ignore[call-arg]
            )
        assert "extra_field" in str(exc_info.value)


class TestOHLCVModel:
    """Test OHLCV pydantic model."""

    def test_create_ohlcv(self) -> None:
        """OHLCV can be created with required fields."""
        ohlcv = OHLCV(
            symbol=Symbol("BTCUSDT"),
            timestamp=Timestamp(datetime(2026, 2, 2, 0, 0, 0, tzinfo=UTC)),
            open=Price(50000.0),
            high=Price(51500.0),
            low=Price(49800.0),
            close=Price(51200.0),
            volume=Size(12345.67),
        )
        assert ohlcv.symbol.root == "BTCUSDT"
        assert ohlcv.open.root == 50000.0
        assert ohlcv.volume.root == 12345.67

    def test_ohlcv_with_quote_volume(self) -> None:
        """OHLCV can include optional quote_volume."""
        ohlcv = OHLCV(
            symbol=Symbol("BTCUSDT"),
            timestamp=Timestamp(datetime(2026, 2, 2, 0, 0, 0, tzinfo=UTC)),
            open=Price(50000.0),
            high=Price(51500.0),
            low=Price(49800.0),
            close=Price(51200.0),
            volume=Size(12345.67),
            quote_volume=Size(628000000.0),
        )
        assert ohlcv.quote_volume is not None
        assert ohlcv.quote_volume.root == 628000000.0


class TestTradeModel:
    """Test Trade pydantic model."""

    def test_create_trade(self) -> None:
        """Trade can be created with all required fields."""
        trade = Trade(
            symbol=Symbol("AAPL"),
            timestamp=Timestamp(datetime(2026, 2, 2, 14, 30, 0, tzinfo=UTC)),
            price=Price(175.50),
            size=Size(100.0),
            side=Side.buy,
        )
        assert trade.symbol.root == "AAPL"
        assert trade.side == Side.buy

    def test_trade_with_sell_side(self) -> None:
        """Trade can have sell side."""
        trade = Trade(
            symbol=Symbol("AAPL"),
            timestamp=Timestamp(datetime(2026, 2, 2, 14, 30, 0, tzinfo=UTC)),
            price=Price(175.50),
            size=Size(100.0),
            side=Side.sell,
        )
        assert trade.side == Side.sell


class TestOrderBookModel:
    """Test OrderBook pydantic model."""

    def test_create_orderbook(self) -> None:
        """OrderBook can be created with bids and asks."""
        orderbook = OrderBook(
            symbol=Symbol("USDJPY"),
            timestamp=Timestamp(datetime(2026, 2, 2, 9, 0, 0, tzinfo=UTC)),
            bids=[
                PriceLevel(price=Price(149.50), size=Size(1000000.0)),
                PriceLevel(price=Price(149.49), size=Size(2000000.0)),
            ],
            asks=[
                PriceLevel(price=Price(149.51), size=Size(1500000.0)),
                PriceLevel(price=Price(149.52), size=Size(3000000.0)),
            ],
        )
        assert len(orderbook.bids) == 2
        assert len(orderbook.asks) == 2
        assert orderbook.bids[0].price.root == 149.50

    def test_create_empty_orderbook(self) -> None:
        """OrderBook can be created with empty bids and asks."""
        orderbook = OrderBook(
            symbol=Symbol("USDJPY"),
            timestamp=Timestamp(datetime(2026, 2, 2, 9, 0, 0, tzinfo=UTC)),
            bids=[],
            asks=[],
        )
        assert len(orderbook.bids) == 0
        assert len(orderbook.asks) == 0


class TestInstrumentModel:
    """Test Instrument pydantic model."""

    def test_create_equity_instrument(self) -> None:
        """Instrument can be created for equity."""
        instrument = Instrument(
            symbol=Symbol("7203.T"),
            asset_class=AssetClass.equity,
            currency=Currency("JPY"),
            exchange=Exchange("XJPX"),
        )
        assert instrument.symbol.root == "7203.T"
        assert instrument.asset_class == AssetClass.equity

    def test_create_crypto_instrument(self) -> None:
        """Instrument can be created for crypto pair."""
        instrument = Instrument(
            symbol=Symbol("BTC/USDT"),
            asset_class=AssetClass.crypto,
            base_currency=Currency("BTC"),
            quote_currency=Currency("USD"),
        )
        assert instrument.asset_class == AssetClass.crypto


class TestDerivativeInfoModel:
    """Test DerivativeInfo pydantic model."""

    def test_create_derivative_info(self) -> None:
        """DerivativeInfo can be created."""
        deriv = DerivativeInfo(
            multiplier=1000.0,
            tick_size=5.0,
            underlying_symbol=Symbol("NK225"),
            underlying_type=UnderlyingType.index_,
            settlement_method=SettlementMethod.cash,
        )
        assert deriv.multiplier == 1000.0
        assert deriv.underlying_type == UnderlyingType.index_

    def test_create_derivative_info_with_settlement_price(self) -> None:
        """DerivativeInfo can include optional settlement_price."""
        deriv = DerivativeInfo(
            multiplier=1000.0,
            tick_size=5.0,
            underlying_symbol=Symbol("NK225"),
            underlying_type=UnderlyingType.index_,
            settlement_price=SettlementPrice(39850.0),
        )
        assert deriv.settlement_price is not None
        assert deriv.settlement_price.root == 39850.0

    def test_derivative_info_settlement_price_optional(self) -> None:
        """DerivativeInfo settlement_price defaults to None."""
        deriv = DerivativeInfo(
            multiplier=1000.0,
            tick_size=5.0,
            underlying_symbol=Symbol("NK225"),
            underlying_type=UnderlyingType.index_,
        )
        assert deriv.settlement_price is None

    def test_settlement_price_accepts_zero(self) -> None:
        """SettlementPrice accepts zero as valid value."""
        sp = SettlementPrice(0.0)
        assert sp.root == 0.0

    def test_settlement_price_rejects_negative_value(self) -> None:
        """SettlementPrice rejects negative values."""
        with pytest.raises(ValidationError) as exc_info:
            SettlementPrice(-1.0)
        assert "greater_than_equal" in str(exc_info.value).lower()


class TestExpiryInfoModel:
    """Test ExpiryInfo pydantic model."""

    def test_create_expiry_info(self) -> None:
        """ExpiryInfo can be created."""
        from marketschema.models.definitions import Date, ExpirySeries

        expiry = ExpiryInfo(
            expiry=ExpirySeries("2026-03"),
            expiration_date=Date("2026-03-13"),
            last_trading_day=Date("2026-03-12"),
        )
        assert expiry.expiration_date.root == "2026-03-13"


class TestOptionInfoModel:
    """Test OptionInfo pydantic model."""

    def test_create_option_info(self) -> None:
        """OptionInfo can be created."""
        option = OptionInfo(
            strike_price=Price(40000.0),
            option_type=OptionType.call,
            exercise_style=ExerciseStyle.european,
        )
        assert option.option_type == OptionType.call
        assert option.exercise_style == ExerciseStyle.european


class TestVolumeInfoModel:
    """Test VolumeInfo pydantic model."""

    def test_create_volume_info(self) -> None:
        """VolumeInfo can be created with required fields."""
        vol = VolumeInfo(
            symbol=Symbol("BTCUSDT"),
            timestamp=Timestamp(datetime(2026, 2, 2, 0, 0, 0, tzinfo=UTC)),
            volume=Size(12345.67),
        )
        assert vol.symbol.root == "BTCUSDT"
        assert vol.volume.root == 12345.67

    def test_volume_info_with_open_interest(self) -> None:
        """VolumeInfo can include optional open_interest."""
        vol = VolumeInfo(
            symbol=Symbol("BTCUSDT"),
            timestamp=Timestamp(datetime(2026, 2, 2, 0, 0, 0, tzinfo=UTC)),
            volume=Size(12345.67),
            open_interest=OpenInterest(125000.0),
        )
        assert vol.open_interest is not None
        assert vol.open_interest.root == 125000.0

    def test_volume_info_open_interest_optional(self) -> None:
        """VolumeInfo open_interest defaults to None."""
        vol = VolumeInfo(
            symbol=Symbol("BTCUSDT"),
            timestamp=Timestamp(datetime(2026, 2, 2, 0, 0, 0, tzinfo=UTC)),
            volume=Size(12345.67),
        )
        assert vol.open_interest is None

    def test_open_interest_accepts_zero(self) -> None:
        """OpenInterest accepts zero as valid value (market with no positions)."""
        oi = OpenInterest(0.0)
        assert oi.root == 0.0

    def test_open_interest_rejects_negative_value(self) -> None:
        """OpenInterest rejects negative values."""
        with pytest.raises(ValidationError) as exc_info:
            OpenInterest(-100.0)
        assert "greater_than_equal" in str(exc_info.value).lower()


class TestEnumValues:
    """Test enum value correctness."""

    def test_side_enum(self) -> None:
        """Side enum has correct values."""
        assert Side.buy.value == "buy"
        assert Side.sell.value == "sell"

    def test_asset_class_enum(self) -> None:
        """AssetClass enum has correct values."""
        assert AssetClass.equity.value == "equity"
        assert AssetClass.crypto.value == "crypto"
        assert AssetClass.future.value == "future"
        assert AssetClass.option.value == "option"

    def test_option_type_enum(self) -> None:
        """OptionType enum has correct values."""
        assert OptionType.call.value == "call"
        assert OptionType.put.value == "put"


class TestSerializationDeserialization:
    """Test JSON serialization and deserialization."""

    def test_quote_to_json_and_back(self) -> None:
        """Quote can be serialized to JSON and back."""
        quote = Quote(
            symbol=Symbol("AAPL"),
            timestamp=Timestamp(datetime(2026, 2, 2, 14, 30, 0, tzinfo=UTC)),
            bid=Price(175.0),
            ask=Price(175.50),
        )
        json_str = quote.model_dump_json()
        assert '"symbol":"AAPL"' in json_str

    def test_quote_model_dump(self) -> None:
        """Quote model_dump returns dict with correct structure."""
        quote = Quote(
            symbol=Symbol("AAPL"),
            timestamp=Timestamp(datetime(2026, 2, 2, 14, 30, 0, tzinfo=UTC)),
            bid=Price(175.0),
            ask=Price(175.50),
        )
        data = quote.model_dump()
        assert data["symbol"] == "AAPL"
        assert data["bid"] == 175.0
        assert data["ask"] == 175.50
