use nix::unistd::{Group, User};
use std::io::{self};
use std::os::unix::fs::{self, PermissionsExt};
use std::path::Path;

pub fn get_uid_by_name(username: &str) -> Option<u32> {
    if let Ok(user) = User::from_name(username) {
        Some(user.unwrap().uid.as_raw())
    } else {
        None
    }
}

pub fn get_gid_by_name(groupname: &str) -> Option<u32> {
    if let Ok(group) = Group::from_name(groupname) {
        println!("{}:{}", groupname, group.clone().unwrap().gid);
        Some(group.unwrap().gid.as_raw())
    } else {
        None
    }
}

pub fn apply_permissions(
    file: &str,
    owner: Option<u32>,
    group: Option<u32>,
    mode: Option<u32>,
) -> io::Result<()> {
    let path = Path::new(file);

    fs::chown(path, owner, group)?;

    if let Some(mode_bits) = mode {
        let permissions = PermissionsExt::from_mode(mode_bits);
        std::fs::set_permissions(path, permissions)?;
    }

    Ok(())
}
