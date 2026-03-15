use std::io::Error;

// checks if all dependencies are installed
pub fn check_deps() -> Result<(), Error> {
    let deps: Vec<&str> = vec!["wtype", "fuzzel", "rbw", "notify-send"];
    let mut missing_deps: Vec<&str> = Vec::new();

    for dep in deps {
        if let Err(err) = which::which(&dep) {
            match err {
                which::Error::CannotFindBinaryPath => {
                    missing_deps.push(dep);
                }

                which::Error::CannotCanonicalize => {
                    // TODO: handle this
                    // eprintln!("");
                }
                which::Error::CannotGetCurrentDirAndPathListEmpty => {
                    // TODO: handle this
                    // eprintln!("");
                }
            }
        }
    }

    if !missing_deps.is_empty() {
        eprintln!("Please install these : {}", missing_deps.join(", "));
    }

    println!("You are all set");

    Ok(())
}

// #[test]
// pub fn test_check_deps() {
// }
