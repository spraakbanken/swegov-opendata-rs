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
    let orig_text_length: usize = body
        .children()
        // .find_all()
        // .filter(|node| node.is_element())
        .map(|node| rcdom_text_len(&node))
        .sum();
    dbg!(&orig_text_length);

    for (i, child) in body.children().enumerate() {
        tracing::trace!("- child {}", i);
        tracing::trace!("converting soup to minidom");
        for node in process_node(&child) {
            tracing::trace!("appending node {:?}", node);
            textelem.append_node(node);
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

fn process_node(node: &rcdom::Handle) -> Vec<minidom::Node> {
    static SPECIAL_SPACES: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"[\n\u{00A0}\u{2006}]").expect("whitespace regex"));
    let mut result = vec![];
    match &node.data {
        NodeData::Element {
            name,
            attrs,
            template_contents: _,
            mathml_annotation_xml_integration_point: _,
        } => {
            if element_to_strip(name.local.as_ref()) {
                tracing::trace!("skipping element <{}>", name.local);

                return result;
            }
            if name.local.as_ref() == "div"
                && attrs.borrow().iter().any(|attr| {
                    attr.name.local.as_ref() == "class" && attr.value.as_ref() == "brask"
                })
            {
                tracing::trace!("skipping element <{} class=\"brask\">", name.local);

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
                for child in node.children() {
                    let child_nodes = process_node(&child);
                    let child_nodes = child_nodes.into_iter().map(|mut n| {
                        if let Some(text) = n.as_text_mut() {
                            *text = format!(" {} ", text);
                        }
                        n
                    });
                    result.extend(child_nodes);
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
                for child_node in process_node(&child) {
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
                result.push(minidom::Node::Element(elem));
            }
        }
        NodeData::Text { contents } => {
            tracing::trace!("contents = {:?}", contents);
            let text = SPECIAL_SPACES
                .replace_all(contents.borrow().as_ref(), " ")
                .to_string();

            if !text.is_empty() {
                result.push(minidom::Node::Text(text));
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
        "table",
        "thead",
        "tbody",
        "form",
        "caption",
        "a",
        "link",
        "span",
        "em",
        "strong",
        "sub",
        "sup",
        "b",
        "i",
        "u",
        "nobr",
        "ul",
        "ol",
        "colgroup",
        "col",
        "tt",
        "dir",
        "del",
        "ins",
        "s",
        "label",
        "pre",
        "spanstyle",
        "metricconverterproductid",
        "spanclass",
        "bstyle",
        "istyle",
        "brclear",
        "brstyle",
        "comment",
        "img",
        "hr",
        "fontsize",
        "aname",
        "metricconverter",
        "astyle",
        "personname",
        "spanlang",
        "date",
        "font",
        "fontcolor",
        "ahref",
        "textovervagande",
        "rubrikavvikandemening",
        "div",
    ]
    .contains(&name)
}
