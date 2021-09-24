use crate::utils::prompt::{manual};

pub fn run() {
    manual(
        "Go to https://github.com/ember-learn/ember-blog/pulls.",
    );
    manual(
        "Merge the relevant blog post if all the teams have approved it.",
    );
}
