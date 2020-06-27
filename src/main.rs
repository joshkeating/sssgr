

// html5 parser
extern crate html5ever;
#[macro_use] extern crate markup5ever;
// reference counted dom lib
extern crate markup5ever_arcdom as arcdom;

use std::{fs, io};
use std::path::Path;
use std::collections::BTreeMap;
use std::borrow::Borrow;

// pull in modules
mod utils;
mod setup;
mod process_article;

fn main() {

    setup::pre_process();

    // read from in/posts
    // need a struct that takes all the metadata from the header
    // add metadata to hashmap that will keep track of all posts
    // parse file line by line, build dom from input
    // when done, serialize to output file

    // process all md files in /in/posts/ adding their metadata to a sorted list (by date)
    // process_article::process_md("./in/posts/predict_airbnb.md");
    // process_article::process_md("./in/posts/average_gpa.md");

    // have two bTrees, one for processing the posts for the index, one for processing the all projects

    let input_dir = "./in/posts/";

    let sorted_metadata = process_all_md(input_dir).expect("Error in processing markdown");

    println!("{}", sorted_metadata.len());

    // iterate sorted_metadata list, building homepage

    // as we process items in sorted_metadata, add them to the list of metadata that needs to be added to the index
}


fn process_all_md(input_dir: &str, ) -> Result<BTreeMap<i32, process_article::Header>, io::Error> {

    let mut sorted_metadata: BTreeMap<i32, process_article::Header> = BTreeMap::new();

    // process all md files in $input_dir
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