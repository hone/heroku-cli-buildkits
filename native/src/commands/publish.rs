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

        let buildpack_id = format!("{}/{}", &self.namespace, &self.name);
        let encoded_buildpack = utf8_percent_encode(&buildpack_id, PATH_SEGMENT_ENCODE_SET);
        let body = json!(CreateRevisions::new(&self.tag));
        let url_path = format!("/buildpacks/{}/revisions", encoded_buildpack);
        let response = api.post_with_version(&url_path, "3.buildpack-registry", body).unwrap();
        match response.status {
            StatusCode::Ok => {
                let json = response.body;
                let blob_url = json["blob_url"].as_str().unwrap_or("no blob_url found");
                println!("Successfully published buildpack '{}/{}' for {}.\n{}", self.namespace, self.name, self.tag, blob_url);
            },
            StatusCode::UnprocessableEntity => response.handle_unprocessable_entity(),
            status => println!("Could not publish buildpack.\nReceived: {}, {}", status, response.body),
        }
    }
}
