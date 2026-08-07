use crate::context::TabContext;
use crate::player::TabPlayer;

use super::TabFeature;

pub struct TabListNameFeature;

impl TabFeature for TabListNameFeature {
    fn name(&self) -> &'static str {
        "tab-list-name"
    }

    fn on_refresh(&mut self, context: &TabContext, player: &TabPlayer<'_>) {
        let settings = context.config().tab_list();
        if !player.is_enabled_in(settings.disabled_worlds()) {
            self.on_disable(player);
            return;
        }

        let name = context.component(settings.format(), player);
        player.handle().set_tab_list_name(Some(name));
    }

    fn on_disable(&mut self, player: &TabPlayer<'_>) {
        player.handle().set_tab_list_name(None);
    }
}
