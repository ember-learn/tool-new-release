use semver::Version;
use utils::TaskType;

use crate::utils::{self, prompt};

pub fn run(version: &Version) {
    prompt(TaskType::Manual, "Go to #core-meta on the Ember Discord.");
    prompt(
        TaskType::Manual,
        "Mark current release done with `!release done blog`.",
    );
    let deadline = ask_next_deadline(&version);

    prompt(
        TaskType::Manual,
        format!(
            "Schedule next release with `!release next {} {}`.",
            version, deadline
        )
        .as_str(),
    )
}

fn ask_next_deadline(target: &Version) -> String {
    let input: String = dialoguer::Input::new()
        .with_prompt(format!("When is {} scheduled to release (YYYY-MM-DD)", target))
        .interact_text()
        .unwrap();
    println!();

    input
}
