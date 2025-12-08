use std::fs;

pub fn checkPackageExistence(pkg_name: &str) -> bool {
    let mut func_response = false;
    let mut lib_in_list_test = false;
    let mut lib_in_dir_test = false;
    let  libs_list_str = fs::read_to_string("./info-cforge.totmb");

    if(libs_list_str.is_ok()) {
        let string = libs_list_str.unwrap();
        let libs_list_vector = string.split("\n").collect::<Vec<&str>>();

        //println!("{:?}", libs_list_vector);

        for this_pkg_name in libs_list_vector.clone() {
            if (this_pkg_name == pkg_name) {
                lib_in_list_test = true;
            }
        }

        let dir = fs::read_dir("./lib/");
        if dir.is_ok() {
            let new_dir = dir.unwrap();

            for entry in new_dir {
                if entry.is_ok() {
                    let name_res = entry.unwrap().file_name().into_string();
                    if name_res.is_ok() {
                        let name = name_res.unwrap();
                        if name == pkg_name.to_string() {
                            lib_in_dir_test = true;
                            break;
                        }
                    } else {
                        println!("An error occurred while reading the file name")
                    }
                } else {
                    println!("An error occurred while reading the file name");
                }
            }
        } else {
            println!("No lib/ directory found");
        }
    } else {
        println!("An error occurred while reading the config file");
    }

    if (lib_in_list_test && lib_in_dir_test ) {
        func_response = true;
    } else if lib_in_dir_test {
        println!("couldn't find library in libraries list in cforge-info.totmb");
    } else if lib_in_list_test {
        println!("couldn't find library in lib/ directory in cforge-info.totmb");
    } else {
        println!("couldn't find library");
    }

    func_response
}

pub fn removePackage(pkg_name: &str) -> Result<(), String> {
    let check_existence = checkPackageExistence(pkg_name);

    if check_existence {
        let info_content = fs::read_to_string("./info-cforge.totmb");

        if info_content.is_ok() {
            let result = info_content.unwrap();
            let mut info = result.split("\n").collect::<Vec<&str>>();

            let mut index = 0;

            let mut search_filename = String::from(pkg_name);
            search_filename.push(' ');

            for entry in info.clone() {
                if entry.starts_with(pkg_name) {
                    info.remove(index);
                }
                index += 1;
            }


            // println!("{:#?}", info);
            // let write_result = fs::write("./info-cforge.totmb", info.join("\n"));
            // if write_result.is_ok() {
                // println!("successfully removed from info-cforge.totmb");
            // }

            let mut test_path = String::from("./lib/");
            test_path.push_str(pkg_name);

            let remove_dir = fs::remove_dir_all(test_path);
            if remove_dir.is_ok() {
                let write_result = fs::write("./info-cforge.totmb", info.join("\n"));
                if write_result.is_ok() {
                    println!("successfully removed from info-cforge.totmb");
                } else {
                    println!("couldn't remove library from info-cforge.totmb. Please, remove it by yourself. It is in here: {} line.", index+1);
                    return Err("Couldn't overwrite info-cforge.totmb".to_string())
                }

            } else {
                println!("An error occured while removing package");
                return Err(String::from("Failed to remove package directory"));
            }
        }
        Ok(())
    } else {
        Err("failed to find a package".to_string())
    }
}