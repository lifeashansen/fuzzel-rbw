use serde_json::Error;
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};

use crate::config::default::UserConfig;
use serde_json;

// reads the config file and deserializes into UserConfig
pub fn from_json_file(config_file: &mut File) -> Result<UserConfig, Error> {
    // resets the position of the cursor to 0
    // during the first run when the config file is created, the cursor position is left at the end.
    // this prevents getting an EOF error
    let _ = config_file.seek(SeekFrom::Start(0));

    let reader = BufReader::new(config_file);
    let user_config = serde_json::from_reader(reader)?;

    Ok(user_config)
}
