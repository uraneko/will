use std::fs;

fn ls(path: &str, count_bad: usize) -> Result<Vec<String>> {
    let dir = fs::read_dir(path);

    if let Ok(dir_entries) = dir {
        return dir_entries
            .map(|de| {
                de.path().to_str().or_else(|| {
                    count_bad += 1;
                    Some("DirEntryNotFound")
                })
            })
            .map(|de| de.unwrap())
            .filter(|de| de != "DirEntryNotFound")
            .map(|de| de.to_string())
            .collect::<Vec<String>>();
    };

    Err(std::io::Error::Other(
        "ls(): could not convert ReadDir to string vector",
    ))
}
