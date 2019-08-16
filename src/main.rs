use skim::{Skim, SkimOptionsBuilder};
use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, Error};

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

fn generate_unicode_table() -> Result<File, Error> {
    let mut out_path = env::temp_dir();
    out_path.push("UnicodeData");

    // If file already exists, assume it's correct.
    if let Ok(file) = File::open(out_path.clone()) {
        Ok(file)
    // Else, generate it
    } else {
        let content = String::from_utf8(include_bytes!("UnicodeData.txt").to_vec()).unwrap();
        let table = content
            .split('\n')
            .flat_map(parse_unicode_data_line)
            .collect::<String>();
        let mut output = File::create(&out_path.clone())?;
        output.write_all(table.as_bytes())?;
        File::open(out_path)
    }
}

fn main() -> Result<(), Error> {
    let options = SkimOptionsBuilder::default()
        .regex(true)
        .reverse(true)
        .height(Some("80%"))
        .build()
        .unwrap();
    let file = generate_unicode_table()?;
    let reader = BufReader::new(file);
    if let Some(output) = Skim::run_with(&options, Some(Box::new(reader))) {
        for item in output.selected_items.iter() {
            println!("{}", item.get_output_text());
        }
    }
    Ok(())
}
