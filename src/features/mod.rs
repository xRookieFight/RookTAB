mod header_footer;
mod sorting;
mod tab_list_name;

use crate::context::TabContext;
use crate::player::TabPlayer;

pub use header_footer::HeaderFooterFeature;
pub use sorting::SortingFeature;
pub use tab_list_name::TabListNameFeature;

pub trait TabFeature: Send + Sync {
    fn name(&self) -> &'static str;

    fn on_join(&mut self, context: &TabContext, player: &TabPlayer<'_>) {
        self.on_refresh(context, player);
    }

    fn on_world_change(&mut self, context: &TabContext, player: &TabPlayer<'_>) {
        self.on_refresh(context, player);
    }

    fn on_refresh(&mut self, context: &TabContext, player: &TabPlayer<'_>);

    fn on_disable(&mut self, player: &TabPlayer<'_>);
}

pub struct FeatureFactory;

impl FeatureFactory {
    pub fn build(context: &TabContext) -> Vec<Box<dyn TabFeature>> {
        let config = context.config();
        let mut features: Vec<Box<dyn TabFeature>> = Vec::new();

        if config.header_footer().enabled() {
            features.push(Box::new(HeaderFooterFeature));
        }
        if config.tab_list().enabled() {
            features.push(Box::new(TabListNameFeature));
        }
        if config.sorting().enabled() {
            features.push(Box::new(SortingFeature));
        }

        features
    }
}
