use core::panic;
use git2::{Direction, PushOptions, Repository};
use std::{
    io::Write,
    path::{PathBuf},
};

static STATIC_STR: &str = "
    <!-- include the Glitch button to show what the webpage is about and
    to make it easier for folks to view source and remix -->
    <div class=\"glitchButton\" style=\"position:fixed;top:20px;right:20px;\"></div>
    <script src=\"https://button.glitch.me/button.js\"></script>
";

pub fn run(mut dir: &mut std::path::PathBuf, opts: &crate::Opts) {
    let version = &format!("v{}", &opts.version);
    let _ember_new_repo = crate::repo::Repo {
        organization: "ember-cli",
        project: "ember-new-output",
        url: None,
    }
    .clone(&mut dir);
    let mut ember_new_dir = dir.clone();
    ember_new_dir.push("ember-new-output");

    let glitch_repo_url = get_glitch_repo_url();
    let glitch_repo = crate::repo::Repo {
        organization: "ember-learn",
        project: "glitch-emberjs",
        url: Some(&glitch_repo_url),
    }
    .clone(&mut dir);
    let mut glitch_dir = dir.clone();
    glitch_dir.push("glitch-emberjs");

    if !opts.dry_run {
        // check out correct branch on ember-new-output
        std::process::Command::new("git")
            .current_dir(&ember_new_dir)
            .args(&["checkout", version])
            .spawn()
            .unwrap()
            .wait()
            .expect("git status");

        // copy over to glitch repo
        let origin_files: Vec<PathBuf> = rfm::ls(&ember_new_dir).unwrap();
        let origin_files = origin_files
            .iter()
            .filter(|&x| !x.ends_with(".git"))
            .collect::<Vec<&_>>();

        let glitch_to_clean = rfm::ls(&glitch_dir).unwrap();
        let glitch_to_clean = glitch_to_clean
            .iter()
            .filter(|&x| !x.ends_with(".git"))
            .collect::<Vec<&_>>();
        rfm::rm(&glitch_to_clean).unwrap();
        rfm::cp(&origin_files, &glitch_dir).unwrap();

        // update glitch repo
        update_repo_files(&glitch_dir, version);
        update_package_json(&mut glitch_dir);
        update_index_html(&mut glitch_dir);

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

        std::process::Command::new("git")
            .current_dir(&glitch_dir)
            .args(&["push"])
            .spawn()
            .unwrap()
            .wait()
            .expect("git status");

        println!("\n");
    }
}

fn get_glitch_repo_url() -> String {
    let vars = crate::utils::heroku_env_vars("ember-glitch-integration");
    let (_, glitch_repo_url) = vars.first().unwrap();

    glitch_repo_url.replace("'", "")
}

fn update_package_json(path: &mut PathBuf) {
    path.push("package.json");
    let original_content = std::fs::read_to_string(&path).unwrap();
    let modified_content = original_content.replace("ember serve", "ember serve -p 4200");
    let mut file = std::fs::File::create(&path).unwrap();
    file.write(modified_content.as_bytes())
        .expect("Could not update package.json");
    path.pop();
}

fn update_index_html(path: &mut PathBuf) {
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
        if let Some(_) = status {
            panic!("Could not push to remote");
        } else {
            Ok(())
        }
    });
    cb.credentials(|_url, username_from_url, _allowed_types| {
        println!("CRED->{:?}", _allowed_types);
        git2::Cred::username(username_from_url.unwrap())
    });
    cb.certificate_check(|cert, str| {
        println!("CERT {:?}", str);
        true
    });

    let mut opts = PushOptions::new();
    opts.remote_callbacks(cb);

    let mut glitch = repo.find_remote("origin").unwrap();
    glitch.connect(Direction::Push).unwrap();
    glitch.push(&["refs/heads/master"] as &[&str], Some(&mut opts))
}

fn update_repo_files(glitch_dir: &PathBuf, version: &str) {
    let mut zip_file = download_ember_new(version);
    unpack_ember_new_output(&mut zip_file, &glitch_dir);
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

    return dir;
}

fn unpack_ember_new_output<'a>(zip_path: &mut PathBuf, glitch: &PathBuf) {
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
        let mut output_path = glitch.clone();
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
