// html5 parser
extern crate html5ever;
#[macro_use] extern crate markup5ever;
extern crate markup5ever_arcdom as arcdom;

// pull in modules
mod setup;
mod process_article;

fn main() {

    setup::pre_process();

    // read from in/posts
    // need a struct that takes all the metadata from the header
    // add metadata to hashmap that will keep track of all posts
    // parse file line by line, build dom from input
    // when done, serialize to output file

    // process all md files in /in/posts/ adding their metadata to a sorted list

    process_article::process_md("./in/posts/predict_airbnb.md");
    process_article::process_md("./in/posts/average_gpa.md");
}
