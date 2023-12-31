use colored::Colorize;
use dirs;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

pub const CONFIG_FILE_PATH: &str = "arch_bak/config.yaml";
pub const CONFIG_DIR_PATH: &str = "arch_bak/";
pub const CONFIG_DIR_FILES_BACKUP_PATH: &str = "files/root/";

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub packages: PackageConfig,
    pub files: Vec<FileConfig>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct PackageConfig {
    pub pacman: PacmanConfig,
}

#[derive(Default, Serialize, Deserialize)]
pub struct PacmanConfig {
    pub explicit: Vec<Package>,
    pub foreign: Vec<Package>,
}

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct Package {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileConfig {
    pub name: String,
    pub ownership: Ownership,
    pub source_path: String,
    pub backup_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct Ownership {
    pub owner: String,
    pub group: String,
    pub permissions: u32,
}

pub enum ConfigFileStatus {
    NewCreated,
    AlreadyExists,
    BackedUp,
    BackupFail,
    Updated,
    UpdateFailed,
}

pub fn write_pacman_packages(packages: Vec<Package>, foreign: bool) {
    let mut config = get_config();
    match foreign {
        true => {
            config.packages.pacman.foreign = packages;
        }
        false => {
            config.packages.pacman.explicit = packages;
        }
    }

    match update_config(config) {
        Err(_) => println!("{}", "Failed to update the config file".red()),
        _ => {}
    }
}

pub fn update_config(config: Config) -> Result<ConfigFileStatus, std::io::Error> {
    let config_dir = dirs::config_dir().expect("");
    let config_path = config_dir.join(CONFIG_FILE_PATH);

    if config_path.exists() {
        match fs::remove_file(&config_path) {
            Ok(_) => {}
            Err(_) => {
                println!("Error updating config file");
            }
        }
        let config_dir = config_path
            .parent()
            .expect("Failed to get parent directory");
        fs::create_dir_all(config_dir)?;

        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&config_path)
            .expect("Couldn't open file");
        serde_yaml::to_writer(f, &config).unwrap();

        Ok(ConfigFileStatus::Updated)
    } else {
        Ok(ConfigFileStatus::UpdateFailed)
    }
}

pub fn get_config() -> Config {
    let config_dir = dirs::config_dir().expect("");
    let config_path = config_dir.join(CONFIG_FILE_PATH);
    let f = std::fs::File::open(config_path).expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

    config
}

pub fn create_config_file_if_not_exists() -> Result<ConfigFileStatus, std::io::Error> {
    let config_dir = dirs::config_dir().expect("");
    let config_path = config_dir.join(CONFIG_FILE_PATH);

    if !config_path.exists() {
        let config_dir = config_path
            .parent()
            .expect("Failed to get parent directory");
        fs::create_dir_all(config_dir)?;

        let config_yaml = Config::default();

        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&config_path)
            .expect("Couldn't open file");
        serde_yaml::to_writer(f, &config_yaml).unwrap();

        Ok(ConfigFileStatus::NewCreated)
    } else {
        Ok(ConfigFileStatus::AlreadyExists)
    }
}

pub fn backup() -> Result<ConfigFileStatus, std::io::Error> {
    match create_config_file_if_not_exists() {
        Ok(ConfigFileStatus::AlreadyExists) => {
            let config_dir = dirs::config_dir().expect("");
            let config_path = config_dir.join(CONFIG_FILE_PATH);

            if config_path.exists() {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                let new_config_path = format!("{}_{}", config_path.to_string_lossy(), timestamp);
                fs::copy(&config_path, &new_config_path)?;
                println!("Created a backup");
                Ok(ConfigFileStatus::BackedUp)
            } else {
                Ok(ConfigFileStatus::BackupFail)
            }
        }
        _ => Ok(ConfigFileStatus::BackupFail),
    }
}

pub fn get_config_dir_path() -> String {
    let config_dir = dirs::config_dir().expect("");
    let config_path = config_dir.join(CONFIG_DIR_PATH);
    format!("{}", config_path.to_string_lossy())
}

pub fn get_config_files_backup_path() -> String {
    let config_dir = dirs::config_dir().expect("");
    let config_path = config_dir.join(format!(
        "{}{}",
        CONFIG_DIR_PATH, CONFIG_DIR_FILES_BACKUP_PATH
    ));
    format!("{}", config_path.to_string_lossy())
}
