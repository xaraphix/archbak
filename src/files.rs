use crate::config::{self, FileConfig};
use crate::user_and_group::{self, apply_permissions};
use colored::Colorize;
use file_owner::PathExt;
use std::fs;
use std::os::unix::prelude::PermissionsExt;
use std::path::Path;

pub fn add(file: &str) {
    match does_file_exist(file) {
        Err(_) => {
            println!("File {} not found", file.red());
            return;
        }
        _ => {}
    }

    let mut config = config::get_config();
    let config_dir = dirs::config_dir().expect("");
    let binding = match file.starts_with("/") {
        true => {
            let mut value = file.chars();
            value.next();
            format!(
                "{}{}",
                config::get_config_files_backup_path(),
                value.as_str()
            )
        }
        _ => match file.starts_with("~") {
            true => {
                let expanded_path = shellexpand::tilde(&file).to_string();
                expanded_path
            }
            _ => String::from(""),
        },
    };
    let mut source_path_str = String::from(binding.as_str());

    let source_path = config_dir
        .join(config::get_config_dir_path())
        .join(source_path_str);
    source_path_str = format!("{}", source_path.to_string_lossy());

    let (owner_name, group_name, mode) = get_permissions(&file);
    config.files.push(config::FileConfig {
        name: String::from(file),
        ownership: config::Ownership {
            owner: owner_name,
            group: group_name,
            permissions: mode,
        },
        backup_path: source_path_str.clone(),
        source_path: String::from(file),
    });

    let _ = config::backup();
    let _ = config::update_config(config);
    backup_file(file, source_path_str.as_str());
}

fn does_file_exist(file: &str) -> Result<(), ()> {
    if Path::new(file).exists() {
        return Ok(());
    } else {
        return Err(());
    }
}

fn get_permissions(file: &str) -> (String, String, u32) {
    let owner = file.owner().unwrap();
    let owner_name = owner.name().unwrap().unwrap();
    let group = file.group().unwrap();
    let group_name = group.name().unwrap().unwrap();
    let metadata = fs::metadata(&file).unwrap();
    let permissions = metadata.permissions();
    let mode = permissions.mode();

    (owner_name, group_name, mode)
}

pub fn remove(file: &str) {
    let mut config = config::get_config();
    let mut files: Vec<FileConfig> = vec![];
    for ele in config.files {
        if ele.source_path.ne(file) {
            files.push(ele)
        }
    }
    config.files = files;
    let _ = config::backup();
    let _ = config::update_config(config);
}

pub fn remove_all() {
    let mut config = config::get_config();
    for ele in &config.files {
        match fs::remove_file(&ele.backup_path) {
            Ok(_) => println!("Removed {} from backup", ele.source_path),
            Err(_) => println!("Error removing {} from backup", ele.source_path),
        }
    }
    config.files.clear();
    let _ = config::backup();
    let _ = config::update_config(config);
}

pub fn backup() {
    let config = config::get_config();
    for ele in config.files {
        backup_file(&ele.source_path, &ele.backup_path);
    }
}

pub fn backup_file(from: &str, to: &str) {
    if let Some(parent_dir) = std::path::Path::new(to).parent() {
        let _ = fs::create_dir_all(parent_dir);
    }

    match fs::copy(from, to) {
        Ok(_) => {
            println!("Successfully backed up {} to {}", from, to)
        }
        Err(_) => {
            println!("Error backing up {} to {}", from, to)
        }
    }
}

pub fn restore_file(file: &str) {
    let config = config::get_config();
    for ele in config.files {
        if ele.source_path.eq(file) {
            match fs::copy(ele.backup_path, &ele.source_path) {
                Ok(_) => {
                    let user = match user_and_group::get_uid_by_name(&ele.ownership.owner) {
                        Some(user) => user,
                        None => {
                            println!("Error finding user {} in system", ele.ownership.owner);
                            continue;
                        }
                    };
                    let group = match user_and_group::get_gid_by_name(&ele.ownership.group) {
                        Some(group) => group,
                        None => {
                            println!("Error finding group {} in system", ele.ownership.group);
                            continue;
                        }
                    };

                    let _ = apply_permissions(
                        &ele.source_path,
                        Some(user),
                        Some(group),
                        Some(ele.ownership.permissions),
                    );
                    println!("Restored {}", ele.name.blue())
                }
                Err(_) => println!("Error restoring {}", ele.name.red()),
            }
        }
    }
}

pub fn show_all() {
    println!("{}", "Files".blue());
    println!("{}", "================".blue());
    for ele in config::get_config().files {
        println!("{}", ele.name)
    }
}

pub fn restore_all() {
    let config = config::get_config();
    for ele in config.files {
        restore_file(&ele.source_path)
    }
}
