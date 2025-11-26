use reqwest::blocking::{Client,ClientBuilder};
//IDK what to use... Maybe Reqwest
//Yeah, I'll go for reqwest
use crate::defaultStructs;

// data list is needed to take dependencies and other packages and to make final set of packages, which will be downloaded
// so this stuff is really necessary
pub fn getPackageInfo (pkg_name: String) /*-> defaultStructs::PackageEntity*//* -> Result<String, &'static str>*/ {
    let reqwest_client = Client::new();
    let mut request_url = "http://localhost:8000/api/pkg-info?name=".to_owned();
    let new_name = pkg_name.to_owned();
    request_url.push_str(new_name.as_str());
    let data_fetch_result = reqwest_client.get(request_url.clone()).send();

    if data_fetch_result.is_ok() { // replace .expect()!!!!
        //println!("pkg_name is {}", data_fetch_result.ok().unwrap().text().expect("CLIENT ERROR 5. For more info read project page and check out a file named ''STD_ERROR_LIST__client.totmb'': https://github.com/Ho1Ai/cforge").to_string());
        let request_result = data_fetch_result.ok().unwrap().text().unwrap().as_str().to_owned();
        let mut data_list = request_result.split("\n").collect::<Vec<&str>>();
        // do smth with data_list

        for entry in data_list.iter_mut() {
            entry.replace("\\\"", "");
        }

        println!("{:#?}", data_list);
    } else {
        println!("Got an error while getting data. \nTry to check internet connection. If it doesn't help, maybe the server is down.");
    }

    // Result::Ok(response.to_string())
}