use crate::manifest::Manifest;

pub fn get_version() -> String {
    let c: &str = include_str!("../Cargo.toml");
    let c: Manifest = c.parse().unwrap();
    c.get("package", "version").unwrap()
}
