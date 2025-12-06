use std::fs;

pub fn writePkgVersion(name: &str, version: &str) -> bool {
    let mut result = false;

    let mut res_str = name.to_string();
    res_str.push_str(" = ");
    res_str.push_str(version);

    let file = fs::read_to_string("./info-cforge.totmb");
    if file.is_ok() {
        res_str.push_str("\n");
        res_str.push_str(file.unwrap().as_str());
        let write_test = fs::write("./info-cforge.totmb", res_str);
        if write_test.is_ok() {
            result = true;
        }
    }

    result
}