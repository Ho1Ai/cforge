use serde::{Deserialize};

pub struct PackageEntity {
    pub id: i32,
    pub name: String,
    pub version: String,
    pub dependencies: Vec<i32>, // dependencies are in IDs
}

pub struct ShortPkgInfo {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct PkgDataResponse {
    pub is_ok: bool,
    pub status_code: i32,
    pub existence: bool,
    pub version: String,
}