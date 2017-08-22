extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate netrc;
extern crate serde_json;
extern crate tokio_core;

use std::error;
use std::env;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use self::netrc::Netrc;
use self::futures::{Future, Stream};
use self::hyper::{Body, Client, Method, Request, header};
use self::hyper::client::HttpConnector;
use self::hyper::error::UriError;
use self::hyper_tls::HttpsConnector;
use self::tokio_core::reactor::Core;

#[cfg(test)]
extern crate tempdir;

#[derive(Debug)]
pub enum HerokuApiError {
    Io(io::Error),
    Netrc(netrc::Error),
    Uri(UriError),
    Hyper(hyper::Error),
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
            HerokuApiError::Uri(ref err) => write!(f, "URI error {}", err),
            HerokuApiError::Hyper(ref err) => write!(f, "Hyper error {}", err),
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
            HerokuApiError::Hyper(ref err) => err.description(),
            HerokuApiError::Uri(ref err) => err.description(),
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

impl From<UriError> for HerokuApiError {
    fn from(error: UriError) -> Self {
        HerokuApiError::Uri(error)
    }
}

impl From<hyper::Error> for HerokuApiError {
    fn from(error: hyper::Error) -> Self {
        HerokuApiError::Hyper(error)
    }
}

impl From<&'static str> for HerokuApiError {
    fn from(error: &'static str) -> Self {
        HerokuApiError::Err(error)
    }
}

mod vars {
    pub const BASE_URL: &'static str = "https://api.heroku.com";
}

pub struct Response {
    pub status: hyper::StatusCode,
    pub body: serde_json::Value,
}

impl Response {
    pub fn new(status: hyper::StatusCode, body: serde_json::Value) -> Self {
        Response {
            status: status,
            body: body,
        }
    }
}

pub struct HerokuApi {
    pub core: Core,
    pub client: Client<HttpsConnector<HttpConnector>, Body>,
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
        let core = Core::new().unwrap();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &core.handle()).unwrap())
            .build(&core.handle());

        HerokuApi {
            core: core,
            client: client,
            base_url: host.unwrap_or(vars::BASE_URL).to_owned(),
        }
    }

    #[allow(dead_code)]
    pub fn get(self, uri: &str) -> Result<Response, HerokuApiError> {
        self.request(uri, Method::Get, None, None)
    }

    #[allow(dead_code)]
    pub fn get_with_version(self, uri: &str, version: &str) -> Result<Response, HerokuApiError> {
        self.request(uri, Method::Get, Some(version), None)
    }

    #[allow(dead_code)]
    pub fn post(self, uri: &str, body: serde_json::Value) -> Result<Response, HerokuApiError> {
        self.request(uri, Method::Post, None, Some(body))
    }

    #[allow(dead_code)]
    pub fn post_with_version(self, uri: &str, version: &str, body: serde_json::Value) -> Result<Response, HerokuApiError> {
        self.request(uri, Method::Post, Some(version), Some(body))
    }

    fn request(self, uri: &str, method: Method, version: Option<&str>, body: Option<serde_json::Value>) -> Result<Response, HerokuApiError> {
        let uri = format!("{}{}", self.base_url, uri).parse()?;
        let mut req = Request::new(method, uri);
        let netrc_path = Self::default_netrc_path()?;
        let token = Self::fetch_credentials(netrc_path)?;
        Self::setup_headers(&mut req, &token, version);

        match body {
            Some(json) => {
                let json_string = json.to_string();
                req.headers_mut().set(header::ContentLength(json_string.len() as u64));
                req.set_body(json_string)
            },
            None => (),
        }

        let work = self.client.request(req).and_then(move |res| {
            let status = res.status();
            res.body().concat2().and_then(move |body| {
                let json: serde_json::Value = serde_json::from_slice(&body).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        e
                    )
                })?;
                Ok(Response::new(status, json))
            })
        });

        let mut core = self.core;
        let response = core.run(work).unwrap_or(Response::new(hyper::StatusCode::default(), json!(null)));

        Ok(response)
    }

    fn setup_headers(req: &mut Request, auth_token: &str, param_version: Option<&str>) {
        let mut headers = req.headers_mut();
        let version = param_version.unwrap_or("3");
        headers.set_raw("Accept", format!("application/vnd.heroku+json; version={}", version));
        headers.set_raw("Content-Type", "application/json");
        headers.set_raw("Authorization", format!("Bearer {}", auth_token));
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
    use self::tempdir::TempDir;
    use self::hyper::header;

    #[test]
    fn setup_headers_default_version() {
        let uri = "https://www.google.com".parse().unwrap();
        let mut request = Request::new(Method::Get, uri);
        let token = String::from("e1bd3f9535a2ed54684ec2af0190e3844aaec8b8");

        HerokuApi::setup_headers(&mut request, &token, None);

        let headers = request.headers();
        assert_eq!(headers.get::<header::Authorization<header::Bearer>>().unwrap().deref().token, token);
        assert_eq!(headers.get::<header::Accept>().unwrap()[0].item, "application/vnd.heroku+json; version=3")
    }

    #[test]
    fn setup_headers_version() {
        let uri = "https://www.google.com".parse().unwrap();
        let mut request = Request::new(Method::Get, uri);
        let token = String::from("e1bd3f9535a2ed54684ec2af0190e3844aaec8b8");
        let version = String::from("3.buildpack-registry");

        HerokuApi::setup_headers(&mut request, &token, Some(&version));

        let headers = request.headers();
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