use semver::Version;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use crate::utils::prompt::automated;

static STATIC_STR: &str = "
    <!-- include the Glitch button to show what the webpage is about and
    to make it easier for folks to view source and remix -->
    <div class=\"glitchButton\" style=\"position:fixed;top:20px;right:20px;\"></div>
    <script src=\"https://button.glitch.me/button.js\"></script>
";

pub fn run(dir: &std::path::Path, opts: &crate::Opts, version: &Version) {
    let version = &format!("v{}", version);

    if !opts.dry_run {
        automated("Cloning Glitch starter app");
        let glitch_repo_url = crate::utils::op::glitch::read();
        let (glitch_repo, glitch_dir) = crate::git::clone::clone(dir, glitch_repo_url);

        automated("Updating Glitch app with content from ember-new-output");
        update_repo_files(&glitch_dir, version);
        update_package_json(glitch_dir.clone());
        update_index_html(glitch_dir.clone());

        let index = crate::git::add::add(&glitch_repo);
        crate::git::commit::commit(index, &glitch_repo, version);

        automated("Pushing changes to Glitch");
        crate::glitch::push::push(glitch_dir);

        println!("\n");
    }
}

fn update_package_json(mut path: PathBuf) {
    path.push("package.json");
    let original_content = std::fs::read_to_string(&path).unwrap();
    let modified_content = original_content.replace("ember serve", "ember serve -p 4200");
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(modified_content.as_bytes())
        .expect("Could not update package.json");
    path.pop();
}

fn update_index_html(mut path: PathBuf) {
    path.push("app/index.html");
    let mut content = std::fs::read_to_string(&path).unwrap();
    let i = content.find("  </body>").unwrap();
    content.insert_str(i, STATIC_STR);
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(content.as_bytes())
        .expect("Could not update index.html");
    path.pop();
    path.pop();
}

fn update_repo_files(glitch_dir: &Path, version: &str) {
    let zip_file = download_ember_new(version);
    unpack_ember_new_output(&zip_file, glitch_dir);
}

fn download_ember_new(version: &str) -> PathBuf {
    let mut zip_path = tempfile::tempdir().unwrap().into_path();
    let zip_url = format!(
        "https://github.com/ember-cli/ember-new-output/archive/{}.zip",
        &version
    );

    let response = reqwest::blocking::get(&zip_url).expect("Could not download ember-new-output");
    if !response.status().is_success() {
        eprintln!("Could not find zip file for {}", &version);
        std::process::exit(-1);
    }
    let zip = response.bytes().unwrap();
    zip_path.push(format!("{}.zip", &version));
    File::create(&zip_path).unwrap().write_all(&zip).unwrap();

    zip_path
}

fn unpack_ember_new_output(zip_path: &Path, glitch: &Path) {
    let file = std::fs::File::open(&zip_path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    let mut names = archive
        .file_names()
        .map(|name| name.to_string())
        .collect::<Vec<_>>();
    names.sort();
    let mut names = names.iter();
    let prefix = names.next().unwrap();

    for name in names {
        let mut file = archive.by_name(name).unwrap();
        let file_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let mut output_path = glitch.to_path_buf();
        output_path.push(file_path.strip_prefix(prefix).unwrap());

        if file.is_dir() {
            std::fs::create_dir_all(&output_path).unwrap();
        } else {
            if let Some(p) = output_path.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = std::fs::File::create(&output_path).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&output_path, std::fs::Permissions::from_mode(mode))
                    .unwrap();
            }
        }
    }
}
