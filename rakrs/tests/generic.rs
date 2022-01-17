use rakrs::start;
use rakrs::RakEvent;
use rakrs::RakNetServer;
use rakrs::RakResult;
use tokio::runtime::Runtime;

#[test]
pub fn test_boot() {
    let server = RakNetServer::new(String::from("0.0.0.0:19136"));
    let channel = netrex_events::Channel::<RakEvent, RakResult>::new();
    let mut unknown = 0;
    let mut listener = |event, _| {
        match event {
            RakEvent::ConnectionCreated(address) => {
                println!("[RakNet] [{}] Client connected", address);
            }
            RakEvent::Disconnect(address, reason) => {
                println!(
                    "[RakNet] [{}] Client disconnected due to: {}",
                    address, reason
                );
            }
            RakEvent::Motd(address, mut motd) => {
                println!("[RakNet] [{}] Client requested motd: {:?}", address, motd);
                motd.name = String::from("Bob Ross and painting is my life.");
                return Some(RakResult::Motd(motd));
            }
            _ => {
                unknown += 1;
                println!("Unknown events: {}", unknown);
            }
        };
        None
    };
    channel.receive(&mut listener);

    let rt = Runtime::new().unwrap();
    let handle = rt.handle();
    handle.block_on(async move {
        let v = start(server, channel).await;
        // share your raknet arc here.
        v.0.await;
    });
}
