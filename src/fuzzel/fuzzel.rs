use std::io::Error;
use std::process::exit;

use crate::config::parser::parse_config_file;
use crate::{command, config::default::UserConfig};

// takes a String as stdin and pipes it to fuzzel
// then returns the stdout as a String
pub fn show(stdin: String) -> (Result<String, Error>, i32) {
    // gets the config file
    let user_config: UserConfig = match parse_config_file() {
        Ok(user_config) => user_config,
        Err(e) => {
            eprintln!("Failed to parse config file\n\t{e}");

            exit(1);
        }
    };

    // store all arguments into a vector and pass it to the command
    let args: Vec<String> = vec![
        "--dmenu".to_string(),
        "--prompt".to_string(),
        user_config.prompt.to_string(),
        "--placeholder".to_string(),
        user_config.placeholder.to_string(),
        "--lines".to_string(),
        user_config.lines.to_string(),
    ];

    // returns fuzzel stdout
    command::with_std_in("fuzzel", args, stdin)
}
