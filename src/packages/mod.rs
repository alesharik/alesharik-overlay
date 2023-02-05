use std::fmt::{Display, Formatter};
use std::path::Path;
use anyhow::Result;
use async_trait::async_trait;

mod sentry_cli;

pub use sentry_cli::SentryCliPackage;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Version {
    Source,
    Semver(semver::Version),
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::Source => write!(f, "9999"),
            Version::Semver(version) => write!(f, "{}", version)
        }
    }
}

impl Version {
    pub fn parse_semver(version: &str) -> Result<Version> {
        Ok(Version::Semver(semver::Version::parse(version)?))
    }
}

#[async_trait]
pub trait Package {
    type Expander: PackageExpander;

    fn group(&self) -> String;

    fn name(&self) -> String;

    fn description(&self) -> String;

    async fn versions(&self, max_count: u8) -> Result<Vec<Version>>;

    async fn new_expander(&self) -> Result<Self::Expander>;
}

#[async_trait]
pub trait PackageExpander {
    async fn expand(&mut self, version: Version, path: &Path) -> Result<()>;
}