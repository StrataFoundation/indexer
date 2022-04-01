use super::prelude::*;

#[derive(Debug, Clone)]
pub struct TwitterProfile {
    pub avatar_url: String,
    pub banner_url: String,
    pub handle: String,
}

#[graphql_object(Context = AppContext)]
impl TwitterProfile {
    pub fn handle(&self) -> &str {
        &self.handle
    }

    pub fn avatar_url(&self) -> &str {
        &self.avatar_url
    }

    pub fn banner_url(&self) -> &str {
        &self.banner_url
    }
}
