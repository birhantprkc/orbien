mod outcome;
mod plan;

pub use outcome::{ReloadLevel, ReloadOutcome};
pub use plan::{
    empty_outcome, outcome_from_plan, outcome_level, plan_reload, ReloadPlan, TunnelChanges,
};
