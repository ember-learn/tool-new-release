use utils::TaskType;

use crate::utils::{self, prompt};

pub fn run() {
    prompt(
        TaskType::Manual,
        "Go to https://github.com/ember-learn/ember-blog/pulls.",
    );
    prompt(
        TaskType::Manual,
        "Merge the relevant blog post if all the teams have approved it.",
    );
}
