mod command;
mod error;
mod handle;
mod manifest;

use crate::command::start_command;
use crate::error::Error;
use crate::manifest::Manifest;
use std::fs;

fn main() -> Result<(), Error> {
    let opts = start_command();

    let manifest_content: String = fs::read_to_string(opts.path.clone())
        .expect("you must be in a folder with a valid Cargo manifest or targeting a cargo manifest with the path")
        .parse()
        .unwrap();
    let mut manifest: Manifest = manifest_content.as_str().parse().unwrap();

    if let Some(kind) = opts.bump {
        let value = handle::bump(&mut manifest, opts.path, kind)?;
        println!("{}", value);
        return Ok(());
    }

    let value = match opts.set {
        Some(v) => handle::write(&mut manifest, opts.table, opts.path, opts.field, v),
        None => handle::read(manifest, opts.table, opts.field),
    }?;

    println!("{}", value);

    Ok(())
}
