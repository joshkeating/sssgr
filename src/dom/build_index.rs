use std::cell::RefCell;
use std::default::Default;
use std::path::PathBuf;
use std::fs::File;
use std::io::{self};
use std::io::{BufReader, prelude::*};
use std::collections::BTreeMap;
use arcdom::{ Node, SerializableHandle};
use arcdom::NodeData::{Text};
use html5ever::{serialize};
use regex::Regex;

use crate::dom::utils::*;
use crate::{process_article, HOMEPAGE_POST_COUNT};
use crate::process_article::Header;

pub(crate) fn build_index(mut cards_to_display: Vec<&Header>) -> std::io::Result<()> {

    let dom = create_base_dom("src/snippets/skeleton_flat.html");

    let doc_handle = &dom.document;
    let html_handle = &doc_handle.children.borrow()[1];

    // get head handle
    let head_handle = &html_handle.children.borrow()[0];
    let title_handle = &head_handle.children.borrow()[3];

    // get main handle
    let body_handle = &html_handle.children.borrow()[1];
    let main_handle = &body_handle.children.borrow()[1];

    // set document header title node value from header metadata
    populate_title("Joshua Keating", title_handle);

    // add cards
    for i in 0..HOMEPAGE_POST_COUNT {
        add_article_card(cards_to_display.pop().unwrap(), main_handle, i);
    }

    // add link to post archive
    add_see_all(main_handle);

    // check ability to serialize, write document to new file in output
    let out_path = format!("out/index.html");
    let mut buffer = File::create(out_path)?;
    let document: SerializableHandle = dom.document.clone().into();
    serialize(&mut buffer, &document, Default::default())
        .ok()
        .expect("serialization failed");

    Ok(())
}

