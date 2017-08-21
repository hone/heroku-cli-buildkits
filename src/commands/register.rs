extern crate hyper;
extern crate serde;

use std::collections::VecDeque;
use options::Options;
use heroku_api::HerokuApi;
use self::hyper::StatusCode;

#[derive(Serialize, Deserialize)]
struct CreateBuildpacks {
    name: String,
    source: CreateBuildpacksSource,
    owner: CreateBuildpacksOwner,
}

impl CreateBuildpacks {
    pub fn new(name: &str, repo_string: &str, owner_id: &str, owner_type: &str) -> Self {
        CreateBuildpacks {
            name: name.to_owned(),
            source: CreateBuildpacksSource::new(repo_string),
            owner: CreateBuildpacksOwner::new(owner_id, owner_type),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CreateBuildpacksSource {
    #[serde(rename = "type")]
    type_name: String,
    owner: String,
    repo: String,
}

impl CreateBuildpacksSource {
    pub fn new(repo_string: &str) -> Self {
        let mut parts: VecDeque<&str> = repo_string.split("/").collect();
        let owner = parts.pop_front().unwrap();
        let repo = Vec::from(parts).join("/");
        CreateBuildpacksSource {
            type_name: "github".to_owned(),
            owner: owner.to_owned(),
            repo: repo,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CreateBuildpacksOwner {
    id: String,
    #[serde(rename = "type")]
    owner_type: String,
}

impl CreateBuildpacksOwner {
    pub fn new(id: &str, owner_type: &str) -> Self {
        CreateBuildpacksOwner {
            id: id.to_owned(),
            owner_type: owner_type.to_owned(),
        }
    }
}

pub struct Register {
    repo: String,
    namespace: String,
    name: String,
}

impl Register {
    pub fn new(options: Options) -> Register {
        Register {
            name: options.arg_name,
            namespace: options.arg_namespace,
            repo: options.arg_repo,
        }
    }

    pub fn execute(self) {
        let api = HerokuApi::new();
        let local_api = HerokuApi::new_with_host("http://localhost:3000");
        let account_response = api.get("/account").unwrap();
        let owner_uuid = &account_response.body["id"].as_str().unwrap();
        let body = json!(CreateBuildpacks::new(&self.name, &self.repo, owner_uuid, "user"));
        let response = local_api.post_with_version(&format!("/buildpacks/{}", &self.namespace), "3.buildpack-registry", body).unwrap();

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

#[cfg(test)]
mod test {
    use super::*;
}
