use super::{Placeholder, PlaceholderContext};

const MAX_TPS: f64 = 20.0;

#[derive(Clone, Copy)]
pub enum ServerPlaceholder {
    Online,
    MaxPlayers,
    Tps,
    Mspt,
    Motd,
}

impl ServerPlaceholder {
    pub fn all() -> [Self; 5] {
        [
            Self::Online,
            Self::MaxPlayers,
            Self::Tps,
            Self::Mspt,
            Self::Motd,
        ]
    }

    pub fn identifier(self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::MaxPlayers => "max_players",
            Self::Tps => "tps",
            Self::Mspt => "mspt",
            Self::Motd => "motd",
        }
    }
}

impl Placeholder for ServerPlaceholder {
    fn resolve(&self, context: &PlaceholderContext<'_>) -> Option<String> {
        let server = context.server();
        let value = match self {
            Self::Online => server.get_player_count().to_string(),
            Self::MaxPlayers => server.get_max_players().to_string(),
            Self::Tps => format!("{:.2}", server.get_tps().min(MAX_TPS)),
            Self::Mspt => format!("{:.2}", server.get_mspt()),
            Self::Motd => server.get_motd(),
        };
        Some(value)
    }
}
