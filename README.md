# tool-new-release

This is a tool to help the learning team with the ember.js releases for the 6-week cycle.

At the moment, the following steps of the [learning team release process](https://github.com/ember-learn/handbook/blob/master/ember-releases.md) are implemented:

1. - [x] Guides
2. - [x] API documentation
3. - [x] Release blog post
4. - [x] Release pages
5. - [x] Glitch Ember starter
6. - [x] Ember Wikipedia

## Prerequisites

- Stable Rust installed (see: https://rustup.rs)
- `heroku-cli` (API docs)

## Install

To run the entire release pipeline, do the following:

- Go to the [releases page](https://github.com/ember-learn/tool-new-release/releases)
- Find the latest release. Should be a draft called "draft".
- Expand the assets and download the relevant one for your platform.
- On macOS or linux, you need to modify permissions for the file you downloaded: `chmod +x <path to the download>`
- If you are on macOS you will need to right-click and select "Open" to get around signage limitations

## How to use

You can use the commands documented in [learning team release process](https://github.com/ember-learn/handbook/blob/master/ember-releases.md) to run this tool step-by-step.

See `tool-new-release --help` for the full list of commands.

