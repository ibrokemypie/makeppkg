#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (
        print!("[MAKEPPKG] {}\n", format_args!($($arg)*))
    )
}
