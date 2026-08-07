use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::player::Player;

use crate::group::ResolvedGroup;
use crate::placeholder::PlaceholderContext;

pub struct TabPlayer<'a> {
    server: &'a Server,
    handle: &'a Player,
    world: String,
    group: ResolvedGroup,
    tick: u64,
}

impl<'a> TabPlayer<'a> {
    pub fn new(
        server: &'a Server,
        handle: &'a Player,
        world: String,
        group: ResolvedGroup,
        tick: u64,
    ) -> Self {
        Self {
            server,
            handle,
            world,
            group,
            tick,
        }
    }

    pub fn handle(&self) -> &Player {
        self.handle
    }

    pub fn group(&self) -> &ResolvedGroup {
        &self.group
    }

    pub fn is_enabled_in(&self, disabled_worlds: &[String]) -> bool {
        !disabled_worlds
            .iter()
            .any(|world| world.eq_ignore_ascii_case(&self.world))
    }

    pub fn placeholder_context(&self) -> PlaceholderContext<'_> {
        PlaceholderContext::new(self.server, Some(self.handle), Some(&self.group), self.tick)
    }
}
