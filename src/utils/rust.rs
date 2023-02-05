use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use anyhow::Result;
use cargo_ebuild::{gen_ebuild_data, write_ebuild};
use tempdir::TempDir;

#[derive(Default)]
pub struct CargoPackageExpander {
    description: Option<String>,
    homepage: Option<String>,
    license: Option<String>,
    template: Option<String>,
}

impl CargoPackageExpander {
    pub fn new() -> Self {
        CargoPackageExpander::default()
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn homepage(mut self, homepage: &str) -> Self {
        self.homepage = Some(homepage.to_string());
        self
    }

    pub fn license(mut self, license: &str) -> Self {
        self.license = Some(license.to_string());
        self
    }

    pub fn template(mut self, template: &str) -> Self {
        self.template = Some(template.to_string());
        self
    }

    pub fn expand(mut self, crate_src: &Path, dest: &Path) -> Result<()> {
        let mut ebuild_data = gen_ebuild_data(Some(&crate_src.join("Cargo.toml")), None, false)?;

        if let Some(desc) = self.description.take() {
            ebuild_data.description = desc;
        }
        if let Some(homepage) = self.homepage.take() {
            ebuild_data.homepage = homepage;
        }
        if let Some(license) = self.license.take() {
            ebuild_data.license = license;
        }
        if let Some(template) = self.template.take() {
            let template_dir = TempDir::new("autooverlaytemp").unwrap();
            let file = template_dir.path().join("template.tera");
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&file)?
                .write(template.as_bytes())?;
            write_ebuild(ebuild_data, dest, Some(&file))?;
        } else {
            write_ebuild(ebuild_data, dest, None)?;
        }

        Ok(())
    }
}