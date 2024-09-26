use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::tree::Node;

// TODO: in rest
// cache whole dirs when they get fetched to front end

const DIR_COMPONENT: &str = include_str!("../../src-front/components/dir.html");
const SINGLE_COMPONENT: &str = include_str!("../../src-front/components/file.html");

pub(crate) fn dir_component<'a>(
    path: &'a PathBuf,
    nodes: &'a [Box<Node>],
    dirs_cache: &'a mut HashMap<&'a PathBuf, String>,
    icons_cache: &'a mut HashMap<String, String>,
) -> &'a str {
    if dirs_cache.contains_key(&path) {
        return dirs_cache.get(&path).unwrap();
    }

    let dir_component = DIR_COMPONENT.replace(
        "{children}",
        &nodes
            .iter()
            .map(|node| node.value().unwrap())
            .map(|value| get_component(value, icons_cache) + "\r\n")
            .collect::<String>(),
    );

    dirs_cache.insert(path, dir_component);
    dirs_cache.get(&path).unwrap()
}

// fills the file/dir component template with the instance values and returns the component html
fn single_component(
    kind: &str,
    name: &str,
    last_modified: SystemTime,
    icon: &str,
    ftype: &str,
    fsize: u64,
) -> String {
    SINGLE_COMPONENT
        .replace("{kind}", kind)
        .replace("{icon}", icon)
        .replace("{name}", name)
        .replace("{type}", &format!("{:?}", ftype))
        .replace("{size}", &fsize.to_string())
        .replace("{last_modified}", &format!("{}", date(last_modified)))
}

// wrapper for file component, takes care of parsing templaing values
// returns file component html data
fn get_component<'a, 'b, 'c>(
    value: &'a PathBuf,
    icons_cache: &'c mut HashMap<String, String>,
) -> String
where
    'a: 'b,
    'c: 'b,
{
    let file = fs::File::open(&value).unwrap();
    let md = file.metadata().unwrap();
    let ftype = match value.is_dir() {
        true => "dir",
        false => match value.extension() {
            None => "_",
            Some(val) => val.to_str().unwrap(),
        },
    };

    let icon = if contains_icon(icons_cache, ftype) {
        load_icon(icons_cache, ftype)
    } else {
        "_"
    };

    let kind = kind(&value);

    single_component(
        kind,
        value
            .file_name()
            .unwrap_or(std::ffi::OsStr::new("???"))
            .to_str()
            .unwrap_or("???"),
        md.modified().unwrap(),
        icon,
        ftype,
        file_size(&md),
    )
}

fn date(time: SystemTime) -> String {
    let to_secs = time.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let spt = 60_u64.pow(2);
    let minutes = to_secs / 60;
    let hours = to_secs / spt;
    let days = to_secs / spt * 24;
    let months = to_secs / spt * 24 * 30;
    let [months, years_plus] = [months % 12, months / 12];
    let years = to_secs / spt * 24 * 30 * 365;
    let years = years + years_plus;

    format!(
        "{}:{}:{} . {}/{}/{}",
        hours,
        minutes,
        to_secs,
        years + 1970,
        months,
        days
    )
}

fn kind(p: &PathBuf) -> &str {
    if p.is_file() {
        "file"
    } else {
        "dir"
    }
}

use std::os::unix::fs::MetadataExt;
// #[cfg(target_family = "wasm")]
// use std::os::wasi::fs::MetadataExt;
// #[cfg(target_family = "windows")]
// use std::os::windows::fs::MetadataExt;

// returns file size
fn file_size(name: &fs::Metadata) -> u64 {
    // if cfg!(unix) {
    name.size()
    // } else if cfg!(windows) {
    //     name.file_size()
    // } else if cfg!(wasi) {
    //     name.size()
    // } else {
    //     0
    // }
}

#[derive(Debug)]
pub(crate) enum CacheErr {
    DirNotFound,
    EssentialIconsNotFound,
}

const ICONS_DIR: &str = "resources/images/icons/";

// returns a result of the local icons dir
fn scan_icons_dir() -> Result<Vec<PathBuf>, CacheErr> {
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

fn cache_icons_dir(cache: &mut HashMap<String, String>, dir: &[PathBuf]) {
    cache.insert(
        "records".into(),
        dir.into_iter()
            .map(|p| p.to_str())
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .fold(String::new(), |acc, p| acc + ":" + p),
    );
}

// checks if local icons dir contains icon name
fn contains_icon(cache: &HashMap<String, String>, ext: &str) -> bool {
    cache.get("records").unwrap().contains::<&str>(&ext)
}

// fetchs icon svg data from local storage
fn fetch_icon(ext: &str) -> String {
    fs::read_to_string(ICONS_DIR.to_string() + ext)
        .unwrap_or(format!("icon for {} type not fount", ext))
}

// loads icon svg, either from cache or from local storage if not already in cache
// appends cache if needed
fn load_icon<'a>(cache: &'a mut HashMap<String, String>, ext: &str) -> &'a str {
    if cache.contains_key(ext) {
        cache.get(ext).unwrap()
    } else {
        if !contains_icon(cache, ext) {
            return "icon for type {} not found";
        }
        let icon = fetch_icon(ext);
        cache.insert(ext.into(), icon);
        cache.get(ext).unwrap()
    }
}
