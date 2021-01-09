use utils::TaskType;

use crate::utils::{self, prompt};

pub fn run() {
    prompt(
        TaskType::Manual,
        "Go to https://en.wikipedia.org/wiki/Ember.js",
    );
    prompt(
        TaskType::Manual,
        "Click the Edit tab. We suggest you log in.",
    );
    prompt(
        TaskType::Manual,
        "Update the following in the Infobox:
1. latest release version (including access-date)
2. latest release date
3. latest preview version
4. latest preview date",
    );
    prompt(TaskType::Manual, "Save!");
}
