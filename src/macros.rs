#[macro_export]
macro_rules! eprintln {
    () => (eprint!("\n"));
    ($($arg:tt)*) => (
        eprint!("[MAKEPPKG] {}\n", format_args!($($arg)*))
    )
}
