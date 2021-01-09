# tool-new-release

This is a tool to help the learning team with the ember.js releases for the 6-week cycle.

At the moment, the following steps of the [learning team process](https://github.com/ember-learn/handbook/blob/master/ember-releases.md) are implemented:

1. - [x] Guides
2. - [x] API documentation
3. - [x] Release blog post
4. - [ ] Release pages
5. - [x] Glitch Ember starter
6. - [ ] Ember Wikipedia

## Prerequisites

- Stable Rust installed (see: https://rustup.rs)
- `heroku-cli` (API docs)

## Install

To run the entire release pipeline, do the following:

- Go to the [releases page](https://github.com/ember-learn/tool-new-release/releases)
- Find the latest release. Should be a draft called "draft".
- Expand the assets and download the relevant one for your platform.
- If you are on macOS you will need to right-click and select "Open" to get around signage limitations

## How to use

For now you are required to specify the version being released, and only minor versions increments are supported.

### Running the entire pipeline

For now you are required to specify the version you intend to release. E.g.:

```bash
tool-new-release --version 3.24.0
```

### Running a specific project

If you need to run the pipeline for one of the projects, you can specify a `--project` (or `-p`) option when invoking the tool.
The possible values are `Guides`, `Api`, and `BlogPost`:

```bash
tool-new-release --version 3.24.0 --project Guides
```

See `tool-new-release --help` for the full list of projects.