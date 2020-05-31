// html5 parser
extern crate html5ever;
#[macro_use] extern crate markup5ever;
extern crate markup5ever_arcdom as arcdom;

use std::borrow::Borrow;
use std::cell::RefCell;
use std::default::Default;
use std::fs::File;
use std::io::{self};
use std::io::{BufReader, prelude::*};
use std::io::Read;
use std::sync::Arc;

use arcdom::{ArcDom, Handle, Node, NodeData, SerializableHandle};
use arcdom::NodeData::{Element, Text};
use html5ever::{parse_document, serialize};
use html5ever::tendril::TendrilSink;
use markup5ever::{Attribute, LocalName, Namespace, Prefix, QualName};

use regex::Regex;
use std::iter::FromIterator;

// pull in modules
mod setup;

fn main() {

    // setup::pre_process();

    // read from in/posts
    // need a struct that takes all the metadata from the header
    // add metadata to hashmap that will keep track of all posts
    // parse file line by line, build dom from input
    // when done, serialize to output file

    process_md("./in/posts/predict_airbnb_test.md");

    // parse_skelly();

}

struct Header {
    link: String,
    title: String,
    date: String,
    summary: String,
}


fn process_md(file_path: &str) -> io::Result<()> {
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

    println!("{}", header.link);
    println!("{}", header.title);
    println!("{}", header.date);
    println!("{}", header.summary);

    // start dom handler

    let dom = create_base_dom("src/snippets/skeleton_flat.html");

    let doc_handle = &dom.document;
    let html_handle = &doc_handle.children.borrow()[1];

    // get head handle
    let head_handle = &html_handle.children.borrow()[0];
    let title_handle = &head_handle.children.borrow()[3];

    // get main handle
    let body_handle = &html_handle.children.borrow()[1];
    let main_handle = &body_handle.children.borrow()[1];


    populate_title("my title", title_handle);

    populate_article_header("Great Title!", "05/30/2020", main_handle);

    add_article_node(main_handle);

    let article_handle = &main_handle.children.borrow()[1];


    // types of lines to process:
    // Text line, can include inline links or code snippets
    // subheader, prefixed with ##
    // image, prefixed with ![alt text]
    // code block, started with ``` ended with ```

    let mut is_codeblock: bool = false;
    let mut codeblock_content: &str = "";

    let mut lines_processed = 0;

    for line in lines_iter {

        let line_res = line.unwrap();

        if line_res.starts_with("![alt text]") {
            // image

        } else if line_res.starts_with("##") {
            // subheading

        } else if line_res.starts_with("```") {
            // code block

            if is_codeblock {
                // done collecting code, reset values and process codeblock node
                is_codeblock = false;
                codeblock_content = "";

            }
            is_codeblock = true;


        } else {
            if is_codeblock {
                // currently in a codeblock, append entire line to codeblock content
                codeblock_content.to_string().push_str(&line_res);
            }

            else if line_res.trim() != "" {
                // line is normal, process

                {
                    let mut parent_handle = article_handle.children.borrow_mut();
                    parent_handle.push(create_node_no_class_name("p"));
                }

                    let re = Regex::new(r"(?P<code>`(.*?)`)|(?P<link>\[this]\(.+\))|(?P<words>[\w\s,.:']+)").unwrap();

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

                                let child_handle = &article_handle.children.borrow()[lines_processed];
                                {
                                    let mut cur_child_content = child_handle.children.borrow_mut();
                                    cur_child_content.push(create_link_node("std-link", link.unwrap()));
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
    }

    let document: SerializableHandle = dom.document.clone().into();
    serialize(&mut io::stdout(), &document, Default::default())
        .ok()
        .expect("serialization failed");

    Ok(())
}


fn create_base_dom(base_html_skeleton: &str) -> ArcDom {
    let payload = read_snippet_from_file(base_html_skeleton);
    // parse given tendril in one go
    let dom = parse_document(ArcDom::default(), Default::default()).one(payload);

    dom
}

fn populate_title(title_string: &str, title_handle: &Arc<Node>) {
    {
        let mut title_content = title_handle.children.borrow_mut();

        title_content.push(Node::new(
            Text { contents: RefCell::new(title_string.parse().unwrap()) }
        ));
    }
}

fn add_article_node(main_handle: &Arc<Node>) {
    {
        let mut main_content = main_handle.children.borrow_mut();
        main_content.push(create_node_with_class_name("article", "post-content"));
    }
}

fn populate_article_header(title: &str, date: &str, main_handle: &Arc<Node>) {
    {
        let mut main_content = main_handle.children.borrow_mut();
        main_content.push(create_node_with_class_name("div", "post-header-container"));
    }

    let container = &main_handle.children.borrow()[0];
    {
        let mut container_content = container.children.borrow_mut();
        container_content.push(create_node_with_class_name("h1", "post-title"));
        container_content.push(create_node_with_class_name("div", "post-date"));
    }

    let title_handle = &container.children.borrow()[0];
    {
        let mut title_content = title_handle.children.borrow_mut();
        title_content.push(Node::new(Text {contents: RefCell::new(title.parse().unwrap())}));
    }

    let date_handle = &container.children.borrow()[1];
    {
        let mut date_content = date_handle.children.borrow_mut();
        date_content.push(Node::new(Text {contents: RefCell::new(date.parse().unwrap())}));
    }
}

fn create_node_with_class_name(node_type: &str, classname: &str) -> Arc<Node> {
    Node::new(Element {
        name: create_element_qualified_name(node_type),
        attrs: RefCell::new(vec![Attribute {
            name: create_class_qualified_name(),
            value: classname.parse().unwrap(),
        }]),
        template_contents: None,
        mathml_annotation_xml_integration_point: false,
    })
}

fn create_link_node(classname: &str, link: &str) -> Arc<Node> {
    Node::new(Element {
        name: create_element_qualified_name("a"),
        attrs: RefCell::new(vec![Attribute {
            name: create_class_qualified_name(),
            value: classname.parse().unwrap(),
        },Attribute {
            name: create_href_qualified_name(),
            value: link.parse().unwrap(),
        }]),
        template_contents: None,
        mathml_annotation_xml_integration_point: false,
    })
}

fn create_node_no_class_name(node_type: &str) -> Arc<Node> {
    Node::new(Element {
        name: create_element_qualified_name(node_type),
        attrs: RefCell::new(Vec::new()),
        template_contents: None,
        mathml_annotation_xml_integration_point: false,
    })
}

fn create_element_qualified_name(element_type: &str) -> QualName {
    let loc_name = element_type.to_string();
    QualName::new(
        None,
        ns!(html),
        LocalName::from(loc_name)
    )
}

fn create_class_qualified_name() -> QualName {
    QualName::new(
        None,
        ns!(),
        local_name!("class")
    )
}

fn create_href_qualified_name() -> QualName {
    QualName::new(
        None,
        ns!(),
        local_name!("href")
    )
}


// fn parse_skelly() {

    // let dom = create_base_dom("src/snippets/skeleton_flat.html");
    //
    // let doc_handle = &dom.document;
    // let html_handle = &doc_handle.children.borrow()[1];
    //
    // // get head handle
    // let head_handle = &html_handle.children.borrow()[0];
    // let title_handle = &head_handle.children.borrow()[3];
    //
    // // get main handle
    // let body_handle = &html_handle.children.borrow()[1];
    // let main_handle = &body_handle.children.borrow()[1];
    //
    //
    // populate_title("my title", title_handle);
    //
    // populate_article_header("Great Title!", "05/30/2020", main_handle);
    //
    // add_article_node(main_handle);
    //
    // let article_handler = &main_handle.children.borrow()[1];
    // {
    //     let mut tmp = article_handler.children.borrow_mut();
    //     tmp.push(create_node_with_class_name("div", "theoretical-node"));
    // }
    //
    // let child_handle = &article_handler.children.borrow()[0];
    // {
    //     let mut cur_child_content = child_handle.children.borrow_mut();
    //     cur_child_content.push(Node::new(Text {contents: RefCell::new("example content".parse().unwrap())}));
    // }
    //
    //
    // let document: SerializableHandle = dom.document.clone().into();
    // serialize(&mut io::stdout(), &document, Default::default())
    //     .ok()
    //     .expect("serialization failed");
// }




fn read_snippet_from_file(target: &str) -> String {

    let mut data = String::new();
    let mut f = File::open(target).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");

    data
}