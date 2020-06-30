use std::default::Default;
use std::fs::File;
use arcdom::{ SerializableHandle};
use html5ever::{serialize};
use crate::dom::utils::*;
use crate::process_article::Header;

pub(crate) fn build_archive(cards_to_display: Vec<&Header>) -> std::io::Result<()> {

    let dom = create_base_dom("src/snippets/skeleton_flat.html");

    let doc_handle = &dom.document;
    let html_handle = &doc_handle.children.borrow()[1];

    // get head handle
    let head_handle = &html_handle.children.borrow()[0];
    let title_handle = &head_handle.children.borrow()[3];

    // get main handle
    let body_handle = &html_handle.children.borrow()[1];
    let main_handle = &body_handle.children.borrow()[1];

    populate_title("Archive", title_handle);

    for (i, card) in cards_to_display.iter().enumerate() {
        add_article_card(card, main_handle, i);
    }
    // for i in 0..HOMEPAGE_POST_COUNT {
    //     add_article_card(cards_to_display.pop().unwrap(), main_handle, i);
    // }

    // check ability to serialize, write document to new file in output
    let out_path = format!("out/archive.html");
    let mut buffer = File::create(out_path)?;
    let document: SerializableHandle = dom.document.clone().into();
    serialize(&mut buffer, &document, Default::default())
        .ok()
        .expect("serialization failed");

    Ok(())
}

