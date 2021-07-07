use crate::error::Error;
use serde_derive::{Deserialize, Serialize};
use toml::value::Table;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Manifest {
    content: toml::Value,
}

const PACKAGE_TABLE: &str =  "package";

impl Manifest {
    pub fn to_string_toml_like(&self) -> Result<String, Error> {
        match toml::to_string(&self.content) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new(
                format!("Fail to parse document to TOML: {:?}", e).as_str(),
            )),
        }
    }

    pub fn get(&self, field: &str) -> Result<String, Error> {
        let package = match self.content.get(PACKAGE_TABLE) {
            Some(v) => Ok(v),
            None => Err(Error::new("File root is in a unexpected format"))
        }?;

        let val = match package.get(field) {
            Some(v) => Ok(v),
            None => Err(Error::new("Field not found"))
        }?;

        Ok(match val.as_str() {
            Some(v) => v.to_string(),
            None => val.to_string(),
        })
    }

    pub fn set(&mut self, field: &str, value: &str) -> Result<toml::Value, Error> {
        let package = match self.content.get_mut(PACKAGE_TABLE) {
            Some(v) => Ok(v),
            None => Err(Error::new("File root is in a unexpected format"))
        }?;

        match package.as_table_mut() {
            Some(v) => set_value_on_table(v, field, value),
            None => Err(Error::new("File root is in a unexpected format"))
        }
    }
}

impl std::str::FromStr for Manifest {
    type Err = Error;

    fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
        let content: toml::Value = toml::from_str(input).unwrap();
        Ok(Manifest{ content })
    }
}

fn set_value_on_table(table: &mut Table, field: &str, value: &str) -> Result<toml::Value, Error> {
    let value: toml::Value = toml::Value::String(value.to_string());
    match table.insert(field.to_string(), value) {
        Some(v) => Ok(v),
        None => Err(Error::new(format!("Unexpected field {}", field).as_str()))
    }
}