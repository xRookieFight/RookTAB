use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub const DEFAULT_REFRESH_TICKS: u64 = 20;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TabConfig {
    refresh_interval_ticks: u64,
    header_footer: HeaderFooterConfig,
    tab_list: TabListConfig,
    sorting: SortingConfig,
    default_group: GroupConfig,
    groups: Vec<GroupConfig>,
    animations: BTreeMap<String, AnimationConfig>,
}

impl TabConfig {
    pub fn refresh_interval_ticks(&self) -> u64 {
        self.refresh_interval_ticks.max(1)
    }

    pub fn header_footer(&self) -> &HeaderFooterConfig {
        &self.header_footer
    }

    pub fn tab_list(&self) -> &TabListConfig {
        &self.tab_list
    }

    pub fn sorting(&self) -> &SortingConfig {
        &self.sorting
    }

    pub fn default_group(&self) -> &GroupConfig {
        &self.default_group
    }

    pub fn groups(&self) -> &[GroupConfig] {
        &self.groups
    }

    pub fn animations(&self) -> &BTreeMap<String, AnimationConfig> {
        &self.animations
    }
}

impl Default for TabConfig {
    fn default() -> Self {
        Self {
            refresh_interval_ticks: DEFAULT_REFRESH_TICKS,
            header_footer: HeaderFooterConfig::default(),
            tab_list: TabListConfig::default(),
            sorting: SortingConfig::default(),
            default_group: GroupConfig::default(),
            groups: vec![
                GroupConfig::new("admin", 100, "&c[Admin] &f", ""),
                GroupConfig::new("moderator", 50, "&9[Mod] &f", ""),
            ],
            animations: BTreeMap::from([("logo".to_owned(), AnimationConfig::default())]),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct HeaderFooterConfig {
    enabled: bool,
    header: Vec<String>,
    footer: Vec<String>,
    disabled_worlds: Vec<String>,
}

impl HeaderFooterConfig {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn header(&self) -> &[String] {
        &self.header
    }

    pub fn footer(&self) -> &[String] {
        &self.footer
    }

    pub fn disabled_worlds(&self) -> &[String] {
        &self.disabled_worlds
    }
}

impl Default for HeaderFooterConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            header: vec![
                "%animation:logo%".to_owned(),
                "&7Welcome &f%player%".to_owned(),
            ],
            footer: vec![
                "&7Players: &f%online%&7/&f%max_players%".to_owned(),
                "&7TPS: &f%tps% &8| &7Ping: &f%ping%ms".to_owned(),
            ],
            disabled_worlds: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TabListConfig {
    enabled: bool,
    format: String,
    disabled_worlds: Vec<String>,
}

impl TabListConfig {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn format(&self) -> &str {
        &self.format
    }

    pub fn disabled_worlds(&self) -> &[String] {
        &self.disabled_worlds
    }
}

impl Default for TabListConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            format: "%group_prefix%%player%%group_suffix%".to_owned(),
            disabled_worlds: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SortingConfig {
    enabled: bool,
    reversed: bool,
}

impl SortingConfig {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn reversed(&self) -> bool {
        self.reversed
    }
}

impl Default for SortingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            reversed: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GroupConfig {
    name: String,
    permission: Option<String>,
    weight: i32,
    prefix: String,
    suffix: String,
}

impl GroupConfig {
    pub fn new(name: &str, weight: i32, prefix: &str, suffix: &str) -> Self {
        Self {
            name: name.to_owned(),
            permission: None,
            weight,
            prefix: prefix.to_owned(),
            suffix: suffix.to_owned(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn permission(&self) -> String {
        self.permission
            .clone()
            .unwrap_or_else(|| format!("RookTAB:group.{}", self.name.to_lowercase()))
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

impl Default for GroupConfig {
    fn default() -> Self {
        Self {
            name: "default".to_owned(),
            permission: None,
            weight: 0,
            prefix: "&7".to_owned(),
            suffix: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AnimationConfig {
    interval_ticks: u64,
    frames: Vec<String>,
}

impl AnimationConfig {
    pub fn interval_ticks(&self) -> u64 {
        self.interval_ticks.max(1)
    }

    pub fn frames(&self) -> &[String] {
        &self.frames
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            interval_ticks: 10,
            frames: vec!["&b&lRook&f&lTAB".to_owned(), "&3&lRook&7&lTAB".to_owned()],
        }
    }
}
