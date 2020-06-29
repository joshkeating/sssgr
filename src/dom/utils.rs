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
use crate::process_article::Header;
use crate::HOMEPAGE_POST_COUNT;

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
        title_content.push(Node::new(Text { contents: RefCell::new(title.parse().unwrap()) }));
    }

    let date_handle = &container.children.borrow()[1];
    {
        let mut date_content = date_handle.children.borrow_mut();
        date_content.push(Node::new(Text { contents: RefCell::new(date.parse().unwrap()) }));
    }
}


pub(crate) fn add_article_card(metadata: &Header, main_handle: &Arc<Node>, index: usize) {
    {
        let mut main_content = main_handle.children.borrow_mut();
        main_content.push(create_node_with_class_name("li", "article-card"));
    }

    let article_link = &main_handle.children.borrow()[index];
    {
        let mut content = article_link.children.borrow_mut();
        content.push(create_link_node("article-card-link", metadata.link.as_str()));
    }

    let article_container = &article_link.children.borrow()[0];
    {
        let mut content = article_container.children.borrow_mut();
        content.push(create_node_with_class_name("article", "article-card-container"));
    }

    let article_content = &article_container.children.borrow()[0];
    {
        let mut content = article_content.children.borrow_mut();
        content.push(create_node_with_class_name("h3", "article-card-title"));
        content.push(create_node_with_class_name("div", "article-card-date"));
        content.push(create_node_with_class_name("p", "article-card-body"));
    }

    let card_title = &article_content.children.borrow()[0];
    {
        let mut content = card_title.children.borrow_mut();
        content.push(create_node_no_class_name("span"));
    }
    let card_title_content = &card_title.children.borrow()[0];
    {
        let mut content = card_title_content.children.borrow_mut();
        content.push(create_text_node(metadata.title.as_str()));
    }

    let card_date_content = &article_content.children.borrow()[1];
    {
        let mut content = card_date_content.children.borrow_mut();
        content.push(create_text_node(metadata.date.as_str()));
    }

    let card_body_content = &article_content.children.borrow()[2];
    {
        let mut content = card_body_content.children.borrow_mut();
        content.push(create_text_node(metadata.summary.as_str()));
    }
}

pub(crate) fn add_see_all(main_handle: &Arc<Node>) {
    {
        let mut main_content = main_handle.children.borrow_mut();
        main_content.push(create_node_with_class_name("div", "see-all"));
    }

    let see_all_link = &main_handle.children.borrow()[HOMEPAGE_POST_COUNT];
    {
        let mut content = see_all_link.children.borrow_mut();
        content.push(create_link_node_no_class_name("all-projects.html"));
    }

    let link_handle = &see_all_link.children.borrow()[0];
    {
        let mut link_content = link_handle.children.borrow_mut();
        link_content.push(Node::new(Text {contents: RefCell::new("See all projects".parse().unwrap())}))
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
        }, Attribute {
            name: create_href_qualified_name(),
            value: link.parse().unwrap(),
        }]),
        template_contents: None,
        mathml_annotation_xml_integration_point: false,
    })
}

pub(crate) fn create_link_node_no_class_name(link: &str) -> Arc<Node> {
    Node::new(Element {
        name: create_element_qualified_name("a"),
        attrs: RefCell::new(vec![Attribute {
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
        }, Attribute {
            name: create_src_qualified_name(),
            value: src.parse().unwrap(),
        }, Attribute {
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

fn create_text_node(value: &str) -> Arc<Node> {
    Node::new(Text { contents: RefCell::new(value.parse().unwrap()) })
}

fn create_element_qualified_name(element_type: &str) -> QualName {
    let loc_name = element_type.to_string();
    QualName::new(
        None,
        ns!(html),
        LocalName::from(loc_name),
    )
}

fn create_class_qualified_name() -> QualName {
    QualName::new(
        None,
        ns!(),
        local_name!("class"),
    )
}

fn create_href_qualified_name() -> QualName {
    QualName::new(
        None,
        ns!(),
        local_name!("href"),
    )
}

fn create_src_qualified_name() -> QualName {
    QualName::new(
        None,
        ns!(),
        local_name!("src"),
    )
}

fn create_alt_qualified_name() -> QualName {
    QualName::new(
        None,
        ns!(),
        local_name!("alt"),
    )
}

fn read_snippet_from_file(target: &str) -> String {
    let mut data = String::new();
    let mut f = File::open(target).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");

    data
}
