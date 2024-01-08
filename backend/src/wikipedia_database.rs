use std::fs::File;
use std::io::{BufReader, Cursor};

use rocksdb::{DB};

use quick_xml::events::{Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;

pub fn populate_database(wiki_xml: File, limit: usize) {
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

    loop {
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
                    page_count += 1;
                    db.put(&current_page_title, page_buffer.clone()).unwrap();
                }
                if last_para && page_count >= limit { 

                    break;
                }
            }
            _ => {panic!("Couldn't write stream to vec")}
        }
    }
}
