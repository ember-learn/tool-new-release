use crate::utils::prompt::manual;
use semver::Version;

pub fn run(version: &Version) {
    manual("Go to #core-meta on the Ember Discord.");
    manual("Mark current release done with `!release done blog`.");
    // let deadline = ask_next_deadline(version);

    manual(
        format!(
            "Schedule next release with `!release next {} {}`.",
            version, "soon"
        )
        .as_str(),
    )
}

// fn ask_next_deadline(target: &Version) -> String {
//     let input: String = dialoguer::Input::new()
//         .with_prompt(format!(
//             "When is {} scheduled to release (YYYY-MM-DD)",
//             target
//         ))
//         .interact_text()
//         .unwrap();
//     println!();

//     input
// }
