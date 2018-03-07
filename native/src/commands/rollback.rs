extern crate reqwest;
extern crate serde;

use heroku_api::HerokuApi;
use self::reqwest::StatusCode;

pub struct Rollback {
    pub namespace: String,
    pub name: String,
}

impl Rollback {
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
                    let body = json!({});
                    let url_path = format!("/buildpacks/{}/actions/rollback", uuid);
                    let response = api.post_with_version(&url_path, "3.buildpack-registry", body).unwrap();
                    match response.status {
                        StatusCode::Accepted => {
                            let json = response.body;
                            println!("Successfully rolled buildpack back'{}/{}'", self.namespace, self.name);
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
