use pumpkin_plugin_api::common::GameMode;

use super::{Placeholder, PlaceholderContext};

#[derive(Clone, Copy)]
pub enum PlayerPlaceholder {
    Name,
    Uuid,
    Ping,
    World,
    GameMode,
    Group,
    GroupPrefix,
    GroupSuffix,
    GroupWeight,
}

impl PlayerPlaceholder {
    pub fn all() -> [Self; 9] {
        [
            Self::Name,
            Self::Uuid,
            Self::Ping,
            Self::World,
            Self::GameMode,
            Self::Group,
            Self::GroupPrefix,
            Self::GroupSuffix,
            Self::GroupWeight,
        ]
    }

    pub fn identifier(self) -> &'static str {
        match self {
            Self::Name => "player",
            Self::Uuid => "player_uuid",
            Self::Ping => "ping",
            Self::World => "world",
            Self::GameMode => "gamemode",
            Self::Group => "group",
            Self::GroupPrefix => "group_prefix",
            Self::GroupSuffix => "group_suffix",
            Self::GroupWeight => "group_weight",
        }
    }
}

impl Placeholder for PlayerPlaceholder {
    fn resolve(&self, context: &PlaceholderContext<'_>) -> Option<String> {
        match self {
            Self::Name => context
                .player()
                .map(pumpkin_plugin_api::player::Player::get_name),
            Self::Uuid => context.player().map(|player| player.get_id().to_string()),
            Self::Ping => context.player().map(|player| player.get_ping().to_string()),
            Self::World => context.player().map(|player| player.get_world().get_id()),
            Self::GameMode => context
                .player()
                .map(|player| game_mode_name(player.get_gamemode()).to_owned()),
            Self::Group => context.group().map(|group| group.name().to_owned()),
            Self::GroupPrefix => context.group().map(|group| group.prefix().to_owned()),
            Self::GroupSuffix => context.group().map(|group| group.suffix().to_owned()),
            Self::GroupWeight => context.group().map(|group| group.weight().to_string()),
        }
    }
}

fn game_mode_name(mode: GameMode) -> &'static str {
    match mode {
        GameMode::Survival => "Survival",
        GameMode::Creative => "Creative",
        GameMode::Adventure => "Adventure",
        GameMode::Spectator => "Spectator",
    }
}
