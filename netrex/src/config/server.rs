use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
	v4_port: u16,
	name: String,
}