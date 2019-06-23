use serenity::{
    framework::standard::{
        Args, CommandResult,
        macros::{command, group},
    },
    model::channel::Message,
    prelude::*,
};

group!({
    name: "inspector",
    options: {},
    commands: [item, spell, unit, site, merc, event],
});

#[command]
#[bucket = "bucket"]
fn item(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let response = search_url("item", &args);
    message.reply(context, &response)?;
    Ok(())
}

#[command]
#[bucket = "bucket"]
fn spell(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let response = search_url("spell", &args);
    message.reply(context, &response)?;
    Ok(())
}

#[command]
#[bucket = "bucket"]
fn unit(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let response = search_url("unit", &args);
    message.reply(context, &response)?;
    Ok(())
}

#[command]
#[bucket = "bucket"]
fn site(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let response = search_url("site", &args);
    message.reply(context, &response)?;
    Ok(())
}

#[command]
#[bucket = "bucket"]
fn merc(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let response = search_url("merc", &args);
    message.reply(context, &response)?;
    Ok(())
}

#[command]
#[bucket = "bucket"]
fn event(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let response = search_url("event", &args);
    message.reply(context, &format!("{}&loadEvents=1", response))?;
    Ok(())
}

fn search_url(search_type: &str, search_args: &Args) -> String {
    let search_term = percent_encoding::utf8_percent_encode(
        search_args.rest(),
        percent_encoding::QUERY_ENCODE_SET
    );
    format!(
        "https://larzm42.github.io/dom5inspector/?page={}&{}q={}&showmodcmds=1&showmoddinginfo=1&showids=1",
        search_type, search_type, search_term
    )
}

