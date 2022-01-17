use std::io::Cursor;
pub mod offline;
pub mod online;

pub struct Packet {
    pub stream: Cursor<Vec<u8>>,
    pub id: u16,
}

#[derive(Clone, Debug)]
pub struct Motd {
    pub name: String,
    pub protocol: u16,
    pub version: String,
    pub player_count: u16,
    pub player_max: u16,
    pub gamemode: String,
    pub server_guid: u64,
    pub port: String,
}

impl Motd {
    pub fn new(server_guid: u64, port: String) -> Self {
        Self {
            name: String::from("Netrex Server"),
            player_count: 10,
            player_max: 100,
            protocol: 448,
            gamemode: String::from("Creative"),
            version: String::from("1.17.10"),
            server_guid,
            port,
        }
    }

    pub fn encode(&self) -> String {
        let mut parsed = String::new();
        let prot = self.protocol.to_string();
        let pcount = self.player_count.to_string();
        let pmax = self.player_max.to_string();
        let server_id = self.server_guid.to_string();
        let props = vec![
            "MCPE",
            self.name.as_str(),
            prot.as_str(),
            self.version.as_str(),
            pcount.as_str(),
            pmax.as_str(),
            server_id.as_str(),
            "Netrex",
            self.gamemode.as_str(),
            "1",
            self.port.as_str(),
            "19133",
        ];

        for prop in props.iter() {
            parsed.push_str(prop);
            parsed.push(';');
        }

        parsed
    }
}
