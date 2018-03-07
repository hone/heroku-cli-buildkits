extern crate percent_encoding;
extern crate reqwest;
extern crate serde;

use heroku_api::HerokuApi;
use self::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};
use self::reqwest::StatusCode;

pub struct Rollback {
    pub namespace: String,
    pub name: String,
}

impl Rollback {
    pub fn execute(self) {
        let api = HerokuApi::new();

        let buildpack_id = format!("{}/{}", &self.namespace, &self.name);
        let encoded_buildpack_id = utf8_percent_encode(&buildpack_id, PATH_SEGMENT_ENCODE_SET);
        let body = json!({});
        let url_path = format!("/buildpacks/{}/actions/rollback", encoded_buildpack_id);
        let response = api.post_with_version(&url_path, "3.buildpack-registry", body).unwrap();
        match response.status {
            StatusCode::Ok => {
                let json = response.body;
                let status = json["status"].as_str().unwrap_or("no status given");
                println!("Started '{}/{}' rollback. Status is {}", self.namespace, self.name, status);
            },
            StatusCode::UnprocessableEntity => response.handle_unprocessable_entity(),
            status => println!("Could not roll buildpack back.\nReceived: {}, {}", status, response.body),
        }
    }
}
