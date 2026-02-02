"""marketschema - Unified market data schema for financial applications."""

from importlib.metadata import version

from marketschema.adapters import (
    AdapterRegistry,
    BaseAdapter,
    ModelMapping,
    Transforms,
    register,
)
from marketschema.exceptions import (
    AdapterError,
    MarketSchemaError,
    TransformError,
    ValidationError,
)
from marketschema.models import (
    OHLCV,
    AssetClass,
    Currency,
    Date,
    DerivativeInfo,
    Exchange,
    ExerciseStyle,
    ExpiryInfo,
    ExpirySeries,
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

__version__ = version("marketschema")

__all__ = [
    # Version
    "__version__",
    # Exceptions
    "MarketSchemaError",
    "ValidationError",
    "TransformError",
    "AdapterError",
    # Adapter infrastructure
    "AdapterRegistry",
    "BaseAdapter",
    "ModelMapping",
    "Transforms",
    "register",
    # Common types
    "AssetClass",
    "Currency",
    "Date",
    "Exchange",
    "ExerciseStyle",
    "ExpirySeries",
    "OptionType",
    "Price",
    "PriceLevel",
    "SettlementMethod",
    "Side",
    "Size",
    "Symbol",
    "Timestamp",
    "UnderlyingType",
    # Market data models
    "Quote",
    "OHLCV",
    "Trade",
    "OrderBook",
    "VolumeInfo",
    # Instrument models
    "Instrument",
    "DerivativeInfo",
    "ExpiryInfo",
    "OptionInfo",
]
