//! All macros for the crate

#[macro_export]
macro_rules! tmux_option {
    ( $( $x:ident, $y:expr ) + ) => {
        $(
            let $x = if let Some(tmux_opt) = $y {
                format!("-{} {}", stringify!($y), tmux_opt)
            } else {
                "".to_string()
            };
        ) +
    };
}

#[macro_export]
macro_rules! create_dir {
    ( $x:expr, $y:expr ) => {
        if $x {
            fs::create_dir($y)?;
        }
    };
}

#[macro_export]
macro_rules! create_file {
    ( $x:expr, $y:expr ) => {
        if $x {
            fs::File::create($y)?;
        } else {
            return Err(Box::new(DirectoryError));
        }
    };
}
