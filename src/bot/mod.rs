use serenity::client::Context;
use serenity::framework::standard::{
    help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::prelude::*;
use std::collections::HashSet;

pub mod commands;

#[help]
#[individual_command_tip = "To know about a specific command, type ;help {command_name}"]
#[command_not_found_text = "Could not find: `{}`."]
#[strikethrough_commands_tip_in_dm = ""]
#[strikethrough_commands_tip_in_guild = ""]
#[lacking_permissions = "nothing"]
async fn bot_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
