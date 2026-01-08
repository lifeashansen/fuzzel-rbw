use crate::command;
use crate::config::constants::FRBW_ICON_NAME;
use crate::config::constants::FRBW_NAME;
use crate::config::parser::parse_config_file;
use crate::fuzzel;
use crate::rbw;
use crate::utils::notify;
use crate::utils::wtype;

use std::collections::HashMap;
use std::io::Error;
use std::process;

// uses the name as key to get the value Vec<String> of users
// joins the users into a String then pipes it to fuzzel
// passes both the name and user to get::password
pub fn get_user(name_choice: String, name_to_users: HashMap<String, Vec<String>>) -> Result<(), Error> {
    let cfg = match parse_config_file() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Failed to parse config file\n\t{err}");

            process::exit(1);
        }
    };

    if let Some(users) = name_to_users.get(&name_choice) {
        let mut users = users.clone();

        // sorts users from a-z
        users.sort();

        let users = users.join("\n");

        let (user, exit_code) = fuzzel::fuzzel::show(users.clone());

        let user = user.unwrap();

        let password: String = rbw::get::password(&name_choice, &user);

        // Copy user only
        if exit_code == cfg.copy_user_exit_code {
            let (result, _exit_code) = command::with_std_in_no_args("wl-copy", user.clone());
            let _result = command::no_std_in("cliphist", vec![String::from("delete-query"), user]);

            match result {
                Ok(_) => {
                    let _ = notify::send(FRBW_ICON_NAME, FRBW_NAME, String::from("User copied"));
                }
                Err(e) => return Err(e),
            }

            process::exit(0);
        } else if exit_code == cfg.copy_password_exit_code {
            // Copy password only
            let (result, _exit_code) = command::with_std_in_no_args("wl-copy", password.clone());
            let _result = command::no_std_in("cliphist", vec![String::from("delete-query"), password]);

            match result {
                Ok(_) => {
                    let _ = notify::send(FRBW_ICON_NAME, FRBW_NAME, String::from("Password copied"));
                }
                Err(e) => return Err(e),
            }

            process::exit(0);
        } else if exit_code == cfg.type_user_exit_code {
            command::no_std_in("wtype", vec![user.clone()])?;

            process::exit(0);
        } else if exit_code == cfg.type_password_exit_code {
            command::no_std_in("wtype", vec![password.clone()])?;

            process::exit(0);
        }

        wtype::key_in(user, password)
    } else {
        Err(Error::new(
            std::io::ErrorKind::NotFound,
            "No users found for the given name",
        ))
    }
}
