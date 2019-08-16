use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::{prelude::*, Error};
use std::path::PathBuf;

fn parse_unicode_data_line(line: &str) -> Option<String> {
    let tokens = line.split(';').collect::<Vec<_>>();
    if !tokens.len() == 15 {
        None
    } else {
        if let Ok(index) = u32::from_str_radix(tokens[0], 16) {
            if let Ok(character) = char::try_from(index) {
                let mut name = tokens[1].to_string();
                if name.to_lowercase().contains("control") {
                    return None;
                } else if name.starts_with("<") {
                    name = name.replace('_', " ");
                    name.retain(|c| c != ',' && c != '<' && c != '>');
                    name = name.to_uppercase()
                }
                Some(format!("{:5} {}\n", character, name.to_lowercase()))
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn generate_unicode_table() -> Result<PathBuf, Error> {
    let mut out_path = env::temp_dir();
    out_path.push("UnicodeData");

    let content = String::from_utf8(include_bytes!("UnicodeData.txt").to_vec()).unwrap();
    let table = content
        .split('\n')
        .flat_map(parse_unicode_data_line)
        .collect::<String>();
    let mut output = File::create(&out_path.clone())?;
    output.write_all(table.as_bytes())?;
    Ok(out_path)
}

fn main() -> Result<(), Error> {
    let path = generate_unicode_table()?;
    println!("Generated data at\n{:#?}", path);
    Ok(())
}
