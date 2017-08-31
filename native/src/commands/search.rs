extern crate hyper;

use options::Options;
use heroku_api::HerokuApi;
use self::hyper::StatusCode;

pub struct Search {
    pub name: String,
}

impl Search {
    pub fn new(options: Options) -> Self {
        Search { name: options.arg_name }
    }

    pub fn execute(self) {
        let api = HerokuApi::new_with_host("http://localhost:3000");
        let response = api.get_with_version("/buildpacks", "3.buildpack-registry").unwrap();

        match response.status {
            StatusCode::Ok => {
                let json = response.body;
                for buildpack in json.as_array().unwrap().into_iter() {
                    println!("{}/{}", buildpack["namespace"].as_str().unwrap(), buildpack["name"].as_str().unwrap());
                }
            },
            status => println!("Could not perform search.\nReceived: {}, {}", status, response.body),
        }
    }
}
