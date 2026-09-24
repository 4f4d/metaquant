"""Property test — config_hash is deterministic and order-independent for sorted keys."""

from hypothesis import given
from hypothesis import strategies as st
from meta_quant.experiments.reproducibility import config_hash


@given(st.dictionaries(st.text(min_size=1, max_size=5), st.integers(), max_size=5))
def test_hash_deterministic(cfg: dict) -> None:
    assert config_hash(cfg) == config_hash(cfg)
