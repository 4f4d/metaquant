// SPDX-License-Identifier: Apache-2.0
//! mq-strategies — common strategy contract + placeholder family modules.
//! Horizon is a configuration property (ADR-009); CAS is a regime, not a strategy (ADR-010).

use mq_core::{MarketEvent, Opportunity};

/// Strategy horizon — config-driven, not hard-coded global.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Horizon {
    Intraday,
    ShortTerm,
    MediumTerm,
}

#[derive(Debug, Clone)]
pub struct StrategyConfig {
    pub strategy_id: String,
    pub horizon: Horizon,
    pub params: serde_json::Value,
    pub config_hash: String,
}

/// Common strategy contract (INTERFACES sec 3, STRATEGY_SPEC sec 1).
pub trait Strategy {
    fn initialize(&mut self, config: StrategyConfig) -> anyhow::Result<()>;
    fn on_event(&mut self, event: &MarketEvent) -> Vec<Opportunity>;
    fn name(&self) -> &str;
}

/// Placeholder basis strategy — real logic is Phase 3 vertical slice.
pub struct BasisStrategy {
    config: Option<StrategyConfig>,
}

impl BasisStrategy {
    pub fn new() -> Self {
        Self { config: None }
    }
}

impl Default for BasisStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for BasisStrategy {
    fn initialize(&mut self, config: StrategyConfig) -> anyhow::Result<()> {
        anyhow::ensure!(config.strategy_id == "basis", "expected basis strategy_id");
        self.config = Some(config);
        Ok(())
    }

    fn on_event(&mut self, _event: &MarketEvent) -> Vec<Opportunity> {
        // Phase 3 will implement fair-value vs observed basis.
        vec![]
    }

    fn name(&self) -> &str {
        "basis"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn basis_initializes_with_correct_id() {
        let mut s = BasisStrategy::new();
        let cfg = StrategyConfig {
            strategy_id: "basis".into(),
            horizon: Horizon::Intraday,
            params: json!({}),
            config_hash: "abc".into(),
        };
        assert!(s.initialize(cfg).is_ok());
    }

    #[test]
    fn basis_rejects_wrong_id() {
        let mut s = BasisStrategy::new();
        let cfg = StrategyConfig {
            strategy_id: "stat_arb".into(),
            horizon: Horizon::Intraday,
            params: json!({}),
            config_hash: "abc".into(),
        };
        assert!(s.initialize(cfg).is_err());
    }

    #[test]
    fn horizon_is_config_property() {
        let cfg = StrategyConfig {
            strategy_id: "basis".into(),
            horizon: Horizon::MediumTerm,
            params: json!({}),
            config_hash: "h".into(),
        };
        assert_eq!(cfg.horizon, Horizon::MediumTerm);
    }
}
