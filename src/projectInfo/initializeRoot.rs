 // Root file structure:
 /* name: info-cforge.totmb
    content:
        1. name
        2. version
        3-n. lib-name = version
 */

 use std::fs;
 use std::path::Path;

 use crate::projectInfo::checkPotentialRoot::{checkRoot};

 pub fn initializeRoot() { // creates root file (info-cforge.totmb)
    let check_root_existence = fs::File::open("./info-cforge.totmb");


    if check_root_existence.is_err() {
        fs::File::create("./info-cforge.totmb").unwrap();

        let check_lib_dir_existence = fs::read_dir("./lib/");
        if check_lib_dir_existence.is_err() {
            fs::create_dir("./lib");
        }

        let check_tmp_dir_existence = fs::read_dir("./tmp/");
        if check_tmp_dir_existence.is_err() {
            fs::create_dir("./tmp/");
        }

        println!("Root has been initialized successfully!");
    } else {
        println!("Already initialized.");
    }
 }

 pub fn initializeRootWithCheck() {
     let posibility = checkRoot();
     if posibility {
         initializeRoot();
     }
}

 pub fn initializeRootNoCheck() { // check if you can't access dir!
    initializeRoot();
 }