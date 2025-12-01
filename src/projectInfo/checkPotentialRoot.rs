use std::fs;
use std::path::Path;

pub fn checkRoot() -> bool {
    let dir_content = fs::read_dir(Path::new("./")).unwrap();

    let mut possibility = true;

    for i in dir_content {
        let file_name = i.unwrap().file_name();

        if file_name != "." || file_name != ".." {
            println!("Current directory is not empty. If you want to initialize root here anyway, write finit instead.");
            possibility = false;

            break;
        }
    }

    possibility

}

pub fn checkRootExistence() -> bool {
    let mut existence: bool = false;

    let mut root_file_existence = false;
    let mut lib_dir_existence = false;

    let curr_dir = fs::read_dir("./").unwrap();

    for i in curr_dir {
        let file_name = i.unwrap().file_name();

        if file_name == "info-cforge.totmb" {
            root_file_existence = true;
            break;
        }
    }

    let check_dir = fs::read_dir("./lib/");

    if check_dir.is_ok() {
        lib_dir_existence = true;
    }

    if lib_dir_existence && root_file_existence {
        existence = true;
    }

    existence
}