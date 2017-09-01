#[derive(Serialize, Deserialize)]
struct CreateRevisions {
    buildpack: CreateRevisionsBuildpack,
    published_by: CreateRevisionsPublishedBy,
    tag: String,
}

impl CreateRevisions {
    pub fn new(buildpack_id: &str, published_by_id: &str, published_by_email: &str, tag: &str) -> Self {
        CreateRevisions {
            buildpack: CreateRevisionsBuildpack::new(buildpack_id),
            published_by: CreateRevisionsPublishedBy::new(published_by_id, published_by_email),
            tag: tag.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CreateRevisionsBuildpack {
    id: String,
}

impl CreateRevisionsBuildpack {
    pub fn new(id: &str) -> Self {
        CreateRevisionsBuildpack {
            id: id.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CreateRevisionsPublishedBy {
    id: String,
    email: String,
}

impl CreateRevisionsPublishedBy {
    pub fn new(id: &str, email: &str) -> Self {
        CreateRevisionsPublishedBy {
            id: id.to_owned(),
            email: email.to_owned(),
        }
    }
}

pub struct Publish {
    name: String,
    treeish: String,
}

impl Publish {
    pub fn execute(self) {
        let api = HerokuApi::new();
    }
}
