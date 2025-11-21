//IDK what to use... Maybe Reqwest
//Yeah, I'll go for reqwest
use crate::defaultStructs;

pub async fn getPackageInfo (pkg_name: String) /*-> defaultStructs::PackageEntity*/ -> Result<String, &'static str> {
    let response = reqwest::get("http://localhost:8000/api/get-pkg-info").await?.text().await?;

    Result::Ok(response)
}