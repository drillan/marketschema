"""Pydantic models for stockanalysis.com adapter."""

from pydantic import ConfigDict

from marketschema.models import OHLCV, definitions


class ExtendedOHLCV(OHLCV):
    """OHLCV with adjusted close price.

    Extends the base OHLCV model with an adjusted close price field,
    which accounts for stock splits and dividend adjustments.

    This model is specifically designed for stockanalysis.com data
    which includes adjusted close in its historical data tables.
    """

    model_config = ConfigDict(extra="forbid")

    adj_close: definitions.Price
    """調整後終値（株式分割・配当調整済み）"""
