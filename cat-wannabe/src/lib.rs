use std::fs::File;
use std::io::prelude::*;
use std::io;

pub fn concat(file_names: &Vec<String>) -> String {
    let mut file_contents: Vec<String> = Vec::new();

    for file_name in file_names {
        let file_content = match get_file_content(file_name) {
            Ok(content) => content,
            Err(err) => format!("{}: {}", file_name, err),
        };

        file_contents.push(file_content);
    }

    file_contents.join("")
}

pub fn get_file_content(file_name: &String) -> Result<String, io::Error> {
    match File::open(file_name) {
        Ok(mut file) => {
            let mut file_content = String::new();

            file.read_to_string(&mut file_content)?;

            Ok(file_content)
        },
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests;
