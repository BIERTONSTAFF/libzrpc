#[macro_export]
macro_rules! log {
    ($templ:expr, $($arg:expr),*) => {
        if cfg!(feature = "log") {
            println!($templ, $($arg),*);
        }
    };
}
