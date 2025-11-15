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