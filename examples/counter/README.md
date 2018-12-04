# Rebar Quickstart

To get started with Rebar, clone this repo, and make the following changes:

-Rename your crate in `Cargo.toml` (The `name` field under `[Package]`)

- Replace both occurances of `appname`  (`/appname.js` and `/appname.wasm`) in `index.html` with your
crate's name

- Make the same replacement in either `build.sh`, or `build.ps1`, depending on your
operating system (`.sh` for Linux, `.ps` for Windows). You may delete the other one

You may also wish to replace the contents of this file.