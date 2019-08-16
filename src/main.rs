use skim::{Skim, SkimOptionsBuilder};
use std::io::{BufReader, Error};

fn main() -> Result<(), Error> {
    let options = SkimOptionsBuilder::default()
        .regex(true)
        .reverse(true)
        .height(Some("80%"))
        .build()
        .unwrap();
    let reader = BufReader::new(&include_bytes!("UnicodeData")[..]);
    if let Some(output) = Skim::run_with(&options, Some(Box::new(reader))) {
        for item in output.selected_items.iter() {
            println!("{}", item.get_output_text());
        }
    }
    Ok(())
}
