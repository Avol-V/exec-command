#![windows_subsystem = "windows"]

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

fn main() {
	// Exit with the child process exit code, or 1 on error
	let exit_code = run();
	process::exit(exit_code);
}

fn run() -> i32 {
	// Get the current executable path
	let exe_path = match env::current_exe() {
		Ok(path) => path,
		Err(_) => return 1,
	};

	// Construct the path to the .txt file
	let txt_path = get_txt_file_path(&exe_path);
	
	// Read the command from the .txt file
	let command_str = match read_command_file(&txt_path) {
		Ok(cmd) => cmd,
		Err(_) => return 1,
	};

	// Parse the command string into program and arguments
	let parsed_args = match shell_words::split(&command_str) {
		Ok(args) => args,
		Err(_) => return 1,
	};

	if parsed_args.is_empty() {
		return 1;
	}

	// First element is the program, rest are arguments
	let program = &parsed_args[0];
	let mut args: Vec<String> = parsed_args[1..].to_vec();

	// Append any arguments passed to this wrapper
	let wrapper_args: Vec<String> = env::args().skip(1).collect();
	args.extend(wrapper_args);

	// Execute the command
	execute_command(program, &args)
}

fn get_txt_file_path(exe_path: &PathBuf) -> PathBuf {
	let mut txt_path = exe_path.clone();
	txt_path.set_extension("txt");
	txt_path
}

fn read_command_file(path: &PathBuf) -> Result<String, std::io::Error> {
	let content = fs::read_to_string(path)?;
	
	// Replace all newline characters (both Unix \n and Windows \r\n) with spaces
	// First replace \r\n with space, then replace remaining \n with space
	let normalized = content.replace("\r\n", " ").replace('\n', " ");
	
	// Trim whitespace from start and end
	Ok(normalized.trim().to_string())
}

#[cfg(windows)]
fn execute_command(program: &str, args: &[String]) -> i32 {
	use std::process::Command;
	
	// CREATE_NO_WINDOW flag to prevent console window from appearing
	const CREATE_NO_WINDOW: u32 = 0x08000000;
	
	let result = Command::new(program)
		.args(args)
		.creation_flags(CREATE_NO_WINDOW)
		.status();
	
	match result {
		Ok(status) => status.code().unwrap_or(1),
		Err(_) => 1,
	}
}

#[cfg(not(windows))]
fn execute_command(program: &str, args: &[String]) -> i32 {
	use std::process::Command;
	
	let result = Command::new(program)
		.args(args)
		.status();
	
	match result {
		Ok(status) => status.code().unwrap_or(1),
		Err(_) => 1,
	}
}
