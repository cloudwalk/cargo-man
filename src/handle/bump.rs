use crate::error::Error;
use crate::handle;
use crate::manifest::Manifest;
use itertools::Itertools;
use std::io::{self, Write};
use std::process::Command;

enum Kind {
    Patch,
    Minor,
    Major,
}

pub fn bump(manifest: &mut Manifest, path: String, kind_str: String) -> Result<String, Error> {
    let kind = match kind_str.to_uppercase().as_str() {
        "MINOR" => Kind::Minor,
        "MAJOR" => Kind::Major,
        _ => Kind::Patch,
    };
    let table = "package".to_string();
    let field = Some("version".to_string());
    let current_version = handle::read(manifest.clone(), table.clone(), field.clone())?;

    let tpl: (String, String, String) = current_version
        .split('.')
        .map(|x| x.to_string())
        .collect_tuple()
        .unwrap();
    let new_version = handle_increment(kind, tpl)?;
    handle::write(manifest, table, path, field, new_version.clone())?;
    update_lock()?;
    commit(new_version.clone())?;
    Ok(new_version)
}

fn handle_increment(kind: Kind, tpl: (String, String, String)) -> Result<String, Error> {
    let z = "0".to_string();
    let (major, minor, patch) = match kind {
        Kind::Patch => (tpl.0, tpl.1, increment_val(tpl.2)?),
        Kind::Minor => (tpl.0, increment_val(tpl.1)?, z),
        Kind::Major => (increment_val(tpl.0)?, z.clone(), z),
    };
    Ok(format!("{}.{}.{}", major, minor, patch))
}

fn increment_val(val: String) -> Result<String, Error> {
    let mut num: u32 = val.parse().unwrap();
    num += 1;
    Ok(num.to_string())
}

fn update_lock() -> Result<(), Error> {
    let output = Command::new("cargo")
        .arg("build")
        .output()
        .expect("process failed to execute");
    io::stdout().write_all(&output.stdout).unwrap();
    Ok(())
}

fn commit(version: String) -> Result<(), Error> {
    let commit_msg = format!("Release {}", version);
    let output = Command::new("git")
        .args(["add", "Cargo.toml", "Cargo.lock"])
        .output()
        .expect("process failed to execute");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    let output = Command::new("git")
        .args(["commit", "-m", commit_msg.as_str()])
        .output()
        .expect("process failed to execute");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    let output = Command::new("git")
        .args(["tag", "-a", version.as_str(), "-m", commit_msg.as_str()])
        .output()
        .expect("process failed to execute");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    Ok(())
}
