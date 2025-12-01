use reqwest::blocking::{Client,ClientBuilder};
use std::fs;
use std::io::{Write};
use tar::Archive;
use flate2::read::GzDecoder;

use crate::defaultStructs::PkgDataResponse;
use crate::projectInfo::checkPotentialRoot::checkRootExistence;
use crate::defaultStructs;

// data list is needed to take dependencies and other packages and to make final set of packages, which will be downloaded
// so this stuff is really necessary
pub fn getPackageInfo (pkg_name: String) /*-> defaultStructs::PackageEntity*//* -> Result<String, &'static str>*/ {
    let reqwest_client = Client::new();
    let mut request_url = "http://localhost:8000/api/pkg-info?name=".to_owned();
    let new_name = pkg_name.to_owned();
    request_url.push_str(new_name.as_str());
    let data_fetch_result = reqwest_client.get(request_url.clone()).send();

    if data_fetch_result.is_ok() { // replace .unwrap()!!!!
        let mut request_result = data_fetch_result.ok().unwrap().json::<PkgDataResponse>().unwrap();

        println!("for package `{}`:\nexistence:{}\nactual version:{}", pkg_name, request_result.existence, request_result.version);
    } else {
        println!("Got an error while getting data. \nTry to check internet connection. If it doesn't help, maybe the server is down.");
    }
}

pub fn downloadPackage (pkg_name: String)  {
    let root_existence = checkRootExistence();

    if root_existence {
        let reqwest_client = Client::new();

        if packageExistenceChecker(pkg_name.clone()) {

            let data_fetch_result = reqwest_client.get("http://localhost:8000/api/pkg-get")
                .header("pkg_name", pkg_name.clone())
                .send();

            if data_fetch_result.is_ok() {
                let mut request_result = data_fetch_result.ok().unwrap().bytes().unwrap();

                let mut pkg_with_path__tmp = "./tmp/".to_string();
                pkg_with_path__tmp.push_str(pkg_name.as_str());
                pkg_with_path__tmp.push_str(".tar.gz");

                let mut pkg_with_path__lib = "./lib/".to_string();
                pkg_with_path__lib.push_str(pkg_name.as_str());
                pkg_with_path__lib.push_str("/");

                let mut curr_file_archive = fs::OpenOptions::new()
                    .read(true)
                    .append(true)
                    .create(true)
                    .open(pkg_with_path__tmp.clone())
                    .unwrap();

                curr_file_archive.write_all(&mut request_result).unwrap();

                let archive_file = fs::File::open(pkg_with_path__tmp).unwrap();

                //let mut decode = GzDecoder::new(archive_file.clone());
                let mut archive = Archive::new(archive_file);

                let extr_test = archive.unpack(pkg_with_path__lib);
                //println!("{:?}", extr_test.is_ok());
                println!("Installed")
            } else {
                println!("An error occured while downloading a library.");
            }
        }
    } else {
        println!("Initialize root first! Run `cforge init` or `cforge finit`");
    }
}


pub fn packageExistenceChecker (pkg_name: String) -> bool {
    let mut existence = false;

    let reqwest_client = Client::new();
    let mut url = "http://localhost:8000/api/pkg-info?name=".to_owned();
    url.push_str(pkg_name.as_str());
    let data_fetch_result = reqwest_client.get(url).send();
    if data_fetch_result.is_ok() {
        let mut request_result = data_fetch_result.ok().unwrap().json::<PkgDataResponse>().unwrap();
        if request_result.existence == true {
            existence = true;
        } else {
            println!("Looks like there's no package with this name...");
        }
    } else {
        println!("Failed to get package data.\nTry to check internet connection. If it is okay, the server may be down...");
    }

    existence
}