use std::str::FromStr;
use serenity::{
    prelude::*,
    model::channel::Message,
    framework::standard::macros::{command, group},
    framework::standard::{StandardFramework, CommandResult, CommandError},
};
use rand::thread_rng;
use dicey_roll::dice::{DiceTuple};

#[command]
fn roll(ctx: &mut Context, msg: &Message) -> CommandResult {
    println!("Gonna roll bby");
    msg.react(&ctx, "👍")?;
    let rng = thread_rng();
    let rolls = msg.content.split(" ")
        .skip(1)
        .map(|s| DiceTuple::from_str(s.trim()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| CommandError(format!("{}", e)))?
        .iter()
        .map(|dt| { 
            let rolls = (0..dt.count)
                .map(|_| format!("{}", dt.dice.roll(rng)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{} {} = {}", dt.count, dt.dice, rolls) })
        .collect::<Vec<_>>()
        .join("\n");

    msg.reply(&ctx, rolls)?;
    Ok(())
}

#[group]
#[commands(roll)]
struct General;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Couldn't get $DISCORD_TOKEN");

    let mut client = Client::new(&token, Handler)
        .expect("Couldn't create client");

    client.with_framework(StandardFramework::new()
                          .configure(|c| c.prefix("!"))
                          .group(&GENERAL_GROUP));

    client.start().expect("Couldn't start client");
}
