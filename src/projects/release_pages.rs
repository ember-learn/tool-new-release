use utils::TaskType;

use crate::utils::{self, prompt};

pub fn run() {
    prompt(
        TaskType::Manual,
        "Go to https://github.com/ember-learn/ember-website.",
    );
    prompt(
        TaskType::Manual,
        "Edit the following files:
1. data/project/ember/lts.md
2. data/project/ember/release.md
3. data/project/ember/beta.md
4. data/project/emberData/release.md
5. data/project/emberData/beta.md",
    );
}
