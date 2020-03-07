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
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    // The shard manager is an interface for mutating, stopping, restarting, and
    // retrieving information about shards.
    let data = ctx.data.read();

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            let _ = msg.channel_id.send_message(&ctx.http, |m| {
                m.content(":x: Cannot retrieve shard information. `MANAGER_NOT_FOUND`");
                m
            });
            return Ok(());
        }
    };

    let manager = shard_manager.lock();
    let runners = manager.runners.lock();

    // Shards are backed by a "shard runner" responsible for processing events
    // over the shard, so we'll get the information about the shard runner for
    // the shard this command was sent over.
    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            let _ = msg.channel_id.send_message(&ctx.http, |m| {
                m.content(":x: Cannot retrieve shard information. `THREAD_NOT_FOUND`");
                m
            });
            return Ok(());
        }
    };

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Shard Latency");
            e.description(&format!(
                "`{:?}ms`",
                runner.latency.and_then(|d| { Some(d.as_millis()) })
            ));
            e
        });
        m
    });

    Ok(())
}
