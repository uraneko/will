use super::ROOT_DIR;
use std::fs;

fn html_template1() -> String {
    fs::read_to_string("src-js/fs_entry.html").unwrap()
}

pub(crate) fn fs_read_dir(path: &str) -> Vec<String> {
    // NOTE: asserting would crash,
    // just return result
    assert!(std::path::PathBuf::from(path).is_dir());
    match fs::read_dir(path) {
        Ok(rd) => rd
            .into_iter()
            .filter_map(|de| de.ok())
            .map(|de| de.path().display().to_string())
            .collect::<Vec<String>>(),
        Err(e) => {
            eprintln!("failed to read dir at: {}\n{:?}", path, e);
            return vec![];
        }
    }
}

// eg. name: dir/.../book.epub, icon epub.svg...
pub(crate) fn fs_entry(path: &str, data: &mut String) -> String {
    // TODO: switch to using metadata for everything
    eprintln!(">>>>>>>>>{}", path);
    let pb = std::path::PathBuf::from(path);
    let extension = pb.is_dir().then_some("dir").unwrap_or_else(|| {
        pb.extension()
            .unwrap_or(std::ffi::OsStr::new("dunno"))
            .to_str()
            .unwrap()
    });
    let root = ROOT_DIR.to_string();
    let icon_path = root.clone() + "/images/" + extension + ".svg";
    let file_path = path;

    eprintln!("\r\npath===>{}", path);

    // TODO: load most used resources after server start and keep in hashmaps
    let icon = match fs::read_to_string(icon_path) {
        Ok(i) => i,
        Err(_) => "dunno.svg goes here".into(),
    };

    let name = pb
        .file_name()
        .unwrap_or(std::ffi::OsStr::new("unavailable"))
        .to_str()
        .unwrap_or("unavailable");

    *data = match fs::read_to_string(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("failed to read file from path {}\n{:?}", path, e);
            "".into()
        }
    };

    use std::os::unix::fs::MetadataExt;

    let last_modified = pb.metadata().unwrap().modified().unwrap();

    let file_size = pb.metadata().unwrap().size().to_string();

    // NOTE: this could be a macro
    html_template1()
        .replace(
            "<div class=\"svg icon\">{icon}</div>",
            &format!("<div class=\"svg icon\">{}</div>", icon),
        )
        .replace(
            "<div class=\"span name\">{name}</div>",
            &format!("<div class=\"span name\">{}</div>", name),
        )
        .replace(
            "<div class=\"span lastModified\">{last_modified}</div>",
            &format!(
                "<div class=\"span lastModified\">{:?}</div>",
                last_modified.elapsed()
            ),
        )
        .replace(
            "<div class=\"span size\">{size}</div>",
            &format!("<div class=\"span size\">{}</div>", &file_size),
        )
}
