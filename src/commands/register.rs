use options::Options;
use heroku_api::HerokuApi;

pub struct Register {
    repo: String,
}

impl Register {
    pub fn new(options: Options) -> Register {
        Register { repo: options.arg_repo }
    }

    pub fn execute(self) {
        let api = HerokuApi::new();
        api.get(String::from("/apps"), None);
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
