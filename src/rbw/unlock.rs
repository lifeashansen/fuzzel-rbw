use crate::config::constants::{FRBW_ICON_NAME, FRBW_NAME};
use crate::fuzzel;
use crate::rbw;
use crate::utils::notify;

use std::io::Error;
use std::process::{self, Command, Stdio};

pub fn run() -> Result<(), Error> {
    let name_choice: String;

    // tries to unlock the database first - shows pinentry popup
    let command_rbw_unlock = Command::new("rbw")
        .arg("unlock")
        .stderr(Stdio::piped())
        .output()
        .expect("rbw unlock failed");

    // sends notification with stderr incase the command fails
    if !command_rbw_unlock.status.success() {
        let error: String = String::from_utf8_lossy(&command_rbw_unlock.stderr).to_string();

        notify::send(FRBW_ICON_NAME, FRBW_NAME, error)?
    } else {
        // stores the name as the key and the possible users in a vector of Strings
        let name_to_users = rbw::list::raw()?;

        let mut names_vec: Vec<String> = name_to_users.keys().cloned().collect();

        // sort the names from a-z
        names_vec.sort();

        let names_string = names_vec.clone().join("\n");

        let (name_choice_result, _code) = fuzzel::fuzzel::show(names_string.clone());

        match name_choice_result {
            Ok(choice) => name_choice = choice,
            Err(e) => {
                eprintln!("An error occured\n\t{e}");

                process::exit(1);
            }
        }

        // ensures we only proceed if we actually selected a name
        if !name_choice.is_empty() {
            rbw::user::get_user(name_choice, name_to_users)?
        }
    }

    Ok(())
}
