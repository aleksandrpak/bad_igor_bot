extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate log;
extern crate simple_logger;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use log::{info, Level};

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let mut core = Core::new().unwrap();

    let token = env::var("IGOR_TOKEN").unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();

    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Sticker {ref data} = message.kind {
                if let Some(set_name) = &data.set_name {
                    if set_name == "igoryambro" {
                        info!("{:?} has sent sticker", message.from.username);
                        api.spawn(message.delete())
                    }
                }
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}
