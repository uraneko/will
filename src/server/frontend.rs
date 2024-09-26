use std::collections::HashMap;
use std::path::PathBuf;
// server is an spa so we only have 1 html, css and js files each

use super::response::status;

const SRC_DIR: &str = "resources/frontend/";
const HTML: &str = include_str!("../../resources/frontend/index.html");
const CSS: &str = include_str!("../../resources/frontend/styles.css");
const JS: &str = include_str!("../../resources/frontend/main.js");

pub(crate) fn src_files() -> HashMap<&'static str, &'static str> {
    HashMap::from([("index.html", HTML), ("styles.css", CSS), ("main.js", JS)])
}

// include main menu and options tab icons
const HOME: &str = include_str!("../../resources/images/home.svg");
const UPLOAD: &str = include_str!("../../resources/images/upload.svg");
const THEMES: &str = include_str!("../../resources/images/themes.svg");
const HELP: &str = include_str!("../../resources/images/help.svg");
const LANG: &str = include_str!("../../resources/images/lang.svg");
const CONFIGS2: &str = include_str!("../../resources/images/configs2.svg");
const BONFIRE: &str = include_str!("../../resources/images/bonfire.svg");

pub(crate) fn app_icons() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("home", HOME),
        ("upload", UPLOAD),
        ("themes", THEMES),
        ("help", HELP),
        ("lang", LANG),
        ("configs2", CONFIGS2),
        ("bonfire", BONFIRE),
    ])
}

// src files cache
// page icons cache
// files icons cache
// dirs cache
// response status
pub(crate) fn load_cache() -> (
    HashMap<&'static str, &'static str>,
    HashMap<&'static str, &'static str>,
    HashMap<String, String>,
    HashMap<&'static PathBuf, String>,
    HashMap<&'static str, &'static str>,
) {
    (
        src_files(),
        app_icons(),
        HashMap::new(),
        HashMap::new(),
        status(),
    )
}
