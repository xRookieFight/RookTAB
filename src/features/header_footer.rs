use pumpkin_plugin_api::text::TextComponent;

use crate::context::TabContext;
use crate::player::TabPlayer;

use super::TabFeature;

pub struct HeaderFooterFeature;

impl TabFeature for HeaderFooterFeature {
    fn name(&self) -> &'static str {
        "header-footer"
    }

    fn on_refresh(&mut self, context: &TabContext, player: &TabPlayer<'_>) {
        let settings = context.config().header_footer();
        if !player.is_enabled_in(settings.disabled_worlds()) {
            self.on_disable(player);
            return;
        }

        let header = context.lines(settings.header(), player);
        let footer = context.lines(settings.footer(), player);
        player.handle().set_tab_list_header_footer(header, footer);
    }

    fn on_disable(&mut self, player: &TabPlayer<'_>) {
        player
            .handle()
            .set_tab_list_header_footer(TextComponent::text(""), TextComponent::text(""));
    }
}
