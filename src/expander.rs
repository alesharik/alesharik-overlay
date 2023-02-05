use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::packages::{Package, PackageExpander};
use anyhow::Result;
use handlebars::Handlebars;
use tracing::{debug, info, instrument};

pub struct Expander {
    target: PathBuf,
    max_versions: u8,
}

impl Expander {
    pub fn new(target: &Path) -> Self {
        Expander {
            target: target.to_owned(),
            max_versions: 3,
        }
    }

    fn write_metadata<P: Package>(&self, path: &Path, package: &P) -> Result<()> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("metadata", include_str!("metadata.xml"))?;

        debug!("Writing metadata into {:?}", path);
        let mut data = BTreeMap::new();
        data.insert("description", package.description());
        let manifest = handlebars.render("metadata", &data)?;
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.join("metadata.xml"))?
            .write(manifest.as_bytes())?;
        Ok(())
    }

    #[instrument(skip_all, fields(group = package.group(), name = package.name()))]
    pub async fn expand<P: Package>(&self, package: &P) -> Result<()> {
        info!("Expanding {}/{}", package.group(), package.name());
        let path = self.target.join(package.group()).join(package.name());
        std::fs::create_dir_all(&path)?;
        self.write_metadata(&path, package)?;
        let versions = package.versions(self.max_versions).await?;
        debug!("Got versions {:?}", versions);
        let mut expander = package.new_expander().await?;
        for version in versions {
            let ebuild = path.join(format!("{}-{}.ebuild", package.name(), &version));
            info!("Expanding version {:?}", &version);
            expander.expand(version, &ebuild).await?;
        }

        Ok(())
    }
}