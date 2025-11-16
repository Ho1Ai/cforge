pub struct PackageEntity {
    pub id: i32,
    pub name: String,
    pub version: String,
    pub dependencies: Vec<i32>, // dependencies are in IDs
}

pub struct shortPkgInfo {
    pub name: String,
    pub version: String,
}