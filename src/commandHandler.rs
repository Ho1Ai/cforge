use crate::projectInfo::initializeRoot;
use crate::projectInfo::initializeRoot::{initializeRoot, initializeRootWithCheck, initializeRootNoCheck};
use crate::projectInfo::readInfo;

pub fn getCommand(args_list: Vec<String>) {
    //println!("{:?}", args_list);
    //println!("{:?}", args_list[1]);
    let mut opt_recognized = false;

    if args_list.get(1).unwrap().as_str() == "init".to_string() {
        opt_recognized = true;
        initializeRootWithCheck();
    }

    if args_list.get(1).unwrap().as_str() == "finit".to_string() {
        opt_recognized = true;
        initializeRootNoCheck();
    }

    if args_list.get(1).unwrap().as_str() == "libs-list".to_string() {
        opt_recognized = true;
        readInfo::readProjInfo();
    }

    if args_list.get(1).unwrap().as_str() == "fetch-libs-info".to_string() {
        opt_recognized = true;
        let mut skip_first = 0; // I know that it is just a stupid cork, but let it be...
        for i in args_list {
            if skip_first < 2 {
                skip_first += 1;
                continue;
            }

            readInfo::findExactLib(i);
        }
    }

    if !opt_recognized {
        println!("Unrecognized option.");
    }
}