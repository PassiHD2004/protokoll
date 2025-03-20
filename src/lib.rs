pub use ::console;
pub use ::chrono;

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        println!(
            "[{}] {}: {}",
            $crate::console::style($crate::chrono::Local::now().format("%Y-%m-%d %H:%M:%S")).cyan(),
            $level,
            format!($($arg)*)
        );
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log!($crate::console::style("INFO").green(), $($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        log!($crate::console::style("WARN").yellow(), $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        log!($crate::console::style("ERROR").red(), $($arg)*);
    };
}