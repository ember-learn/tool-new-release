use crate::utils::prompt::manual;

pub fn run() {
    manual("Go to https://github.com/ember-learn/ember-website.");
    manual(
        "Edit the following files:
1. data/project/ember/lts.md
2. data/project/ember/release.md
3. data/project/ember/beta.md
4. data/project/emberData/release.md
5. data/project/emberData/beta.md",
    );
}
