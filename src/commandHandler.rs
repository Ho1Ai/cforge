use crate::projectInfo::initializeRoot;
use crate::projectInfo::initializeRoot::{initializeRoot, initializeRootWithCheck, initializeRootNoCheck};
use crate::projectInfo::readInfo;
use crate::defaultStructs;

use crate::cforgeInfo::environmental;

pub fn getCommand(args_list: Vec<String>) {
    //println!("{:?}", args_list);
    //println!("{:?}", args_list[1]);
    let mut opt_recognized = false;
    if args_list.len() < 2 {
        println!("Couldn't get any content for args_list");
        return
    }

    if args_list.get(1).unwrap().as_str() == "cforge-version-check".to_string() {
        opt_recognized = true;
        println!("{}", environmental::VERSION);
    }

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

    if args_list.get(1).unwrap().as_str() == "installed-libs-info".to_string() {
        opt_recognized = true;
        let mut skip_first = 0; // I know that it is just a stupid cork, but let it be...
        let args_list_clone = args_list.clone();

        for i in args_list_clone {
            if skip_first < 2 {
                skip_first += 1;
                continue;
            }

            let package : defaultStructs::ShortPkgInfo = readInfo::findExactLib(i);

            println!("{} => {}", package.name, package.version);
        }
    }

    if args_list.get(1).unwrap().as_str() == "fetch-data".to_string() {
        opt_recognized = true;
        let test = crate::projectInfo::reqSender::getPackageInfo(args_list.get(2).unwrap().to_string());
    }

    if (args_list.get(1).unwrap().as_str() == "get".to_string()) {
        opt_recognized = true;
        if args_list.len() < 3 {
            println!("Too few arguments for `cforge get`");
        } else {
            let downloader = crate::projectInfo::reqSender::downloadPackage(args_list.get(2).unwrap().to_string());
        }
    }

    if (args_list.get(1).unwrap().as_str() == "check-existence".to_string()) {
        opt_recognized = true;
        let check_existence = crate::projectInfo::reqSender::packageExistenceChecker(args_list.get(2).unwrap().to_string());
        println!("Existence check status: {}", check_existence);
    }

    if (args_list.get(1).unwrap().as_str() == "remove".to_string()) {
        opt_recognized = true;
        if args_list.len() <2 {
            println!("Too few arguments for `cforge remove`");
        } else {
            let result = crate::projectInfo::packageRemover::removePackage(args_list.get(2).unwrap());
        }
    }

    if (args_list.get(1).unwrap().as_str() == "help".to_string()) {
        opt_recognized = true;
        println!("Available commands:

        cforge get <pkg_name> - get library. Saves libraries to lib/ directory
        cforge fetch-data <pkg_name> - get some information about a library
        cforge installed-libs-info <pkg_name> - check library info (at the moment only name and version)
        cforge libs-list - check info about libraries from info-cforge.totmb
        cforge finit - initialize root in not empty directory
        cforge init - initialize root in an empty directory");
    }

    if !opt_recognized {
        println!("Unrecognized option. Write `cforge help` in order to get some info");
    }
}