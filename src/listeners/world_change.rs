use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::events::EventHandler;
use pumpkin_plugin_api::events::player::PlayerChangeWorldEvent;
use pumpkin_plugin_api::events_wit::PlayerChangeWorldEventData;

pub struct WorldChangeListener;

impl EventHandler<PlayerChangeWorldEvent> for WorldChangeListener {
    fn handle(
        &self,
        server: Server,
        event: PlayerChangeWorldEventData,
    ) -> PlayerChangeWorldEventData {
        super::synchronize_world(&server, &event.player, event.new_world.get_id());
        event
    }
}
