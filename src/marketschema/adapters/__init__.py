"""Adapter infrastructure for transforming data from various sources."""

from marketschema.adapters.base import BaseAdapter
from marketschema.adapters.mapping import ModelMapping
from marketschema.adapters.registry import AdapterRegistry, register
from marketschema.adapters.transforms import Transforms

__all__ = [
    "BaseAdapter",
    "ModelMapping",
    "Transforms",
    "AdapterRegistry",
    "register",
]
