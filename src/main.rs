#![feature(iter_array_chunks)]
use std::fs::File;
use std::io::{BufReader};

use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::env::args;
use std::io::Cursor;
use sha2::{Sha256, Digest};
use hex::encode;

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
    let in_file = BufReader::new(in_file);
    let mut reader = Reader::from_reader(in_file);

    let limit = args[2].parse::<usize>().unwrap();

    let mut depth = 0;
    let mut in_page = false;
    let mut page_depth = std::isize::MAX;
    let mut in_title = false;
    let mut last_para = false;
    let mut pages: Vec<Vec<u8>> = Vec::new();

    let mut buf = Vec::new();
    loop {
        if last_para { last_para = false }
        buf.clear();
        let event = reader.read_event_into(&mut buf); 
        match event {
           Ok(Event::Start(ref e)) => {
                let name = e.name();
                let name = name.as_ref();
                if !in_page && in_title {
                    panic!("You've lost track of pages and titles");
                }
                if !in_page && name == b"page" {
                    in_page = true;
                    pages.push(Vec::new());
                    page_depth = depth;
                } else if in_page && depth == page_depth + 1 && name == b"title" {
                    in_title = true;
                } else if in_title {
                    panic!("There shouldn't be any Elements in the title");
                }
                
                depth += 1;
            }
            Ok(Event::End(ref e)) => {
                let name = e.name();
                let name = name.as_ref();
                depth -= 1;
		        if in_page && depth == page_depth && name == b"page" {
                    in_page = false;
                    page_depth = std::isize::MAX;
                    // quick sanity check
                    if in_title { panic!("Title element not closed properly"); }
                    // println!("Page of 'length' {0} recorded", pages[pages.len() - 1].len());
                    // If we have the limit of pages end the loop
                    last_para = true;
                } else if in_title &&
                          depth == page_depth + 1 &&
                          name == b"title" {
                    in_title = false;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                panic!("Error: {e}");
            } 
            _ => {}
        }
        match event {
            Ok(el) => {
                if in_page || last_para{
                    let len = pages.len();
                    let mut buffer = Vec::new();
                    let cursor = Cursor::new(&mut buffer);
                    let mut writer = Writer::new(cursor);
                    let _ = writer.write_event(el).is_ok();
                    pages[len - 1].append(&mut buffer);
                }
                // hash replace the page
                if last_para {
                    let len = pages.len();
                    let page = &pages[len-1];
                    let mut hasher = Sha256::new();
                    hasher.update(&page);
                    let result_bytes = hasher.finalize().to_vec();
                    // println!("{}", encode(&result_bytes));
                    pages[len-1] = result_bytes;
                }
                if last_para && pages.len() >= limit { 

                    break;
                }
            }
            _ => {panic!("Couldn't write stream to vec")}
        }
    }



    let len = pages.len();
    println!("{len}");
    let iterations = u64::BITS - (len-1).leading_zeros(); // log2(len).ceil()
 
    // Start the hashing process
    // Hashes are grouped in to tuples of 2 such that they can be hashed together.
    // I'm sure this section can be taken off the shelf somehwere, check juno or reth for example
    // Note we drop a lot of pages if it's odd / rip TODO, handle this
    for _ in 1..iterations {
        pages = pages.into_iter().array_chunks::<2>().map(|[h1, h2]| {
            let buffer = [h1, h2].concat();
            let mut hasher = Sha256::new();
            hasher.update(&buffer);
            let result_bytes = hasher.finalize().to_vec();
            result_bytes
        }).collect::<Vec<_>>();
    }
    println!("\nhash of the first {0} pages: \n\n\t 0x{1}", len, encode(&pages[0]));
}