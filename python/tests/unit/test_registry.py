"""Test AdapterRegistry functionality."""

import pytest

from marketschema.adapters import AdapterRegistry, BaseAdapter, ModelMapping, register
from marketschema.exceptions import AdapterError


class TestAdapterRegistry:
    """Test AdapterRegistry class."""

    def setup_method(self) -> None:
        """Clear registry before each test."""
        AdapterRegistry.clear()

    def test_register_adapter(self) -> None:
        """Adapter can be registered."""

        class TestAdapter(BaseAdapter):
            source_name = "test_source"

        AdapterRegistry.register(TestAdapter)

        assert AdapterRegistry.is_registered("test_source")
        assert "test_source" in AdapterRegistry.list_adapters()

    def test_get_adapter(self) -> None:
        """Registered adapter can be retrieved."""

        class TestAdapter(BaseAdapter):
            source_name = "test_source"

        AdapterRegistry.register(TestAdapter)
        adapter = AdapterRegistry.get("test_source")

        assert isinstance(adapter, TestAdapter)
        assert adapter.source_name == "test_source"

    def test_get_unknown_adapter_raises_keyerror(self) -> None:
        """Getting unknown adapter raises KeyError."""
        with pytest.raises(KeyError, match="No adapter registered for 'unknown'"):
            AdapterRegistry.get("unknown")

    def test_duplicate_registration_raises_error(self) -> None:
        """Registering duplicate source_name raises AdapterError."""

        class TestAdapter1(BaseAdapter):
            source_name = "duplicate"

        class TestAdapter2(BaseAdapter):
            source_name = "duplicate"

        AdapterRegistry.register(TestAdapter1)

        with pytest.raises(AdapterError, match="already registered"):
            AdapterRegistry.register(TestAdapter2)

    def test_register_without_source_name_raises_error(self) -> None:
        """Registering adapter without source_name raises AdapterError."""

        class NoSourceAdapter(BaseAdapter):
            pass

        with pytest.raises(AdapterError, match="must define source_name"):
            AdapterRegistry.register(NoSourceAdapter)

    def test_list_adapters(self) -> None:
        """list_adapters returns all registered source names."""

        class Adapter1(BaseAdapter):
            source_name = "source1"

        class Adapter2(BaseAdapter):
            source_name = "source2"

        AdapterRegistry.register(Adapter1)
        AdapterRegistry.register(Adapter2)

        adapters = AdapterRegistry.list_adapters()
        assert "source1" in adapters
        assert "source2" in adapters
        assert len(adapters) == 2

    def test_clear_removes_all_adapters(self) -> None:
        """clear() removes all registered adapters."""

        class TestAdapter(BaseAdapter):
            source_name = "test"

        AdapterRegistry.register(TestAdapter)
        assert AdapterRegistry.is_registered("test")

        AdapterRegistry.clear()
        assert not AdapterRegistry.is_registered("test")
        assert len(AdapterRegistry.list_adapters()) == 0


class TestRegisterDecorator:
    """Test @register decorator."""

    def setup_method(self) -> None:
        """Clear registry before each test."""
        AdapterRegistry.clear()

    def test_register_decorator(self) -> None:
        """@register decorator registers adapter."""

        @register
        class DecoratedAdapter(BaseAdapter):
            source_name = "decorated"

        assert AdapterRegistry.is_registered("decorated")
        adapter = AdapterRegistry.get("decorated")
        assert isinstance(adapter, DecoratedAdapter)

    def test_register_decorator_returns_class(self) -> None:
        """@register decorator returns the original class."""

        @register
        class DecoratedAdapter(BaseAdapter):
            source_name = "decorated"

        # Class should still be usable directly
        adapter = DecoratedAdapter()
        assert adapter.source_name == "decorated"


class TestAdapterRegistrySingleton:
    """Test AdapterRegistry singleton behavior."""

    def setup_method(self) -> None:
        """Clear registry before each test."""
        AdapterRegistry.clear()

    def test_singleton_instance(self) -> None:
        """AdapterRegistry is a singleton."""
        registry1 = AdapterRegistry()
        registry2 = AdapterRegistry()

        assert registry1 is registry2

    def test_registrations_persist_across_instances(self) -> None:
        """Registrations are visible across all instances."""

        class TestAdapter(BaseAdapter):
            source_name = "persistent"

        AdapterRegistry()  # Create instance
        AdapterRegistry.register(TestAdapter)

        registry2 = AdapterRegistry()
        assert registry2.is_registered("persistent")


class TestAdapterWithMappings:
    """Test registered adapters work with mappings."""

    def setup_method(self) -> None:
        """Clear registry before each test."""
        AdapterRegistry.clear()

    def test_registered_adapter_has_mappings(self) -> None:
        """Registered adapter retains its mapping methods."""

        @register
        class MappingAdapter(BaseAdapter):
            source_name = "mapping_test"

            def get_quote_mapping(self) -> list[ModelMapping]:
                return [
                    ModelMapping("symbol", "s"),
                    ModelMapping("timestamp", "t"),
                ]

        adapter = AdapterRegistry.get("mapping_test")
        mappings = adapter.get_quote_mapping()

        assert len(mappings) == 2
        assert mappings[0].target_field == "symbol"
        assert mappings[0].source_field == "s"
