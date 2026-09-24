"""Unit tests — reproducibility harness (MQ-FR-013 / AGENTS sec 6)."""

from meta_quant.experiments.reproducibility import config_hash, default_meta


def test_config_hash_is_deterministic() -> None:
    cfg = {"strategy": "basis", "min_deviation_bps": 15}
    assert config_hash(cfg) == config_hash(cfg)


def test_config_hash_differs_on_change() -> None:
    assert config_hash({"a": 1}) != config_hash({"a": 2})


def test_default_meta_has_required_fields() -> None:
    m = default_meta(
        experiment_id="e1",
        dataset_version="v1",
        time_range="2025-09-01/2025-09-24",
        strategy_version="basis_v0.1",
        config_dict={"x": 1},
    )
    assert m.experiment_id == "e1"
    assert m.config_hash == config_hash({"x": 1})
    assert m.python_version
    assert m.host
