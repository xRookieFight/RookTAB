mod handlers;

use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::command::{Command, CommandError, CommandNode};

use crate::permissions::COMMAND_MANAGE;
use crate::service::TabService;
use crate::{state, text};

use handlers::{HelpHandler, InfoHandler, ReloadHandler};

pub fn register(context: &Context) {
    let command = Command::new(
        &[
            "rooktab".to_owned(),
            "rtab".to_owned(),
            "tablist".to_owned(),
        ],
        "Manage the RookTAB tab list.",
    );

    let command = command.execute(HelpHandler);

    command.then(CommandNode::literal("help").execute(HelpHandler));
    command.then(CommandNode::literal("info").execute(InfoHandler));
    command.then(CommandNode::literal("reload").execute(ReloadHandler));

    context.register_command(command, COMMAND_MANAGE);
}

pub(crate) fn service<R>(action: impl FnOnce(&mut TabService) -> R) -> Result<R, CommandError> {
    state::with_service(action).ok_or_else(|| failed("RookTAB is not initialised."))
}

pub(crate) fn failed(message: impl AsRef<str>) -> CommandError {
    CommandError::CommandFailed(text::failure(message.as_ref()))
}
