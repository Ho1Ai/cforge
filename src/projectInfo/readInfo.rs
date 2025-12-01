use std::fs;
use std::io::Read;
use crate::defaultStructs::ShortPkgInfo;

pub fn readProjInfo() {
    if fs::exists("./info-cforge.totmb").unwrap() {
        let new_test = fs::read_to_string("./info-cforge.totmb").unwrap();
        println!("{}", new_test);
    }
}

pub fn findExactLib(search: String) -> ShortPkgInfo {
    let mut test = ShortPkgInfo{
        name: search.clone() + " - not found", // ah, forgot that it can be moved from here
        version: String::from("undefined"),
    };

    let read_file = fs::read_to_string("./info-cforge.totmb").unwrap();
    let mut lines = read_file.lines();

    for curr_str in lines {
        let line_pair = curr_str.split(" = ").collect::<Vec<&str>>();

        if line_pair[0].to_string() == search {
            test.name = search.clone();
            test.version = line_pair[1].to_string();
            break;
        }

    }

    test
}