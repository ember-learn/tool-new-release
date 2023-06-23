use std::str::from_utf8;

use crate::utils::prompt::manual;
use chrono::format::StrftimeItems;
use serde_json::Value;

pub fn run() {
    let (preview_version, preview_date) = get_version_modified_pair("ember-source@beta");
    let today = chrono::offset::Local::now().format("%-d %B %Y");

    let infobox = format!(
        "| latest preview version = {preview_version}
| latest preview date    = {{{{Start date and age|{preview_date}}}}}<ref name=\"versions\">{{{{cite web|title=Releases · emberjs/ember.js|url=https://github.com/emberjs/ember.js/releases|website=[[GitHub]]|access-date={today}|language=en}}}}</ref>",
        preview_version = preview_version,
        preview_date = preview_date,
        today = today
    );

    manual("Go to the wiki data that the wikipedia page loads in: https://www.wikidata.org/wiki/Q13592527.\nWe suggest you log in.",
    );

    manual(
        format!(
            "At the botton of the giant list of versions, add a new entry. See earlier entries to find out what data to include:\n\n{}\n",
            infobox
        )
        .as_str(),
    );
    
    manual("While you have the data record open for editing, click the up arrow to mark it as 'preferred.' Edit the previous release version, and click the center dot to mark it as 'normal' rank. Make sure to publish your changes.",
    );
    
    manual("Visit https://en.wikipedia.org/w/index.php?title=Ember.js to see if your changes show up in the Stable Release section.",
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
        .date_naive();
    let modified_formatted = modified_date.format("%Y|%m|%d");

    (semver::Version::parse(version).unwrap(), modified_formatted)
}
