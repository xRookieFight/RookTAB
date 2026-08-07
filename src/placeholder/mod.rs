mod animation;
mod player;
mod server;

use std::collections::BTreeMap;

use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::player::Player;

use crate::config::TabConfig;
use crate::group::ResolvedGroup;

pub use animation::AnimationPlaceholder;
pub use player::PlayerPlaceholder;
pub use server::ServerPlaceholder;

const MARKER: char = '%';
const ANIMATION_NAMESPACE: &str = "animation:";

pub struct PlaceholderContext<'a> {
    server: &'a Server,
    player: Option<&'a Player>,
    group: Option<&'a ResolvedGroup>,
    tick: u64,
}

impl<'a> PlaceholderContext<'a> {
    pub fn new(
        server: &'a Server,
        player: Option<&'a Player>,
        group: Option<&'a ResolvedGroup>,
        tick: u64,
    ) -> Self {
        Self {
            server,
            player,
            group,
            tick,
        }
    }

    pub fn server(&self) -> &Server {
        self.server
    }

    pub fn player(&self) -> Option<&Player> {
        self.player
    }

    pub fn group(&self) -> Option<&ResolvedGroup> {
        self.group
    }

    pub fn tick(&self) -> u64 {
        self.tick
    }
}

pub trait Placeholder: Send + Sync {
    fn resolve(&self, context: &PlaceholderContext<'_>) -> Option<String>;
}

pub struct PlaceholderRegistry {
    entries: BTreeMap<String, Box<dyn Placeholder>>,
}

impl PlaceholderRegistry {
    pub fn new(config: &TabConfig) -> Self {
        let mut registry = Self {
            entries: BTreeMap::new(),
        };

        for placeholder in ServerPlaceholder::all() {
            registry.register(placeholder.identifier(), Box::new(placeholder));
        }

        for placeholder in PlayerPlaceholder::all() {
            registry.register(placeholder.identifier(), Box::new(placeholder));
        }

        for (name, animation) in config.animations() {
            registry.register(
                format!("{ANIMATION_NAMESPACE}{name}"),
                Box::new(AnimationPlaceholder::new(animation)),
            );
        }

        registry
    }

    pub fn register(&mut self, identifier: impl Into<String>, placeholder: Box<dyn Placeholder>) {
        self.entries.insert(identifier.into(), placeholder);
    }

    pub fn resolve(&self, identifier: &str, context: &PlaceholderContext<'_>) -> Option<String> {
        self.entries
            .get(identifier)
            .and_then(|placeholder| placeholder.resolve(context))
    }

    pub fn format(&self, template: &str, context: &PlaceholderContext<'_>) -> String {
        let mut output = String::with_capacity(template.len());
        let mut rest = template;

        while let Some(start) = rest.find(MARKER) {
            let tail = &rest[start + MARKER.len_utf8()..];
            let Some(end) = tail.find(MARKER) else {
                break;
            };

            let identifier = &tail[..end];
            output.push_str(&rest[..start]);
            match self.resolve(identifier, context) {
                Some(value) => output.push_str(&value),
                None => {
                    output.push(MARKER);
                    output.push_str(identifier);
                    output.push(MARKER);
                }
            }
            rest = &tail[end + MARKER.len_utf8()..];
        }

        output.push_str(rest);
        output
    }
}
