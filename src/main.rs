

// html5 parser
extern crate html5ever;
#[macro_use] extern crate markup5ever;
// reference counted dom lib
extern crate markup5ever_arcdom as arcdom;

use std::{fs, io};
use std::path::Path;
use std::collections::BTreeMap;
use crate::process_article::Header as Header;
use crate::dom::build_index::build_index;
use crate::dom::build_archive::build_archive;

// pull in modules
mod dom;
mod utils;
mod setup;
mod process_article;

const MARKDOWN_LOC: &str = "./in/posts/";
const HOMEPAGE_POST_COUNT: usize = 4;

fn main() {
    println!(":: Setting up...");
    setup::pre_process();
    println!();

    println!(":: Processing all markdown...");
    // process all md files in MARKDOWN_LOC adding their metadata to a sorted list (by date)
    let sorted_metadata: BTreeMap<i32, Header>  = process_all_md(MARKDOWN_LOC)
        .expect("Error in processing markdown");
    println!();

    println!(":: Building index page...");
    // create index page from sorted_metadata
    let meta_to_display: Vec<&Header> = sorted_metadata.values().clone().collect();
    build_index(meta_to_display)
        .expect("Error building index");
    println!();

    println!(":: Building archive page...");
    build_archive(sorted_metadata.values().rev().collect())
        .expect("Error building archive");
    println!();
    println!("Site built successfully!");
}

fn process_all_md(input_dir: &str) -> Result<BTreeMap<i32, Header>, io::Error> {

    let mut sorted_metadata: BTreeMap<i32, Header> = BTreeMap::new();

    // process all md files in input_dir
    for entry in fs::read_dir(Path::new(&input_dir))? {
        let entry = entry?;
        let input_path = entry.path();
        if input_path.is_file() {
            let metadata = process_article::process_md(input_path).expect("Failed to process markdown");

            // only want to index posts that have a date (eg. not the about page)
            if !metadata.date.is_empty() {
                let meta_key = utils::date_str_to_int(metadata.date.as_ref());
                sorted_metadata.insert(meta_key, metadata);
            }
        }
    }
    Ok(sorted_metadata)
}