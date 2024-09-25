use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use super::tree::Node;

const FILE_COMPONENT: &[u8] = include_bytes!("../../src-front/components/file.html");
const DIR_COMPONENT: &[u8] = include_bytes!("../../src-front/components/dir.html");

// initializes the icons svg data cache
// starts with js, html and css svgs already cached
async fn icons_cache(dir: &[PathBuf]) -> Result<HashMap<&'static str, String>, CacheErr> {
    if !contains_icon(dir, "html") || !contains_icon(dir, "css") || !contains_icon(dir, "js") {
        return Err(CacheErr::EssentialExtensionsNotFound);
    }

    Ok(HashMap::from([
        ("js", fetch_icon("js")),
        ("html", fetch_icon("html")),
        ("css", fetch_icon("css")),
    ]))
}

// fills the file component template with the instance values and returns the component html
fn file_component(
    name: &str,
    last_modified: std::time::SystemTime,
    icon: &str,
    ftype: &str,
    fsize: u64,
) -> String {
    String::from_utf8(FILE_COMPONENT.to_vec())
        .unwrap()
        .replace("{{{icon}}}", icon)
        .replace("{{{name}}}", name)
        .replace("{{{type}}}", &format!("{:?}", ftype))
        .replace("{{{size}}}", &fsize.to_string())
        .replace("{{{last_modified}}}", &format!("{:?}", last_modified))
}

// wrapper for file component, takes care of parsing templaing values
// returns file component html data
fn get_component(cache: &mut HashMap<&str, String>, dir: &[PathBuf], node: &Node) -> String {
    let value = node.value().unwrap();

    let file = fs::File::open(&value).unwrap();
    let md = file.metadata().unwrap();
    let ftype = match value.is_dir() {
        true => "dir",
        false => match value.extension() {
            None => "_",
            Some(val) => val.to_str().unwrap(),
        },
    };

    let icon = if contains_icon(dir, ftype) {
        load_icon(dir, cache, ftype)
    } else {
        "_"
    };

    file_component(
        value.to_str().unwrap_or("_"),
        md.modified().unwrap(),
        icon,
        ftype,
        file_size(&value),
    )
}

const ICONS_DIR: &str = "resources/images/icons/";

enum CacheErr {
    DirNotFound,
    EssentialExtensionsNotFound,
}

use std::os::unix::fs::MetadataExt;
use std::os::wasi::fs::MetadataExt;
use std::os::windows::fs::MetadataExt;

// returns file size
fn file_size(name: &fs::Metadata) -> u64 {
    if cfg!(unix) {
        name.size()
    } else if cfg!(windows) {
        name.file_size()
    } else if cfg!(wasi) {
        name.size()
    }
}

// returns a result of the local icons dir
fn cache_icons_dir() -> Result<Vec<PathBuf>, CacheErr> {
    let dir = fs::read_dir(ICONS_DIR);
    if dir.is_err() {
        return Err(CacheErr::DirNotFound);
    }
    Ok(dir
        .unwrap()
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap().path())
        .collect::<Vec<PathBuf>>())
}

// checks if local icons dir contains icon name
fn contains_icon(icons_dir: &[PathBuf], ext: &str) -> bool {
    icons_dir.contains(&ext.into())
}

// fetchs icon svg data from local storage
fn fetch_icon(ext: &str) -> String {
    fs::read_to_string(ICONS_DIR.to_string() + ext)
        .unwrap_or(format!("icon for {} type not fount", ext))
}

// loads icon svg, either from cache or from local storage if not already in cache
// appends cache if needed
fn load_icon<'a>(
    dir: &[PathBuf],
    cache: &'a mut HashMap<&'a str, String>,
    ext: &'a str,
) -> &'a str {
    if cache.contains_key(ext) {
        cache.get(ext).unwrap()
    } else {
        if !contains_icon(dir, ext) {
            return "icon for type {} not found";
        }
        let icon = fetch_icon(ext);
        cache.insert(ext, icon);
        cache.get(ext).unwrap()
    }
}
