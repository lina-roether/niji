use std::{fmt::Arguments, sync::RwLock};

use crate::console::Console;

static CONSOLE: RwLock<Option<Console>> = RwLock::new(None);

pub(crate) fn set_console(console: Console) {
	match CONSOLE.write() {
		Ok(mut global_console) => {
			global_console.replace(console);
		}
		Err(err) => eprintln!("Failed to acquire global console lock: {err}"),
	}
}

pub(crate) fn use_console<T>(cb: impl FnOnce(&Console) -> T) -> Option<T> {
	match CONSOLE.read() {
		Ok(global_console) => global_console.as_ref().map(cb),
		Err(err) => {
			eprintln!("Failed to acquire global console lock: {err}");
			None
		}
	}
}

macro_rules! api_fn {
	($fn:ident($($arg:ident : $ty:ty),*) -> $out:ty : $default:expr) => {
        pub fn $fn($($arg: $ty),*) -> ::anyhow::Result<$out> {
            let result = use_console(|console| console.$fn($($arg),*)).unwrap_or(Ok($default))?;
            Ok(result)
        }
    };
}

api_fn!(log_error(args: &Arguments) -> () : ());
api_fn!(log_warn(args: &Arguments) -> () : ());
api_fn!(log_info(args: &Arguments) -> () : ());
api_fn!(log_debug(args: &Arguments) -> () : ());
api_fn!(log_trace(args: &Arguments) -> () : ());
api_fn!(prompt(args: &Arguments, default: Option<bool>) -> bool : default.unwrap_or(false));
api_fn!(heading(args: &Arguments) -> () : ());
api_fn!(println(args: Option<&Arguments>) -> () : ());
api_fn!(flush() -> () : ());
