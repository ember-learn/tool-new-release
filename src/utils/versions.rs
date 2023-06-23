use semver::Version;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct GuidesVersionsAttributes {
    #[serde(rename = "all-versions")]
    pub all_versions: Vec<String>,
    #[serde(rename = "current-version")]
    pub current_version: String,
    #[serde(rename = "lts-versions")]
    pub lts_versions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct GuidesVersionsData {
    #[serde(rename = "type")]
    data_type: String,
    id: String,
    attributes: GuidesVersionsAttributes,
}

#[derive(Serialize, Deserialize)]
struct GuidesVersions {
    data: GuidesVersionsData,
}
#[derive(Debug)]
pub struct CurrentVersions {
    pub deployed: Version,
    pub target: Version,
}

impl CurrentVersions {
    pub fn from_guides(major_version: bool) -> Self {
        let versions: GuidesVersions =
            reqwest::blocking::get("https://guides.emberjs.com/content/versions.json")
                .expect("Can't connect to Ember Guides' API")
                .json()
                .unwrap();
        let mut prefixed_version = versions.data.attributes.current_version.chars();
        prefixed_version.next();
        let version = prefixed_version.as_str();

        let deployed = semver::Version::parse(version).unwrap();
        let mut target = deployed.clone();

        if major_version {
            target.major += 1;
        } else {
            target.minor += 1;
        }

        Self { deployed, target }
    }

    pub fn from_versions(versions: &Self) -> Self {
        let deployed = versions.deployed.clone();
        let target = versions.target.clone();

        Self { deployed, target }
    }

    // Does not work for M.0.0 versions
    pub fn from_target_version(target: &Version) -> Self {
        if target.minor == 0 {
            panic!("Does not support M.0.0 versions");
        }

        Self {
            deployed: Version::new(target.major, target.minor - 1, 0),
            target: target.clone(),
        }
    }
}
