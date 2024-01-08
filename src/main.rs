#![feature(iter_array_chunks)]
use std::fs::File;
use clap::{Parser, Subcommand};
use hex::encode;

mod wikipedia_database;
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
|     \\_/\\_/   |_||_\\_\\ |_||_|\\_||_|     |_|    \n
";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a merkle hash of wikipedia
    Hash {
        /// Wikipedia XML file path
        #[arg(value_name = "FILE")]
        input_path: String,

        /// The number of wikipedia pages to parse
        #[arg(short, long, value_name = "LIMIT")]
        page_parse_limit: Option<usize>,
    },

    Database {
        /// Wikipedia XML file path
        #[arg(value_name = "FILE")]
        input_path: String,

        /// The number of wikipedia pages to parse
        #[arg(short, long, value_name = "LIMIT")]
        page_parse_limit: Option<usize>,

//        /// Databse path
//        #[arg(short, long, value_name = "DATABASE_PATH")]
//        database_path: Option<String>,
    }
}

const PAGE_COUNT: usize = 23099380;

fn main() {
    
    // A nice visual launch
    println!("{BANNER}");

    let cli = Cli::parse();

    // Start an XML parser for the file

    match &cli.command {
        Commands::Hash { input_path, page_parse_limit } => {
            println!("Computing the merkle hash of Wikipedia");
            let page_parse_limit = page_parse_limit.unwrap_or(PAGE_COUNT);
            let wiki_file: File = File::open(input_path).unwrap();
            let (root, page_count) = merkle::compute_wikipedia_merkle_root(wiki_file, page_parse_limit);
            println!("\nhash of the first {0} pages: \n\n\t 0x{1}", page_count, encode(&root));

        }
        Commands::Database { input_path, page_parse_limit  } => {
            println!("Transforming XML file to Database");
            let page_parse_limit = page_parse_limit.unwrap_or(PAGE_COUNT);
            let wiki_file: File = File::open(input_path).unwrap();
            wikipedia_database::populate_database(wiki_file, page_parse_limit);
        }
    }
}
