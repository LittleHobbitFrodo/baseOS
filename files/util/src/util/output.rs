


/// Reports error
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use $crate::Colorize;
        print!("{}: ", "error".red());
        print!($($arg)*);
    }};
}

/// Reports error and breaks the line
#[macro_export]
macro_rules! errorln {
    ($($arg:tt)*) => {{
        use $crate::Colorize;
        print!("{}: ", "error".red());
        println!($($arg)*);
    }};
}

/// Reports warning
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{
        use $crate::Colorize;
        print!("{}: ", "warning".yellow());
        print!($($arg)*);
    }};
}


/// Reports warning and breaks the line
#[macro_export]
macro_rules! warningln {
    ($($arg:tt)*) => {{
        use $crate::Colorize;
        print!("{}: ", "warning".yellow());
        println!($($arg)*);
    }};
}

/// Notifies user
#[macro_export]
macro_rules! note {
    ($($arg:tt)*) => {{
        use $crate::Colorize;
        print!("{}: ", "note".blue());
        print!($($arg)*);
    }};
}

/// Notifies user and breaks the line
#[macro_export]
macro_rules! noteln {
    ($($arg:tt)*) => {{
        use $crate::Colorize;
        print!("{}: ", "note".blue());
        println!($($arg)*);
    }};
}

/// Prints error message and exits the program
/// 
/// usage:
/// - internal error: `fail!(internal: "formatted {}", "message")`
/// - user error: `fail!(user: "formatted {}", "message")`
///   - user error triggers the help menu
#[macro_export]
macro_rules! fail {
    (internal: $($arg:tt)*) => {{
        $crate::error!($($arg)*);
        std::process::exit($crate::exit_code::INTERNAL_ERROR);
    }};
    (user: $($arg:tt)*) => {{
        $crate::error!($($arg)*);
        std::process::exit($crate::exit_code::USER_ERROR);
    }};
    (internal) => {{
        std::process::exit($crate::exit_code::INTERNAL_ERROR);
    }};
    (user) => {{
        std::process::exit($crate::exit_code::USER_ERROR);
    }};
}

/// Prints error message, breaks line and exits the program
/// 
/// usage:
/// - internal error: `failln!(internal: "formatted {}", "message")`
/// - user error: `failln!(user: "formatted {}", "message")`
///   - user error triggers the help menu
#[macro_export]
macro_rules! failln {
    (internal: $($arg:tt)*) => {{
        $crate::errorln!($($arg)*);
        std::process::exit($crate::exit_code::INTERNAL_ERROR);
    }};
    (user: $($arg:tt)*) => {{
        $crate::errorln!($($arg)*);
        std::process::exit($crate::exit_code::USER_ERROR);
    }};
    (internal) => {{
        std::process::exit($crate::exit_code::INTERNAL_ERROR);
    }};
    (user) => {{
        std::process::exit($crate::exit_code::USER_ERROR);
    }};
}