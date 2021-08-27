mod command;
mod error;
mod manifest;

use crate::command::start_command;
use crate::error::Error;
use crate::manifest::Manifest;
use std::fs;

fn handle_read(manifest: Manifest, table: String, field: Option<String>) -> Result<String, Error> {
    if let Some(field) = field {
        let val = manifest.get(table.as_str(), field.as_str())?;
        return Ok(val.to_string());
    }
    Ok(format!("{}", manifest.to_string_toml_like()?))
}

fn handle_write(
    manifest: &mut Manifest,
    table: String,
    path: String,
    field: Option<String>,
    value: String,
) -> Result<String, Error> {
    if let Some(field) = field {
        manifest.set(table.as_str(), field.as_str(), value.as_str())?;
        let new_f = manifest.to_string_toml_like()?;
        if let Err(e) = fs::write(path, new_f) {
            return Err(Error::new(format!("Fail to write file: {:?}", e).as_str()));
        }
        return Ok(value);
    }
    Err(Error::new("You must define a field to set a new value"))
}

fn main() -> Result<(), Error> {
    let opts = start_command();

    let manifest_content: String = fs::read_to_string(opts.path.clone())
        .expect("you must be in a folder with a valid Cargo manifest or targeting a cargo manifest with the path")
        .parse()
        .unwrap();
    let mut manifest: Manifest = manifest_content.as_str().parse().unwrap();

    let value = match opts.set {
        Some(v) => handle_write(&mut manifest, opts.table, opts.path, opts.field, v),
        None => handle_read(manifest, opts.table, opts.field),
    }?;

    println!("{}", value);

    Ok(())
}
