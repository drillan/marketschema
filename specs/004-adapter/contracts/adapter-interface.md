# Adapter Interface Contract

**Feature**: 004-adapter
**Date**: 2026-02-03

> Note: アダプターインターフェースは Python ライブラリ内部 API のため、Python の型シグネチャとしてコントラクトを定義する。

## Module: `marketschema.adapters.base`

### BaseAdapter

```python
class BaseAdapter:
    """Abstract base class for data source adapters.

    Adapters transform data from external sources (exchanges, data providers)
    into standardized marketschema models.

    Subclasses must define:
    - source_name: Identifier for the data source
    - get_*_mapping() methods for each supported model type

    Example:
        class BitbankAdapter(BaseAdapter):
            source_name = "bitbank"

            def get_quote_mapping(self) -> list[ModelMapping]:
                return [
                    ModelMapping("bid", "buy", transform=self.transforms.to_float),
                    ModelMapping("ask", "sell", transform=self.transforms.to_float),
                    ModelMapping("timestamp", "timestamp", transform=self.transforms.unix_timestamp_ms),
                ]
    """

    # Required class attribute
    source_name: str = ""

    # Default transforms class
    transforms: type[Transforms] = Transforms

    def __init__(
        self,
        http_client: AsyncHttpClient | None = None,
    ) -> None:
        """Initialize the adapter.

        Args:
            http_client: Optional HTTP client. If not provided, one will be
                created lazily when http_client property is accessed.

        Raises:
            AdapterError: If source_name is not defined (empty string).
        """
        ...

    @property
    def http_client(self) -> AsyncHttpClient:
        """Get the HTTP client (lazy initialization).

        If no http_client was provided during initialization, creates a new
        AsyncHttpClient on first access.

        Returns:
            The HTTP client instance.
        """
        ...

    async def close(self) -> None:
        """Close the HTTP client if owned by this adapter.

        Only closes the client if it was created internally (not injected).
        """
        ...

    async def __aenter__(self) -> Self:
        """Enter async context manager.

        Returns:
            Self for use in async with statement.
        """
        ...

    async def __aexit__(
        self,
        exc_type: type[BaseException] | None,
        exc_val: BaseException | None,
        exc_tb: TracebackType | None,
    ) -> None:
        """Exit async context manager and close resources."""
        ...

    def get_quote_mapping(self) -> list[ModelMapping]:
        """Return field mappings for Quote model.

        Override this method to provide mappings for your data source.

        Returns:
            List of ModelMapping definitions. Empty list if not supported.
        """
        ...

    def get_ohlcv_mapping(self) -> list[ModelMapping]:
        """Return field mappings for OHLCV model.

        Override this method to provide mappings for your data source.

        Returns:
            List of ModelMapping definitions. Empty list if not supported.
        """
        ...

    def get_trade_mapping(self) -> list[ModelMapping]:
        """Return field mappings for Trade model.

        Override this method to provide mappings for your data source.

        Returns:
            List of ModelMapping definitions. Empty list if not supported.
        """
        ...

    def get_orderbook_mapping(self) -> list[ModelMapping]:
        """Return field mappings for OrderBook model.

        Override this method to provide mappings for your data source.

        Returns:
            List of ModelMapping definitions. Empty list if not supported.
        """
        ...

    def get_instrument_mapping(self) -> list[ModelMapping]:
        """Return field mappings for Instrument model.

        Override this method to provide mappings for your data source.

        Returns:
            List of ModelMapping definitions. Empty list if not supported.
        """
        ...

    def _apply_mapping(
        self,
        raw_data: dict[str, Any],
        mappings: list[ModelMapping],
        model_class: type[T],
    ) -> T:
        """Apply mappings to transform raw data into a model instance.

        Args:
            raw_data: Dictionary containing source data.
            mappings: List of ModelMapping definitions to apply.
            model_class: Target model class (e.g., Quote, Trade).

        Returns:
            Instance of model_class with mapped data.

        Raises:
            AdapterError: If mapping application or model instantiation fails.
                Wraps MappingError, TransformError, TypeError, ValueError.
        """
        ...

    def _get_nested_value(self, data: dict[str, Any], path: str) -> Any | None:
        """Get a value from nested dictionary using dot notation.

        Args:
            data: Dictionary to extract value from.
            path: Dot-separated path (e.g., "ticker.price").

        Returns:
            The value at the path, or None if not found.
        """
        ...
```

## Module: `marketschema.adapters.mapping`

### ModelMapping

```python
@dataclass(frozen=True, slots=True)
class ModelMapping:
    """Defines how to map a source field to a target field.

    This is an immutable dataclass that encapsulates the mapping logic
    from source data fields to target model fields.

    Attributes:
        target_field: Name of the field in the target model.
        source_field: Path to the field in the source data.
            Supports dot notation for nested fields (e.g., "price.bid").
        transform: Optional callable to transform the source value.
            Should raise TransformError on failure.
        default: Optional default value if source field is missing or None.
        required: If True, raise MappingError when field is missing.
            Default is True.

    Example:
        # Simple mapping
        ModelMapping("bid", "buy_price")

        # With transformation
        ModelMapping("bid", "buy_price", transform=Transforms.to_float)

        # Nested field access
        ModelMapping("bid", "prices.buy", transform=Transforms.to_float)

        # With default value
        ModelMapping("volume", "vol", default=0.0, required=False)
    """

    target_field: str
    source_field: str
    transform: Callable[[Any], Any] | None = None
    default: Any | None = None
    required: bool = True

    def apply(self, source_data: dict[str, Any]) -> Any:
        """Apply the mapping to source data and return the transformed value.

        Args:
            source_data: Dictionary containing the source data.

        Returns:
            The transformed value for the target field.
            Returns None if not required and value is missing.
            Returns default if value is missing and default is set.

        Raises:
            MappingError: If source field is required but missing.
            TransformError: If transformation fails (propagated from transform).
        """
        ...

    def _get_nested_value(self, data: dict[str, Any], path: str) -> Any | None:
        """Get a value from nested dictionary using dot notation.

        Args:
            data: Dictionary to extract value from.
            path: Dot-separated path (e.g., "best_bid.price").

        Returns:
            The value at the path, or None if not found.
        """
        ...
```

## Module: `marketschema.adapters.registry`

### AdapterRegistry

```python
class AdapterRegistry:
    """Registry for managing adapter instances by source name.

    This is a singleton registry that allows adapters to be registered
    and retrieved by their source_name. Uses class-level state to maintain
    a single registry across the application.

    Example:
        # Register using decorator
        @register
        class BitbankAdapter(BaseAdapter):
            source_name = "bitbank"
            ...

        # Get adapter by name
        adapter = AdapterRegistry.get("bitbank")

        # List all registered adapters
        names = AdapterRegistry.list_adapters()

        # Check if registered
        if AdapterRegistry.is_registered("bitbank"):
            ...
    """

    _instance: "AdapterRegistry | None" = None
    _adapters: dict[str, type[BaseAdapter]]

    def __new__(cls) -> "AdapterRegistry":
        """Create singleton instance."""
        ...

    @classmethod
    def register[T: BaseAdapter](cls, adapter_class: type[T]) -> type[T]:
        """Register an adapter class with the registry.

        Args:
            adapter_class: Adapter class to register. Must have source_name defined.

        Returns:
            The adapter class unchanged (for use as decorator).

        Raises:
            AdapterError: If adapter has no source_name or name is already registered.
        """
        ...

    @classmethod
    def get(cls, source_name: str) -> BaseAdapter:
        """Get an adapter instance by source name.

        Creates a new instance of the registered adapter class.

        Args:
            source_name: Name of the data source.

        Returns:
            New instance of the registered adapter.

        Raises:
            KeyError: If no adapter is registered for the source name.
                Error message includes list of available adapters.
        """
        ...

    @classmethod
    def list_adapters(cls) -> list[str]:
        """List all registered adapter source names.

        Returns:
            List of registered source names.
        """
        ...

    @classmethod
    def is_registered(cls, source_name: str) -> bool:
        """Check if an adapter is registered for a source name.

        Args:
            source_name: Name of the data source.

        Returns:
            True if an adapter is registered, False otherwise.
        """
        ...

    @classmethod
    def clear(cls) -> None:
        """Clear all registered adapters.

        Primarily useful for testing to reset registry state.
        """
        ...


def register[T: BaseAdapter](adapter_class: type[T]) -> type[T]:
    """Decorator to register an adapter class with the global registry.

    Convenience function that delegates to AdapterRegistry.register().

    Example:
        @register
        class MyAdapter(BaseAdapter):
            source_name = "my_source"
            ...

    Args:
        adapter_class: Adapter class to register.

    Returns:
        The adapter class unchanged.

    Raises:
        AdapterError: If adapter has no source_name or name is already registered.
    """
    ...
```

## Module: `marketschema.exceptions`

### Adapter Exceptions

```python
class AdapterError(MarketSchemaError):
    """Base exception for adapter-related errors.

    Raised when adapter initialization fails or mapping operations fail.

    Attributes:
        message: Error description.
    """

    def __init__(self, message: str) -> None: ...


class MappingError(MarketSchemaError):
    """Error during field mapping.

    Raised when a required field is missing from source data.

    Attributes:
        message: Error description including field name.
    """

    def __init__(self, message: str) -> None: ...


class TransformError(MarketSchemaError):
    """Error during value transformation.

    Raised when a transform function cannot process the input value.

    Attributes:
        message: Error description including value and expected type.
    """

    def __init__(self, message: str) -> None: ...
```

## Type Exports

```python
# marketschema.adapters.__init__.py
__all__ = [
    # Base
    "BaseAdapter",

    # Mapping
    "ModelMapping",

    # Registry
    "AdapterRegistry",
    "register",

    # Transforms
    "Transforms",
]
```
