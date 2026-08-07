use crate::context::TabContext;
use crate::player::TabPlayer;

use super::TabFeature;

const DEFAULT_ORDER: i32 = 0;

pub struct SortingFeature;

impl TabFeature for SortingFeature {
    fn name(&self) -> &'static str {
        "sorting"
    }

    fn on_refresh(&mut self, context: &TabContext, player: &TabPlayer<'_>) {
        let weight = player.group().weight();
        let order = if context.config().sorting().reversed() {
            weight.saturating_neg()
        } else {
            weight
        };
        player.handle().set_tab_list_order(order);
    }

    fn on_disable(&mut self, player: &TabPlayer<'_>) {
        player.handle().set_tab_list_order(DEFAULT_ORDER);
    }
}
