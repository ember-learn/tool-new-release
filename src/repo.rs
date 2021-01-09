use std::path::PathBuf;

use git2::Repository;

trait PushPop {
    fn map<T, F>(&mut self, path: &str, f: F) -> T
    where
        F: Fn(&Self) -> T;
}

impl PushPop for std::path::PathBuf {
    fn map<T, F>(&mut self, path: &str, f: F) -> T
    where
        F: Fn(&Self) -> T,
    {
        self.push(path);
        let r = f(&self);
        self.pop();
        r
    }
}

pub struct Repo<'a> {
    pub organization: &'a str,
    pub project: &'a str,
    pub url: Option<&'a str>
}

impl Repo<'_> {
    pub fn clone(&self, folder: &mut PathBuf) -> Repository {
        let github = format!("https://github.com/{}/{}.git", self.organization, self.project);

        let repo_url = match self.url {
            Some(url) => url,
            None => github.as_str()
        };

        folder
            .map(self.project, |f| {
                git2::Repository::clone(&repo_url, f)
            })
            .expect(format!("Could not clone {}/{}", self.organization, self.project).as_str())
    }
}
