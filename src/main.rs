use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};
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
    about = "\nFind Unicode characters with ease.\n\nSimply type a description of the character you are looking for. Once you found the character you were after, hit Enter. Selecting multiple characters is also possible: hit tab to select a character and continue browsing."
)]
struct Options {
    #[structopt(help = "Initial query, if any")]
    initial_query: Option<String>,
    #[structopt(
        possible_values = &Search::variants(),
        case_insensitive = true,
        long = "search",
        help = "Search mode",
        default_value = "Regex"
    )]
    search: Search,
    #[structopt(
        possible_values = &Layout::variants(),
        case_insensitive = true,
        long = "layout",
        help = "Position of fu's window relative to the prompt",
        default_value = "Below"
    )]
    layout: Layout,
    #[structopt(
        long = "height",
        help = "Height of fu's window relative to the terminal window",
        default_value = "50%"
    )]
    height: String,
}

fn main() -> Result<(), Error> {
    let options = Options::from_args();
    let query = options.initial_query.unwrap_or(String::new());
    let options = SkimOptionsBuilder::default()
        .query(Some(&query))
        .regex(options.search == Search::Regex)
        .exact(options.search == Search::Exact)
        .reverse(options.layout == Layout::Below)
        .height(Some(&options.height))
        .multi(true)
        .inline_info(true)
        .build()
        .unwrap();
    let unicode_data = BufReader::new(&include_bytes!("UnicodeData")[..]);
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(unicode_data);
    Skim::run_with(&options, Some(items))
        .map(|output| output.selected_items)
        .iter()
        .flatten()
        .for_each(|item| {
            println!("{}", item.output());
        });
    Ok(())
}
