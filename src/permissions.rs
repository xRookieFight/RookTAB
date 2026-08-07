use pumpkin_plugin_api::permission::{Permission, PermissionDefault, PermissionLevel};
use pumpkin_plugin_api::{Context, Result};

use crate::config::TabConfig;

pub const COMMAND_MANAGE: &str = "RookTAB:command.manage";

pub fn register(context: &Context, config: &TabConfig) -> Result<()> {
    context.register_permission(&Permission {
        node: COMMAND_MANAGE.to_owned(),
        description: "Allows managing the RookTAB configuration.".to_owned(),
        default: PermissionDefault::Op(PermissionLevel::Three),
        children: Vec::new(),
    })?;

    for group in config.groups() {
        context.register_permission(&Permission {
            node: group.permission(),
            description: format!("Assigns the '{}' tab list group.", group.name()),
            default: PermissionDefault::Op(PermissionLevel::Four),
            children: Vec::new(),
        })?;
    }

    Ok(())
}
