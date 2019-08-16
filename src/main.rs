use skim::{Skim, SkimOptions};
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
                Some(format!("{:100} {}\n", name.to_lowercase(), character))
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn generate_unicode_table(input: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(input)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;

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
    let mut options = SkimOptions::default();
    options.regex = true;
    let path = generate_unicode_table("./assets/UnicodeData.txt")?;
    let output = File::open(path)?;
    let reader = BufReader::new(output);

    Skim::run_with(&options, Some(Box::new(reader)));
    Ok(())
}
