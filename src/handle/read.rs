use crate::error::Error;
use crate::manifest::Manifest;

pub fn read(manifest: Manifest, table: String, field: Option<String>) -> Result<String, Error> {
    if let Some(field) = field {
        let val = manifest.get(table.as_str(), field.as_str())?;
        return Ok(val);
    }
    manifest.to_string_toml_like()
}
