mod commands;
mod config;
mod context;
mod features;
mod group;
mod listeners;
mod permissions;
mod placeholder;
mod player;
mod service;
mod state;
mod text;

use pumpkin_plugin_api::permissions as host_permissions;
use pumpkin_plugin_api::scheduler::SchedulerExt;
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata, Result, register_plugin};
use tracing::info;

use crate::config::ConfigLoader;
use crate::service::TabService;

pub(crate) const DATA_FOLDER: &str = "data";

struct RookTab;

impl Plugin for RookTab {
    fn new() -> Self {
        Self
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "RookTAB".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["xRookieFight".into()],
            description:
                "Tab list header, footer, name formatting and sorting plugin for PumpkinMC.".into(),
            dependencies: vec![],
            permissions: vec![
                host_permissions::FS_READ_DATA.into(),
                host_permissions::FS_WRITE_DATA.into(),
            ],
        }
    }

    fn on_load(&mut self, context: Context) -> Result<()> {
        let config = ConfigLoader::new(DATA_FOLDER).load()?;
        permissions::register(&context, &config)?;

        let service = TabService::new(config);
        let interval = service.refresh_interval_ticks();
        let features = service.feature_names().join(", ");
        state::install(service);

        listeners::register(&context)?;
        commands::register(&context);

        context.schedule_repeating_task(interval, interval, |server| {
            state::with_service(|service| service.tick(&server));
        });

        info!("RookTAB loaded with features: {features} (every {interval} ticks).");
        Ok(())
    }

    fn on_unload(&mut self, context: Context) -> Result<()> {
        let server = context.get_server();
        state::with_service(|service| service.disable(&server));
        state::clear();
        Ok(())
    }
}

register_plugin!(RookTab);
