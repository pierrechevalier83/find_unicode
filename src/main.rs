use skim::{Skim, SkimOptionsBuilder};
use std::io::{BufReader, Error};
use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(PartialEq)]
    enum Search {
        Regex,
        Exact,
        Fuzzy
    }
}

arg_enum! {
    #[derive(PartialEq)]
    enum Layout {
        Above,
        Below,
    }
}

#[derive(StructOpt)]
#[structopt(
    name = "fu",
    about = "Find Unicode characters with ease. Simply type a description of the character you are looking for."
)]
struct Options {
    #[structopt(
        raw(possible_values = "&Search::variants()", case_insensitive = "true",),
        long = "search",
        help = "Search mode",
        default_value = "Regex"
    )]
    search: Search,
    #[structopt(
        raw(possible_values = "&Layout::variants()", case_insensitive = "true",),
        long = "layout",
        help = "Position of fu's window relative to the prompt",
        default_value = "Below"
    )]
    layout: Layout,
    #[structopt(
        long = "height",
        help = "Height of fu's window relative to the terminal window",
        default_value = "80%"
    )]
    height: String,
}

fn main() -> Result<(), Error> {
    let options = Options::from_args();
    let options = SkimOptionsBuilder::default()
        .regex(options.search == Search::Regex)
        .reverse(options.layout == Layout::Below)
        .height(Some(&options.height))
        .build()
        .unwrap();
    let unicode_data = BufReader::new(&include_bytes!("UnicodeData")[..]);
    if let Some(output) = Skim::run_with(&options, Some(Box::new(unicode_data))) {
        for item in output.selected_items.iter() {
            println!("{}", item.get_output_text());
        }
    }
    Ok(())
}
