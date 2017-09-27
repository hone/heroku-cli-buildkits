extern crate percent_encoding;
extern crate reqwest;
extern crate serde;

use heroku_api::HerokuApi;
//use self::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};
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

        let response = &api.get_with_version("/buildpacks", "3.buildpack-registry").unwrap();
        if let StatusCode::Ok = response.status {
            let json = &response.body;
            let buildpack = json
                .as_array()
                .unwrap()
                .into_iter()
                .find(|buildpack| {
                    buildpack["namespace"] == self.namespace && buildpack["name"] == self.name
                });
            if let Some(buildpack) = buildpack {
                    let uuid = buildpack["id"].as_str().unwrap();
                    let body = json!(CreateRevisions::new(&self.tag));
                    let url_path = format!("/buildpacks/{}/revisions", uuid);
                    let response = api.post_with_version(&url_path, "3.buildpack-registry", body).unwrap();
                    match response.status {
                        StatusCode::Ok => {
                            let json = response.body;
                            let s3_url = json["s3_http_url"].as_str().unwrap();
                            println!("Successfully published buildpack '{}/{}' for {}.\n{}", self.namespace, self.name, self.tag, s3_url);
                        },
                        StatusCode::UnprocessableEntity => response.handle_unprocessable_entity(),
                        status => println!("Could not publish buildpack.\nReceived: {}, {}", status, response.body),
                    }
            } else {
                eprintln!("No buildpack found with that namespace/name: {}/{}", self.namespace, self.name)
            }
        } else {
            eprintln!("Could not get buildpack uuid.\nReceived: {}, {}", response.status, response.body)
        }
    }
}
