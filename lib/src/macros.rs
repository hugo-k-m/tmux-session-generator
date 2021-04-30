//! All macros

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
