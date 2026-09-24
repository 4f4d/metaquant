//! mq-risk — deterministic risk engine, authoritative over all model outputs (ADR-006).

use mq_core::{Opportunity, RiskDecision, TimestampNs};

#[derive(Debug, Clone)]
pub struct RiskLimits {
    pub max_position: i64,
    pub max_gross_exposure: i64,
    pub max_order_notional_ticks: i64,
    pub kill_switch: bool,
    pub shock_restrict: bool,
}

impl Default for RiskLimits {
    fn default() -> Self {
        Self {
            max_position: 10000,
            max_gross_exposure: 1_000_000,
            max_order_notional_ticks: 500_000,
            kill_switch: false,
            shock_restrict: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AccountState {
    pub position: i64,
    pub gross_exposure: i64,
}

/// Deterministic risk engine — no model may override (ADR-006, RISK_SPEC).
pub struct RiskEngine {
    pub limits: RiskLimits,
}

impl RiskEngine {
    pub fn new(limits: RiskLimits) -> Self {
        Self { limits }
    }

    pub fn evaluate(
        &self,
        opp: &Opportunity,
        account: &AccountState,
        now_ns: TimestampNs,
    ) -> RiskDecision {
        let mut reasons = Vec::new();

        if self.limits.kill_switch {
            reasons.push("KILL_SWITCH".into());
        }
        if self.limits.shock_restrict {
            reasons.push("SHOCK_RESTRICT".into());
        }
        let order_qty: i64 = opp.instrument_legs.iter().map(|l| l.quantity).sum();
        if account.position + order_qty > self.limits.max_position {
            reasons.push("MAX_POSITION".into());
        }
        if account.gross_exposure > self.limits.max_gross_exposure {
            reasons.push("MAX_GROSS_EXPOSURE".into());
        }
        // Simplified notional check (qty * assumed price tick proxy)
        if order_qty * 10000 > self.limits.max_order_notional_ticks {
            reasons.push("MAX_ORDER_NOTIONAL".into());
        }

        let decision = if reasons.is_empty() {
            "Approved"
        } else {
            "Rejected"
        };
        RiskDecision {
            decision: decision.into(),
            reason_codes: reasons,
            timestamp_ns: now_ns,
        }
    }
}

impl Default for RiskEngine {
    fn default() -> Self {
        Self::new(RiskLimits::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mq_core::{Leg, Opportunity};
    use rust_decimal::Decimal;
    use serde_json::json;

    fn opp(qty: i64) -> Opportunity {
        Opportunity {
            opportunity_id: "o1".into(),
            strategy_id: "basis".into(),
            detected_at_ns: 1,
            instrument_legs: vec![Leg {
                instrument_id: "RELIANCE".into(),
                side: "Buy".into(),
                quantity: qty,
            }],
            gross_edge_bps: Decimal::new(10, 2),
            config_hash: "abc".into(),
            metadata: json!({}),
        }
    }

    #[test]
    fn approves_within_limits() {
        let engine = RiskEngine::default();
        let d = engine.evaluate(
            &opp(10),
            &AccountState {
                position: 0,
                gross_exposure: 0,
            },
            1,
        );
        assert_eq!(d.decision, "Approved");
    }

    #[test]
    fn rejects_on_kill_switch() {
        let mut limits = RiskLimits::default();
        limits.kill_switch = true;
        let engine = RiskEngine::new(limits);
        let d = engine.evaluate(
            &opp(1),
            &AccountState {
                position: 0,
                gross_exposure: 0,
            },
            1,
        );
        assert_eq!(d.decision, "Rejected");
        assert!(d.reason_codes.contains(&"KILL_SWITCH".to_string()));
    }

    #[test]
    fn rejects_on_position_limit() {
        let engine = RiskEngine::default();
        let d = engine.evaluate(
            &opp(20000),
            &AccountState {
                position: 0,
                gross_exposure: 0,
            },
            1,
        );
        assert_eq!(d.decision, "Rejected");
        assert!(d.reason_codes.contains(&"MAX_POSITION".to_string()));
    }

    #[test]
    fn model_confidence_cannot_override() {
        // Even with "high confidence" metadata, risk still vetoes.
        let mut o = opp(1);
        o.metadata = json!({"model_confidence": 0.99});
        let mut limits = RiskLimits::default();
        limits.kill_switch = true;
        let engine = RiskEngine::new(limits);
        let d = engine.evaluate(
            &o,
            &AccountState {
                position: 0,
                gross_exposure: 0,
            },
            1,
        );
        assert_eq!(
            d.decision, "Rejected",
            "model confidence must not bypass risk"
        );
    }
}
