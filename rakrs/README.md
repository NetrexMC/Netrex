# RakNet

A fully functional RakNet implementation in rust.

## Installation

> :warning: This library uses rust `nightly-2021`. 
> 
> It is not recommended to setup nightly as your default toolchain if you are in a production environment.



#### Starting a RakNet Server

```rust
use rakrs::Motd;
use rakrs::RakEvent;
use rakrs::RakNetServer;
use rakrs::RakResult;
use rakrs::start;
#[tokio::main]
async fn main() {
    let server = RakNetServer::new(String::from("0.0.0.0:19132"));
    let channel = netrex_events::Channel::<RakEvent, RakResult>::new();
    let mut unknown = 0;
    let mut listener = |event, result| {
        match event {
            RakEvent::ConnectionCreated(_) => {
                println!("Client connected");
            }
            RakEvent::Disconnect(_, _) => {
                println!("Client disconnected");
            }
            _ => {
                unknown += 1;
                println!("Unknown events: {}", unknown);
            }
        };
        None
    };
    channel.receive(&mut listener);
    // Start the raknet server!    
    start(server, channel).await;
}
```
