use crate::command;

// gets the password
pub fn password(name: &String, user: &String) -> String {
    let args: Vec<String> = vec![String::from("get"), name.to_owned(), user.to_owned()];

    // rbw get <name> <user>
    command::no_std_in("rbw", args).unwrap()
}
