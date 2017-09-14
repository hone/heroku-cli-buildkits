extern crate reqwest;
extern crate serde;
extern crate url;

use std::collections::VecDeque;
use heroku_api::HerokuApi;
use self::reqwest::StatusCode;
use self::url::{Url};

#[derive(Serialize, Deserialize)]
struct CreateBuildpacks {
    name: String,
    namespace: String,
    source: CreateBuildpacksSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<CreateBuildpacksOwner>,
}

impl CreateBuildpacks {
    pub fn new(name: &str, namespace: &str, repo_string: &str, team: Option<String>) -> Self {
        let owner = team.map(|team_name| CreateBuildpacksOwner::new(&team_name, "team"));

        CreateBuildpacks {
            name: name.to_owned(),
            namespace: namespace.to_owned(),
            source: CreateBuildpacksSource::new(repo_string),
            owner: owner,
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
        let repo_url = Url::parse(repo_string).unwrap();
        let mut parts: VecDeque<&str> = repo_url.path_segments().unwrap().collect();
        let owner = parts.pop_front().unwrap();
        let repo = Vec::from(parts).join("/").replace(".git", "");
        CreateBuildpacksSource {
            type_name: "github".to_owned(),
            owner: owner.to_owned(),
            repo: repo,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CreateBuildpacksOwner {
    name: String,
    #[serde(rename = "type")]
    owner_type: String,
}

impl CreateBuildpacksOwner {
    pub fn new(name: &str, owner_type: &str) -> Self {
        CreateBuildpacksOwner {
            name: name.to_owned(),
            owner_type: owner_type.to_owned(),
        }
    }
}

pub struct Register {
    pub repo: String,
    pub namespace: String,
    pub name: String,
    pub team: Option<String>,
}

impl Register {
    pub fn execute(self) {
        let api = HerokuApi::new();
        let body = json!(CreateBuildpacks::new(&self.name, &self.namespace, &self.repo, self.team));
        let response = api.post_with_version("/buildpacks", "3.buildpack-registry", body).unwrap();

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
