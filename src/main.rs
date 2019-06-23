mod config;
use crate::config::Config;
mod error;
use crate::error::Error;
use log::*;

mod commands;
use commands::inspector::INSPECTOR_GROUP;

use serenity::prelude::*;
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::framework::standard::CommandError;

struct Handler;
impl EventHandler for Handler {}

fn main() -> Result<(), Error> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("samog_bot=info")).init();

    let config = Config::from_env()?;

    let mut client = Client::new(&config.discord_api_token, Handler)?;
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!"))
            .after(|context, message, _, result|
                handle_message_error(context, message, result))
            .bucket("bucket", |b| b.delay(1))
            .group(&INSPECTOR_GROUP)
    );
    info!("Starting");
    client.start()?;
    Err("Client crashed!")?
}

fn handle_message_error(
    context: &mut Context,
    message: &Message,
    result: Result<(), CommandError>) {
    if let Err(err) = result {
        let error_text = format!("ERROR: {}", err.0);
        error!("{}", error_text);
        if let Err(reply_error) = message.reply(context, &error_text) {
            error!("Failed when replying an error: {}", reply_error);
        }
    }
}
