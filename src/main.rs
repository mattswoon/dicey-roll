use serenity::{
    prelude::*,
    model::{channel::Message, gateway::Ready},
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        match msg.content.split(" ").next() {
            Some("!roll") => println!("Gonna roll some dice"),
            _ => ()
        }
    }
//
//    fn ready(&self, _: Context, ready: Ready) {
//        println!("{} is connected", reader.user.name);
//    }
}

fn main() {
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Couldn't get $DISCORD_TOKEN");

    let mut client = Client::new(&token, Handler)
        .expect("Couldn't create client");

    client.start().expect("Couldn't start client");
}
