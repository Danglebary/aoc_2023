use std::fs;

use super::types::Result;

pub fn read(path: &str) -> Result<String> {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Error occurred while reading file at path '{path}': {err}"));

    Ok(content)
}

pub fn read_file_lines(path: &str) -> Result<Vec<String>> {
    let content = read(path).unwrap();
    Ok(content.lines().map(|line| line.to_string()).collect())
}

#[allow(dead_code)]
pub fn read_file_split(path: &str, delim: &str) -> Result<Vec<String>> {
    let content = read(path).unwrap();
    Ok(content.split(delim).map(|line| line.to_string()).collect())
}

// pub fn dump_to_file(path: &str, body: String) -> Result<()> {
//     // we need to write the body at the path
//     fs::write(path, body)
//         .expect(&format!("Error occurred while writing to file at path: '{path}'"));

//     Ok(())
// }
