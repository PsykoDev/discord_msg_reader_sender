#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!("\x1B[31m[Erreur] {}\x1B[0m", format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        eprintln!("\x1B[33m[Avertissement] {}\x1B[0m", format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        eprintln!("\x1B[32m[Information] {}\x1B[0m", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! ping {
    ($($arg:tt)*) => {
        eprintln!("\x1B[92m{}\x1B[0m", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! msg {
    ($($arg:tt)*) => {
        eprintln!("\x1B[97m{}\x1B[0m", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! dm_ping {
    ($($arg:tt)*) => {
        eprintln!("\x1B[35m{}\x1B[0m", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! dm_msg {
    ($($arg:tt)*) => {
        eprintln!("\x1B[36m{}\x1B[0m", format_args!($($arg)*))
    };
}

