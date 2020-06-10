use std::cell::RefCell;
use std::default::Default;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use arcdom::{ArcDom, Node};
use arcdom::NodeData::{Element, Text};
use html5ever::{parse_document};
use html5ever::tendril::TendrilSink;
use markup5ever::{Attribute, LocalName, QualName};



pub(crate) fn create_base_dom(base_html_skeleton: &str) -> ArcDom {
    let payload = read_snippet_from_file(base_html_skeleton);
    // parse given tendril in one go
    let dom = parse_document(ArcDom::default(), Default::default()).one(payload);

    dom
}

pub(crate) fn populate_title(title_string: &str, title_handle: &Arc<Node>) {
    {
        let mut title_content = title_handle.children.borrow_mut();

        title_content.push(Node::new(
            Text { contents: RefCell::new(title_string.parse().unwrap()) }
        ));
    }
}

pub(crate) fn add_article_node(main_handle: &Arc<Node>) {
    {
        let mut main_content = main_handle.children.borrow_mut();
        main_content.push(create_node_with_class_name("article", "post-content"));
    }
}

pub(crate) fn populate_article_header(title: &str, date: &str, main_handle: &Arc<Node>) {
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

pub(crate) fn create_node_with_class_name(node_type: &str, classname: &str) -> Arc<Node> {
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

pub(crate) fn create_link_node(classname: &str, link: &str) -> Arc<Node> {
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

pub(crate) fn create_img_node(classname: &str, src: &str, alt: &str) -> Arc<Node> {
    Node::new(Element {
        name: create_element_qualified_name("img"),
        attrs: RefCell::new(vec![Attribute {
            name: create_class_qualified_name(),
            value: classname.parse().unwrap(),
        },Attribute {
            name: create_src_qualified_name(),
            value: src.parse().unwrap(),
        },Attribute {
            name: create_alt_qualified_name(),
            value: alt.parse().unwrap(),
        }]),
        template_contents: None,
        mathml_annotation_xml_integration_point: false,
    })
}

pub(crate) fn create_node_no_class_name(node_type: &str) -> Arc<Node> {
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

fn create_src_qualified_name() -> QualName {
    QualName::new(
        None,
        ns!(),
        local_name!("src")
    )
}

fn create_alt_qualified_name() -> QualName {
    QualName::new(
        None,
        ns!(),
        local_name!("alt")
    )
}

fn read_snippet_from_file(target: &str) -> String {

    let mut data = String::new();
    let mut f = File::open(target).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");

    data
}