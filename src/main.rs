#![feature(iter_array_chunks)]
use std::fs::File;
use hex::encode;
use clap::{Parser, Subcommand};

mod merkle;

const BANNER: &str =
"
|                              )  (             \n\
|    (  (             )     ( /(  )\\ )   *   )  \n\
|    )\\))(   ' (   ( /( (   )\\())(()/( ` )  /(  \n\
|   ((_)()\\ )  )\\  )\\()))\\ ((_)\\  /(_)) ( )(_)) \n\
|   _(())\\_)()((_)((_)\\((_) _((_)(_))_|(_(_())  \n\
|   \\ \\((_)/ / (_)| |(_)(_)| \\| || |_  |_   _|  \n\
|    \\ \\/\\/ /  | || / / | || .` || __|   | |    \n\
|     \\_/\\_/   |_||_\\_\\ |_||_|\\_||_|     |_|    \
";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Wikipedia XML file path
    #[arg(value_name = "FILE")]
    input_path: String,

    /// The number of wikipedia pages to parse
    #[arg(short, long, value_name = "LIMIT")]
    page_parse_limit: Option<usize>,
}

fn main() {
    
    // A nice visual launch
    println!("{BANNER}");

    let cli = Cli::parse();
    let file_path = cli.input_path;
    let page_parse_limit = cli.page_parse_limit.unwrap_or(std::usize::MAX);

    // Start an XML parser for the file
    let wiki_file: File = File::open(file_path).unwrap();

    let (root, page_count) = merkle::compute_wikipedia_merkle_root(wiki_file, page_parse_limit);
    println!("\nhash of the first {0} pages: \n\n\t 0x{1}", page_count, encode(&root));
}
