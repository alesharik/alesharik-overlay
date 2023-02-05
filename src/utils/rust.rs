use std::path::Path;
use anyhow::Result;
use cargo_ebuild::{gen_ebuild_data, write_ebuild};

#[derive(Default)]
pub struct CargoPackageExpander {
    description: Option<String>,
    homepage: Option<String>,
    license: Option<String>
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
        ebuild_data.crates.push(format!("\t{}-{}\n", &ebuild_data.name, &ebuild_data.version));

        write_ebuild(ebuild_data, dest, None)?;
        Ok(())
    }
}