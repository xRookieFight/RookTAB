use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::PlayerLeaveEvent;
use pumpkin_plugin_api::events_wit::PlayerLeaveEventData;

use crate::state;

pub struct LeaveListener;

impl EventHandler<PlayerLeaveEvent> for LeaveListener {
    fn handle(&self, server: Server, event: PlayerLeaveEventData) -> PlayerLeaveEventData {
        state::with_service(|service| service.refresh_all(&server));
        event
    }
}
