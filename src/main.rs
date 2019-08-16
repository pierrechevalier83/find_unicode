use skim::{Skim, SkimOptionsBuilder};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{prelude::*, BufReader};

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

fn generate_unicode_table() -> Result<String, std::io::Error> {
    let content = String::from_utf8(include_bytes!("UnicodeData.txt").to_vec()).unwrap();

    let table = content
        .split('\n')
        .flat_map(parse_unicode_data_line)
        .collect::<String>();
    let out_dir = "gen".to_owned();
    std::fs::create_dir_all(&out_dir)?;
    let out_path = out_dir + "/table";
    let mut output = File::create(&out_path)?;
    output.write_all(table.as_bytes())?;
    Ok(out_path)
}

fn main() -> Result<(), std::io::Error> {
    let options = SkimOptionsBuilder::default()
        .regex(true)
        .reverse(true)
        .height(Some("80%"))
        .build()
        .unwrap();
    let path = generate_unicode_table()?;
    let output = File::open(path)?;
    let reader = BufReader::new(output);
    if let Some(output) = Skim::run_with(&options, Some(Box::new(reader))) {
        for item in output.selected_items.iter() {
            println!("{}", item.get_output_text());
        }
    }
    Ok(())
}
