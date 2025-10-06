use std::fmt::Arguments;

use parking_lot::RwLock;

use crate::console::Console;

static CONSOLE: RwLock<Option<Console>> = RwLock::new(None);

pub(crate) fn set_console(console: Console) {
	CONSOLE.write().replace(console);
}

pub(crate) fn use_console<T>(cb: impl FnOnce(&Console) -> T) -> Option<T> {
	CONSOLE.read().as_ref().map(cb)
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
