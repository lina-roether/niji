use log::{Level, LevelFilter};

use crate::api;

pub(crate) struct Logger {
	level: LevelFilter,
}

impl Logger {
	pub fn new(level: LevelFilter) -> Self {
		Self { level }
	}
}

impl log::Log for Logger {
	fn enabled(&self, metadata: &log::Metadata) -> bool {
		self.level >= metadata.level()
	}

	fn log(&self, record: &log::Record) {
		match record.level() {
			Level::Error => api::log_error(record.args()).unwrap(),
			Level::Warn => api::log_warn(record.args()).unwrap(),
			Level::Info => api::log_info(record.args()).unwrap(),
			Level::Debug => api::log_debug(record.args()).unwrap(),
			Level::Trace => api::log_trace(record.args()).unwrap(),
		}
		self.flush();
	}

	fn flush(&self) {
		api::flush().unwrap();
	}
}
