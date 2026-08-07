use std::sync::Arc;

use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::player::Player;

use crate::config::TabConfig;
use crate::context::TabContext;
use crate::features::{FeatureFactory, TabFeature};
use crate::player::TabPlayer;

pub struct TabService {
    context: Arc<TabContext>,
    features: Vec<Box<dyn TabFeature>>,
    tick: u64,
}

impl TabService {
    pub fn new(config: TabConfig) -> Self {
        let context = Arc::new(TabContext::new(config));
        let features = FeatureFactory::build(&context);
        Self {
            context,
            features,
            tick: 0,
        }
    }

    pub fn refresh_interval_ticks(&self) -> u64 {
        self.context.config().refresh_interval_ticks()
    }

    pub fn feature_names(&self) -> Vec<&'static str> {
        self.features.iter().map(|feature| feature.name()).collect()
    }

    pub fn handle_join(&mut self, server: &Server, player: &Player) {
        let context = Arc::clone(&self.context);
        let tab_player = self.wrap(server, player, player.get_world().get_id());
        for feature in &mut self.features {
            feature.on_join(&context, &tab_player);
        }
    }

    pub fn handle_world_change(&mut self, server: &Server, player: &Player, world: String) {
        let context = Arc::clone(&self.context);
        let tab_player = self.wrap(server, player, world);
        for feature in &mut self.features {
            feature.on_world_change(&context, &tab_player);
        }
    }

    pub fn tick(&mut self, server: &Server) {
        self.tick = self.tick.wrapping_add(self.refresh_interval_ticks());
        self.refresh_all(server);
    }

    pub fn refresh_all(&mut self, server: &Server) {
        let context = Arc::clone(&self.context);
        for player in server.get_all_players() {
            let tab_player = self.wrap(server, &player, player.get_world().get_id());
            for feature in &mut self.features {
                feature.on_refresh(&context, &tab_player);
            }
        }
    }

    pub fn reload(&mut self, server: &Server, config: TabConfig) {
        self.disable(server);
        self.context = Arc::new(TabContext::new(config));
        self.features = FeatureFactory::build(&self.context);
        self.refresh_all(server);
    }

    pub fn disable(&mut self, server: &Server) {
        for player in server.get_all_players() {
            let tab_player = self.wrap(server, &player, player.get_world().get_id());
            for feature in &mut self.features {
                feature.on_disable(&tab_player);
            }
        }
    }

    fn wrap<'a>(&self, server: &'a Server, player: &'a Player, world: String) -> TabPlayer<'a> {
        let group = self.context.groups().resolve(player);
        TabPlayer::new(server, player, world, group, self.tick)
    }
}
