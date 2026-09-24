"""Reproducibility helper — records experiment metadata (MQ-FR-013)."""

from __future__ import annotations

import hashlib
import json
import platform
import subprocess
from dataclasses import asdict, dataclass
from datetime import UTC, datetime


@dataclass(frozen=True)
class ExperimentMeta:
    experiment_id: str
    git_sha: str | None
    dataset_version: str
    time_range: str
    strategy_version: str
    config_hash: str
    model_version: str | None
    model_hash: str | None
    random_seed: int | None
    train_window: str | None
    valid_window: str | None
    test_window: str | None
    execution_model_version: str | None
    cost_model_version: str | None
    python_version: str
    host: str

    def to_json(self) -> str:
        return json.dumps(asdict(self), indent=2, sort_keys=True)


def git_sha() -> str | None:
    try:
        return subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip()
    except Exception:
        return None


def config_hash(config_dict: dict[str, object]) -> str:
    payload = json.dumps(config_dict, sort_keys=True).encode()
    return hashlib.sha256(payload).hexdigest()[:16]


def default_meta(
    experiment_id: str,
    dataset_version: str,
    time_range: str,
    strategy_version: str,
    config_dict: dict[str, object],
) -> ExperimentMeta:
    return ExperimentMeta(
        experiment_id=experiment_id,
        git_sha=git_sha(),
        dataset_version=dataset_version,
        time_range=time_range,
        strategy_version=strategy_version,
        config_hash=config_hash(config_dict),
        model_version=None,
        model_hash=None,
        random_seed=None,
        train_window=None,
        valid_window=None,
        test_window=None,
        execution_model_version=None,
        cost_model_version=None,
        python_version=platform.python_version(),
        host=platform.node(),
    )


def utc_now_iso() -> str:
    return datetime.now(UTC).isoformat()
