use duct::cmd;

use crate::config::default::UserConfig;
use crate::config::parser::get_user_cfg;
use crate::fuzzel;
use crate::rbw;
use crate::utils::notify;

// Unlocks the vault
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: UserConfig = get_user_cfg()?;

    let unlock = cmd!("rbw", "unlock").stderr_capture().run()?;

    if !unlock.status.success() {
        notify::send_notification(&cfg, "RBW Unlock", String::from_utf8_lossy(&unlock.stderr).to_string())?
    }

    // stores the name as the key and the possible users in a Vec<String>
    let name_to_users = rbw::list::get_name_to_users()?;

    let mut names_vec: Vec<String> = name_to_users.keys().cloned().collect();

    // sort the names from a-z
    names_vec.sort();

    let names_string = names_vec.clone().join("\n");

    println!("{}", names_string);

    let (name_choice_result, _code) = fuzzel::fuzzel::show(names_string.clone());

    match name_choice_result {
        Ok(choice) => {
            if !choice.is_empty() {
                rbw::user::get_user(&cfg, choice, name_to_users)?
            }
        }
        Err(e) => notify::send_notification(&cfg, "Operation Failed", e.to_string())?,
    }

    Ok(())
}
