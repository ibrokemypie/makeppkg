extern crate xdg;

use std::process;
use std::env::args;

fn main() {
	let xdg_dirs = xdg::BaseDirectories::with_prefix("makeppkg").unwrap();
	let xdghome = xdg_dirs.get_config_home().into_os_string().into_string().unwrap();

	let mut arguments = args().enumerate();
	let mut options: Vec<String> = args().collect();
	let mut location = xdghome;

	if arguments.find(|(_, x)| x == &"-f".to_string()) != None{
		let string = arguments.next();
		if string.is_some() {
			let unwrapped = string.unwrap();
			let index= unwrapped.0;
			options.remove(index);
			options.remove(index - 1);
			location = unwrapped.1;
		} else {
			println!("Provide a location when using the -l option");
			process::exit(1);
		}
	}
	options.remove(0);
	println!("Patch location: {}, makepkg arguments: {}", location, options.join(" "));

	run_makepkg(options);
}

fn run_makepkg(options: Vec<String>) {
	let output = process::Command::new("echo")
				 .args(options)
				 .output()
				 .expect("Failed");
	if output.status.success() {
		print!("{}", String::from_utf8_lossy(&output.stdout));
	} else {
		println!("{}", String::from_utf8_lossy(&output.stderr));
	}
}
