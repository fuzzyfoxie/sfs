extern crate chrono;
extern crate fern;
extern crate log;
extern crate serenity;
extern crate url;

use log::{error, info, warn};
use serenity::client::Client;
use serenity::framework::standard::StandardFramework;
use serenity::prelude::EventHandler;

use std::sync::Arc;

mod modules;
mod utils;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    utils::logging::setup_logging();
    info!("SFS v{} - written by Skye.", env!("CARGO_PKG_VERSION"));

    // Login with a bot token from the environment
    let mut client = Client::new(
        &"NTQxMDI0MzY4NTc1MzgxNTA0.XmQIfg.xOp2B9BHRE9ZpxxCh2KncV0kQ40",
        Handler,
    )
    .expect("Error creating client");

    {
        let mut data = client.data.write();
        data.insert::<utils::data::ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("~"))
            .group(&modules::util::UTIL_GROUP)
            .before(|ctx, msg, command_name| {
                info!(
                    "{} ({}) => {}",
                    msg.author.tag(),
                    msg.author.id,
                    command_name
                );
                true
            }),
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        error!("Could not start SFS: {:?}", why);
    }
}
