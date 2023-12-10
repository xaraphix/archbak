use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    author = "Suyash Singh",
    version = "0.1.0",
    about = "Utility to backup and restore configuration files and packages to be installed"
)]
pub struct ArchBakArgs {
    #[clap(subcommand)]
    pub entity: Entity,
}

#[derive(Subcommand, Debug)]
pub enum Entity {
    Pacman(PacmanCommand),
    Files(FilesCommand),
}

#[derive(Args, Debug)]
pub struct PacmanCommand {
    #[clap(subcommand)]
    pub options: PacmanCommandOptions,
}

#[derive(Subcommand, Debug)]
pub enum PacmanCommandOptions {
    Backup(PackageBackupCommand),
    Show(PackageShowCommand),
    Install(PackageInstallCommand),
}

#[derive(Args, Debug)]
pub struct PackageBackupCommand {
    #[clap(subcommand)]
    pub options: PackageBackup,
}

#[derive(Args, Debug)]
pub struct PackageInstallCommand {
    #[clap(subcommand)]
    pub options: PackageRestore,
}

#[derive(Args, Debug)]
pub struct PackageShowCommand {
    #[clap(subcommand)]
    pub options: PackageShow,
}

#[derive(Subcommand, Debug)]
pub enum PackageBackup {
    Explicit,
    Foreign,
    All,
}

#[derive(Subcommand, Debug, Clone)]
pub enum PackageShow {
    Explicit,
    Foreign,
    All,
}

#[derive(Subcommand, Debug, Clone)]
pub enum PackageRestore {
    Explicit,
    Foreign,
    All,
}

#[derive(Args, Debug)]
pub struct FilesCommand {
    #[clap(subcommand)]
    pub options: FileCommandOptions,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FileCommandOptions {
    Add(FileAddCommand),
    Remove(FileRemoveCommand),
    Backup(FileBackupCommand),
    Show(FileShowCommand),
    ShowAll(FileShowAllCommand),
    Restore(FileRestoreCommand),
    RestoreAll(FileRestoreAllCommand),
}

#[derive(Args, Debug, Clone)]
pub struct FileAddCommand {
    pub file: String,
}

#[derive(Args, Debug, Clone)]
pub struct FileRemoveCommand {
    pub file: String,
}

#[derive(Args, Debug, Clone)]
pub struct FileBackupCommand {}

#[derive(Args, Debug, Clone)]
pub struct FileShowCommand {
    pub file: String,
}

#[derive(Args, Debug, Clone)]
pub struct FileShowAllCommand {}

#[derive(Args, Debug, Clone)]
pub struct FileRestoreCommand {
    pub file: String,
}

#[derive(Args, Debug, Clone)]
pub struct FileRestoreAllCommand {}
