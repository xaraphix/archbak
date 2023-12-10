use args::ArchBakArgs;
use clap::Parser;

mod args;
mod config;
mod files;
mod pacman;

use args::*;
fn main() {
    let args = ArchBakArgs::parse();
    match args.entity {
        Entity::Pacman(pacman_cmd) => match pacman_cmd.options {
            PacmanCommandOptions::Backup(opts) => match opts {
                PackageBackupCommand { options } => match options {
                    PackageBackup::Explicit => pacman::backup_explicit(),
                    PackageBackup::Foreign => pacman::backup_foreign(),
                    PackageBackup::All => pacman::backup_all(),
                },
            },
            PacmanCommandOptions::Install(opts) => match opts {
                PackageInstallCommand { options } => match options {
                    PackageRestore::Explicit => pacman::install_explicit(),
                    PackageRestore::Foreign => pacman::install_foreign(),
                    PackageRestore::All => pacman::install_all(),
                },
            },
            PacmanCommandOptions::Show(opts) => match opts {
                PackageShowCommand { options } => match options {
                    PackageShow::Explicit => pacman::show_explicit(),
                    PackageShow::Foreign => pacman::show_foreign(),
                    PackageShow::All => pacman::show_all(),
                },
            },
        },
        Entity::Files(files_cmd) => match files_cmd.options {
            FileCommandOptions::Add(FileAddCommand { file }) => files::add(file.as_str()),
            FileCommandOptions::Remove(FileRemoveCommand { file }) => files::remove(file.as_str()),
            FileCommandOptions::Show(FileShowCommand { file }) => files::show(file.as_str()),
            FileCommandOptions::ShowAll(_) => files::show_all(),
            FileCommandOptions::RestoreAll(_) => files::restore_all(),
            FileCommandOptions::Backup(_) => files::backup(),
            FileCommandOptions::Restore(FileRestoreCommand { file }) => {
                files::restore(file.as_str())
            }
        },
    }
}
