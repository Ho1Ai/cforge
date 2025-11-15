use crate::projectInfo::initializeRoot;
use crate::projectInfo::initializeRoot::{initializeRoot, initializeRootWithCheck, initializeRootNoCheck};

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

    if !opt_recognized {
        println!("Unrecognized option.");
    }
}