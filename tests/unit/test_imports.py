"""Smoke tests — package imports and scaffold presence."""

import importlib


def test_package_imports() -> None:
    assert importlib.import_module("meta_quant") is not None
    assert importlib.import_module("meta_quant.experiments.reproducibility") is not None
    assert importlib.import_module("meta_quant.data") is not None
    assert importlib.import_module("meta_quant.features") is not None
