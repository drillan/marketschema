"""Custom exceptions for marketschema."""


class MarketSchemaError(Exception):
    """Base exception for all marketschema errors."""


class ValidationError(MarketSchemaError):
    """Raised when data validation fails."""


class TransformError(MarketSchemaError):
    """Raised when data transformation fails."""


class AdapterError(MarketSchemaError):
    """Raised when adapter operations fail."""


__all__ = [
    "MarketSchemaError",
    "ValidationError",
    "TransformError",
    "AdapterError",
]
