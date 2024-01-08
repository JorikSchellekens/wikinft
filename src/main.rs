#![feature(iter_array_chunks)]
use std::fs::File;

use std::env::args;
use hex::encode;

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

fn main() {
    
    // A nice visual launch
    println!("{BANNER}");

    // Basic argument parsing
    let args: Vec<String> = args().collect();
    println!("{}", args[1]);

    // Start an XML parser for the file
    let in_file: File = File::open(args[1].clone()).unwrap();

    let limit = args[2].parse::<usize>().unwrap();

    let (root, page_count) = merkle::compute_wikipedia_merkle_root(in_file, limit);
    println!("\nhash of the first {0} pages: \n\n\t 0x{1}", page_count, encode(&root));
}
