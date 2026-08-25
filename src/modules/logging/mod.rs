pub mod timestamp;

#[macro_export]
macro_rules! log {
    ($x:expr) => {
        let timestamp=$crate::modules::logging::timestamp::now();
        println!("{} {}", timestamp, $x)
    };
    ( $($x:tt)*) => {
        let timestamp=$crate::modules::logging::timestamp::now();
        println!("{} {}", timestamp, format_args!($($x)*))
    };
}
