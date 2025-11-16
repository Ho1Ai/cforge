use std::fs;
use std::io::Read;
use crate::defaultStructs::shortPkgInfo;

pub fn readProjInfo() {
    let new_test = fs::read_to_string("./info-cforge.totmb").unwrap();
    println!("{:?}", new_test);
}

pub fn findExactLib(search: String) -> shortPkgInfo {
    let test = shortPkgInfo{
        name: search + " - not found",
        version: String::from("undefined"),
    };
    let read_file = fs::read_to_string("./README.md").unwrap();

  //  for i in read_dir {

//    }

    println!("{:?}", read_file);

    test
}