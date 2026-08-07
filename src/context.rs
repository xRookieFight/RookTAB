use pumpkin_plugin_api::text::TextComponent;

use crate::config::TabConfig;
use crate::group::GroupResolver;
use crate::placeholder::PlaceholderRegistry;
use crate::player::TabPlayer;
use crate::text;

const LINE_SEPARATOR: &str = "\n";

pub struct TabContext {
    config: TabConfig,
    placeholders: PlaceholderRegistry,
    groups: GroupResolver,
}

impl TabContext {
    pub fn new(config: TabConfig) -> Self {
        let placeholders = PlaceholderRegistry::new(&config);
        let groups = GroupResolver::new(&config);
        Self {
            config,
            placeholders,
            groups,
        }
    }

    pub fn config(&self) -> &TabConfig {
        &self.config
    }

    pub fn groups(&self) -> &GroupResolver {
        &self.groups
    }

    pub fn format(&self, template: &str, player: &TabPlayer<'_>) -> String {
        self.placeholders
            .format(template, &player.placeholder_context())
    }

    pub fn component(&self, template: &str, player: &TabPlayer<'_>) -> TextComponent {
        text::parse(&self.format(template, player))
    }

    pub fn lines(&self, templates: &[String], player: &TabPlayer<'_>) -> TextComponent {
        let joined = templates
            .iter()
            .map(|template| self.format(template, player))
            .collect::<Vec<_>>()
            .join(LINE_SEPARATOR);
        text::parse(&joined)
    }
}
