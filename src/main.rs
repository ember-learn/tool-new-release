mod projects {
    pub mod api;
}
mod utils;

fn main() {
    crate::projects::api::deploy_api_documentation()
        .ok()
        .unwrap();
}
