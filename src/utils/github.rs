use std::sync::Arc;
use octocrab::Octocrab;
use crate::packages::Version;
use anyhow::Result;

#[derive(Clone)]
pub struct GithubClient {
    client: Arc<Octocrab>
}

impl GithubClient {
    pub fn new() -> Self {
        GithubClient {
            client: octocrab::instance()
        }
    }

    pub async fn get_versions(&self, owner: &str, repo: &str, count: u8) -> Result<Vec<Version>> {
        Ok(self.client.repos(owner, repo)
            .releases()
            .list()
            .per_page(count)
            .page(0u32)
            .send()
            .await?
            .items
            .into_iter()
            .map(|e| Version::parse_semver(&e.name.unwrap()).unwrap())
            .collect())
    }
}