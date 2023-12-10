use args::ArchBakArgs;
use clap::Parser;

mod args;
use args::*;
fn main() {
    let args = ArchBakArgs::parse();
    match args.entity {
        Entity::Pacman(pacman_cmd) => match pacman_cmd.options {
            PacmanCommandOptions::Backup(opts) => match opts {
                PackageBackupCommand { options } => match options {
                    PackageBackup::Explicit => {
                        println!(" explicit backup")
                    }
                    PackageBackup::Foreign => {
                        println!(" foreign backup ")
                    }
                    PackageBackup::All => {
                        println!(" all backup")
                    }
                },
            },
            PacmanCommandOptions::Restore(opts) => match opts {
                PackageRestoreCommand { options } => match options {
                    PackageRestore::Explicit => {
                        println!("explicit restore")
                    }
                    PackageRestore::Foreign => {
                        println!("foregin restore")
                    }
                    PackageRestore::All => {
                        println!("all restore")
                    }
                },
            },
            PacmanCommandOptions::Show(opts) => match opts {
                PackageShowCommand { options } => match options {
                    PackageShow::Explicit => {
                        println!("explicit show")
                    }
                    PackageShow::Foreign => {
                        println!("foregin show")
                    }
                    PackageShow::All => {
                        println!("show all packages ")
                    }
                },
            },
        },
        Entity::Files(files_cmd) => match files_cmd.options {
            FileCommandOptions::Add(FileAddCommand { file }) => {
                println!("{:?} add file", file)
            }
            FileCommandOptions::Remove(FileRemoveCommand { file }) => {
                println!("{:?} remove file", file)
            }
            FileCommandOptions::Backup(_) => println!("backup files"),
            FileCommandOptions::Show(FileShowCommand { file }) => {
                println!("{:?} show file", file)
            }
            FileCommandOptions::ShowAll(_) => println!("show all files"),
            FileCommandOptions::RestoreAll(_) => {
                println!("restore all files")
            }
            FileCommandOptions::Restore(FileRestoreCommand { file }) => {
                println!("{:?} restore file", file)
            }
        },
    }
}
