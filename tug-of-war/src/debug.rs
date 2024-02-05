macro_rules! info {
    () => {
        #[cfg(debug_assertions)]
        rtt_target::rprintln!();
    };
    ($fmt:expr) => {
        #[cfg(debug_assertions)]
        rtt_target::rprintln!($fmt);
    };
    ($fmt:expr, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        rtt_target::rprintln!($fmt, $($arg)*);
    };
}

pub(crate) use info;
