use colored::Colorize;
use std::process::Command;

use crate::config::{self, backup, Package};

pub fn backup_explicit() {
    let packages = get_packages(false);
    config::write_pacman_packages(packages, false);
}

pub fn backup_foreign() {
    let packages = get_packages(true);
    config::write_pacman_packages(packages, true);
}

pub fn backup_all() {
    match backup() {
        Ok(_) => {
            backup_explicit();
            backup_foreign();
        }
        Err(_) => {
            println!("{}", "Backup Failed".red())
        }
    }
}

pub fn show_explicit() {
    let output = Command::new("pacman")
        .arg("-Qent")
        .output()
        .expect("failed to execute process");

    match output.status.code() {
        Some(0) => {
            println!();
            println!("{}", "Explicit Packages".green());
            println!("{}", "================".green());
            println!("{:?}", get_packages(false));
        }

        Some(_) => {}
        None => {}
    }
}

pub fn show_foreign() {
    let output = Command::new("pacman")
        .arg("-Qm")
        .output()
        .expect("failed to execute process");

    match output.status.code() {
        Some(0) => {
            println!();
            println!("{}", "Foreign Packages".red());
            println!("{}", "================".red());
            println!("{:?}", get_packages(true));
        }
        Some(_) => {}
        None => {}
    }
}

fn get_packages(foreign: bool) -> Vec<Package> {
    let output = match foreign {
        true => Command::new("pacman")
            .arg("-Qm")
            .output()
            .expect("failed to execute process"),
        false => Command::new("pacman")
            .arg("-Qent")
            .output()
            .expect("failed to execute process"),
    };

    let result: Vec<Package> = match output.status.code() {
        Some(0) => String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|l| -> Package {
                let tokens: Vec<&str> = l.split_whitespace().collect();
                if tokens.len() >= 2 {
                    Package {
                        name: tokens[0].to_string(),
                        version: tokens[1].to_string(),
                    }
                } else {
                    Package {
                        name: tokens[0].to_string(),
                        ..Default::default()
                    }
                }
            })
            .collect(),
        _ => {
            vec![]
        }
    };

    result
}

pub fn show_all() {
    show_explicit();
    show_foreign();
}

pub fn install_explicit() {
    let config = config::get_config();
    let packages: Vec<&str> = config
        .packages
        .pacman
        .explicit
        .iter()
        .map(|p| p.name.as_str())
        .collect();

    println!();
    println!("{}", "Explicit packages".blue());
    println!("{}", "=================".blue());
    println!("{}", packages.join(" "));
}

pub fn install_foreign() {
    {
        let config = config::get_config();
        let packages: Vec<&str> = config
            .packages
            .pacman
            .foreign
            .iter()
            .map(|p| p.name.as_str())
            .collect();

        println!();
        println!("{}", "Foreign packages".blue());
        println!("{}", "=================".blue());
        println!("{}", packages.join(" "));
    }
}

pub fn install_all() {
    install_explicit();
    install_foreign();
}
