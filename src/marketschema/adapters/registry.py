"""Adapter registry for managing multiple data source adapters."""

from marketschema.adapters.base import BaseAdapter
from marketschema.exceptions import AdapterError


class AdapterRegistry:
    """Registry for managing adapter instances by source name.

    This is a singleton registry that allows adapters to be registered
    and retrieved by their source_name.

    Example:
        @register
        class BinanceAdapter(BaseAdapter):
            source_name = "binance"
            ...

        # Get adapter by name
        adapter = AdapterRegistry.get("binance")
    """

    _instance: "AdapterRegistry | None" = None
    _adapters: dict[str, type[BaseAdapter]]

    def __new__(cls) -> "AdapterRegistry":
        """Create singleton instance."""
        if cls._instance is None:
            cls._instance = super().__new__(cls)
            cls._instance._adapters = {}
        return cls._instance

    @classmethod
    def register[T: BaseAdapter](cls, adapter_class: type[T]) -> type[T]:
        """Register an adapter class with the registry.

        Args:
            adapter_class: Adapter class to register

        Returns:
            The adapter class (for use as decorator)

        Raises:
            AdapterError: If adapter has no source_name or name is already registered
        """
        instance = cls()

        source_name = adapter_class.source_name
        if not source_name:
            raise AdapterError(
                f"{adapter_class.__name__} must define source_name"
            )

        if source_name in instance._adapters:
            raise AdapterError(
                f"Adapter for '{source_name}' is already registered"
            )

        instance._adapters[source_name] = adapter_class
        return adapter_class

    @classmethod
    def get(cls, source_name: str) -> BaseAdapter:
        """Get an adapter instance by source name.

        Args:
            source_name: Name of the data source

        Returns:
            Instance of the registered adapter

        Raises:
            KeyError: If no adapter is registered for the source name
        """
        instance = cls()

        if source_name not in instance._adapters:
            available = ", ".join(instance._adapters.keys()) or "none"
            raise KeyError(
                f"No adapter registered for '{source_name}'. "
                f"Available adapters: {available}"
            )

        adapter_class = instance._adapters[source_name]
        return adapter_class()

    @classmethod
    def list_adapters(cls) -> list[str]:
        """List all registered adapter source names.

        Returns:
            List of registered source names
        """
        instance = cls()
        return list(instance._adapters.keys())

    @classmethod
    def clear(cls) -> None:
        """Clear all registered adapters.

        Primarily useful for testing.
        """
        instance = cls()
        instance._adapters.clear()

    @classmethod
    def is_registered(cls, source_name: str) -> bool:
        """Check if an adapter is registered for a source name.

        Args:
            source_name: Name of the data source

        Returns:
            True if an adapter is registered, False otherwise
        """
        instance = cls()
        return source_name in instance._adapters


def register[T: BaseAdapter](adapter_class: type[T]) -> type[T]:
    """Decorator to register an adapter class with the global registry.

    Example:
        @register
        class MyAdapter(BaseAdapter):
            source_name = "my_source"
            ...

    Args:
        adapter_class: Adapter class to register

    Returns:
        The adapter class
    """
    return AdapterRegistry.register(adapter_class)


__all__ = ["AdapterRegistry", "register"]
