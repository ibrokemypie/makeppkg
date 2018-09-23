use std::process;

// TODO: run makepkg instead of echo
pub fn run_makepkg(options: Vec<String>) {
    let output = process::Command::new("echo")
        .args(options)
        .output()
        .expect("Failed");
    // TODO: connect stdout and stderr streams
    if output.status.success() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}
