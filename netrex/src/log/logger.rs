use super::Loggable;

pub struct Logger {
	name: Option<String>
}

impl Logger {
	pub fn new(name: String) -> Self {
		Self {
			name: Some(name)
		}
	}

	pub(crate) fn do_log(&self, message: &str) {
		match &self.name {
			Some(name) => {
				println!("{}: {}", name, message);
			},
			None => {
				println!("{}", message);
			}
		};
	}
}

impl Default for Logger {
	fn default() -> Self {
		Self {
			name: None
		}
	}
}

impl Loggable for Logger {
	fn log(&self, message: &str) {
		self.do_log(message);
	}
}