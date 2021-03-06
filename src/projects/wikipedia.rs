use std::str::from_utf8;

use chrono::format::StrftimeItems;
use serde_json::Value;
use utils::TaskType;

use crate::utils::{self, prompt};

pub fn run() {
    let (stable_version, stable_date) = get_version_modified_pair("ember-source@latest");
    let (preview_version, preview_date) = get_version_modified_pair("ember-source@beta");
    let current_date = chrono::offset::Utc::now().date().format("%d %B %Y");

    let infobox = format!(
        "| latest release version = {stable_version}
| latest release date    = {{{{Start date and age|{stable_date}}}}}<ref name=\"versions\">{{{{cite web|title=Releases · emberjs/ember.js|url=https://github.com/emberjs/ember.js/releases|website=[[GitHub]]|access-date={access_date}|language=en}}}}</ref>
| latest preview version = {preview_version}
| latest preview date    = {{{{Start date and age|{preview_date}}}}}<ref name=\"versions\" />",
        stable_version = stable_version,
        stable_date = stable_date,
        preview_version = preview_version,
        preview_date = preview_date,
        access_date = current_date
    );

    prompt(
        TaskType::Manual,
        "Go to https://en.wikipedia.org/w/index.php?title=Ember.js&action=edit.\nWe suggest you log in.",
    );

    prompt(
        TaskType::Manual,
        format!(
            "Replace the relevant release section of the Infobox with the following:\n\n{}\n",
            infobox
        )
        .as_str(),
    );

    prompt(
        TaskType::Manual,
        "Write a small summary and publish your changes!",
    );
}

fn get_version_modified_pair(
    package: &str,
) -> (
    semver::Version,
    chrono::format::DelayedFormat<StrftimeItems>,
) {
    let output = std::process::Command::new("npm")
        .args(&["show", package, "version"])
        .output()
        .expect("Could not find latest ember-source version.");
    let version = from_utf8(&output.stdout).unwrap().trim();

    let time_output = std::process::Command::new("npm")
        .args(&["show", package, "time", "--json"])
        .output()
        .expect("Could not find latest ember-source version.");
    let times_json = from_utf8(&time_output.stdout).unwrap();
    let times: Value = serde_json::from_str(times_json).unwrap();
    let modified_str = times[version].as_str().unwrap();
    let modified_date = chrono::DateTime::parse_from_rfc3339(modified_str)
        .unwrap()
        .date();
    let modified_formatted = modified_date.format("%Y|%m|%d");

    (semver::Version::parse(version).unwrap(), modified_formatted)
}
