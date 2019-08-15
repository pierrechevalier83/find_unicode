use skim::{Skim, SkimOptions};
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), std::io::Error> {
    let options = SkimOptions::default();
    let f = File::open("./assets/UnicodeData.txt")?;
    let reader = BufReader::new(f);
    Skim::run_with(&options, Some(Box::new(reader)));
    Ok(())
}
