#[macro_export]
macro_rules! prompt {
	(default: $default:expr, $($arg:tt)+) => {
        $crate::api::prompt(&format_args!($($arg)+), Some($default)).unwrap()
    };
    ($($arg:tt)+) => {
        $crate::api::prompt(&format_args!($($arg)+), None).unwrap()
    }
}

#[macro_export]
macro_rules! heading {
    ($($arg:tt)+) => {
        $crate::api::heading(&format_args!($($arg)+)).unwrap()
    }
}

#[macro_export]
macro_rules! println {
	($($arg:tt)+) => {
		$crate::api::println(Some(&format_args!($($arg)+))).unwrap()
	};
    () => {
        $crate::api::println(None).unwrap()
    }
}
