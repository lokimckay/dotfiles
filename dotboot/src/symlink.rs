use crate::{
    Error,
    cli::{Cli, Command},
    config::{BootConfig, Platform},
    fs,
};
use colored::Colorize;
use globset::{Glob, GlobSetBuilder};
use shellexpand;
use std::{
    fs as stdfs,
    path::PathBuf,
    process::Command as ProcessCommand,
    time::{SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

pub fn run(cli: Cli) -> Result<(), Error> {
    let config_str = stdfs::read_to_string(&cli.config)
        .map_err(|err| format!("Failed to read config file at '{}': {}", cli.config, err))?;

    let config: BootConfig = toml::from_str(&config_str)
        .map_err(|err| format!("Failed to parse config file: {}", err))?;

    match cli.command {
        Command::Install => install(config, &cli),
        Command::Remove => remove(config, &cli),
    }
}

fn timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn resolve_path(input: &str, convert_for_platform: Option<Platform>) -> Result<PathBuf, Error> {
    let expanded = shellexpand::full(input)
        .map_err(|e| format!("Failed to expand '{}': {}", input, e))?
        .to_string();

    let arg = match convert_for_platform {
        Some(Platform::Wsl) => "-u",
        Some(Platform::Windows) => "-w",
        None => return Ok(PathBuf::from(expanded)),
    };

    let output = ProcessCommand::new("wslpath")
        .arg(arg)
        .arg(&expanded)
        .output()
        .map_err(|e| format!("Failed to run wslpath: {}", e))?;

    if !output.status.success() {
        return Err(format!("wslpath failed for '{}'", expanded).into());
    }

    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim().to_string(),
    ))
}

fn log_link(tag: &str, tag_color: fn(&str) -> colored::ColoredString, link: &PathBuf, target: &PathBuf) {
    println!("{} {} -> {}", tag_color(tag), link.display().to_string().cyan().bold(), target.display());
}

fn install(config: BootConfig, cli: &Cli) -> Result<(), Error> {
    for rule in config.symlink {
        let link_root = resolve_path(&rule.src, None)?;
        let target_root = resolve_path(&rule.dest, None)?;

        let include = {
            let mut b = GlobSetBuilder::new();
            for pat in rule.include.clone().unwrap_or_else(|| vec!["**/*".into()]) {
                b.add(Glob::new(&pat)?);
            }
            b.build()?
        };

        let exclude = {
            let mut b = GlobSetBuilder::new();
            for pat in rule.exclude.clone().unwrap_or_default() {
                b.add(Glob::new(&pat)?);
            }
            b.build()?
        };

        for entry in WalkDir::new(&target_root).into_iter().filter_map(Result::ok) {
            let path = entry.path();

            if entry.file_type().is_dir() {
                continue;
            }

            if path.components().any(|c| c.as_os_str() == ".git") {
                continue;
            }

            let rel = match path.strip_prefix(&target_root) {
                Ok(r) => r,
                Err(_) => continue,
            };

            if exclude.is_match(rel) || !include.is_match(rel) {
                continue;
            }

            let link = link_root.join(rel);
            let target_raw = stdfs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
            let target = resolve_path(&target_raw.to_string_lossy(), rule.dest_convert)?;

            if link.starts_with(&target_root) {
                println!("{} recursive link prevented: {}", "[SKIP]".yellow(), link.display());
                continue;
            }

            if cli.dry_run {
                log_link("[DRY]", |s| s.yellow(), &link, &target);
                continue;
            }

            // Handle anything already at the link path
            if let Ok(meta) = stdfs::symlink_metadata(&link) {
                if meta.file_type().is_symlink() {
                    let existing_target = stdfs::read_link(&link)
                        .ok()
                        .and_then(|p| stdfs::canonicalize(p).ok())
                        .and_then(|p| resolve_path(&p.to_string_lossy(), rule.dest_convert).ok());

                    if existing_target.as_ref() == Some(&target) {
                        log_link("[OK]", |s| s.green(), &link, &target);
                        continue;
                    }

                    // Stale or wrong symlink — back it up or remove if broken
                    match stdfs::canonicalize(&link) {
                        Ok(_) => {
                            let backup = format!("{}-backup-{}", link.display(), timestamp());
                            stdfs::rename(&link, &backup)?;
                            println!("{} {} -> {}", "[EXISTING SYMLINK]".yellow(), link.display(), backup);
                        }
                        Err(_) => {
                            stdfs::remove_file(&link)?;
                        }
                    }
                } else {
                    let backup = format!("{}-backup-{}", link.display(), timestamp());
                    stdfs::rename(&link, &backup)?;
                    println!("{} {} -> {}", "[EXISTING FILE]".yellow(), link.display(), backup);
                }
            }

            if let Some(parent) = link.parent() {
                if !parent.exists() {
                    stdfs::create_dir_all(parent)?;
                }
            }

            fs::create_symlink(&target, &link)?;
            log_link("[OK]", |s| s.green(), &link, &target);
        }
    }

    Ok(())
}

fn remove(_config: BootConfig, _cli: &Cli) -> Result<(), Error> {
    println!("Remove not implemented yet");
    Ok(())
}