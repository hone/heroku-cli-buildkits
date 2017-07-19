use options::Options;
use heroku_api::HerokuApi;

pub struct Search {
    name: String,
}

impl Search {
    pub fn new(options: Options) -> Self {
        Search { name: options.arg_name }
    }

    pub fn execute(self) {
        let api = HerokuApi::new();
        api.get_with_version("/buildpacks", "3.buildpack-registry");
    }
}
