use std::{fmt::Arguments, io};

use crate::console::Console;

static mut CONSOLE: Option<Console> = None;

pub(crate) fn set_console(console: Console) {
	unsafe { CONSOLE = Some(console) }
}

pub(crate) fn get_console() -> Option<&'static Console> {
	unsafe { CONSOLE.as_ref() }
}

macro_rules! api_fn {
	($fn:ident($($arg:ident : $ty:ty),*) -> $out:ty : $default:expr) => {
        pub fn $fn($($arg: $ty),*) -> Result<$out, io::Error> {
            if let Some(console) = get_console() {
                console.$fn($($arg),*)
            } else {
                Ok($default)
            }
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
