#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        println!(
            "[{}] {}: {}",
            style(chrono::Local::now().format("%Y-%m-%d %H:%M:%S")).cyan(),
            $level,
            format!($($arg)*)
        );
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        use console::style;
        log!(style("INFO").green(), $($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        log!(style("WARN").yellow(), $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        log!(style("ERROR").red(), $($arg)*);
    };
}