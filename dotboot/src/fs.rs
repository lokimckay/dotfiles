use crate::Error;
use std::{path::Path, process::Command};

pub fn create_symlink(src: &Path, dest: &Path) -> Result<(), Error> {
    #[cfg(windows)]
    {
        if src.is_dir() {
            std::os::windows::fs::symlink_dir(src, dest)?;
        } else {
            std::os::windows::fs::symlink_file(src, dest)?;
        }
        return Ok(());
    }

    #[cfg(unix)]
    std::os::unix::fs::symlink(src, dest)?;
    Ok(())
}

pub fn create_windows_symlink(src: &Path, dest: &Path) -> Result<(), Error> {
    // /D for directories, omit for files
    let dir_flag = if src.is_dir() { Some("/D") } else { None };

    let mut args = vec!["/c", "mklink"];
    if let Some(flag) = dir_flag.as_deref() {
        args.push(flag);
    }

    let src_str = src.to_str().ok_or("Invalid src path")?;
    let dest_str = dest.to_str().ok_or("Invalid dest path")?;

    // mklink [/D] <link> <target>
    args.push(dest_str);
    args.push(src_str);

    let output = Command::new("cmd.exe")
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to run cmd.exe: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "mklink failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}