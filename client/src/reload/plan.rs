use orbien_core::config::{ClientConfig, TunnelConfig};
use std::collections::HashMap;

use super::{ReloadLevel, ReloadOutcome};

#[derive(Debug, Clone, PartialEq)]
pub struct TunnelChanges {
    pub added: Vec<TunnelConfig>,
    pub removed: Vec<String>,
    pub updated: Vec<TunnelConfig>,
}

impl TunnelChanges {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.updated.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReloadPlan {
    Noop,
    Apply {
        changes: TunnelChanges,
        connection_settings_changed: bool,
    },
}

pub fn plan_reload(old: &ClientConfig, new: &ClientConfig) -> ReloadPlan {
    if old == new {
        return ReloadPlan::Noop;
    }
    let changes = diff_tunnels(&old.tunnels, &new.tunnels);
    let connection_settings_changed = !old.connection_settings_eq(new);
    if changes.is_empty() && !connection_settings_changed {
        ReloadPlan::Noop
    } else {
        ReloadPlan::Apply {
            changes,
            connection_settings_changed,
        }
    }
}

pub fn outcome_level(plan: &ReloadPlan) -> ReloadLevel {
    match plan {
        ReloadPlan::Noop => ReloadLevel::Noop,
        ReloadPlan::Apply {
            connection_settings_changed: true,
            ..
        } => ReloadLevel::ReconnectControl,
        ReloadPlan::Apply { .. } => ReloadLevel::TunnelsOnly,
    }
}

pub fn empty_outcome(plan: &ReloadPlan) -> ReloadOutcome {
    ReloadOutcome {
        level: outcome_level(plan),
        connection_settings_changed: matches!(
            plan,
            ReloadPlan::Apply {
                connection_settings_changed: true,
                ..
            }
        ),
        ..Default::default()
    }
}

pub fn outcome_from_plan(plan: &ReloadPlan, changes: &TunnelChanges) -> ReloadOutcome {
    let mut outcome = empty_outcome(plan);
    if matches!(
        plan,
        ReloadPlan::Apply {
            connection_settings_changed: true,
            ..
        }
    ) {
        outcome.level = ReloadLevel::ReconnectControl;
    }
    outcome.added = changes.added.iter().map(|t| t.name.clone()).collect();
    outcome.removed = changes.removed.clone();
    outcome.updated = changes.updated.iter().map(|t| t.name.clone()).collect();
    outcome
}

fn diff_tunnels(old: &[TunnelConfig], new: &[TunnelConfig]) -> TunnelChanges {
    let old_map: HashMap<&str, &TunnelConfig> = old.iter().map(|t| (t.name.as_str(), t)).collect();
    let new_map: HashMap<&str, &TunnelConfig> = new.iter().map(|t| (t.name.as_str(), t)).collect();

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut updated = Vec::new();

    for (name, cfg) in &new_map {
        match old_map.get(name) {
            None => added.push((*cfg).clone()),
            Some(old_cfg) if *old_cfg != *cfg => updated.push((*cfg).clone()),
            Some(_) => {}
        }
    }
    for name in old_map.keys() {
        if !new_map.contains_key(name) {
            removed.push((*name).to_string());
        }
    }

    added.sort_by(|a, b| a.name.cmp(&b.name));
    updated.sort_by(|a, b| a.name.cmp(&b.name));
    removed.sort();

    TunnelChanges {
        added,
        removed,
        updated,
    }
}
