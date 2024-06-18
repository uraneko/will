use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub(crate) fn ls(path: &PathBuf) -> Result<Vec<String>, Error> {
    let dir = fs::read_dir(path);

    if let Ok(dir_entries) = dir {
        return Ok(dir_entries
            .filter_map(|de| de.ok())
            .map(|de| de.path().display().to_string())
            .collect::<Vec<String>>());
    };

    Err(std::io::Error::other(
        "ls(): could not convert ReadDir to string vector",
    ))
}

fn dir_entry_template(data: String) -> String {
    format!("<div class=\"DirEntry\">{}</div>\n", data)
}

pub(crate) fn dir_template(data: Result<Vec<String>, Error>) -> String {
    let iter = match data {
        Ok(v) => v.into_iter(),
        Err(e) => {
            return format!(
                "batch_templates(): error, unwrapped read dir handled results on an err value: {:?}",
                e
            );
        }
    };

    iter.fold(String::new(), |acc, de| acc + &dir_entry_template(de))
}

pub(crate) fn svg(name: &PathBuf) -> String {
    let path = format!("resources/icons/{}.svg", name.display());
    let file = fs::read_to_string(&path);
    match file {
        Ok(svg) => svg,
        Err(e) => format!(
            "could not read the svg file from the provided path correctly\n{:?}",
            e
        ),
    }
}

fn batch_svg(paths: Vec<&PathBuf>) -> Vec<String> {
    paths.into_iter().map(|p| svg(p)).collect::<Vec<String>>()
}

pub(crate) fn js(path: &PathBuf) -> Result<String, Error> {
    if path.extension().unwrap().to_str().unwrap() != "js" {
        eprintln!("requested resource is not a proper js file");
    }
    // TODO: handle file errors here

    fs::read_to_string(path)
}

pub(crate) fn css(path: &PathBuf) -> Result<String, Error> {
    if path.extension().unwrap().to_str().unwrap() != "css" {
        eprintln!("requested resource is not a proper css file");
    }

    fs::read_to_string(path)
}

pub(crate) fn html(path: &PathBuf) -> Result<String, Error> {
    if path.extension().unwrap().to_str().unwrap() != "html" {
        eprintln!("requested resource is not a proper html document");
    }

    fs::read_to_string(path)
}
