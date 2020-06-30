use std::cell::RefCell;
use std::default::Default;
use std::path::PathBuf;
use std::fs::File;
use std::io::{self};
use std::io::{BufReader, prelude::*};
use arcdom::{ Node, SerializableHandle};
use arcdom::NodeData::{Text};
use html5ever::{serialize};
use regex::Regex;

use crate::dom::utils::*;

pub struct Header {
    pub link: String,
    pub title: String,
    pub date: String,
    pub summary: String,
}

pub fn process_md(file_path: PathBuf) -> io::Result<Header> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut lines_iter = reader.lines();

    let mut link;
    let mut title;
    let mut date;
    let mut summary;

    if let Some(_link) = lines_iter.next() {
        link = _link.unwrap().trim_start_matches("[//]: # (document_link=\"").to_string();
        link = link.trim_end_matches("\")").to_string();
    } else { link = "parse error".parse().unwrap() }

    if let Some(_title) = lines_iter.next() {
        title = _title.unwrap().trim_start_matches("[//]: # (document_title=\"").to_string();
        title = title.trim_end_matches("\")").to_string();
    } else { title = "parse error".parse().unwrap() }

    if let Some(_date) = lines_iter.next() {
        date = _date.unwrap().trim_start_matches("[//]: # (date=\"").to_string();
        date = date.trim_end_matches("\")").to_string();
    } else { date = "parse error".parse().unwrap() }

    if let Some(_summary) = lines_iter.next() {
        summary = _summary.unwrap().trim_start_matches("[//]: # (summary=\"").to_string();
        summary = summary.trim_end_matches("\")").to_string();
    } else { summary = "parse error".parse().unwrap() }

    let header = Header { link, title, date, summary };

    println!("  -> Header processed successfully");

    // build dom line by line
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
    populate_title(header.title.as_ref(), title_handle);

    // add post title and date to document body using values from header metadata
    populate_article_header(header.title.as_ref(), header.date.as_ref(), main_handle);

    // add node that will contain all visible content from md file
    add_article_node(main_handle);

    let article_handle = &main_handle.children.borrow()[1];

    let mut is_codeblock: bool = false;
    let mut codeblock_content = "".to_string();
    let re = Regex::new(r"(?P<code>`(.*?)`)|(?P<link>\[[\w\s]+]\([\w:.\-/]*\))|(?P<words>[\w\s,.:'\-]+)").unwrap();
    let mut lines_processed = 0;

    for line in lines_iter {

        let line_res = line.unwrap();

        if line_res.starts_with("```") || is_codeblock {

            if line_res.starts_with("```") && !is_codeblock {
                // beginning of codeblock
                is_codeblock = true;
            }
            else if line_res.starts_with("```") && is_codeblock {
                // end of codeblock: add node, reset values, and process

                codeblock_content.pop(); // remove trailing newline char
                {
                    let mut parent_handle = article_handle.children.borrow_mut();
                    parent_handle.push(create_node_with_class_name("pre", "codeblock"));
                }
                {
                    let block_handle = &article_handle.children.borrow()[lines_processed];
                    let mut block_content = block_handle.children.borrow_mut();
                    block_content.push(Node::new(Text {contents: RefCell::new(codeblock_content.parse().unwrap())}))
                }
                lines_processed += 1;

                is_codeblock = false;
                codeblock_content = "".to_string();
                // process node here
            } else {
                // in a codeblock, append entire line to current codeblock content with newline added
                codeblock_content = format!("{}{}\n", codeblock_content, line_res);
            }
        } else if line_res.starts_with("![alt text]") {
            // image

            // get parts we care about for creating image node
            let split_image_link: Vec<&str> = line_res.split(|c| c == '(' || c == '"').collect();
            let file_path = split_image_link[1].trim();
            let alt_text = split_image_link[2].trim();

            {
                let mut parent_handle = article_handle.children.borrow_mut();
                parent_handle.push(create_img_node("img-responsive", file_path, alt_text));
            }
            lines_processed += 1;

        } else if line_res.starts_with("## ") {
            // subheading

            // strip preceding hashes
            let header_text = line_res.trim_start_matches("## ");

            {
                let mut parent_handle = article_handle.children.borrow_mut();
                parent_handle.push(create_node_with_class_name("h2", "post-subheading"));
            }
            {
                let header_handle = &article_handle.children.borrow()[lines_processed];
                let mut header_content = header_handle.children.borrow_mut();
                header_content.push(Node::new(Text {contents: RefCell::new(header_text.parse().unwrap())}))
            }
            lines_processed += 1;

        }  else if line_res.trim() != "" {
            // line is normal, process
            {
                let mut parent_handle = article_handle.children.borrow_mut();
                parent_handle.push(create_node_no_class_name("p"));
            }

            let mut cur_index = 0;
            for caps in re.captures_iter(line_res.as_ref()) {
                if caps.name("code").is_some() {
                    let content = &caps["code"];

                    let child_handle = &article_handle.children.borrow()[lines_processed];
                    {
                        let mut cur_child_content = child_handle.children.borrow_mut();
                        cur_child_content.push(create_node_with_class_name("code", "inlinecode"));
                    }

                    let code_handle = &child_handle.children.borrow()[cur_index];
                    {
                        let mut code_content = code_handle.children.borrow_mut();
                        code_content.push(Node::new(Text {contents: RefCell::new(content.parse().unwrap())}))
                    }
                }
                if caps.name("link").is_some() {
                    let raw_content = &caps["link"];
                    let mut split_link: Vec<&str> = raw_content.split("(").collect();
                    let link = split_link.pop();
                    let id = split_link.pop();
                    if link.is_some() && id.is_some() {
                        let unbracketed_id = id.unwrap().trim_end_matches("]").trim_start_matches("[");
                        let clean_link = link.unwrap().trim_end_matches(")");
                        let child_handle = &article_handle.children.borrow()[lines_processed];
                        {
                            let mut cur_child_content = child_handle.children.borrow_mut();
                            cur_child_content.push(create_link_node("std-link", clean_link));
                        }

                        let link_handle = &child_handle.children.borrow()[cur_index];
                        {
                            let mut link_content = link_handle.children.borrow_mut();
                            link_content.push(Node::new(Text {contents: RefCell::new(unbracketed_id.parse().unwrap())}))
                        }
                    }
                }
                if caps.name("words").is_some() {
                    let content = &caps["words"];
                    let child_handle = &article_handle.children.borrow()[lines_processed];
                    {
                        let mut cur_child_content = child_handle.children.borrow_mut();
                        cur_child_content.push(Node::new(Text {contents: RefCell::new(content.parse().unwrap())}));
                    }
                }
                cur_index += 1;
            }
            lines_processed += 1;
        }
    }

    println!("  -> Finished building dom tree");
    println!("  -> Significant lines of markdown parsed: [{}]", lines_processed);

    // check ability to serialize, write document to new file in output
    let out_path = format!("out/{}", header.link);
    let mut buffer = File::create(out_path)?;
    let document: SerializableHandle = dom.document.clone().into();
    serialize(&mut buffer, &document, Default::default())
        .ok()
        .expect("serialization failed");

    println!("  -> Dom tree serialized successfully");
    println!("==> Wrote to output file: [out/{}]", header.link);

    Ok(header)
}