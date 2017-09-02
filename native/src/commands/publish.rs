extern crate percent_encoding;
extern crate reqwest;
extern crate serde;

use heroku_api::HerokuApi;
use self::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};
use self::reqwest::StatusCode;

#[derive(Serialize, Deserialize)]
struct CreateRevisions {
    tag: String,
}

impl CreateRevisions {
    pub fn new(tag: &str) -> Self {
        CreateRevisions {
            tag: tag.to_owned(),
        }
    }
}

pub struct Publish {
    pub namespace: String,
    pub name: String,
    pub tag: String,
}

impl Publish {
    pub fn execute(self) {
        let api = HerokuApi::new();
        let body = json!(CreateRevisions::new(&self.tag));
        let buildpack = format!("{}/{}", &self.namespace, &self.name);
        let url_path = format!("/buildpacks/{}/revisions", utf8_percent_encode(&buildpack, PATH_SEGMENT_ENCODE_SET).to_string());
        let response = api.post_with_version(&url_path, "3.buildpack-registry", body).unwrap();

        match response.status {
            StatusCode::Ok => {
                let json = response.body;
                let name = json["name"].as_str().unwrap();
                println!("Successfully registered buildpack '{}'.", name);
            },
            StatusCode::UnprocessableEntity => {
                let json = response.body;
                for entry in json.as_object().unwrap() {
                    let (key, value) = entry;
                    let error_message = value.as_array().unwrap().into_iter().map(|item| item.as_str().unwrap()).collect::<Vec<&str>>().join("\n");
                    println!("{} {}", key, error_message);
                }
            },
            status => println!("Could not register buildpack.\nReceived: {}, {}", status, response.body),
        }
    }
}
