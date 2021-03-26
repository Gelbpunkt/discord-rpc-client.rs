extern crate discord_rpc_client;
extern crate simplelog;

use discord_rpc_client::{models::Event, Client as DiscordRPC};
use simplelog::*;
use std::{thread, time};

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    let mut drpc = DiscordRPC::new(425407036495495169);

    drpc.start();

    drpc.subscribe(Event::ActivityJoin, |j| j.secret("123456"))
        .expect("Failed to subscribe to event");

    drpc.subscribe(Event::ActivitySpectate, |s| s.secret("123456"))
        .expect("Failed to subscribe to event");

    drpc.subscribe(Event::ActivityJoinRequest, |s| s)
        .expect("Failed to subscribe to event");

    drpc.unsubscribe(Event::ActivityJoinRequest, |j| j)
        .expect("Failed to unsubscribe from event");

    loop {
        thread::sleep(time::Duration::from_millis(500));
    }
}
