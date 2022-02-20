use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::{prelude::*, Error};
use std::path::PathBuf;

fn generate_font_awesome_table() -> String {
    let content = String::from_utf8(include_bytes!("NerdFontsData.css").to_vec()).unwrap();
    let labels = content
        .split('\n')
        .filter_map(|s| s.split(".nf-").nth(1))
        .map(|s| s.replace(":before {", ""))
        .map(|s| s.replace('-', " "));
    let unicode = content
        .split('\n')
        .filter_map(|s| s.split("content: ").nth(1))
        .map(|s| s.replace("\"\\", ""))
        .map(|s| s.replace("\";", ""));
    labels
        .zip(unicode)
        .map(|(label, unicode)| {
            let character = try_char_from_string_index(&unicode).unwrap();
            format_line(character, &label)
        })
        .collect()
}

fn try_char_from_string_index(s: &str) -> Option<char> {
    let index = u32::from_str_radix(s, 16).ok()?;
    char::try_from(index).ok()
}

fn format_line(character: char, name: &str) -> String {
    format!("{}\t{}\n", character, name.to_lowercase())
}

fn parse_unicode_data_line(line: &str) -> Option<String> {
    let tokens = line.split(';').collect::<Vec<_>>();
    if !tokens.len() == 15 {
        None
    } else if let Some(character) = try_char_from_string_index(tokens[0]) {
        let name = tokens[1].to_string();
        if name.is_empty() || name.starts_with('<') || name.to_lowercase().contains("control") {
            return None;
        }
        Some(format_line(character, &name))
    } else {
        None
    }
}

fn generate_ucd_table() -> String {
    let content = String::from_utf8(include_bytes!("UnicodeData.txt").to_vec()).unwrap();
    content
        .split('\n')
        .flat_map(parse_unicode_data_line)
        .collect()
}

fn generate_unicode_table() -> Result<PathBuf, Error> {
    let mut out_path = env::temp_dir();
    out_path.push("UnicodeData");

    let ucd_table = generate_ucd_table();
    let fa_table = generate_font_awesome_table();
    let mut output = File::create(&out_path.clone())?;
    output.write_all(ucd_table.as_bytes())?;
    output.write_all(fa_table.as_bytes())?;
    Ok(out_path)
}

fn main() -> Result<(), Error> {
    let path = generate_unicode_table()?;
    println!("Generated data at\n{:#?}", path);
    Ok(())
}
