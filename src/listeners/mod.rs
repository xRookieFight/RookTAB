mod join;
mod leave;
mod world_change;

use pumpkin_plugin_api::events::EventPriority;
use pumpkin_plugin_api::events::player::{
    PlayerChangeWorldEvent, PlayerJoinEvent, PlayerLeaveEvent,
};
use pumpkin_plugin_api::player::Player;
use pumpkin_plugin_api::{Context, Result, Server};

use crate::state;

use join::JoinListener;
use leave::LeaveListener;
use world_change::WorldChangeListener;

pub fn register(context: &Context) -> Result<()> {
    context.register_event_handler::<PlayerJoinEvent, _>(
        JoinListener,
        EventPriority::Highest,
        true,
    )?;
    context.register_event_handler::<PlayerLeaveEvent, _>(
        LeaveListener,
        EventPriority::Highest,
        true,
    )?;
    context.register_event_handler::<PlayerChangeWorldEvent, _>(
        WorldChangeListener,
        EventPriority::Highest,
        true,
    )?;
    Ok(())
}

pub fn synchronize(server: &Server, player: &Player) {
    state::with_service(|service| service.handle_join(server, player));
}

pub fn synchronize_world(server: &Server, player: &Player, world: String) {
    state::with_service(|service| service.handle_world_change(server, player, world));
}
