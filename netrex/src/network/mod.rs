#![allow(dead_code)]
use tokio::sync::Notify;

pub struct Network {
	kill_notifier: Notify
}

impl Network {
	pub fn new() -> Self {
		Self {
			kill_notifier: Notify::new()
		}
	}
}