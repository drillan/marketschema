"""Model mapping definitions for adapter transformations."""

from collections.abc import Callable
from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True, slots=True)
class ModelMapping:
    """Defines how to map a source field to a target field.

    Attributes:
        target_field: Name of the field in the target model
        source_field: Path to the field in the source data (supports dot notation for nested fields)
        transform: Optional callable to transform the source value
        default: Optional default value if source field is missing or None
    """

    target_field: str
    source_field: str
    transform: Callable[[Any], Any] | None = None
    default: Any | None = None

    def apply(self, source_data: dict[str, Any]) -> Any:
        """Apply the mapping to source data and return the transformed value.

        Args:
            source_data: Dictionary containing the source data

        Returns:
            The transformed value for the target field

        Raises:
            KeyError: If source field is required but missing
            ValueError: If transformation fails
        """
        value = self._get_nested_value(source_data, self.source_field)

        if value is None:
            if self.default is not None:
                return self.default
            return None

        if self.transform is not None:
            return self.transform(value)

        return value

    def _get_nested_value(self, data: dict[str, Any], path: str) -> Any | None:
        """Get a value from nested dictionary using dot notation.

        Args:
            data: Dictionary to extract value from
            path: Dot-separated path (e.g., "best_bid.price")

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


__all__ = ["ModelMapping"]
