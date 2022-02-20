use clap::{ArgEnum, Parser};
use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};
use std::error::Error;

#[derive(ArgEnum, Clone, Copy, PartialEq, Eq)]
enum Search {
    Regex,
    Exact,
    Fuzzy,
}

#[derive(ArgEnum, Clone, Copy, PartialEq, Eq)]
enum Layout {
    Above,
    Below,
}

/// Find Unicode characters with ease.
///
/// Simply type a description of the character you are looking for. Once you found the character
/// you were after, hit Enter. Selecting multiple characters is also possible: hit tab to select a
/// character and continue browsing.
#[derive(Parser)]
#[clap(name = "fu", version, about)]
struct Options {
    /// Initial query, if any
    initial_query: Option<String>,
    /// Search mode
    #[clap(arg_enum, long, default_value = "regex")]
    search: Search,
    /// Position of fu's window relative to the prompt
    #[clap(arg_enum, long, default_value = "below")]
    layout: Layout,
    /// Height of fu's window relative to the terminal window
    #[clap(long, default_value = "50%")]
    height: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::parse();
    let query = options.initial_query.unwrap_or_default();
    let options = SkimOptionsBuilder::default()
        .query(Some(&query))
        .regex(options.search == Search::Regex)
        .exact(options.search == Search::Exact)
        .reverse(options.layout == Layout::Below)
        .height(Some(&options.height))
        .multi(true)
        .inline_info(true)
        .build()?;
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(&include_bytes!("UnicodeData")[..]);
    Skim::run_with(&options, Some(items))
        .map(|output| output.selected_items)
        .iter()
        .flatten()
        .for_each(|item| {
            println!("{}", item.output());
        });
    Ok(())
}
