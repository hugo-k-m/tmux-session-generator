#[macro_export]
macro_rules! tmux_option {
    ( $x:expr ) => {
        if let Some(tmux_opt) = $x {
            format!("-{} {}", stringify!($x), tmux_opt)
        } else {
            "".to_string()
        }
    };
}
