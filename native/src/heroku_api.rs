extern crate netrc;
extern crate reqwest;
extern crate serde_json;

use std::error;
use std::env;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use self::netrc::Netrc;
use self::reqwest::header::{Headers, ContentLength};
use self::reqwest::{Method, StatusCode};

#[cfg(test)]
extern crate tempdir;

#[derive(Debug)]
pub enum HerokuApiError {
    Io(io::Error),
    Netrc(netrc::Error),
    Err(&'static str),
}

impl fmt::Display for HerokuApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HerokuApiError::Io(ref err) => write!(f, "IO error {}", err),
            HerokuApiError::Netrc(ref err) => match err {
                &netrc::Error::Io(ref err) => write!(f, "Netrc IO error {}", err),
                &netrc::Error::Parse(ref msg, ref lnum) => write!(f, "Netrc error, line: {}, error: {}", lnum, msg),
            },
            HerokuApiError::Err(ref err) => write!(f, "Err {}", err),
        }
    }
}

impl error::Error for HerokuApiError {
    fn description(&self) -> &str {
        match *self {
            HerokuApiError::Io(ref err) => err.description(),
            HerokuApiError::Netrc(ref err) => match err {
                &netrc::Error::Io(ref io_err) => &io_err.description(),
                &netrc::Error::Parse(ref msg, _) => msg,
            },
            HerokuApiError::Err(ref err) => err,
        }
    }
}

impl From<io::Error> for HerokuApiError {
    fn from(error: io::Error) -> Self {
        HerokuApiError::Io(error)
    }
}

impl From<netrc::Error> for HerokuApiError {
    fn from(error: netrc::Error) -> Self {
        HerokuApiError::Netrc(error)
    }
}

impl From<&'static str> for HerokuApiError {
    fn from(error: &'static str) -> Self {
        HerokuApiError::Err(error)
    }
}

mod vars {
    pub const BASE_URL: &'static str = "https://buildpack-registry.herokuapp.com";
}

pub struct Response {
    pub status: StatusCode,
    pub body: serde_json::Value,
}

impl Response {
    pub fn new(status: StatusCode, body: serde_json::Value) -> Self {
        Response {
            status: status,
            body: body,
        }
    }

    pub fn handle_unprocessable_entity(&self) {
        if let Some(object) = self.body.as_object() {
            for (key, value) in object {
                let error_message = match value.as_array() {
                    Some(array) => {
                        array.into_iter()
                            .map(|item| {
                                item.as_str()
                                    .unwrap_or("")
                            }).collect::<Vec<&str>>().join("\n")
                    },
                    None => value.as_str().unwrap_or("").to_string(),
                };

                eprintln!("{}: {}", key, error_message);
            }
        }
    }
}

pub struct HerokuApi {
    pub client: reqwest::Client,
    pub base_url: String,
}

impl HerokuApi {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::new_options(None)
    }

    #[allow(dead_code)]
    pub fn new_with_host(host: &str) -> Self {
        Self::new_options(Some(host))
    }

    fn new_options(host: Option<&str>) -> Self {
        let base_url = env::var("HEROKU_BUILDPACK_REGISTRY_URL").unwrap_or(host.unwrap_or(vars::BASE_URL).to_owned());
        HerokuApi {
            client: reqwest::Client::new().unwrap(),
            base_url: base_url,
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, uri: &str) -> Result<Response, HerokuApiError> {
        self.request(uri, Method::Get, None, None)
    }

    #[allow(dead_code)]
    pub fn get_with_version(&self, uri: &str, version: &str) -> Result<Response, HerokuApiError> {
        self.request(uri, Method::Get, Some(version), None)
    }

    #[allow(dead_code)]
    pub fn post(&self, uri: &str, body: serde_json::Value) -> Result<Response, HerokuApiError> {
        self.request(uri, Method::Post, None, Some(body))
    }

    #[allow(dead_code)]
    pub fn post_with_version(&self, uri: &str, version: &str, body: serde_json::Value) -> Result<Response, HerokuApiError> {
        self.request(uri, Method::Post, Some(version), Some(body))
    }

    fn request(&self, uri: &str, method: Method, version: Option<&str>, body: Option<serde_json::Value>) -> Result<Response, HerokuApiError> {
        let uri = format!("{}{}", self.base_url, uri);
        let mut req = self.client.request(method, &uri).unwrap();
        let netrc_path = Self::default_netrc_path()?;
        let token = Self::fetch_credentials(netrc_path)?;
        let headers = Self::construct_headers(&token, version);
        let req = req.headers(headers);

        let req = match body {
            Some(json) => {
                let json_string = json.to_string();
                req.header(ContentLength(json_string.len() as u64))
                    .body(json_string)
            },
            None => req,
        };

        let mut response = req.send().unwrap();
        let status = response.status();
        match status {
            StatusCode::Ok => {
                Ok(Response::new(status, response.json().unwrap()))
            },
            _ => {
                let body = response.json();
                match body {
                    Ok(json_body) => Ok(Response::new(status, json_body)),
                    Err(_) => {
                        let mut string_body = String::new();
                        response.read_to_string(&mut string_body).unwrap();
                        Ok(Response::new(status, json!({"error": string_body})))
                    },
                }
            },
        }
    }

    fn construct_headers(auth_token: &str, param_version: Option<&str>) -> Headers {
        let mut headers = Headers::new();
        let version = param_version.unwrap_or("3");
        headers.set_raw("Accept", format!("application/vnd.heroku+json; version={}", version));
        headers.set_raw("Content-Type", "application/json");
        headers.set_raw("Authorization", format!("Bearer {}", auth_token));

        headers
    }

    fn default_netrc_path() -> Result<PathBuf, HerokuApiError> {
        let mut netrc_path = env::home_dir().ok_or("Impossible to get your home directory")?;
        netrc_path.push(".netrc");

        Ok(netrc_path)
    }

    fn fetch_credentials<P: AsRef<Path>>(file_path: P) -> Result<String, HerokuApiError> {
        let file = File::open(file_path)?;
        let input = BufReader::new(file);
        let netrc = Netrc::parse(input)?;
        let machine = netrc.hosts.into_iter().find(|host| {
            let hostname = &host.0;
            hostname == "api.heroku.com"
        }).ok_or("api.heroku.com not found")?.1;

        Ok(machine.password.ok_or("no password found for api.heroku.com")?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;
    use std::ops::Deref;
    use self::reqwest::header;
    use self::tempdir::TempDir;

    #[test]
    fn construct_headers_default_version() {
        let uri = "https://www.google.com";
        let token = String::from("e1bd3f9535a2ed54684ec2af0190e3844aaec8b8");

        let headers = HerokuApi::construct_headers(&token, None);

        assert_eq!(headers.get::<header::Authorization<header::Bearer>>().unwrap().deref().token, token);
        assert_eq!(headers.get::<header::Accept>().unwrap()[0].item, "application/vnd.heroku+json; version=3")
    }

    #[test]
    fn construct_headers_version() {
        let uri = "https://www.google.com";
        let token = String::from("e1bd3f9535a2ed54684ec2af0190e3844aaec8b8");
        let version = String::from("3.buildpack-registry");

        let headers = HerokuApi::construct_headers(&token, Some(&version));

        let ref accept = headers.get::<header::Accept>().unwrap()[0];
        let expected: &str = &format!("application/vnd.heroku+json; version={}", version);
        assert_eq!(accept.item, expected);
    }

    #[test]
    fn fetch_credentials() {
        let tmpdir = TempDir::new("test").unwrap();
        let netrc_filepath = tmpdir.path().join(Path::new(".netrc"));
        let password = "e1bd3f9535a2ed54684ec2af0190e3844aaec8b8";
        let mut f = File::create(&netrc_filepath).unwrap();
        f.write_all(format!("
machine api.heroku.com
  login terence@heroku.com
  password {}
",
        password).as_bytes()).unwrap();
        assert_eq!(password, HerokuApi::fetch_credentials(&netrc_filepath).unwrap());
    }
}
