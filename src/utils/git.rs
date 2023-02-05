use std::path::PathBuf;
use tempdir::TempDir;
use anyhow::Result;
use git2::Repository;

pub struct GitRepository {
    path: TempDir,
    repo: Repository,
}

impl GitRepository {
    pub fn clone(url: &str) -> Result<GitRepository> {
        let path = TempDir::new("autooverlay")?;
        let repo = Repository::clone(url, path.path())?;
        Ok(GitRepository {
            path,
            repo
        })
    }

    pub fn checkout(&self, rev: &str) -> Result<()> {
        let (object, reference) = self.repo.revparse_ext(&rev.to_string())?;
        self.repo.checkout_tree(&object, None)?;
        match reference {
            // gref is an actual reference like branches or tags
            Some(gref) => self.repo.set_head(gref.name().unwrap()),
            // this is a commit, not a reference
            None => self.repo.set_head_detached(object.id()),
        }?;
        Ok(())
    }

    pub fn path(&self) -> PathBuf {
        self.path.path().to_path_buf()
    }
}