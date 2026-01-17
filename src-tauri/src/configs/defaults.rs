use directories::{BaseDirs, ProjectDirs, UserDirs};
use log::warn;
use std::fs;
use std::path::{Path, PathBuf};

use crate::configs::constants::{APP_PATH, CONFIG_PATH, GAME_PATH, PLUGIN_PATH};

pub fn get_app_path() -> PathBuf {
    let user_dirs = UserDirs::new().expect("Unable to determine user directories");
    user_dirs
        .document_dir()
        .expect("User has no Documents directory")
        .to_path_buf()
        .join(APP_PATH)
}

pub fn get_game_path() -> PathBuf {
    let path = get_app_path().join(GAME_PATH);

    if !path.exists() {
        warn!("Game path does not exist. Creating directory: {:?}", path);
        fs::create_dir_all(&path).expect("Failed to create game directory");
    }

    path
}

pub fn get_config_path() -> PathBuf {
    let path = get_app_path().join(CONFIG_PATH);

    if !path.exists() {
        warn!("Config path does not exist. Creating directory: {:?}", path);
        fs::create_dir_all(&path).expect("Failed to create config directory");
    }

    path
}

pub fn get_plugin_path() -> PathBuf {
    let path = get_app_path().join(PLUGIN_PATH);

    if !path.exists() {
        warn!("Plugin path does not exist. Creating directory: {:?}", path);
        fs::create_dir_all(&path).expect("Failed to create config directory");
    }

    path
}
