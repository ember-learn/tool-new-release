use core::panic;
use git2::{Direction, PushOptions, Repository};
use semver::Version;
use std::{
    io::Write,
    path::{Path, PathBuf},
};

use crate::utils::prompt::{automated, manual};

static STATIC_STR: &str = "
    <!-- include the Glitch button to show what the webpage is about and
    to make it easier for folks to view source and remix -->
    <div class=\"glitchButton\" style=\"position:fixed;top:20px;right:20px;\"></div>
    <script src=\"https://button.glitch.me/button.js\"></script>
";

pub fn run(dir: &std::path::Path, opts: &crate::Opts, version: &Version) {
    let version = &format!("v{}", version);

    if !opts.dry_run {
        manual("Cloning Glitch starter app");
        let glitch_repo_url = get_glitch_repo_url();
        let (glitch_repo, glitch_dir) = crate::clone::glitch(dir, &glitch_repo_url);

        manual("Updating Glitch app with content from ember-new-output"
        );
        update_repo_files(&glitch_dir, version);
        update_package_json(glitch_dir.clone());
        update_index_html(glitch_dir.clone());

        // stage modified files
        let mut index = glitch_repo.index().unwrap();
        index
            .add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();

        // commit modified files
        let tree_id = index.write_tree().unwrap();
        let tree = glitch_repo.find_tree(tree_id).unwrap();
        let sig = glitch_repo.signature().unwrap();
        let head_id = glitch_repo.refname_to_id("HEAD").unwrap();
        let parent = glitch_repo.find_commit(head_id).unwrap();
        let _commit_id = glitch_repo
            .commit(
                Some("HEAD"),
                &sig,
                &sig,
                &format!("{} (glitch)", version),
                &tree,
                &[&parent],
            )
            .unwrap();

        automated("Pushing changes to Glitch",
        );
        push_to_glitch(glitch_dir);

        println!("\n");
    }
}

fn push_to_glitch(dir: PathBuf) {
    std::process::Command::new("git")
        .current_dir(&dir)
        .args(&["push"])
        .spawn()
        .unwrap()
        .wait()
        .expect("git status");
}

fn get_glitch_repo_url() -> String {
    let vars = crate::utils::heroku::get_env_vars("ember-glitch-integration");
    let (_, glitch_repo_url) = vars.first().unwrap();

    glitch_repo_url.replace("'", "")
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

#[allow(dead_code)]
fn push_to_git(repo: &Repository) -> std::result::Result<(), git2::Error> {
    let mut cb = git2::RemoteCallbacks::new();
    cb.push_update_reference(|refname, status| {
        println!("-> {}", refname);
        if status.is_some() {
            panic!("Could not push to remote");
        } else {
            Ok(())
        }
    });
    cb.credentials(|_url, username_from_url, _allowed_types| {
        println!("CRED->{:?}", _allowed_types);
        git2::Cred::username(username_from_url.unwrap())
    });
    cb.certificate_check(|_cert, str| {
        println!("CERT {:?}", str);
        true
    });

    let mut opts = PushOptions::new();
    opts.remote_callbacks(cb);

    let mut glitch = repo.find_remote("origin").unwrap();
    glitch.connect(Direction::Push).unwrap();
    glitch.push(&["refs/heads/master"] as &[&str], Some(&mut opts))
}

fn update_repo_files(glitch_dir: &Path, version: &str) {
    let zip_file = download_ember_new(version);
    unpack_ember_new_output(&zip_file, glitch_dir);
}

fn download_ember_new(version: &str) -> PathBuf {
    let mut dir = tempfile::tempdir().unwrap().into_path();
    let zip_url = format!(
        "https://github.com/ember-cli/ember-new-output/archive/{}.zip",
        &version
    );

    std::process::Command::new("wget")
        .current_dir(&dir)
        .args(&[zip_url])
        .spawn()
        .unwrap()
        .wait()
        .expect("git status");

    dir.push(format!("{}.zip", &version));

    dir
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
