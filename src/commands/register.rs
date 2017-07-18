extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate netrc;
extern crate tokio_core;

#[cfg(test)]
extern crate tempdir;

use options::Options;
use heroku_api::HerokuApi;
use std::env;
use std::io::{self, Write};
use self::futures::{Future, Stream};
use self::hyper::{Client, Method, Request};
use self::hyper_tls::HttpsConnector;
use self::tokio_core::reactor::Core;

pub struct Register {
    repo: String,
}

impl Register {
    pub fn new(options: Options) -> Register {
        Register { repo: options.arg_repo }
    }

    pub fn execute(self) {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .build(&handle);
        let uri = "https://api.heroku.com/apps".parse().unwrap();
        let mut req = Request::new(Method::Get, uri);
        {
            let mut netrc_path = env::home_dir().unwrap();
            netrc_path.push(".netrc");
            let mut headers = req.headers_mut();
            headers.set_raw("Accept", "application/vnd.heroku+json; version=3");
            let api = HerokuApi { };
            let credentials = api.fetch_credentials(netrc_path).unwrap();
            headers.set_raw("Authorization", format!("Bearer {}", credentials));
        }
        let work = client.request(req).and_then(|res| {
            println!("Response: {}", res.status());
            res.body().for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map(|_| ())
                    .map_err(From::from)
            })
        });
        core.run(work).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
