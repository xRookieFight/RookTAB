use pumpkin_plugin_api::player::Player;

use crate::config::{GroupConfig, TabConfig};

#[derive(Debug, Clone)]
pub struct ResolvedGroup {
    name: String,
    weight: i32,
    prefix: String,
    suffix: String,
}

impl ResolvedGroup {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn weight(&self) -> i32 {
        self.weight
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn suffix(&self) -> &str {
        &self.suffix
    }
}

impl From<&GroupConfig> for ResolvedGroup {
    fn from(group: &GroupConfig) -> Self {
        Self {
            name: group.name().to_owned(),
            weight: group.weight(),
            prefix: group.prefix().to_owned(),
            suffix: group.suffix().to_owned(),
        }
    }
}

pub struct GroupResolver {
    candidates: Vec<(String, ResolvedGroup)>,
    fallback: ResolvedGroup,
}

impl GroupResolver {
    pub fn new(config: &TabConfig) -> Self {
        let mut candidates: Vec<(String, ResolvedGroup)> = config
            .groups()
            .iter()
            .map(|group| (group.permission(), ResolvedGroup::from(group)))
            .collect();
        candidates.sort_by_key(|(_, group)| std::cmp::Reverse(group.weight()));

        Self {
            candidates,
            fallback: ResolvedGroup::from(config.default_group()),
        }
    }

    pub fn resolve(&self, player: &Player) -> ResolvedGroup {
        self.candidates
            .iter()
            .find(|(permission, _)| player.has_permission(permission))
            .map_or_else(|| self.fallback.clone(), |(_, group)| group.clone())
    }
}
