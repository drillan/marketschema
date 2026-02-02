"""Base adapter class for transforming external data to marketschema models."""

from typing import Any, TypeVar

from marketschema.adapters.mapping import ModelMapping
from marketschema.adapters.transforms import Transforms
from marketschema.exceptions import AdapterError

T = TypeVar("T")


class BaseAdapter:
    """Abstract base class for data source adapters.

    Adapters transform data from external sources (exchanges, data providers)
    into standardized marketschema models.

    Subclasses must define:
    - source_name: Identifier for the data source
    - get_*_mapping() methods for each supported model type

    Example:
        class BinanceAdapter(BaseAdapter):
            source_name = "binance"

            def get_quote_mapping(self) -> list[ModelMapping]:
                return [
                    ModelMapping("symbol", "s"),
                    ModelMapping("timestamp", "T", transform=self.transforms.unix_timestamp_ms),
                    ModelMapping("bid", "b", transform=self.transforms.to_float),
                    ModelMapping("ask", "a", transform=self.transforms.to_float),
                ]
    """

    source_name: str = ""
    transforms: type[Transforms] = Transforms

    def __init__(self) -> None:
        """Initialize the adapter."""
        if not self.source_name:
            raise AdapterError(f"{self.__class__.__name__} must define source_name")

    def get_quote_mapping(self) -> list[ModelMapping]:
        """Return field mappings for Quote model.

        Override this method to provide mappings for your data source.

        Returns:
            List of ModelMapping definitions
        """
        return []

    def get_ohlcv_mapping(self) -> list[ModelMapping]:
        """Return field mappings for OHLCV model.

        Override this method to provide mappings for your data source.

        Returns:
            List of ModelMapping definitions
        """
        return []

    def get_trade_mapping(self) -> list[ModelMapping]:
        """Return field mappings for Trade model.

        Override this method to provide mappings for your data source.

        Returns:
            List of ModelMapping definitions
        """
        return []

    def get_orderbook_mapping(self) -> list[ModelMapping]:
        """Return field mappings for OrderBook model.

        Override this method to provide mappings for your data source.

        Returns:
            List of ModelMapping definitions
        """
        return []

    def get_instrument_mapping(self) -> list[ModelMapping]:
        """Return field mappings for Instrument model.

        Override this method to provide mappings for your data source.

        Returns:
            List of ModelMapping definitions
        """
        return []

    def _apply_mapping(
        self,
        raw_data: dict[str, Any],
        mappings: list[ModelMapping],
        model_class: type[T],
    ) -> T:
        """Apply mappings to transform raw data into a model instance.

        Args:
            raw_data: Dictionary containing source data
            mappings: List of ModelMapping definitions to apply
            model_class: Target model class (e.g., Quote, Trade)

        Returns:
            Instance of model_class with mapped data

        Raises:
            AdapterError: If mapping or model instantiation fails
        """
        try:
            mapped_data: dict[str, Any] = {}

            for mapping in mappings:
                value = mapping.apply(raw_data)
                if value is not None:
                    mapped_data[mapping.target_field] = value

            return model_class(**mapped_data)
        except Exception as e:
            raise AdapterError(
                f"Failed to apply mapping for {model_class.__name__}: {e}"
            ) from e

    def _get_nested_value(self, data: dict[str, Any], path: str) -> Any | None:
        """Get a value from nested dictionary using dot notation.

        Args:
            data: Dictionary to extract value from
            path: Dot-separated path (e.g., "ticker.price")

        Returns:
            The value at the path, or None if not found
        """
        keys = path.split(".")
        current: Any = data

        for key in keys:
            if not isinstance(current, dict):
                return None
            current = current.get(key)
            if current is None:
                return None

        return current


__all__ = ["BaseAdapter"]
