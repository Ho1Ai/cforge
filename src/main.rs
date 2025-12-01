mod defaultStructs;
mod projectInfo;
mod commandHandler;
mod cforgeInfo;

use std::env;

fn main() {
    let args_list: Vec<String> = env::args().collect();

    commandHandler::getCommand(args_list);

    //println!("Hello, world!");
}
