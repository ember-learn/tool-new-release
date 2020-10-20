mod projects {
    pub mod api;
    pub mod guides;
}
mod utils;

fn main() {
    let mut dir = tempfile::tempdir().unwrap().into_path();

    crate::projects::guides::deploy_guides(&mut dir).unwrap();
    crate::projects::api::deploy_api_documentation(&mut dir).unwrap();
}
