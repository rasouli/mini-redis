//! Subscribe to a redis channel example.
//!
//! A simple client that connects to a mini-redis server, subscribes to "foo" and "bar" channels
//! and awaits messages published on those channels
//!
//! You can test this out by running:
//!
//!     cargo run --bin server
//!
//! Then in another terminal run:
//!
//!     cargo run --example sub
//!
//! And then in another terminal run:
//!
//!     cargo run --example pub

#![warn(rust_2018_idioms)]

use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut subscriber = client::connect("127.0.0.1:6379").await?
        .into_subscriber();

    // subscribe to channel foo
    subscriber.subscribe(vec!["foo".into()]).await?;

    // await messages on channel foo
    let msg = subscriber.next_message().await? ;
    println!("got message from the channel: {}; message = {:?}", msg.channel, msg.content);

    Ok(())
}
