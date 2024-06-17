use std::fs;

fn ls(path: &str, count_bad: usize) -> Result<Vec<String>> {
    let dir = fs::read_dir(path);

    if let Ok(dir_entries) = dir {
        return Ok(dir_entries
            .map(|de| {
                de.path().to_str().or_else(|| {
                    count_bad += 1;
                    Some("DirEntryNotFound")
                })
            })
            .map(|de| de.unwrap())
            .filter(|de| de != "DirEntryNotFound")
            .map(|de| de.to_string())
            .collect::<Vec<String>>());
    };

    Err(std::io::Error::Other(
        "ls(): could not convert ReadDir to string vector",
    ))
}

fn generate_template(data: String) -> String {
    format!("<div class=\"DirEntry\">{}</div>\n", data)
}

fn batch_templates(data: Result<Vec<String>>) -> String {
    let mut html = String::new();
    let iter = match data {
        Ok(v) => v.into_iter(),
        Err(e) => {
            return format!(
                "batch_templates(): error, unwrapped read dir handled results on an err value: {:?}",
                e
            );
        }
    };

    iter.fold(String::new(), |acc, de| acc + generate_template(de))
}

fn svg(name: &str) -> String {
    let path = format!("resources/icons/{}.svg", name);
    let file = fs::read_to_string(&path);
    match file {
        Ok(svg) => svg,
        Err(e) => format!(
            "could not read the svg file from the provided path correctly\n{:?}",
            e
        ),
    }
}

fn batch_svg(paths: Vec<&str>) -> Vec<String> {
    paths
        .into_iter()
        .for_each(|p| svg(p))
        .collect::<Vec<String>>()
}
