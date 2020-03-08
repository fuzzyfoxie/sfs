use serenity::client::bridge::gateway::ShardId;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use super::super::utils::data::ShardManagerContainer;

#[group]
#[commands(ping)]
struct Util;

#[command]
pub fn kick(ctx: &mut Context, msg: &Message) -> CommandResult {
    Ok(())
}
