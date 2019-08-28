extern crate futures;
extern crate telegram_bot;
extern crate tokio;
extern crate log;
extern crate simple_logger;

use std::env;

use telegram_bot::*;
use crate::tokio::prelude::*;
use log::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(Level::Info).unwrap();

    let token = env::var("IGOR_TOKEN").expect("IGOR_TOKEN not set");
    let api = Api::new(token);

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Sticker {ref data} = message.kind {
                if let Some(set_name) = &data.set_name {
                    if set_name == "igoryambro" {
                        info!("{:?} has sent sticker", message.from.username);
                        api.send(message.delete()).await?;
                    }
                }
            }
        }
    }

    Ok(())
}
