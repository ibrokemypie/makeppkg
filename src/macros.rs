#[macro_export]
macro_rules! eprintln {
    () => (eprint!("\n"));
    ($($arg:tt)*) => (
        eprint!("[MAKEPPKG] {}\n", format_args!($($arg)*))
    )
}

#[macro_export]
macro_rules! choose_algo {
    ($line:expr, $($i:expr, $name:expr),*) => {
    $(
        if $line.starts_with(&(String::from($name) + "sums = ")) {
            println!("{:?}", $name);
            return Ok(Algorithm {
                name: String::from($name),
                function: |path: &Path| -> String {
                    let file = read(path).unwrap();
                    $i(&file)[0].to_string()
                },
            });
        })*
    };
}
