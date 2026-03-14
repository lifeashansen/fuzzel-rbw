use crate::command;

// gets the totp
pub fn totp(name: &String, user: &String) -> Result<String, std::io::Error> {
    let args: Vec<String> = vec![String::from("totp"), name.to_owned(), user.to_owned()];

    // rbw totp <name> <user>
    command::no_std_in("rbw", args)
}
