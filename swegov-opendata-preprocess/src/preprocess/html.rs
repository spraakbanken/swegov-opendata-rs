use std::borrow::Cow;

use html5ever::rcdom::{self, NodeData};
use minidom::Element;
use once_cell::sync::Lazy;
use regex::Regex;
use soup::prelude::*;

use crate::nodeinfo::{dbg_rcdom_node, minidom_text_len, rcdom_text_len};

/// Process the actual text content of the document.
pub fn process_html(elem: &str, textelem: &mut Element, filename: &Cow<'_, str>) {
    // let contents = format!("<text>{}</text>", elem);
    let contentsxml = Soup::new(&elem);
    let body = contentsxml.tag("body").find().unwrap();
    dbg!(dbg_rcdom_node(&body));
    let orig_text_length: usize = body
        .children()
        // .find_all()
        // .filter(|node| node.is_element())
        .map(|node| rcdom_text_len(&node))
        .sum();
    dbg!(&orig_text_length);

    let mut curr_meta_data: Option<(String, String)> = None;

    for (i, child) in body
        .children() /*.filter(|node| node.is_element())*/
        .enumerate()
    {
        println!("process_html: - child[{}]={}", i, dbg_rcdom_node(&child));
        dbg!(i, &textelem);
        // tracing::trace!("- child {}", i);
        tracing::trace!("converting soup to minidom");
        if let Some((name, value)) = curr_meta_data.take() {
            println!("child[{}] looking for metadata value", i);
            if value.is_empty() {
                if let Some(value) = extract_metadata_value(&child) {
                    println!(
                        "child[{}] setting attr: name={:?}, value={:?}",
                        i, name, value
                    );
                    textelem.set_attr(name, value);
                    continue;
                } else {
                    curr_meta_data = Some((name, value));
                }
            }
        }
        let ProcessNodeOutput { nodes, attrs } = process_node(&child);
        for node in nodes {
            // tracing::trace!("appending node {:?}", node);
            println!("process_html: node={:?}", &node);
            if textelem.nodes().len() == 0 {
                if let minidom::Node::Element(elem) = node {
                    println!("child[{}] appending element {:?}", i, elem);
                    textelem.append_child(elem);
                } else {
                    println!("child[{}] skipping text {:?}", i, node);
                }
            } else {
                println!("child[{}] appending node {:?}", i, node);
                textelem.append_node(node);
            }
        }
        for (name, value) in attrs {
            println!("child[{}] attrs: name={:?}, value={:?}", i, name, value);
            if name.is_empty() {
                todo!("handle name empty");
            } else if value.is_empty() {
                println!("child[{}] handle value empty, setting curr_meta_data", i);
                curr_meta_data = Some((name, value));
            } else {
                println!(
                    "child[{}] setting attr: name={:?}, value={:?}",
                    i, name, value
                );
                textelem.set_attr(name, value);
            }
        }
    }

    let text_length: usize = textelem
        .nodes()
        // .find_all()
        // .filter(|node| node.is_element())
        .map(|node| minidom_text_len(&node))
        .sum();
    dbg!(&text_length);
    if text_length < orig_text_length {
        let diff = orig_text_length - text_length;
        tracing::warn!("Contents were lost in {filename} ({diff} chars missing)");
    } else if text_length > orig_text_length {
        tracing::warn!("Contents differ in {filename} (found {text_length} but expected {orig_text_length} chars)");
    }

    // Remove unnecessary whitespace
    for text in textelem.texts_mut() {
        *text = text.trim().replace('\u{00A0}', " ");
        // if element.tail is not None and not element.tail.strip():
        //     element.tail = None
        // if element.text and not element.text.strip():
        //     element.text = None
    }
}

#[derive(Debug, Default)]
pub struct ProcessNodeOutput {
    pub nodes: Vec<minidom::Node>,
    pub attrs: Vec<(String, String)>,
}
fn process_node(node: &rcdom::Handle) -> ProcessNodeOutput {
    dbg!(dbg_rcdom_node(&node));

    static SPECIAL_SPACES: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"[\n\u{00A0}\u{2006}]").expect("whitespace regex"));
    let mut result = ProcessNodeOutput::default();
    match &node.data {
        NodeData::Element {
            name,
            attrs,
            template_contents: _,
            mathml_annotation_xml_integration_point: _,
        } => {
            // println!("{}", dbg_rcdom_node(&node));
            if name.local.as_ref() == "b" {
                println!("{}", dbg_rcdom_node(&node));
                if let Some(attr) = extract_metadata_key(node) {
                    result.attrs.push((attr, String::new()));
                    return result;
                }
            }
            if element_to_strip(name.local.as_ref()) {
                tracing::trace!("skipping element <{}>", name.local);

                return result;
            }
            if name.local.as_ref() == "div"
                && attrs.borrow().iter().any(|attr| {
                    let name = attr.name.local.as_ref();
                    let value = attr.value.as_ref();
                    name == "class" && (value == "brask" || value == "sfstoc")
                })
            {
                tracing::trace!("skipping element <{} class=\"brask\">", name.local);
                println!("skipping element <{} class=\"brask\">", name.local);

                return result;
            }

            let elem_name = if let Some(_attr) = attrs.borrow().iter().find(|attr| {
                attr.name.local.as_ref() == "id" && attr.value.as_ref().starts_with("page_")
            }) {
                "page"
            } else if [
                "title", "h1", "h2", "h3", "h4", "h5", "h6", "li", "tr", "td", "th",
            ]
            .contains(&name.local.as_ref())
            {
                "p"
            } else {
                // tracing::warn!("renaming <{}> to <p>", name.local.as_ref());
                name.local.as_ref()
            };

            if tag_to_strip(elem_name) {
                tracing::trace!("skipping tag <{}>, keeping contents", name.local);
                println!("skipping tag <{}>, keeping contents", name.local);
                for (i, child) in node.children().enumerate() {
                    println!("process_node: striping child[{}] result={:?}", i, result);

                    let ProcessNodeOutput {
                        nodes: child_nodes,
                        attrs: _,
                    } = process_node(&child);
                    let child_nodes = child_nodes.into_iter().map(|mut n| {
                        if let Some(text) = n.as_text_mut() {
                            *text = format!(" {} ", text);
                        }
                        n
                    });
                    result.nodes.extend(child_nodes);
                    // if let Some(child_node) = process_node(&child) {
                    //     elem.append_node(child_node);
                    // }
                }
                return result;
            }
            let mut elem_builder = Element::builder(elem_name, "");

            // Remove attributes from p and remove nested ps (but keep contents)
            if elem_name != "p" {
                for attr in &*attrs.borrow() {
                    let name = attr.name.local.as_ref();
                    let mut value = attr.value.as_ref();
                    if attribute_to_strip(name) {
                        tracing::trace!("skipping attr='{}'", name);
                        continue;
                    }
                    if name == "id" && value.starts_with("page_") {
                        value = &value[5..];
                    }
                    tracing::trace!("attr: {} = '{}'", name, value);
                    elem_builder = elem_builder.attr(name, value);
                }
            }
            for child in node.children() {
                let ProcessNodeOutput {
                    nodes: child_nodes,
                    attrs,
                } = process_node(&child);
                for child_node in child_nodes {
                    match child_node {
                        minidom::Node::Element(child_elem) => {
                            if child_elem.name() == elem_name {
                                tracing::trace!("removing nested tags <{}>", elem_name);
                                for child_elem_node in child_elem.nodes() {
                                    elem_builder = elem_builder.append(child_elem_node.clone());
                                }
                            } else {
                                elem_builder =
                                    elem_builder.append(minidom::Node::Element(child_elem));
                            }
                        }
                        minidom::Node::Text(child_text) => {
                            elem_builder = elem_builder.append(minidom::Node::Text(child_text));
                        }
                    }
                    // elem_builder = elem_builder.append(child_node);
                }
            }
            let elem = elem_builder.build();
            if elem.nodes().count() > 0 {
                result.nodes.push(minidom::Node::Element(elem));
            }
        }
        NodeData::Text { contents } => {
            tracing::trace!("contents = {:?}", contents);
            let text = SPECIAL_SPACES
                .replace_all(contents.borrow().as_ref(), " ")
                .to_string();

            if !text.is_empty() {
                result.nodes.push(minidom::Node::Text(text));
            }
        }
        NodeData::Comment { contents } => {
            tracing::trace!("contents = {:?}", contents);
            // todo!("handle comment");
        }
        _ => todo!("handle {}", dbg_rcdom_node(node)),
    }
    result
}

pub fn extract_metadata_key(node: &rcdom::Handle) -> Option<String> {
    for child in node.children() {
        println!(
            "extract_metadata_key: node.child={}",
            dbg_rcdom_node(&child)
        );
        match &child.data {
            NodeData::Text { contents } => {
                println!("{:?}", contents.borrow().trim());
                if ["Ändringsregister", "Källa"].contains(&contents.borrow().trim()) {
                    return Some(contents.borrow().trim().to_lowercase());
                }
            }
            _ => (),
        }
    }
    None
}

pub fn extract_metadata_value(node: &rcdom::Handle) -> Option<String> {
    println!("extract_metadata_value: node={}", dbg_rcdom_node(&node));
    match &node.data {
        NodeData::Element {
            name,
            attrs,
            template_contents: _,
            mathml_annotation_xml_integration_point: _,
        } => {
            if name.local.as_ref() == "a" {
                if let Some(attr) = attrs
                    .borrow()
                    .iter()
                    .find(|attr| attr.name.local.as_ref() == "href")
                {
                    return Some(attr.value.to_string());
                }
            }
        }
        _ => (),
    }
    for child in node.children() {
        println!(
            "extract_metadata_value: node.child={}",
            dbg_rcdom_node(&child)
        );
        match &child.data {
            NodeData::Text { contents } => {
                println!("{:?}", contents.borrow().trim());
                if contents.borrow().trim() == "Ändringsregister" {
                    return Some(contents.borrow().trim().to_lowercase());
                }
            }
            NodeData::Element {
                name,
                attrs,
                template_contents: _,
                mathml_annotation_xml_integration_point: _,
            } => {
                if name.local.as_ref() == "a" {
                    if let Some(attr) = attrs
                        .borrow()
                        .iter()
                        .find(|attr| attr.name.local.as_ref() == "href")
                    {
                        return Some(attr.value.to_string());
                    }
                }
            }
            _ => (),
        }
    }
    None
}

fn element_to_strip(tag: &str) -> bool {
    [
        "style",
        "STYLE",
        "meta",
        "META",
        "ingenbild",
        "INGENBILD",
        "script",
        "SCRIPT",
        "br",
    ]
    .contains(&tag)
}
fn attribute_to_strip(name: &str) -> bool {
    [
        "style",
        "class",
        "cellpadding",
        "cellspacing",
        "colspan",
        "images",
        ".",
        "align",
        "valign",
        "name",
        "rowspan",
    ]
    .contains(&name)
}

fn tag_to_strip(name: &str) -> bool {
    [
        "a",
        "ahref",
        "aname",
        "astyle",
        "b",
        "brclear",
        "brstyle",
        "bstyle",
        "caption",
        "col",
        "colgroup",
        "comment",
        "date",
        "del",
        "dir",
        // "div",
        "em",
        "font",
        "fontcolor",
        "fontsize",
        "form",
        "hr",
        "i",
        "img",
        "ins",
        "istyle",
        "label",
        "link",
        "metricconverter",
        "metricconverterproductid",
        "nobr",
        "ol",
        "personname",
        "pre",
        "rubrikavvikandemening",
        "s",
        "span",
        "spanclass",
        "spanlang",
        "spanstyle",
        "strong",
        "sub",
        "sup",
        "table",
        "tbody",
        "textovervagande",
        "thead",
        "tt",
        "u",
        "ul",
    ]
    .contains(&name)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::nodeinfo::minidom::asserts::assert_elem_equal;

    use super::*;
    #[test]
    fn soup_test() {
        let soup = Soup::new(r#"<p>hi<p> </p></p>"#);

        let text: Vec<_> = soup
            .tag("p")
            .find()
            .unwrap()
            .children()
            .map(|node| match &node.data {
                NodeData::Element {
                    name,
                    attrs,
                    template_contents,
                    mathml_annotation_xml_integration_point,
                } => format!("<{}>", name.local.as_ref()),
                NodeData::Text { contents } => contents.borrow().to_string(),
                _ => String::new(),
            })
            .collect();

        assert_eq!(
            text,
            vec!["hi".to_string(), "<p>".to_string(), " ".to_string()]
        )
    }

    #[rstest]
    #[case(
        r#"<text xmlns="">hi<p></p></p></text>"#,
        r#"<text xmlns=""><p>hi</p></text>"#
    )]
    #[case(r#"<p>hi<p> </p></p>"#, r#"<text xmlns=""><p>hi<p> </p></p></text>"#)]
    #[case(
        r#"<text xmlns="">hi<p> <p> </p></p></p></text>"#,
        r#"<text xmlns=""><p>hi</p></text>"#
    )]
    #[case(
        r#"<text xmlns="">this is<p> a <p>sentence</p></p></p></text>"#,
        r#"<text xmlns=""><p>this is a sentence</p></text>"#
    )]
    #[case(
        r#"<text xmlns="">
                <b><span>Civilutskottets betänkanden nr 13 år 1971</span>
                </b><b><span>    </span></b><b><span>CU 1971</span></b></p></text>"#,
        r#"<text xmlns=""><p>Civilutskottets betänkanden nr 13 år 1971 CU 1971</p></text>"#
    )]
    #[case(
        r#"<text xmlns=""><div>
                <p>  Civilutskottets betänkanden nr 13 år 1971  </p>
                </div></text>"#,
        r#"<text xmlns=""><p>Civilutskottets betänkanden nr 13 år 1971</p></text>"#
    )]
    fn test_process_html(#[case] given: &str, #[case] expected: &str) {
        let mut elem = Element::bare("text", "");
        let expected: Element = expected.parse().unwrap();

        process_html(given, &mut elem, &Cow::from(""));
        // assert_eq!(
        //     cleaned.is_some(),
        //     expected.is_some(),
        //     "{:?} != {:?}",
        //     cleaned,
        //     expected
        // );
        // if expected.is_some() {
        assert_elem_equal(&elem, &expected);
        // }
    }
}
