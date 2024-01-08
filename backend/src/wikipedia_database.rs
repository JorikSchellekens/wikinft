use std::fs::File;
use std::io::{BufReader, Cursor};
use std::process::Command;

use serde::{Serialize, Deserialize};
use meilisearch_sdk::client::*;
use rocksdb::{DB};
use futures::executor::block_on;

use quick_xml::events::{Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;

const MEILISEARCH_URL: &str = "http://localhost:7700";
const MEILISEARCH_API_KEY: &str = "aSampleMasterKey";

#[derive(Serialize, Deserialize, Debug)]
struct WikiPage {
    id: usize,
    title: String,
    body: String,
}

pub fn populate_database(wiki_xml: File, limit: usize) {block_on(async move {
    let buffered_wiki_xml = BufReader::new(wiki_xml);
    let mut reader = Reader::from_reader(buffered_wiki_xml);
    let mut depth = 0;
    let mut in_page = false;
    let mut page_depth = std::isize::MAX;
    let mut in_title = false;
    let mut last_para = false;
    let mut page_count = 0;
    let mut current_page_title = Vec::new();
    let mut event_buffer = Vec::new();
    let mut page_buffer = Vec::new();

    let db = DB::open_default("./wikipedia.rocksdb").unwrap();
    let meilisearch_client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    let wiki_pages = meilisearch_client.index("wiki_pages");
    


    loop {
        println!("{}", page_count);
        // let mut tx = persy.begin().unwrap();
        if last_para { last_para = false }
        event_buffer.clear();
        let event = reader.read_event_into(&mut event_buffer); 
        match event {
           Ok(Event::Start(ref e)) => {
                let name = e.name();
                let name = name.as_ref();
                if !in_page && in_title {
                    panic!("You've lost track of pages and titles");
                }
                if !in_page && name == b"page" {
                    in_page = true;
                    page_buffer.clear();
                    page_depth = depth;
                } else if in_page && depth == page_depth + 1 && name == b"title" {
                    in_title = true;
                } else if in_title {
                    // This is just a sanity check
                    panic!("There shouldn't be any Elements in the title");
                }

                depth += 1;
            }
            Ok(Event::Text(ref e)) => {
                if in_title {
                    current_page_title = e.clone().into_owned().to_vec();
                }
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
                if in_page || last_para {
                    let mut buffer = Vec::new();
                    let cursor = Cursor::new(&mut buffer);
                    let mut writer = Writer::new(cursor);
                    let _ = writer.write_event(el).is_ok();
                    page_buffer.append(&mut buffer);
                }
                // hash replace the page
                if last_para {
                    //db.put(&current_page_title, page_buffer.clone()).unwrap();
                    wiki_pages.add_documents(&[
                        WikiPage {
                            id: page_count,
                            title: String::from_utf8(current_page_title.clone()).unwrap(),
                            body: String::from_utf8(page_buffer.clone()).unwrap()
                        }
                    ], Some("id")).await.unwrap();
                    page_count += 1;
                }
                if last_para && page_count >= limit { 

                    break;
                }
            }
            _ => {panic!("Couldn't write stream to vec")}
        }
    }
})}
