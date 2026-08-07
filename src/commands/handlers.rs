use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::command::{CommandError, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::commands::CommandHandler;

use crate::config::ConfigLoader;
use crate::text;
use crate::{DATA_FOLDER, state};

use super::{failed, service};

const USAGES: [(&str, &str); 3] = [
    ("/rooktab help", "Shows this help"),
    ("/rooktab info", "Shows the active features and interval"),
    ("/rooktab reload", "Reloads the configuration from disk"),
];

pub struct HelpHandler;

impl CommandHandler for HelpHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        text::send_info(
            &sender,
            &format!(
                "RookTAB {} - aliases: /rtab, /tablist",
                env!("CARGO_PKG_VERSION")
            ),
        );

        text::send(&sender, text::heading("Commands"));
        for (command, description) in USAGES {
            text::send(&sender, text::usage(command, description));
        }
        Ok(1)
    }
}

pub struct InfoHandler;

impl CommandHandler for InfoHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let (features, interval) = service(|tab| {
            (
                tab.feature_names().join(", "),
                tab.refresh_interval_ticks().to_string(),
            )
        })?;

        text::send(&sender, text::heading("RookTAB"));
        text::send(&sender, text::entry("Features", &features));
        text::send(
            &sender,
            text::entry("Refresh", &format!("{interval} ticks")),
        );
        text::send(
            &sender,
            text::entry("Players", &server.get_player_count().to_string()),
        );
        Ok(1)
    }
}

pub struct ReloadHandler;

impl CommandHandler for ReloadHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let config = ConfigLoader::new(DATA_FOLDER).load().map_err(failed)?;

        state::with_service(|tab| tab.reload(&server, config))
            .ok_or_else(|| failed("RookTAB is not initialised."))?;

        text::send_success(&sender, "Reloaded the configuration from disk.");
        Ok(1)
    }
}
