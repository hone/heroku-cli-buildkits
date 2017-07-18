extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate netrc;
extern crate tokio_core;

use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{BufReader, Write};
use std::path::Path;
use self::netrc::Netrc;

#[cfg(test)]
extern crate tempdir;

mod vars {
    const BASE_URL: &'static str = "https://api.heroku.com";
}

#[derive(Debug)]
enum HerokuApiError {
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

pub struct HerokuApi {
}

impl HerokuApi {
    pub fn fetch_credentials<P: AsRef<Path>>(self, file_path: P) -> Result<String, HerokuApiError> {
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
    use self::tempdir::TempDir;

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
        let api = HerokuApi { };
        assert_eq!(password, api.fetch_credentials(&netrc_filepath).unwrap());
    }
}
