use std::path::Path;
use async_trait::async_trait;
use crate::packages::{Package, PackageExpander, Version};
use crate::utils::git::GitRepository;
use anyhow::Result;
use crate::utils::github::GithubClient;
use crate::utils::rust::CargoPackageExpander;

pub struct SentryCliPackage {}

impl SentryCliPackage {
    pub fn new() -> Self {
        SentryCliPackage {}
    }
}

#[async_trait]
impl Package for SentryCliPackage {
    type Expander = SentryCliPackageExpander;

    fn group(&self) -> String {
        "dev-util".to_string()
    }

    fn name(&self) -> String {
        "sentry-cli".to_string()
    }

    fn description(&self) -> String {
        "This is a Sentry command line client for some generic tasks.".to_string()
    }

    async fn versions(&self, max_count: u8) -> Result<Vec<Version>> {
        GithubClient::new().get_versions("getsentry", "sentry-cli", max_count).await
    }

    async fn new_expander(&self) -> Result<Self::Expander> {
        let repo = GitRepository::clone("https://github.com/getsentry/sentry-cli.git")?;
        Ok(SentryCliPackageExpander { repo })
    }
}

pub struct SentryCliPackageExpander {
    repo: GitRepository,
}

#[async_trait]
impl PackageExpander for SentryCliPackageExpander {
    async fn expand(&mut self, version: Version, path: &Path) -> Result<()> {
        self.repo.checkout(&version.to_string())?;
        CargoPackageExpander::new()
            .description("This is a Sentry command line client for some generic tasks.")
            .homepage("https://docs.sentry.io/product/cli/")
            .license("BSD-3")
            .expand(&self.repo.path(), path)
    }
}