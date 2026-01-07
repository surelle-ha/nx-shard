use std::path::{Path, PathBuf};
use directories::{BaseDirs, ProjectDirs, UserDirs};

use crate::configs::constants::{APP_PATH, CONFIG_PATH, GAME_PATH};

pub fn get_app_path() -> PathBuf {
    let user_dirs = UserDirs::new().expect("Unable to determine user directories");
    user_dirs
        .document_dir()
        .expect("User has no Documents directory")
        .to_path_buf()
}

pub fn get_game_path() -> PathBuf {
    get_app_path().join(APP_PATH).join(GAME_PATH)
}

pub fn get_config_path() -> PathBuf {
    get_app_path().join(APP_PATH).join(CONFIG_PATH)
}
