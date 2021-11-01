use crate::error::Error;
use crate::manifest::Manifest;
use std::fs;

pub fn write(
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
